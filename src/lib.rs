// Copyright 2018 The Exonum Team
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//   http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

// FIXME: Sometimes clippy incorrectly calculates lifetimes.
#![cfg_attr(feature="cargo-clippy", allow(let_and_return))]

#[macro_use]
extern crate display_derive;
#[macro_use]
extern crate enum_primitive_derive;
#[macro_use]
extern crate exonum;
#[macro_use]
extern crate serde_json;

extern crate bodyparser;
extern crate byteorder;
extern crate chrono;
extern crate exonum_time;
extern crate iron;
extern crate num_traits;
extern crate rand;
extern crate router;
extern crate serde;

/// Unique service identifier
pub const CRYPTOOWLS_SERVICE_ID: u16 = 521;
/// Unique service name which will be used in API and configuration
pub const CRYPTOOWLS_SERVICE_NAME: &str = "cryptoowls";

/// Sum to be issued each time
pub const ISSUE_AMOUNT: u64 = 100;

/// Timeout (seconds) before user will be able to issue funds again
pub const ISSUE_TIMEOUT: i64 = 60;

/// Timeout (seconds) before user will be able to breed owl again
pub const BREEDING_TIMEOUT: i64 = 60;

/// Breeding price
pub const BREEDING_PRICE: u64 = 42;

/// Data structures stored in blockchain
mod data_layout {

    use chrono::{DateTime, Utc};
    use exonum::crypto::{Hash, PublicKey};

    encoding_struct! {
        /// CryptoOwl. Unique identifier of the owl is a hash of this data structure.
        struct CryptoOwl {
            /// Name (should be unique)
            name: &str,
            /// DNA
            dna: u32,
        }
    }

    encoding_struct! {
        /// Current owl state
        struct CryptoOwlState {
            /// Owl
            owl: CryptoOwl,
            /// Owner
            owner: &PublicKey,
            /// Time of the last breeding
            last_breeding: DateTime<Utc>,
        }
    }

    encoding_struct! {
        /// User
        struct User {
            /// Public key
            public_key: &PublicKey,
            /// Name
            name: &str,
            /// Current balance
            balance: u64,
            /// Time of the last issue of funds
            last_fillup: DateTime<Utc>,
        }
    }

    encoding_struct! {
        /// Order to buy an owl
        struct Order {
            /// Order author
            public_key: &PublicKey,
            /// Owl identifier
            owl_id: &Hash,
            /// pending, accepted or declined
            status: &str,
            /// Price
            price: u64,
        }
    }
}

/// Database schema
pub mod schema {
    use exonum::storage::{Fork, ListIndex, ProofMapIndex, Snapshot, ValueSetIndex};
    use exonum::crypto::{Hash, PublicKey};

    use data_layout::{CryptoOwlState, Order, User};

    pub struct CryptoOwlsSchema<T> {
        view: T,
    }

    /// Read-only tables
    impl<T> CryptoOwlsSchema<T>
    where
        T: AsRef<Snapshot>,
    {
        pub fn new(view: T) -> Self {
            CryptoOwlsSchema { view }
        }
        /// Users
        pub fn users(&self) -> ProofMapIndex<&T, PublicKey, User> {
            ProofMapIndex::new("cryptoowls.users", &self.view)
        }
        /// Owls and their states (see data_layout::CryptoOwlState)
        pub fn owls_state(&self) -> ProofMapIndex<&T, Hash, CryptoOwlState> {
            ProofMapIndex::new("cryptoowls.owls_state", &self.view)
        }
        /// Owl orders
        pub fn orders(&self) -> ProofMapIndex<&T, Hash, Order> {
            ProofMapIndex::new("cryptoowls.orders", &self.view)
        }
        /// Helper table for linking user and his owls
        pub fn user_owls(&self, public_key: &PublicKey) -> ValueSetIndex<&T, Hash> {
            ValueSetIndex::new_in_family("cryptoowls.user_owls", public_key, &self.view)
        }
        /// Helper table for linking user and his orders
        pub fn user_orders(&self, public_key: &PublicKey) -> ListIndex<&T, Hash> {
            ListIndex::new_in_family("cryptoowls.user_orders", public_key, &self.view)
        }
        /// Helper table for linking owl and her orders
        pub fn owl_orders(&self, owl_id: &Hash) -> ListIndex<&T, Hash> {
            ListIndex::new_in_family("cryptoowls.owl_orders", owl_id, &self.view)
        }

        /// Method to get state hash. Depends on `users`, `owls_state` and `orders` tables.
        pub fn state_hash(&self) -> Vec<Hash> {
            vec![
                self.users().merkle_root(),
                self.orders().merkle_root(),
                self.owls_state().merkle_root(),
            ]
        }
    }

    /// Mutable accessors for all our tables
    impl<'a> CryptoOwlsSchema<&'a mut Fork> {
        pub fn users_mut(&mut self) -> ProofMapIndex<&mut Fork, PublicKey, User> {
            ProofMapIndex::new("cryptoowls.users", self.view)
        }

        pub fn owls_state_mut(&mut self) -> ProofMapIndex<&mut Fork, Hash, CryptoOwlState> {
            ProofMapIndex::new("cryptoowls.owls_state", self.view)
        }

        pub fn orders_mut(&mut self) -> ProofMapIndex<&mut Fork, Hash, Order> {
            ProofMapIndex::new("cryptoowls.orders", self.view)
        }

        pub fn user_owls_mut(&mut self, public_key: &PublicKey) -> ValueSetIndex<&mut Fork, Hash> {
            ValueSetIndex::new_in_family("cryptoowls.user_owls", public_key, self.view)
        }

        pub fn user_orders_mut(&mut self, public_key: &PublicKey) -> ListIndex<&mut Fork, Hash> {
            ListIndex::new_in_family("cryptoowls.user_orders", public_key, self.view)
        }

        pub fn owl_orders_mut(&mut self, owl_id: &Hash) -> ListIndex<&mut Fork, Hash> {
            ListIndex::new_in_family("cryptoowls.owl_orders", owl_id, self.view)
        }
    }
}

/// Module with description of all transactions
pub mod transactions {
    use byteorder::{BigEndian, ReadBytesExt};
    use chrono::{DateTime, Utc};
    use rand::{IsaacRng, Rng, SeedableRng};
    use rand::distributions::{Sample, Weighted, WeightedChoice};
    use num_traits::ToPrimitive;

    use exonum::crypto::{CryptoHash, Hash, PublicKey};
    use exonum::storage::{Fork, Snapshot};
    use exonum::blockchain::{ExecutionError, ExecutionResult, Schema, Transaction};
    use exonum::messages::Message;
    use exonum_time::TimeSchema;

    use std::io::Cursor;

    use data_layout::{CryptoOwl, CryptoOwlState, Order, User};
    use schema;
    use schema::CryptoOwlsSchema;

    use {BREEDING_PRICE, BREEDING_TIMEOUT, CRYPTOOWLS_SERVICE_ID, ISSUE_AMOUNT, ISSUE_TIMEOUT};

    transactions! {
        pub Transactions {
            const SERVICE_ID = CRYPTOOWLS_SERVICE_ID;

            /// Transaction to create a new user
            struct CreateUser {
                /// Public user identifier
                public_key: &PublicKey,
                /// Name
                name: &str,
            }

            /// Transaction to area an owl. A new random owl created if mother and father
            /// are not defined (have zero identifiers).
            struct MakeOwl {
                /// Public user identifier
                public_key: &PublicKey,
                /// Owl name
                name: &str,
                /// Father identifier
                father_id: &Hash,
                /// Mother identifier
                mother_id: &Hash,
                /// Timestamp. Is required to breed owls with the same identifiers.
                seed: DateTime<Utc>,
            }

            /// Transaction to issue funds
            struct Issue {
                /// Public user identifier
                public_key: &PublicKey,
                /// Timestamp. Is required to create transactions owls with the same fields.
                seed: DateTime<Utc>,
            }

            /// Transaction to make a new order
            struct CreateOrder
            {
                /// Public user identifier
                public_key: &PublicKey,
                /// Owl identifier
                owl_id: &Hash,
                /// Price
                price: u64,
                /// Timestamp. Is required to create transactions owls with the same fields.
                seed: DateTime<Utc>,
            }

            /// Transaction to accept order (and sell owl)
            struct AcceptOrder
            {
                /// Public user identifier
                public_key: &PublicKey,
                /// Order identifier
                order_id: &Hash,
            }
        }
    }

    impl Transaction for CreateUser {
        fn verify(&self) -> bool {
            self.verify_signature(self.public_key())
        }

        fn execute(&self, fork: &mut Fork) -> ExecutionResult {
            let ts = {
                let time_schema = TimeSchema::new(&fork);
                time_schema.time().get().unwrap()
            };

            let state_hash = {
                let info_schema = Schema::new(&fork);
                info_schema.state_hash_aggregator().merkle_root()
            };

            let key = self.public_key();
            let mut schema = schema::CryptoOwlsSchema::new(fork);

            // Ignore if the user with the same public identifier is already exists
            if schema.users().get(key).is_none() {
                let user = User::new(&key, self.name(), ISSUE_AMOUNT, ts);
                schema.users_mut().put(key, user);

                // New user get 2 random owls
                let starter_pack = vec![
                    schema.make_uniq_owl((1, 0), &format!("{}'s Adam", self.name()), &state_hash),
                    schema.make_uniq_owl(
                        (1, 100_042),
                        &format!("{}'s Eve", self.name()),
                        &key.hash(),
                    ),
                ];
                schema.refresh_owls(key, starter_pack, ts);
            }
            Ok(())
        }
    }

    impl Transaction for MakeOwl {
        fn verify(&self) -> bool {
            self.verify_signature(self.public_key())
        }

        fn execute(&self, fork: &mut Fork) -> ExecutionResult {
            let ts = {
                let time_schema = TimeSchema::new(&fork);
                time_schema.time().get().unwrap()
            };

            let state_hash = {
                let info_schema = Schema::new(&fork);
                info_schema.state_hash_aggregator().merkle_root()
            };

            let mut schema = schema::CryptoOwlsSchema::new(fork);

            // Find mother and father
            // If someone is missed will get None response
            let parents = [self.mother_id(), self.father_id()]
                .iter()
                .map(|&i| schema.owls_state().get(&i))
                .collect::<Option<Vec<CryptoOwlState>>>();

            let user = schema.users().get(self.public_key()).unwrap();
            let key = user.public_key();

            // Ignore transaction if mother of father is not found
            if let Some(parents) = parents {
                // Check if user is owl owner
                if parents.iter().any(|ref p| p.owner() != key) {
                    return Err(ErrorKind::AccessViolation.into());
                }

                let (mother, father) = (parents[0].owl(), parents[1].owl());
                // Can not use the same owl as mother and father at the same time
                if mother == father {
                    return Err(ErrorKind::SelfBreeding.into());
                }

                // User has enough funds for breeding
                if user.balance() < BREEDING_PRICE {
                    return Err(ErrorKind::InsufficientFunds.into());
                }

                // Check last breeding time for each owl
                if parents
                    .iter()
                    .any(|ref p| (ts - p.last_breeding()).num_seconds() < BREEDING_TIMEOUT)
                {
                    return Err(ErrorKind::EarlyBreeding.into());
                }

                // All conditions are fulfilled, start breeding
                let son =
                    schema.make_uniq_owl((father.dna(), mother.dna()), self.name(), &state_hash);
                let owls_to_update = vec![son, mother, father];
                schema.refresh_owls(&key, owls_to_update, ts);

                let user = User::new(&key, user.name(), user.balance() - BREEDING_PRICE, ts);
                schema.users_mut().put(&key, user);
            }

            Ok(())
        }
    }

    impl Transaction for Issue {
        fn verify(&self) -> bool {
            self.verify_signature(self.public_key())
        }

        fn execute(&self, fork: &mut Fork) -> ExecutionResult {
            let ts = {
                let time_schema = TimeSchema::new(&fork);
                time_schema.time().get().unwrap()
            };

            let mut schema = schema::CryptoOwlsSchema::new(fork);
            let key = self.public_key();
            let user = schema.users().get(key).unwrap();

            if (ts - user.last_fillup()).num_seconds() >= ISSUE_TIMEOUT {
                schema.set_user_balance(&key, user.balance() + ISSUE_AMOUNT, Some(ts));
                Ok(())
            } else {
                // Issue timeout is not expired
                Err(ErrorKind::EarlyIssue.into())
            }
        }
    }

    impl Transaction for CreateOrder {
        fn verify(&self) -> bool {
            self.verify_signature(self.public_key())
        }

        fn execute(&self, fork: &mut Fork) -> ExecutionResult {
            let mut schema = schema::CryptoOwlsSchema::new(fork);
            let key = self.public_key();
            let user = schema.users().get(&key).unwrap();

            // Execute code if the owl is found
            if let Some(owl) = schema.owls_state().get(self.owl_id()) {
                // Check if buyer is not owl owner and he has enough funds
                if owl.owner() != key && self.price() <= user.balance() {
                    let order = Order::new(&key, self.owl_id(), "pending", self.price());
                    let order_hash = order.hash();
                    schema.orders_mut().put(&order.hash(), order);
                    schema.user_orders_mut(&key).push(order_hash);
                    schema.owl_orders_mut(&self.owl_id()).push(order_hash);
                }
            }

            Ok(())
        }
    }

    impl Transaction for AcceptOrder {
        fn verify(&self) -> bool {
            self.verify_signature(self.public_key())
        }

        fn execute(&self, fork: &mut Fork) -> ExecutionResult {
            let mut schema = schema::CryptoOwlsSchema::new(fork);
            if let Some(accepted_order) = schema.accept_order(self.public_key(), self.order_id()) {
                let owl_state = schema.owls_state().get(accepted_order.owl_id()).unwrap();

                schema.refresh_owls(
                    accepted_order.public_key(),
                    vec![owl_state.owl()],
                    owl_state.last_breeding(),
                );

                schema
                    .user_owls_mut(self.public_key())
                    .remove(accepted_order.owl_id());
            }
            Ok(())
        }
    }

    /// Read-only tables
    impl<T> CryptoOwlsSchema<T>
    where
        T: AsRef<Snapshot>,
    {
        // Method to generate new unique owl
        pub fn make_uniq_owl(&self, genes: (u32, u32), name: &str, hash_seed: &Hash) -> CryptoOwl {
            // Hash is a byte array [u8; 32]. To seed random number generator an array
            // of 32-bit numbers &[u32] is required. So we use `std::io::Cursor` and build
            // a new u32 number of each 4 bytes.

            let hash_seed: &[u8] = hash_seed.as_ref();
            let mut seed = [0u32; 4];
            let mut cursor = Cursor::new(hash_seed);
            for i in 0..4 {
                seed[i] = cursor.read_u32::<BigEndian>().unwrap();
            }
            let mut rng = IsaacRng::from_seed(&seed);

            // Create a unique owl using infinite loop. Call `break` if resulted owl is unique.
            loop {
                let mut son_dna = 0u32;
                // Checking every bit in parent DNAs
                for i in 0..32 {
                    // Step by all `genes` and set them in accordance with parents genes
                    let mask = 2u32.pow(i);
                    let (fg, mg) = (genes.0 & mask, genes.1 & mask);
                    if fg == mg {
                        // With a probability of 8/10 the child bits will be equal to parents bits
                        // in the case if parents bits are equal.
                        let mut possible_genes = vec![
                            Weighted {
                                weight: 8,
                                item: fg,
                            },
                            Weighted {
                                weight: 2,
                                item: fg ^ mask,
                            },
                        ];

                        let mut choices = WeightedChoice::new(&mut possible_genes);
                        son_dna |= choices.sample(&mut rng);
                    } else if rng.gen() {
                        // If bits are different, the resulting bit will be selected
                        // with probability 1/2.
                        son_dna |= mask;
                    }
                }

                // Create a new owls with given DNA.
                // Break out of the loop if the resulted owl is unique.
                // Otherwise, try again.
                let newborn = CryptoOwl::new(name, son_dna);
                if self.owls_state().get(&newborn.hash()).is_none() {
                    break newborn;
                }
            }
        }
    }

    /// Mutable accessors for all our tables
    impl<'a> CryptoOwlsSchema<&'a mut Fork> {
        /// Helper method to update owl state after breed or create
        pub fn refresh_owls(
            &mut self,
            owner_key: &PublicKey,
            owls: Vec<CryptoOwl>,
            ts: DateTime<Utc>,
        ) {
            for owl in owls {
                self.user_owls_mut(owner_key).insert(owl.hash());
                self.owls_state_mut()
                    .put(&owl.hash(), CryptoOwlState::new(owl, owner_key, ts));
            }
        }

        /// Helper method to change user balance
        pub fn set_user_balance(
            &mut self,
            public_key: &PublicKey,
            balance: u64,
            last_fillup: Option<DateTime<Utc>>,
        ) {
            if let Some(user) = self.users().get(public_key) {
                let last_fillup = last_fillup.unwrap_or(user.last_fillup());
                let new_user = User::new(public_key, user.name(), balance, last_fillup);
                self.users_mut().put(public_key, new_user)
            }
        }

        /// Helper method to accept order
        /// Function will check that buyer has enough funds, order of status allows to accept order.
        /// Then function will update order, buyer and seller balances.
        /// Finally function will mark all other orders as declined.
        pub fn accept_order(&mut self, acceptor_key: &PublicKey, order_id: &Hash) -> Option<Order> {
            if let Some(order) = self.orders().get(order_id) {
                let buyer = self.users().get(order.public_key()).unwrap();
                let seller = self.users().get(acceptor_key).unwrap();

                if order.status() == "pending" {
                    if buyer.balance() >= order.price()
                        && self.user_owls(acceptor_key).contains(order.owl_id())
                    {
                        let new_order = Order::new(
                            order.public_key(),
                            order.owl_id(),
                            "accepted",
                            order.price(),
                        );

                        self.orders_mut().put(order_id, new_order.clone());

                        self.set_user_balance(
                            seller.public_key(),
                            seller.balance() + order.price(),
                            None,
                        );
                        self.set_user_balance(
                            buyer.public_key(),
                            buyer.balance() - order.price(),
                            None,
                        );

                        // Decline all other owl orders
                        let order_ids: Vec<Hash> = {
                            let idx = self.owl_orders(order.owl_id());
                            let order_ids = idx.iter().collect();
                            order_ids
                        };

                        for order_id in order_ids {
                            self.decline_order(&order_id);
                        }
                        return Some(new_order);
                    }
                    self.decline_order(order_id);
                }
            }
            None
        }

        /// Helper method to decline order. It is used only inside this module, so it is private.
        pub fn decline_order(&mut self, order_id: &Hash) {
            if let Some(order) = self.orders().get(order_id) {
                if order.status() == "pending" {
                    let new_order = Order::new(
                        order.public_key(),
                        order.owl_id(),
                        "declined",
                        order.price(),
                    );

                    self.orders_mut().put(order_id, new_order);
                }
            }
        }
    }

    #[derive(Display, Primitive)]
    pub enum ErrorKind {
        #[display(fmt = "Too early for breeding.")]
        EarlyBreeding = 1,
        #[display(fmt = "Too early for balance refill.")]
        EarlyIssue = 2,
        #[display(fmt = "Insufficient funds.")]
        InsufficientFunds = 3,
        #[display(fmt = "Not your property.")]
        AccessViolation = 4,
        #[display(fmt = "Perversion.")]
        SelfBreeding = 5,
    }

    impl ErrorKind {
        /// Converts error to the raw code
        pub fn as_code(self) -> u8 {
            self.to_u8().unwrap()
        }
    }

    impl From<ErrorKind> for ExecutionError {
        fn from(e: ErrorKind) -> ExecutionError {
            let err_txt = format!("{}", e);
            ExecutionError::with_description(e.as_code(), err_txt)
        }
    }

}

/// Module with API implementation
mod api {
    use bodyparser;
    use iron::prelude::*;

    use router::Router;

    use exonum::api::{Api, ApiError};
    use exonum::crypto::{Hash, PublicKey};

    use exonum::node::{ApiSender, TransactionSend};
    use exonum::blockchain::{Blockchain, Transaction};

    use schema;
    use data_layout::{CryptoOwlState, Order, User};
    use transactions::Transactions;

    #[derive(Clone)]
    pub struct CryptoOwlsApi {
        pub channel: ApiSender,
        pub blockchain: Blockchain,
    }

    impl Api for CryptoOwlsApi {
        fn wire(&self, router: &mut Router) {
            let self_ = self.clone();
            let get_user = move |req: &mut Request| {
                let public_key: PublicKey = self_.url_fragment(req, "pub_key")?;
                if let Some(user) = self_.get_user(&public_key) {
                    self_.ok_response(&json!(user))
                } else {
                    self_.not_found_response(&json!("User not found"))
                }
            };

            let self_ = self.clone();
            let get_users = move |_: &mut Request| {
                let users = self_.get_users();
                self_.ok_response(&json!(&users))
            };

            let self_ = self.clone();
            let get_users_orders = move |req: &mut Request| {
                let public_key: PublicKey = self_.url_fragment(req, "pub_key")?;
                if let Some(orders) = self_.get_users_orders(&public_key) {
                    self_.ok_response(&json!(orders))
                } else {
                    self_.not_found_response(&json!("User not found"))
                }
            };

            let self_ = self.clone();
            let get_owl = move |req: &mut Request| {
                let owl_hash = self_.url_fragment(req, "owl_hash")?;
                if let Some(owl) = self_.get_owl(&owl_hash) {
                    self_.ok_response(&json!(owl))
                } else {
                    self_.not_found_response(&json!("Owl not found"))
                }
            };

            let self_ = self.clone();
            let get_owls = move |_: &mut Request| {
                let owls = self_.get_owls();
                self_.ok_response(&json!(&owls))
            };

            let self_ = self.clone();
            let get_owls_orders = move |req: &mut Request| {
                let owl_hash = self_.url_fragment(req, "owl_hash")?;
                if let Some(orders) = self_.get_owls_orders(&owl_hash) {
                    self_.ok_response(&json!(orders))
                } else {
                    self_.not_found_response(&json!("Owl not found"))
                }
            };

            let self_ = self.clone();
            let get_user_owls = move |req: &mut Request| {
                let public_key: PublicKey = self_.url_fragment(req, "pub_key")?;
                if let Some(orders) = self_.get_user_owls(&public_key) {
                    self_.ok_response(&json!(orders))
                } else {
                    self_.not_found_response(&json!("User not found"))
                }
            };

            let self_ = self.clone();
            let transaction =
                move |req: &mut Request| match req.get::<bodyparser::Struct<Transactions>>() {
                    Ok(Some(transaction)) => {
                        let tx_hash = self_.post_transaction(transaction).map_err(ApiError::from)?;
                        let json = json!({ "tx_hash": tx_hash });
                        self_.ok_response(&json)
                    }
                    Ok(None) => Err(ApiError::BadRequest("Empty request body".into()))?,
                    Err(e) => Err(ApiError::BadRequest(e.to_string()))?,
                };

            // View-only handlers
            router.get("/v1/users", get_users, "get_users");
            router.get("/v1/user/:pub_key", get_user, "get_user");

            router.get(
                "/v1/user/:pub_key/orders",
                get_users_orders,
                "get_users_orders",
            );

            router.get("/v1/user/:pub_key/owls", get_user_owls, "get_user_owls");

            router.get("/v1/owl/:owl_hash", get_owl, "get_owl");
            router.get("/v1/owls", get_owls, "get_owls");

            router.get(
                "/v1/owl/:owl_hash/orders",
                get_owls_orders,
                "get_owls_orders",
            );

            // Transactions
            router.post("/v1/transaction", transaction, "post_transaction");
        }
    }

    impl CryptoOwlsApi {
        /// User profile
        fn get_user(&self, public_key: &PublicKey) -> Option<User> {
            let snapshot = self.blockchain.snapshot();
            let schema = schema::CryptoOwlsSchema::new(snapshot);
            schema.users().get(public_key)
        }

        /// All users
        fn get_users(&self) -> Vec<User> {
            let snapshot = self.blockchain.snapshot();
            let schema = schema::CryptoOwlsSchema::new(snapshot);
            let idx = schema.users();
            let users: Vec<User> = idx.values().collect();
            users
        }

        /// Owl profile
        fn get_owl(&self, owl_id: &Hash) -> Option<CryptoOwlState> {
            let snapshot = self.blockchain.snapshot();
            let schema = schema::CryptoOwlsSchema::new(snapshot);
            schema.owls_state().get(&owl_id)
        }

        /// All owls
        fn get_owls(&self) -> Vec<CryptoOwlState> {
            let snapshot = self.blockchain.snapshot();
            let schema = schema::CryptoOwlsSchema::new(snapshot);
            let idx = schema.owls_state();
            let owls: Vec<CryptoOwlState> = idx.values().collect();
            owls
        }

        /// User owls list
        fn get_user_owls(&self, public_key: &PublicKey) -> Option<Vec<CryptoOwlState>> {
            let snapshot = self.blockchain.snapshot();
            let schema = schema::CryptoOwlsSchema::new(snapshot);

            schema.users().get(&public_key).and({
                let idx = schema.user_owls(&public_key);
                // Attention, iterator type is ValueSetIndexIter<'_, Hash> !!!
                let owls = idx.iter()
                    .map(|h| schema.owls_state().get(&h.1))
                    .collect::<Option<Vec<CryptoOwlState>>>()
                    .or(Some(vec![]));
                owls
            })
        }

        /// Owl orders
        fn get_owls_orders(&self, owl_id: &Hash) -> Option<Vec<Order>> {
            let snapshot = self.blockchain.snapshot();
            let schema = schema::CryptoOwlsSchema::new(snapshot);

            schema.owls_state().get(owl_id).and({
                let idx = schema.owl_orders(owl_id);
                let orders = idx.iter()
                    .map(|h| schema.orders().get(&h))
                    .collect::<Option<Vec<Order>>>()
                    .or(Some(vec![]));
                orders
            })
        }

        /// Orders made by user
        fn get_users_orders(&self, users_key: &PublicKey) -> Option<Vec<Order>> {
            let snapshot = self.blockchain.snapshot();
            let schema = schema::CryptoOwlsSchema::new(snapshot);

            schema.users().get(users_key).and({
                let idx = schema.user_orders(users_key);
                let orders = idx.iter()
                    .map(|h| schema.orders().get(&h))
                    .collect::<Option<Vec<Order>>>()
                    .or(Some(vec![]));
                orders
            })
        }

        fn post_transaction(&self, transaction: Transactions) -> Result<Hash, ApiError> {
            let transaction: Box<Transaction> = transaction.into();
            let tx_hash = transaction.hash();
            self.channel.send(transaction)?;
            Ok(tx_hash)
        }
    }
}

/// Collect everything together
pub mod service {
    use iron::Handler;
    use router::Router;

    use exonum::api::Api;
    use exonum::crypto::Hash;
    use exonum::encoding;
    use exonum::storage::Snapshot;
    use exonum::blockchain::{ApiContext, Service, Transaction, TransactionSet};
    use exonum::helpers::fabric::{Context, ServiceFactory};
    use exonum::messages::RawTransaction;

    use api::CryptoOwlsApi;
    use schema::CryptoOwlsSchema;
    use transactions::Transactions;

    use CRYPTOOWLS_SERVICE_ID;

    pub struct CryptoOwlsService;

    impl CryptoOwlsService {
        pub fn new() -> Self {
            CryptoOwlsService {}
        }
    }

    pub struct CryptoOwlsServiceFactory;

    impl ServiceFactory for CryptoOwlsServiceFactory {
        fn make_service(&mut self, _: &Context) -> Box<Service> {
            Box::new(CryptoOwlsService::new())
        }
    }

    impl Service for CryptoOwlsService {
        fn service_name(&self) -> &'static str {
            "cryptoowls"
        }

        fn service_id(&self) -> u16 {
            CRYPTOOWLS_SERVICE_ID
        }

        // Method to deserialize transacitons
        fn tx_from_raw(&self, raw: RawTransaction) -> Result<Box<Transaction>, encoding::Error> {
            let tx = Transactions::tx_from_raw(raw)?;
            Ok(tx.into())
        }

        // Tables hashes to be included into blockchain state hash
        fn state_hash(&self, snapshot: &Snapshot) -> Vec<Hash> {
            let schema = CryptoOwlsSchema::new(snapshot);
            schema.state_hash()
        }

        // Handling requests to a node
        fn public_api_handler(&self, ctx: &ApiContext) -> Option<Box<Handler>> {
            let mut router = Router::new();
            let api = CryptoOwlsApi {
                channel: ctx.node_channel().clone(),
                blockchain: ctx.blockchain().clone(),
            };
            api.wire(&mut router);
            Some(Box::new(router))
        }
    }
}

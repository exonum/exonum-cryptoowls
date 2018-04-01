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
extern crate exonum_time;
extern crate iron;
extern crate num_traits;
extern crate rand;
extern crate router;
extern crate serde;

/// Unique service identifier.
pub const CRYPTOOWLS_SERVICE_ID: u16 = 521;
/// Unique service name, which will be used in API and configuration.
pub const CRYPTOOWLS_SERVICE_NAME: &str = "cryptoowls";

/// Amount of FIXME.
pub const ISSUE_AMMOUNT: u64 = 100;

/// Timeout before issuing more money.
pub const ISSUE_TIMEOUT: u64 = 60;

/// Timeout before breeding.
pub const BREEDING_TIMEOUT: u64 = 60;

/// Breeding price.
pub const BREEDING_PRICE: u64 = 42;

/// Data structures stored in blockchain.
mod data_layout {

    use std::time::SystemTime;
    use exonum::crypto::{Hash, PublicKey};

    encoding_struct! {
        /// CryptoOwl. Its unique identifier is the hash of this structure.
        struct CryptoOwl {
            /// Owl's name.
            name: &str,
            /// Owl's DNA.
            dna: u32,
        }
    }

    encoding_struct! {
        /// State of the cryptoowl.
        struct CryptoOwlState {
            /// Owl itself.
            owl: CryptoOwl,
            /// Owner.
            owner: &PublicKey,
            /// Time of last breeding.
            last_breeding: SystemTime,
        }
    }

    encoding_struct! {
        /// Data about user and his owls.
        struct User {
            /// His public key.
            public_key: &PublicKey,
            /// His name.
            name: &str,
            /// His balance.
            balance: u64,
            /// Time of the last fillup.
            last_fillup: SystemTime,
        }
    }

    encoding_struct! {
        /// Order for buying owl.
        struct Order {
            /// Who made order.
            public_key: &PublicKey,
            /// Owl's identifier.
            owl_id: &Hash,
            /// Status. Could be `pending`, `accepted` or `declined`.
            status: &str,
            /// Price for the owl.
            price: u64,
        }
    }

    impl Order {
        pub fn to_accepted(&self) -> Self {
            Order::new(self.public_key(), self.owl_id(), "accepted", self.price())
        }

        pub fn to_declined(&self) -> Self {
            Order::new(self.public_key(), self.owl_id(), "declined", self.price())
        }
    }
}

/// Database schema.
pub mod schema {
    use byteorder::{BigEndian, ReadBytesExt};
    use rand::{IsaacRng, Rng, SeedableRng};
    use rand::distributions::{Sample, Weighted, WeightedChoice};

    use exonum::storage::{Fork, ListIndex, ProofMapIndex, Snapshot, ValueSetIndex};
    use exonum::blockchain::gen_prefix;
    use exonum::crypto::{CryptoHash, Hash, PublicKey};

    use std::time::SystemTime;
    use std::io::Cursor;

    use data_layout::{CryptoOwl, CryptoOwlState, Order, User};

    pub struct CryptoOwlsSchema<T> {
        view: T,
    }

    /// Read-only tables.
    impl<T> CryptoOwlsSchema<T>
    where
        T: AsRef<Snapshot>,
    {
        pub fn new(view: T) -> Self {
            CryptoOwlsSchema { view }
        }

        /// Table for all users.
        pub fn users(&self) -> ProofMapIndex<&T, PublicKey, User> {
            ProofMapIndex::new("cryptoowls.users", &self.view)
        }

        /// Table for states of the owls.
        pub fn owls_state(&self) -> ProofMapIndex<&T, Hash, CryptoOwlState> {
            ProofMapIndex::new("cryptoowls.owls_state", &self.view)
        }

        /// Table for orders.
        pub fn orders(&self) -> ProofMapIndex<&T, Hash, Order> {
            ProofMapIndex::new("cryptoowls.orders", &self.view)
        }

        /// Helper table for connecting users and their owls.
        pub fn user_owls(&self, public_key: &PublicKey) -> ValueSetIndex<&T, Hash> {
            ValueSetIndex::with_prefix("cryptoowls.user_owls", gen_prefix(public_key), &self.view)
        }

        /// Helper table for connecting users and orders made by them.
        pub fn user_orders(&self, public_key: &PublicKey) -> ListIndex<&T, Hash> {
            ListIndex::with_prefix("cryptoowls.user_orders", gen_prefix(public_key), &self.view)
        }

        /// Helper table for connecting owls and orders for them.
        pub fn owl_orders(&self, owl_id: &Hash) -> ListIndex<&T, Hash> {
            ListIndex::with_prefix("cryptoowls.owl_orders", gen_prefix(owl_id), &self.view)
        }

        /// State only depends on three first tables.
        pub fn state_hash(&self) -> Vec<Hash> {
            vec![
                self.users().root_hash(),
                self.orders().root_hash(),
                self.owls_state().root_hash(),
            ]
        }

        /// Generates new unique owl.
        pub fn make_uniq_owl(&self, genes: (u32, u32), name: &str, hash_seed: &Hash) -> CryptoOwl {
            // Hash is byte array of 32 elements, while we need slice of `u32` to seed random generator.
            // We use `std::io::Cursor` and convert every four bytes of hash to the `u32` number.

            let hash_seed: &[u8] = hash_seed.as_ref();
            let mut seed = [0u32; 4];
            let mut cursor = Cursor::new(hash_seed);
            for i in 0..4 {
                seed[i] = cursor.read_u32::<BigEndian>().unwrap();
            }
            let mut rng = IsaacRng::from_seed(&seed);

            // Creating unique owl using infinite loop (we're calling `break` if created owl is unique).
            loop {
                let mut son_dna = 0u32;
                // Checking every bit in parent DNAs.
                for i in 0..32 {
                    let mask = 2u32.pow(i);
                    let (fg, mg) = (genes.0 & mask, genes.1 & mask);
                    if fg == mg {
                        // If parent bits are equal, resulting bit would be the same with 80% chance.
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
                    } else {
                        // If parent bits aren't equal, resulting bit would be fully random (with 50/50 chance).
                        if rng.gen() {
                            son_dna |= mask;
                        }
                    }
                }

                // Creating new owl with specific name and DNA and break out of loop if it's unique.
                // Otherwise trying again.
                let newborn = CryptoOwl::new(name, son_dna);
                if self.owls_state().get(&newborn.hash()).is_none() {
                    break newborn;
                }
            }
        }
    }

    /// Mutable accessors for all our tables.
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
            ValueSetIndex::with_prefix("cryptoowls.user_owls", gen_prefix(public_key), self.view)
        }

        pub fn user_orders_mut(&mut self, public_key: &PublicKey) -> ListIndex<&mut Fork, Hash> {
            ListIndex::with_prefix("cryptoowls.user_orders", gen_prefix(public_key), self.view)
        }

        pub fn owl_orders_mut(&mut self, owl_id: &Hash) -> ListIndex<&mut Fork, Hash> {
            ListIndex::with_prefix("cryptoowls.owl_orders", gen_prefix(owl_id), self.view)
        }

        /// Helper to update states of the owls after creating.
        pub fn put_owls_into_state(
            &mut self,
            owner_key: &PublicKey,
            owls: Vec<CryptoOwl>,
            time: SystemTime,
        ) {
            for owl in owls {
                self.user_owls_mut(owner_key).insert(owl.hash());
                self.owls_state_mut().put(
                    &owl.hash(),
                    CryptoOwlState::new(owl, owner_key, time),
                );
            }
        }

        /// Helper for changing balance of the user.
        pub fn set_user_balance(
            &mut self,
            public_key: &PublicKey,
            balance: u64,
            last_fillup: Option<SystemTime>,
        ) {
            if let Some(user) = self.users().get(public_key) {
                let last_fillup = last_fillup.unwrap_or(user.last_fillup());
                let new_user = User::new(public_key, user.name(), balance, last_fillup);
                self.users_mut().put(public_key, new_user)
            }
        }
    }
}

/// Модуль с описанием транзакций для демки.
pub mod transactions {
    use exonum::crypto::{CryptoHash, Hash, PublicKey};
    use exonum::blockchain::{ExecutionError, ExecutionResult, Schema, Transaction};
    use exonum::storage::Fork;
    use exonum::messages::Message;
    use num_traits::ToPrimitive;

    use schema;
    use data_layout::{CryptoOwlState, Order, User};
    use exonum_time::TimeSchema;

    use std::time::SystemTime;

    use {BREEDING_PRICE, BREEDING_TIMEOUT, CRYPTOOWLS_SERVICE_ID, ISSUE_AMMOUNT, ISSUE_TIMEOUT};

    transactions! {
        pub Transactions {
            const SERVICE_ID = CRYPTOOWLS_SERVICE_ID;

            /// Транзакция создания пользователя
            struct CreateUser {
                /// Публичный идентификатор пользователя
                public_key: &PublicKey,
                /// Имя
                name: &str,
            }

            /// Транзакция создания совы. Если идентификаторы отца и матери это нули,
            /// то выводится базовая сова
            struct MakeOwl {
                /// Публичный идентификатор пользователя
                public_key: &PublicKey,
                /// Имя совенка
                name: &str,
                /// Идентификатор отца
                father_id: &Hash,
                /// Идентификатор матери
                mother_id: &Hash,
                /// Необходимо для того, чтоб создавать транзакции с одинаковыми полями.
                seed: SystemTime,
            }

            /// Транзакция запроса новых средств
            struct Issue {
                /// Публичный идентификатор пользователя
                public_key: &PublicKey,
                /// Необходимо для того, чтоб создавать транзакции с одинаковыми полями.
                seed: SystemTime,
            }

            /// Транзакция размещения нового предложения
            struct CreateOrder
            {
                /// Публичный идентификатор пользователя
                public_key: &PublicKey,
                /// Идентификатор совы
                owl_id: &Hash,
                /// Желаемая цена
                price: u64,
                /// Необходимо для того, чтоб создавать транзакции с одинаковыми полями.
                seed: SystemTime,
            }

            /// Принятие предложения на покупку
            struct AcceptOrder
            {
                /// Публичный идентификатор пользователя
                public_key: &PublicKey,
                /// Идентификатор предложения
                order_id: &Hash,
            }
        }
    }

    impl Transaction for CreateUser {
        fn verify(&self) -> bool {
            self.verify_signature(self.public_key())
        }

        fn execute(&self, fork: &mut Fork) -> ExecutionResult {
            let current_time = {
                let time_schema = TimeSchema::new(&fork);
                time_schema.time().get().unwrap()
            };

            let state_hash = {
                let info_schema = Schema::new(&fork);
                info_schema.state_hash_aggregator().root_hash()
            };

            let key = self.public_key();
            let mut schema = schema::CryptoOwlsSchema::new(fork);

            // Если пользователь с таким ключём уже существует - игнорируем.
            if schema.users().get(key).is_none() {
                let user = User::new(&key, self.name(), ISSUE_AMMOUNT, current_time);
                schema.users_mut().put(key, user);

                // Новый пользователь получает `в подарок` 2 примитивных совы со случайным геномом.
                let starter_pack = vec![
                    schema.make_uniq_owl(
                        (1u32, 0u32),
                        &format!("{}'s Adam", self.name()),
                        &state_hash
                    ),
                    schema.make_uniq_owl(
                        (1u32, 100042u32),
                        &format!("{}'s Eve", self.name()),
                        &key.hash()
                    ),
                ];

                schema.put_owls_into_state(key, starter_pack, current_time);

                Ok(())
            } else {
                Err(Error::UserAlreadyExists)?
            }
        }
    }

    impl Transaction for MakeOwl {
        fn verify(&self) -> bool {
            self.verify_signature(self.public_key())
        }

        fn execute(&self, fork: &mut Fork) -> ExecutionResult {
            let current_time = {
                let time_schema = TimeSchema::new(&fork);
                time_schema.time().get().unwrap()
            };

            let state_hash = {
                let info_schema = Schema::new(&fork);
                info_schema.state_hash_aggregator().root_hash()
            };

            let mut schema = schema::CryptoOwlsSchema::new(fork);

            // Найдём обоих родителей.
            // Если хотя бы одного из них нет мы получим None
            let parents = [self.mother_id(), self.father_id()]
                .iter()
                .map(|&i| schema.owls_state().get(&i))
                .collect::<Option<Vec<CryptoOwlState>>>();

            let user = schema.users().get(self.public_key()).ok_or(Error::UserNotFound)?;
            let key = user.public_key();

            // Если нам удалось найти родителей идём дальше
            // если нет - игнорируем транзакцию
            if let Some(parents) = parents {
                // Проверяем наши права на сов
                if parents.iter().any(|ref p| p.owner() != key) {
                    Err(Error::AccessViolation)?
                }

                let (mother, father) = (parents[0].owl(), parents[1].owl());
                // Для скрещевания необходимы 2 родителя
                if mother == father {
                    Err(Error::SelfBreeding)?
                }

                // Достаточно ли средств для разведения?
                if user.balance() < BREEDING_PRICE {
                    Err(Error::InsufficientFunds)?
                }

                // Проверяем время последнего спаривания для каждой совы
                if parents.iter().any(|ref p| {
                    current_time.duration_since(p.last_breeding()).unwrap().as_secs() < BREEDING_TIMEOUT
                })
                {
                    Err(Error::EarlyBreeding)?
                }

                // Все условия выполнены, можем размножаться
                let son =
                    schema.make_uniq_owl((father.dna(), mother.dna()), self.name(), &state_hash);
                let owls_to_update = vec![son, mother, father];
                schema.put_owls_into_state(&key, owls_to_update, current_time);

                schema.set_user_balance(&key, user.balance() - BREEDING_PRICE, None);
            }

            Ok(())
        }
    }

    impl Transaction for Issue {
        fn verify(&self) -> bool {
            self.verify_signature(self.public_key())
        }

        fn execute(&self, fork: &mut Fork) -> ExecutionResult {
            let current_time = {
                let time_schema = TimeSchema::new(&fork);
                time_schema.time().get().unwrap()
            };

            let mut schema = schema::CryptoOwlsSchema::new(fork);
            let key = self.public_key();
            let user = schema.users().get(key).ok_or(Error::UserNotFound)?;

            if current_time.duration_since(user.last_fillup()).unwrap().as_secs() >= ISSUE_TIMEOUT {
                schema.set_user_balance(&key, user.balance() + ISSUE_AMMOUNT, Some(current_time));
                Ok(())
            } else {
                //таймаут пополнения не истёк
                Err(Error::EarlyIssue)?
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
            let user = schema.users().get(&key).ok_or(Error::UserNotFound)?;

            // код выполнится только если такая сова найдётся
            if let Some(owl) = schema.owls_state().get(self.owl_id()) {
                // проверим что не мы её владелец и достаточно ли у нас денег для покупки
                if owl.owner() == key {
                    Err(Error::InvalidOrder)?
                }

                if user.balance() < self.price() {
                    Err(Error::InsufficientFunds)?
                }

                let order = Order::new(&key, self.owl_id(), "pending", self.price());
                let order_hash = order.hash();
                schema.orders_mut().put(&order_hash, order);
                schema.user_orders_mut(&key).push(order_hash);
                schema.owl_orders_mut(&self.owl_id()).push(order_hash);
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

            let order = schema.orders().get(self.order_id()).ok_or(Error::InvalidOrder)?;

            let buyer = schema.users().get(order.public_key()).ok_or(Error::UserNotFound)?;
            let seller = schema.users().get(self.public_key()).ok_or(Error::UserNotFound)?;

            if order.status() != "pending" || !schema.user_owls(self.public_key()).contains(order.owl_id()) {
                schema.orders_mut().put(self.order_id(), order.to_declined());
                Err(Error::InvalidOrder)?
            }

            if buyer.balance() < order.price() {
                schema.orders_mut().put(self.order_id(), order.to_declined());
                Err(Error::InsufficientFunds)?
            }

            schema.orders_mut().put(self.order_id(), order.to_accepted());

            schema.set_user_balance(buyer.public_key(), buyer.balance() - order.price(), None);
            schema.set_user_balance(seller.public_key(), seller.balance() + order.price(), None);

            let order_ids: Vec<Hash> = {
                let idx = schema.owl_orders(order.owl_id());
                let order_ids = idx.iter().collect();
                order_ids
            };

            for order_id in &order_ids {
                let order = schema.orders().get(order_id).ok_or(Error::InvalidOrder)?;
                schema.orders_mut().put(order_id, order.to_declined());
            }

            let owl_state = schema.owls_state().get(order.owl_id()).ok_or(Error::InvalidOrder)?;

            schema.put_owls_into_state(order.public_key(), vec![owl_state.owl()], owl_state.last_breeding());

            schema.user_owls_mut(self.public_key()).remove(order.owl_id());

            Ok(())
        }
    }

    #[derive(Display, Primitive)]
    pub enum Error {
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
        #[display(fmt = "User not found")]
        UserNotFound = 6,
        #[display(fmt = "User already exists")]
        UserAlreadyExists = 7,
        #[display(fmt = "Order not found or invalid")]
        InvalidOrder = 8,
    }

    impl Error {
        /// Converts error to the raw code
        pub fn into_code(self) -> u8 {
            self.to_u8().unwrap()
        }
    }

    impl From<Error> for ExecutionError {
        fn from(e: Error) -> ExecutionError {
            let err_txt = e.to_string();
            ExecutionError::with_description(e.into_code(), err_txt)
        }
    }

}

/// Модуль с реализацией api
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
                        let json = json!({
                            "tx_hash": tx_hash
                        });
                        self_.ok_response(&json)
                    }
                    Ok(None) => Err(ApiError::BadRequest("Empty request body".into()))?,
                    Err(e) => Err(ApiError::BadRequest(e.to_string()))?,
                };

            // View-only хэндлеры
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

            // Транзакции.
            router.post("/v1/transaction", transaction, "post_transaction");
        }
    }

    impl CryptoOwlsApi {
        /// Информация о пользователе
        fn get_user(&self, public_key: &PublicKey) -> Option<User> {
            let snapshot = self.blockchain.snapshot();
            let schema = schema::CryptoOwlsSchema::new(snapshot);
            schema.users().get(public_key)
        }

        /// Полный список пользователей
        fn get_users(&self) -> Vec<User> {
            let snapshot = self.blockchain.snapshot();
            let schema = schema::CryptoOwlsSchema::new(snapshot);
            let idx = schema.users();
            let users: Vec<User> = idx.values().collect();
            users
        }

        /// Информация о cове
        fn get_owl(&self, owl_id: &Hash) -> Option<CryptoOwlState> {
            let snapshot = self.blockchain.snapshot();
            let schema = schema::CryptoOwlsSchema::new(snapshot);
            schema.owls_state().get(&owl_id)
        }

        /// Полный список сов
        fn get_owls(&self) -> Vec<CryptoOwlState> {
            let snapshot = self.blockchain.snapshot();
            let schema = schema::CryptoOwlsSchema::new(snapshot);
            let idx = schema.owls_state();
            let owls: Vec<CryptoOwlState> = idx.values().collect();
            owls
        }

        /// Cписок сов для пользователя
        fn get_user_owls(&self, public_key: &PublicKey) -> Option<Vec<CryptoOwlState>> {
            let snapshot = self.blockchain.snapshot();
            let schema = schema::CryptoOwlsSchema::new(snapshot);

            schema.users().get(&public_key).and({
                let idx = schema.user_owls(&public_key);
                // Внимание, тип итератора - ValueSetIndexIter<'_, Hash> !!!
                let owls = idx.iter()
                    .map(|h| schema.owls_state().get(&h.1))
                    .collect::<Option<Vec<CryptoOwlState>>>()
                    .or(Some(vec![]));
                owls
            })
        }

        /// Информация об всех предложениях на cову
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

        /// Информация об предложениях выставленных юзером
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

/// Собираем всё вместе.
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

        // Метод десериализации для транзакций
        fn tx_from_raw(&self, raw: RawTransaction) -> Result<Box<Transaction>, encoding::Error> {
            let tx = Transactions::tx_from_raw(raw)?;
            Ok(tx.into())
        }

        // Хэши таблиц, которые будут включены в общий стейт хэш
        fn state_hash(&self, snapshot: &Snapshot) -> Vec<Hash> {
            let schema = CryptoOwlsSchema::new(snapshot);
            schema.state_hash()
        }

        // Хэндлер для обработки запросов к ноде
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

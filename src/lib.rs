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

/// Module for the protobuf generated structs.
mod proto;

/// Unique service identifier.
pub const CRYPTOOWLS_SERVICE_ID: u16 = 521;
/// Unique service name which will be used in API and configuration.
pub const CRYPTOOWLS_SERVICE_NAME: &str = "cryptoowls";

/// Sum to be issued each time.
pub const ISSUE_AMOUNT: u64 = 100;

/// Timeout (seconds) before user will be able to issue funds again.
pub const ISSUE_TIMEOUT: i64 = 60;

/// Timeout (seconds) before user will be able to breed owl again.
pub const BREEDING_TIMEOUT: i64 = 60;

/// Breeding price.
pub const BREEDING_PRICE: u64 = 42;

/// Data structures stored in blockchain.
pub mod data_layout {
    use chrono::{DateTime, Utc};
    use serde_derive::{Deserialize, Serialize};

    use exonum::crypto::{Hash, PublicKey};
    use exonum_derive::ProtobufConvert;

    /// CryptoOwl. Unique identifier of the owl is a hash of this data structure.
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ProtobufConvert)]
    #[exonum(pb = "crate::proto::CryptoOwl")]
    pub struct CryptoOwl {
        /// Name (should be unique).
        pub name: String,
        /// DNA.
        pub dna: u32,
    }

    /// Current owl state.
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ProtobufConvert)]
    #[exonum(pb = "crate::proto::CryptoOwlState")]
    pub struct CryptoOwlState {
        /// Owl.
        pub owl: CryptoOwl,
        /// Owner.
        pub owner: PublicKey,
        /// Time of the last breeding.
        pub last_breeding: DateTime<Utc>,
    }

    /// User
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ProtobufConvert)]
    #[exonum(pb = "crate::proto::User")]
    pub struct User {
        /// Public key.
        pub public_key: PublicKey,
        /// Name.
        pub name: String,
        /// Current balance.
        pub balance: u64,
        /// Reserved money that participate in the auction.
        pub reserved: u64,
        /// Time of the last issue of funds.
        pub last_fillup: DateTime<Utc>,
    }

    /// Auction bid.
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ProtobufConvert)]
    #[exonum(pb = "crate::proto::Bid")]
    pub struct Bid {
        /// Bidder is some participant identified by their public key.
        pub public_key: PublicKey,
        /// Value of the bid.
        pub value: u64,
    }

    /// Information about auction.
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ProtobufConvert)]
    #[exonum(pb = "crate::proto::Auction")]
    pub struct Auction {
        /// Participant selling the owl.
        pub public_key: PublicKey,
        /// Owl with `owl_id` is auctioned.
        pub owl_id: Hash,
        /// Start price.
        pub start_price: u64,
        /// Bids are during the `duration` seconds starting from `started_at`.
        /// Type `Duration` is not used because
        /// the trait `ProtobufConvert` is not implemented for `chrono::Duration`.
        pub duration: u64,
    }

    /// Auction state.
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ProtobufConvert)]
    #[exonum(pb = "crate::proto::AuctionState")]
    pub struct AuctionState {
        /// Auction identifier.
        pub id: u64,
        /// Auction information.
        pub auction: Auction,
        /// Start time of the auction.
        pub started_at: DateTime<Utc>,
        /// Merkle root of history of bids. Last bid wins.
        pub bidding_merkle_root: Hash,
        /// If closed => no auctions are accepted.
        pub closed: bool,
    }
}

/// Database schema.
pub mod schema {
    use exonum::crypto::{Hash, PublicKey};
    use exonum::storage::{
        Fork, ListIndex, MapIndex, ProofListIndex, ProofMapIndex, Snapshot, ValueSetIndex,
    };

    use crate::data_layout::{AuctionState, Bid, CryptoOwlState, User};

    pub struct CryptoOwlsSchema<T> {
        pub view: T,
    }

    /// Read-only tables.
    impl<T> CryptoOwlsSchema<T>
    where
        T: AsRef<dyn Snapshot>,
    {
        pub fn new(view: T) -> Self {
            CryptoOwlsSchema { view }
        }

        /// Users.
        pub fn users(&self) -> ProofMapIndex<&T, PublicKey, User> {
            ProofMapIndex::new("cryptoowls.users", &self.view)
        }

        /// Owls and their states (see data_layout::CryptoOwlState).
        pub fn owls_state(&self) -> ProofMapIndex<&T, Hash, CryptoOwlState> {
            ProofMapIndex::new("cryptoowls.owls_state", &self.view)
        }

        /// Owl auctions.
        pub fn auctions(&self) -> ProofListIndex<&T, AuctionState> {
            ProofListIndex::new("cryptoowls.auctions", &self.view)
        }

        /// Owl auction bids.
        pub fn auction_bids(&self, auction_id: u64) -> ProofListIndex<&T, Bid> {
            ProofListIndex::new_in_family("cryptoowls.auction_bids", &auction_id, &self.view)
        }

        /// Helper table for linking user and his owls.
        pub fn user_owls(&self, public_key: &PublicKey) -> ValueSetIndex<&T, Hash> {
            ValueSetIndex::new_in_family("cryptoowls.user_owls", public_key, &self.view)
        }

        /// Helper table for linking user and his auctions.
        pub fn user_auctions(&self, public_key: &PublicKey) -> ListIndex<&T, u64> {
            ListIndex::new_in_family("cryptoowls.user_auctions", public_key, &self.view)
        }

        /// Helper table for linking owl and its open auction.
        pub fn owl_auction(&self) -> MapIndex<&T, Hash, u64> {
            MapIndex::new("cryptoowls.owl_auctions", &self.view)
        }

        /// Method to get state hash. Depends on `users`, `owls_state` and `auctions` tables.
        pub fn state_hash(&self) -> Vec<Hash> {
            vec![
                self.users().merkle_root(),
                self.owls_state().merkle_root(),
                self.auctions().merkle_root(),
            ]
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

        pub fn auctions_mut(&mut self) -> ProofListIndex<&mut Fork, AuctionState> {
            ProofListIndex::new("cryptoowls.auctions", self.view)
        }

        pub fn auction_bids_mut(&mut self, auction_id: u64) -> ProofListIndex<&mut Fork, Bid> {
            ProofListIndex::new_in_family("cryptoowls.auction_bids", &auction_id, self.view)
        }

        pub fn user_owls_mut(&mut self, public_key: &PublicKey) -> ValueSetIndex<&mut Fork, Hash> {
            ValueSetIndex::new_in_family("cryptoowls.user_owls", public_key, self.view)
        }

        pub fn user_auctions_mut(&mut self, public_key: &PublicKey) -> ListIndex<&mut Fork, u64> {
            ListIndex::new_in_family("cryptoowls.user_auctions", public_key, self.view)
        }

        pub fn owl_auction_mut(&mut self) -> MapIndex<&mut Fork, Hash, u64> {
            MapIndex::new("cryptoowls.owl_auctions", self.view)
        }
    }
}

/// Module with description of all transactions.
pub mod transactions {
    use std::io::Cursor;

    use byteorder::{BigEndian, ReadBytesExt};
    use chrono::{DateTime, Duration, Utc};
    use enum_primitive_derive::Primitive;
    use failure_derive::Fail;
    use num_traits::ToPrimitive;
    use rand::distributions::{Sample, Weighted, WeightedChoice};
    use rand::{IsaacRng, Rng, SeedableRng};
    use serde_derive::{Deserialize, Serialize};

    use exonum::blockchain::{
        ExecutionError, ExecutionResult, Schema, Transaction, TransactionContext,
    };
    use exonum::crypto::{CryptoHash, Hash, PublicKey};
    use exonum::storage::{Fork, Snapshot};
    use exonum_derive::{ProtobufConvert, TransactionSet};
    use exonum_time::schema::TimeSchema;

    use crate::{
        data_layout::*, schema::CryptoOwlsSchema, BREEDING_PRICE, BREEDING_TIMEOUT, ISSUE_AMOUNT,
        ISSUE_TIMEOUT,
    };

    //     use byteorder::{BigEndian, ReadBytesExt};
    //     use chrono::{DateTime, Duration, Utc};
    //     use num_traits::ToPrimitive;
    //     use rand::distributions::{Sample, Weighted, WeightedChoice};
    //     use rand::{IsaacRng, Rng, SeedableRng};

    //     use exonum::blockchain::{ExecutionError, ExecutionResult, Schema, Transaction};
    //     use exonum::crypto::{CryptoHash, Hash, PublicKey};
    //     use exonum::messages::Message;
    //     use exonum::storage::{Fork, Snapshot};
    //     use exonum_time::schema::TimeSchema;

    //     use data_layout::{Auction, AuctionState, Bid, CryptoOwl, CryptoOwlState, User};
    //     use schema;
    //     use CryptoOwlsSchema;

    //     use {BREEDING_PRICE, BREEDING_TIMEOUT, CRYPTOOWLS_SERVICE_ID, ISSUE_AMOUNT, ISSUE_TIMEOUT};

    /// Transaction to create a new user.
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ProtobufConvert)]
    #[exonum(pb = "crate::proto::CreateUser")]
    pub struct CreateUser {
        /// Name.
        pub name: String,
    }

    /// Transaction to create an owl. A new random owl created if mother and father
    /// are not defined (zero identifiers passed).
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ProtobufConvert)]
    #[exonum(pb = "crate::proto::MakeOwl")]
    pub struct MakeOwl {
        /// Owl name.
        pub name: String,
        /// Father identifier.
        pub father_id: Hash,
        /// Mother identifier.
        pub mother_id: Hash,
        /// Timestamp. Is required to breed owls with the same identifiers.
        pub seed: DateTime<Utc>,
    }

    /// Transaction to issue funds.
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ProtobufConvert)]
    #[exonum(pb = "crate::proto::Issue")]
    pub struct Issue {
        /// Timestamp. Is required to repeat transaction.
        pub seed: DateTime<Utc>,
    }

    /// Transaction type for adding a new item.
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ProtobufConvert)]
    #[exonum(pb = "crate::proto::CreateAuction")]
    pub struct CreateAuction {
        /// Owl with `owl_id` is auctioned.
        pub owl_id: Hash,
        /// Start price.
        pub start_price: u64,
        /// Bids are during the `duration` seconds starting from `started_at`.
        /// Type `Duration` is not used because
        /// the trait `ProtobufConvert` is not implemented for `chrono::Duration`.
        pub duration: u64,
    }

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ProtobufConvert)]
    #[exonum(pb = "crate::proto::MakeBid")]
    pub struct MakeBid {
        /// Auction ID where a bid must be made.
        pub auction_id: u64,
        /// Bid value.
        pub value: u64,
    }

    #[derive(Serialize, Deserialize, Clone, Debug, PartialEq, TransactionSet)]
    /// Crypto owls transactions.
    pub enum Transactions {
        /// Create user transaction.
        CreateUser(CreateUser),
        /// Create auction transaction.
        CreateAuction(CreateAuction),
        /// Make new owl transaction.
        MakeOwl(MakeOwl),
        /// Make new bid transaction.
        MakeBid(MakeBid),
        /// Issue transaction.
        Issue(Issue),
    }

    impl Transaction for CreateUser {
        fn execute(&self, mut context: TransactionContext) -> ExecutionResult {
            let ts = current_time(context.fork()).unwrap();

            let state_hash = {
                let info_schema = Schema::new(context.fork());
                info_schema.state_hash_aggregator().merkle_root()
            };

            let author = context.author();
            let mut schema = CryptoOwlsSchema::new(context.fork());

            // Reject tx if the user with the same public key is already exists.
            if schema.users().get(&author).is_some() {
                return Err(ErrorKind::UserAlreadyRegistered.into());
            }

            let user = User {
                public_key: author,
                name: self.name.clone(),
                balance: ISSUE_AMOUNT,
                reserved: 0,
                last_fillup: ts,
            };
            schema.users_mut().put(&author, user);

            // New user gets 2 random owls.
            let starter_pack = vec![
                schema.make_uniq_owl((1, 0), &format!("{}'s Adam", self.name), &state_hash),
                schema.make_uniq_owl(
                    (1, 100_042),
                    &format!("{}'s Eve", self.name),
                    &author.hash(),
                ),
            ];
            schema.refresh_owls(&author, starter_pack, ts);
            Ok(())
        }
    }

    impl Transaction for MakeOwl {
        fn execute(&self, mut context: TransactionContext) -> ExecutionResult {
            let author = context.author();
            let ts = current_time(context.fork()).unwrap();

            let state_hash = {
                let info_schema = Schema::new(context.fork());
                info_schema.state_hash_aggregator().merkle_root()
            };

            let mut schema = CryptoOwlsSchema::new(context.fork());

            // Find mother and father.
            // If someone is missed will get None response.
            // Reject transaction if mother or father is not found.
            let parents = [&self.mother_id, &self.father_id]
                .iter()
                .map(|i| schema.owls_state().get(i))
                .collect::<Option<Vec<CryptoOwlState>>>()
                .ok_or_else(|| ErrorKind::OwlNotFound)?;

            let user = schema.users().get(&author).unwrap();

            // Check if user owns these owls.
            if parents.iter().any(|p| p.owner != user.public_key) {
                return Err(ErrorKind::AccessViolation.into());
            }

            let (mother, father) = (parents[0].owl.clone(), parents[1].owl.clone());
            // Can not use the same owl as mother and father at the same time.
            if mother == father {
                return Err(ErrorKind::SelfBreeding.into());
            }

            // Check if user has enough funds for breeding.
            if user.balance < BREEDING_PRICE {
                return Err(ErrorKind::InsufficientFunds.into());
            }

            // Check last breeding time for each owl.
            if parents
                .iter()
                .any(|p| (ts - p.last_breeding).num_seconds() < BREEDING_TIMEOUT)
            {
                return Err(ErrorKind::EarlyBreeding.into());
            }

            // All conditions are fulfilled, start breeding.
            let son = schema.make_uniq_owl((father.dna, mother.dna), &self.name, &state_hash);
            let owls_to_update = vec![son, mother, father];
            schema.refresh_owls(&user.public_key, owls_to_update, ts);

            schema.decrease_user_balance(&user.public_key, BREEDING_PRICE);

            Ok(())
        }
    }

    impl Transaction for Issue {
        fn execute(&self, mut context: TransactionContext) -> ExecutionResult {
            let ts = current_time(context.fork()).unwrap();

            let author = context.author();
            let mut schema = CryptoOwlsSchema::new(context.fork());
            let user = schema.users().get(&author).unwrap();

            if (ts - user.last_fillup).num_seconds() < ISSUE_TIMEOUT {
                // Issue timeout is not expired.
                return Err(ErrorKind::EarlyIssue.into());
            }

            schema.increase_user_balance(&user.public_key, ISSUE_AMOUNT, Some(ts));
            Ok(())
        }
    }

    impl Transaction for CreateAuction {
        fn execute(&self, mut context: TransactionContext) -> ExecutionResult {
            let author = context.author();
            let ts = current_time(context.fork()).unwrap();

            let mut schema = CryptoOwlsSchema::new(context.fork());
            let auction = Auction {
                public_key: author,
                owl_id: self.owl_id,
                start_price: self.start_price,
                duration: self.duration,
            };

            // Check if the user is registered.
            let user = schema
                .users()
                .get(&auction.public_key)
                .ok_or_else(|| ErrorKind::UserIsNotRegistered)?;

            // Check if the owl exists.
            let owl = schema
                .owls_state()
                .get(&auction.owl_id)
                .ok_or_else(|| ErrorKind::OwlNotFound)?;

            // Check if the user owns the owl.
            if owl.owner != user.public_key {
                return Err(ErrorKind::OwlNotOwned.into());
            }

            // Check if the owl isn't auctioned already.
            if schema.owl_auction().get(&auction.owl_id).is_some() {
                return Err(ErrorKind::OwlAlreadyAuctioned.into());
            }

            // Establish a new auction.
            let auction_id = schema.auctions().len();
            let owl_id = auction.owl_id;
            let state = AuctionState {
                id: auction_id,
                auction,
                started_at: ts,
                bidding_merkle_root: Hash::zero(),
                closed: false,
            };

            schema.auctions_mut().push(state);
            schema.owl_auction_mut().put(&owl_id, auction_id);
            schema.user_auctions_mut(&user.public_key).push(auction_id);

            Ok(())
        }
    }

    impl Transaction for MakeBid {
        fn execute(&self, mut context: TransactionContext) -> ExecutionResult {
            let author = context.author();
            let mut schema = CryptoOwlsSchema::new(context.fork());

            // Check if the user is registered.
            let user = schema
                .users()
                .get(&author)
                .ok_or_else(|| ErrorKind::UserIsNotRegistered)?;

            // Check if the auction exists.
            let auction_state = schema
                .auctions()
                .get(self.auction_id)
                .ok_or_else(|| ErrorKind::AuctionNotFound)?;

            let auction = auction_state.auction;

            // Check if the auction is open.
            if auction_state.closed {
                return Err(ErrorKind::AuctionClosed.into());
            }

            // Check if the user has enough funds.
            if user.balance < self.value {
                return Err(ErrorKind::InsufficientFunds.into());
            }

            // Bidding in own auction is prohibited.
            if user.public_key == auction.public_key {
                return Err(ErrorKind::NoSelfBidding.into());
            }

            // Get the bid to beat.
            let min_bid = match schema.auction_bids(auction_state.id).last() {
                Some(bid) => bid.value,
                None => auction.start_price,
            };

            // Check if the bid is higher than the min bid.
            if min_bid >= self.value {
                return Err(ErrorKind::BidTooLow.into());
            }

            // Release balance of the previous bidder if any.
            if let Some(b) = schema.auction_bids(auction_state.id).last() {
                let prev_bid_user = schema.users().get(&b.public_key).unwrap();
                schema.release_user_balance(&prev_bid_user.public_key, min_bid);
            }

            // Reserve value in user wallet.
            schema.reserve_user_balance(&user.public_key, self.value);

            // Make a bid.
            let bid = Bid {
                public_key: author,
                value: self.value,
            };
            schema.auction_bids_mut(self.auction_id).push(bid);

            // Refresh the auction state.
            let bidding_merkle_root = schema.auction_bids(self.auction_id).merkle_root();
            schema.auctions_mut().set(
                auction_state.id,
                AuctionState {
                    id: auction_state.id,
                    auction,
                    started_at: auction_state.started_at,
                    bidding_merkle_root,
                    closed: auction_state.closed,
                },
            );

            Ok(())
        }
    }

    /// Helper methods.
    impl<T> CryptoOwlsSchema<T>
    where
        T: AsRef<dyn Snapshot>,
    {
        // Method to generate new unique owl
        pub fn make_uniq_owl(&self, genes: (u32, u32), name: &str, hash_seed: &Hash) -> CryptoOwl {
            // Hash is a byte array [u8; 32]. To seed random number generator an array
            // of 32-bit numbers &[u32] is required. So we use `std::io::Cursor` and build
            // a new u32 number of each 4 bytes.

            let hash_seed: &[u8] = hash_seed.as_ref();
            let mut seed = [0u32; 4];
            let mut cursor = Cursor::new(hash_seed);
            for seed in seed.iter_mut().take(4) {
                *seed = cursor.read_u32::<BigEndian>().unwrap();
            }
            let mut rng = IsaacRng::from_seed(&seed);

            // Create a unique owl using infinite loop. Call `break` if resulted owl is unique.
            loop {
                let mut son_dna = 0u32;
                // Checking every bit in parent DNAs.
                for i in 0..32 {
                    // Step by all `genes` and set them in accordance with parents genes.
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
                let newborn = CryptoOwl {
                    name: name.to_owned(),
                    dna: son_dna,
                };
                if self.owls_state().get(&newborn.hash()).is_none() {
                    break newborn;
                }
            }
        }
    }

    /// Mutable helper methods.
    impl<'a> CryptoOwlsSchema<&'a mut Fork> {
        /// Helper method to update owl state after breeding or creating.
        pub fn refresh_owls(
            &mut self,
            owner_key: &PublicKey,
            owls: Vec<CryptoOwl>,
            last_breeding: DateTime<Utc>,
        ) {
            for owl in owls {
                self.user_owls_mut(owner_key).insert(owl.hash());
                self.owls_state_mut().put(
                    &owl.hash(),
                    CryptoOwlState {
                        owl,
                        owner: *owner_key,
                        last_breeding,
                    },
                );
            }
        }

        /// Helper method to increase user balance.
        pub fn increase_user_balance(
            &mut self,
            user_id: &PublicKey,
            balance: u64,
            last_fillup: Option<DateTime<Utc>>,
        ) {
            let user = self.users().get(user_id).expect("User should be exist.");
            let last_fillup = last_fillup.unwrap_or_else(|| user.last_fillup);
            self.users_mut().put(
                &user.public_key,
                User {
                    public_key: user.public_key,
                    name: user.name,
                    balance: user.balance + balance,
                    reserved: user.reserved,
                    last_fillup,
                },
            );
        }

        /// Helper method to decrease user balance.
        pub fn decrease_user_balance(&mut self, user_id: &PublicKey, balance: u64) {
            let user = self.users().get(user_id).expect("User should be exist.");
            self.users_mut().put(
                &user.public_key,
                User {
                    public_key: user.public_key,
                    name: user.name,
                    balance: user.balance - balance,
                    reserved: user.reserved,
                    last_fillup: user.last_fillup,
                },
            );
        }

        /// Helper method to decrease user reserved balance.
        pub fn reserve_user_balance(&mut self, user_id: &PublicKey, reserve: u64) {
            let user = self.users().get(user_id).expect("User should be exist.");
            self.users_mut().put(
                &user.public_key,
                User {
                    public_key: user.public_key,
                    name: user.name,
                    balance: user.balance - reserve,
                    reserved: user.reserved + reserve,
                    last_fillup: user.last_fillup,
                },
            );
        }

        /// Helper method to decrease user reserved balance.
        pub fn release_user_balance(&mut self, user_id: &PublicKey, reserve: u64) {
            let user = self.users().get(user_id).expect("User should be exist.");
            self.users_mut().put(
                &user.public_key,
                User {
                    public_key: user.public_key,
                    name: user.name,
                    balance: user.balance + reserve,
                    reserved: user.reserved - reserve,
                    last_fillup: user.last_fillup,
                },
            );
        }

        /// Helper method to decrease user bid with value.
        pub fn confirm_user_bid(&mut self, user_id: &PublicKey, bid_value: u64) {
            let user = self.users().get(user_id).expect("User should be exist.");
            self.users_mut().put(
                &user.public_key,
                User {
                    public_key: user.public_key,
                    name: user.name,
                    balance: user.balance,
                    reserved: user.reserved - bid_value,
                    last_fillup: user.last_fillup,
                },
            );
        }

        pub fn close_auction(&mut self, auction_id: u64) {
            let ts = current_time(self.view).unwrap();
            // Check if auction exists.
            let auction_state = self
                .auctions()
                .get(auction_id)
                .expect("Auction with the given id should be exist.");

            assert!(!auction_state.closed);
            let auction_ends_at = auction_state.ends_at();
            assert!(ts >= auction_ends_at);

            if let Some(winner_bid) = self.auction_bids(auction_state.id).last() {
                // Decrease winner balance.
                let winner = self.users().get(&winner_bid.public_key).unwrap();
                self.confirm_user_bid(&winner.public_key, winner_bid.value);

                // Increase seller balance.
                let seller = self.users().get(&auction_state.auction.public_key).unwrap();
                self.increase_user_balance(&seller.public_key, winner_bid.value, None);

                // Remove possession from the seller.
                self.user_owls_mut(&seller.public_key)
                    .remove(&auction_state.auction.owl_id);

                // Pass it to the winner.
                self.user_owls_mut(&winner.public_key)
                    .insert(auction_state.auction.owl_id);

                // Change owl owner.
                let owl_state = self
                    .owls_state()
                    .get(&auction_state.auction.owl_id)
                    .unwrap();
                self.owls_state_mut().put(
                    &auction_state.auction.owl_id,
                    CryptoOwlState {
                        owl: owl_state.owl,
                        owner: winner.public_key,
                        last_breeding: owl_state.last_breeding,
                    },
                );
            };

            self.owl_auction_mut().remove(&auction_state.auction.owl_id);
            // Close auction
            self.auctions_mut().set(
                auction_state.id,
                AuctionState {
                    id: auction_state.id,
                    auction: auction_state.auction,
                    started_at: auction_state.started_at,
                    bidding_merkle_root: auction_state.bidding_merkle_root,
                    closed: true,
                },
            );
        }
    }

    impl AuctionState {
        pub fn ends_at(&self) -> DateTime<Utc> {
            self.started_at + Duration::seconds(self.auction.duration as i64)
        }
    }

    // A helper function to get current time from the time oracle.
    pub fn current_time(snapshot: &dyn Snapshot) -> Option<DateTime<Utc>> {
        let time_schema = TimeSchema::new(snapshot);
        time_schema.time().get()
    }

    #[derive(Debug, Fail, Primitive)]
    pub enum ErrorKind {
        #[fail(display = "Too early for breeding.")]
        EarlyBreeding = 1,
        //
        #[fail(display = "Too early for balance refill.")]
        EarlyIssue = 2,
        //
        #[fail(display = "Insufficient funds.")]
        InsufficientFunds = 3,
        //
        #[fail(display = "Not your property.")]
        AccessViolation = 4,
        //
        #[fail(display = "You need two different owls.")]
        SelfBreeding = 5,
        //
        #[fail(display = "User is already registered")]
        UserAlreadyRegistered = 6,
        //
        #[fail(display = "Participant is not registered")]
        UserIsNotRegistered = 7,
        //
        #[fail(display = "Owl does not exist")]
        OwlNotFound = 8,
        //
        #[fail(display = "You do not own of the item")]
        OwlNotOwned = 9,
        //
        #[fail(display = "Owl is already auctioned")]
        OwlAlreadyAuctioned = 10,
        //
        #[fail(display = "Auction does not exist")]
        AuctionNotFound = 11,
        //
        #[fail(display = "Auction is closed")]
        AuctionClosed = 12,
        //
        #[fail(display = "Bid is below the current highest bid")]
        BidTooLow = 13,
        // CloseAuction can only be performed by the validator nodes.
        #[fail(display = "Transaction is not authorized.")]
        UnauthorizedTransaction = 14,
        //
        #[fail(display = "You may not bid on your own item.")]
        NoSelfBidding = 15,
    }

    impl ErrorKind {
        /// Converts error to the raw code.
        pub fn as_code(&self) -> u8 {
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

/// Module with API implementation.
mod api {
    use serde_derive::{Deserialize, Serialize};

    use exonum::api::{self, ServiceApiBuilder, ServiceApiState};
    use exonum::crypto::{Hash, PublicKey};

    use crate::{
        data_layout::{AuctionState, Bid, CryptoOwlState, User},
        schema::CryptoOwlsSchema,
    };

    #[derive(Debug)]
    pub struct CryptoOwlsApi;

    #[derive(Debug, Clone, Copy, Serialize, Deserialize)]
    pub struct OwlQuery {
        pub id: Hash,
    }

    #[derive(Debug, Clone, Copy, Serialize, Deserialize)]
    pub struct UserQuery {
        pub pub_key: PublicKey,
    }

    #[derive(Debug, Clone, Copy, Serialize, Deserialize)]
    pub struct AuctionQuery {
        pub id: u64,
    }

    impl CryptoOwlsApi {
        /// User profile.
        fn get_user(state: &ServiceApiState, query: UserQuery) -> api::Result<Option<User>> {
            let snapshot = state.snapshot();
            let schema = CryptoOwlsSchema::new(snapshot);
            Ok(schema.users().get(&query.pub_key))
        }

        /// All users.
        fn get_users(state: &ServiceApiState, _query: ()) -> api::Result<Vec<User>> {
            let snapshot = state.snapshot();
            let schema = CryptoOwlsSchema::new(snapshot);
            let idx = schema.users();
            let users: Vec<User> = idx.values().collect();
            Ok(users)
        }

        /// Owl profile.
        fn get_owl(
            state: &ServiceApiState,
            query: OwlQuery,
        ) -> api::Result<Option<CryptoOwlState>> {
            let snapshot = state.snapshot();
            let schema = CryptoOwlsSchema::new(snapshot);
            Ok(schema.owls_state().get(&query.id))
        }

        /// All owls.
        fn get_owls(state: &ServiceApiState, _query: ()) -> api::Result<Vec<CryptoOwlState>> {
            let snapshot = state.snapshot();
            let schema = CryptoOwlsSchema::new(snapshot);
            let idx = schema.owls_state();
            let owls: Vec<CryptoOwlState> = idx.values().collect();
            Ok(owls)
        }

        /// User owls list.
        fn get_user_owls(
            state: &ServiceApiState,
            query: UserQuery,
        ) -> api::Result<Option<Vec<CryptoOwlState>>> {
            let snapshot = state.snapshot();
            let schema = CryptoOwlsSchema::new(snapshot);

            Ok(schema.users().get(&query.pub_key).and({
                let idx = schema.user_owls(&query.pub_key);
                // Attention, iterator type is ValueSetIndexIter<'_, Hash> !!!
                idx.iter()
                    .map(|h| schema.owls_state().get(&h.1))
                    .collect::<Option<Vec<CryptoOwlState>>>()
                    .or_else(|| Some(Vec::new()))
            }))
        }

        /// Auctions made by user.
        fn get_users_auctions(
            state: &ServiceApiState,
            query: UserQuery,
        ) -> api::Result<Option<Vec<AuctionState>>> {
            let snapshot = state.snapshot();
            let schema = CryptoOwlsSchema::new(snapshot);

            Ok(schema.users().get(&query.pub_key).map(|user| {
                let user_auctions = schema.user_auctions(&user.public_key);
                user_auctions
                    .into_iter()
                    .map(|auction_id| schema.auctions().get(auction_id).unwrap())
                    .collect()
            }))
        }

        /// Auctions and bids by auction identifier.
        fn get_auction_with_bids(
            state: &ServiceApiState,
            query: AuctionQuery,
        ) -> api::Result<Option<(AuctionState, Vec<Bid>)>> {
            let snapshot = state.snapshot();
            let schema = CryptoOwlsSchema::new(snapshot);

            Ok(schema.auctions().get(query.id).map(|auction_state| {
                let auction_bids = schema.auction_bids(auction_state.id);
                let bids = auction_bids.into_iter().collect();
                (auction_state, bids)
            }))
        }

        /// Auction bids by its identifier.
        fn get_auction_bids(
            state: &ServiceApiState,
            query: AuctionQuery,
        ) -> api::Result<Option<Vec<Bid>>> {
            let snapshot = state.snapshot();
            let schema = CryptoOwlsSchema::new(snapshot);

            Ok(schema.auctions().get(query.id).map(|auction_state| {
                let auction_bids = schema.auction_bids(auction_state.id);
                auction_bids.into_iter().collect()
            }))
        }

        /// All auctions.
        fn get_auctions(state: &ServiceApiState, _query: ()) -> api::Result<Vec<AuctionState>> {
            let snapshot = state.snapshot();
            let schema = CryptoOwlsSchema::new(snapshot);
            let auctions = schema.auctions();
            let auctions = auctions.into_iter().collect::<Vec<_>>();
            Ok(auctions)
        }

        // Links the service api implementation to the Exonum.
        pub fn wire(builder: &mut ServiceApiBuilder) {
            builder
                .public_scope()
                .endpoint("v1/users", Self::get_users)
                .endpoint("v1/user", Self::get_user)
                .endpoint("v1/owls", Self::get_owls)
                .endpoint("v1/owl", Self::get_owl)
                .endpoint("v1/user/owls", Self::get_user_owls)
                .endpoint("v1/user/auctions", Self::get_users_auctions)
                .endpoint("v1/auction/bids", Self::get_auction_bids)
                .endpoint("v1/auction", Self::get_auction_with_bids)
                .endpoint("v1/auctions", Self::get_auctions);
        }
    }
}

/// Collecting everything together.
pub mod service {
    use exonum::{
        api::ServiceApiBuilder,
        blockchain::{Service, Transaction, TransactionSet},
        crypto::Hash,
        helpers::fabric::{Context, ServiceFactory},
        messages::RawTransaction,
        storage::{Fork, Snapshot},
    };

    use crate::{
        api::CryptoOwlsApi,
        schema::CryptoOwlsSchema,
        transactions::{self, Transactions},
        CRYPTOOWLS_SERVICE_ID, CRYPTOOWLS_SERVICE_NAME,
    };

    #[derive(Debug, Default)]
    pub struct CryptoOwlsService;

    #[derive(Debug, Default)]
    pub struct CryptoOwlsServiceFactory;

    impl ServiceFactory for CryptoOwlsServiceFactory {
        fn service_name(&self) -> &str {
            CRYPTOOWLS_SERVICE_NAME
        }

        fn make_service(&mut self, _: &Context) -> Box<dyn Service> {
            Box::new(CryptoOwlsService)
        }
    }

    impl Service for CryptoOwlsService {
        fn service_id(&self) -> u16 {
            CRYPTOOWLS_SERVICE_ID
        }

        fn service_name(&self) -> &'static str {
            "cryptoowls"
        }

        // Tables hashes to be included into blockchain state hash.
        fn state_hash(&self, snapshot: &dyn Snapshot) -> Vec<Hash> {
            let schema = CryptoOwlsSchema::new(snapshot);
            schema.state_hash()
        }

        // Method to deserialize transactions.
        fn tx_from_raw(&self, raw: RawTransaction) -> Result<Box<dyn Transaction>, failure::Error> {
            let tx = Transactions::tx_from_raw(raw)?;
            Ok(tx.into())
        }

        // Check open auctions state after each block's commit.
        fn before_commit(&self, fork: &mut Fork) {
            let current_time = if let Some(time) = transactions::current_time(fork) {
                time
            } else {
                return;
            };

            // Check open auctions and close them if time expires.
            let mut schema = CryptoOwlsSchema::new(fork);
            let open_auctions = schema
                .owl_auction()
                .into_iter()
                .map(|(_, auction_id)| auction_id)
                .collect::<Vec<_>>();
            for auction_id in open_auctions {
                let auction_state = schema.auctions().get(auction_id).unwrap();
                if auction_state.ends_at() <= current_time {
                    schema.close_auction(auction_id);
                }
            }
        }

        // Handling requests to a node.
        fn wire_api(&self, builder: &mut ServiceApiBuilder) {
            CryptoOwlsApi::wire(builder)
        }
    }
}

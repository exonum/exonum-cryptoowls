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
#![cfg_attr(feature = "cargo-clippy", allow(let_and_return))]

#[macro_use]
extern crate display_derive;
#[macro_use]
extern crate enum_primitive_derive;
#[macro_use]
extern crate exonum;
#[macro_use]
extern crate log;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;

extern crate byteorder;
extern crate chrono;
extern crate exonum_time;
extern crate num_traits;
extern crate rand;
extern crate serde;

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
    use exonum::crypto::{Hash, PublicKey};

    encoding_struct! {
        /// CryptoOwl. Unique identifier of the owl is a hash of this data structure.
        struct CryptoOwl {
            /// Name (should be unique).
            name: &str,
            /// DNA.
            dna: u32,
        }
    }

    encoding_struct! {
        /// Current owl state.
        struct CryptoOwlState {
            /// Owl.
            owl: CryptoOwl,
            /// Owner.
            owner: &PublicKey,
            /// Time of the last breeding.
            last_breeding: DateTime<Utc>,
        }
    }

    encoding_struct! {
        /// User
        struct User {
            /// Public key.
            public_key: &PublicKey,
            /// Name.
            name: &str,
            /// Current balance.
            balance: u64,
            /// Reserved money that participate in the auction.
            reserved: u64,
            /// Time of the last issue of funds.
            last_fillup: DateTime<Utc>,
        }
    }

    encoding_struct! {
        /// Auction bid.
        struct Bid {
            /// Bidder is some participant identified by their public key.
            public_key: &PublicKey,
            /// Value of the bid.
            value: u64,
        }
    }

    encoding_struct! {
        /// Information about auction.
        struct Auction {
            /// Participant selling the owl.
            public_key: &PublicKey,
            /// Owl with `owl_id` is auctioned.
            owl_id: &Hash,
            /// Start price.
            start_price: u64,
            /// Bids are during the `duration` seconds starting from `started_at`.
            /// Type `Duration` is not used because
            /// the trait `exonum::encoding::SegmentField<'_>` is not implemented for `chrono::Duration`.
            duration: u64,
        }
    }

    encoding_struct! {
        /// Auction state.
        struct AuctionState {
            /// Auction identifier.
            id: u64,
            /// Auction information.
            auction: Auction,
            /// Start time of the auction.
            started_at: DateTime<Utc>,
            /// Merkle root of history of bids. Last bid wins.
            bidding_merkle_root: &Hash,
            /// If closed => no auctions are accepted.
            closed: bool,
        }
    }
}

/// Database schema.
pub mod schema {
    use exonum::crypto::{Hash, PublicKey};
    use exonum::storage::{
        Fork, ListIndex, MapIndex, ProofListIndex, ProofMapIndex, Snapshot, ValueSetIndex,
    };

    use data_layout::{AuctionState, Bid, CryptoOwlState, User};

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
    use byteorder::{BigEndian, ReadBytesExt};
    use chrono::{DateTime, Duration, Utc};
    use num_traits::ToPrimitive;
    use rand::distributions::{Sample, Weighted, WeightedChoice};
    use rand::{IsaacRng, Rng, SeedableRng};

    use exonum::blockchain::{ExecutionError, ExecutionResult, Schema, Transaction};
    use exonum::crypto::{CryptoHash, Hash, PublicKey};
    use exonum::messages::Message;
    use exonum::storage::{Fork, Snapshot};
    use exonum_time::schema::TimeSchema;

    use std::io::Cursor;

    use data_layout::{Auction, AuctionState, Bid, CryptoOwl, CryptoOwlState, User};
    use schema;
    use schema::CryptoOwlsSchema;

    use {BREEDING_PRICE, BREEDING_TIMEOUT, CRYPTOOWLS_SERVICE_ID, ISSUE_AMOUNT, ISSUE_TIMEOUT};

    transactions! {
        pub Transactions {
            const SERVICE_ID = CRYPTOOWLS_SERVICE_ID;

            /// Transaction to create a new user.
            struct CreateUser {
                /// Public key, used as identifier.
                public_key: &PublicKey,
                /// Name.
                name: &str,
            }

            /// Transaction to create an owl. A new random owl created if mother and father
            /// are not defined (zero identifiers passed).
            struct MakeOwl {
                /// Public user identifier.
                public_key: &PublicKey,
                /// Owl name.
                name: &str,
                /// Father identifier.
                father_id: &Hash,
                /// Mother identifier.
                mother_id: &Hash,
                /// Timestamp. Is required to breed owls with the same identifiers.
                seed: DateTime<Utc>,
            }

            /// Transaction to issue funds.
            struct Issue {
                /// Public user identifier.
                public_key: &PublicKey,
                /// Timestamp. Is required to repeat transaction.
                seed: DateTime<Utc>,
            }

            /// Transaction type for adding a new item.
            struct CreateAuction {
                /// Participant selling the owl.
                public_key: &PublicKey,
                /// Owl with `owl_id` is auctioned.
                owl_id: &Hash,
                /// Start price.
                start_price: u64,
                /// Bids are during the `duration` seconds starting from `started_at`.
                /// Type `Duration` is not used because
                /// the trait `exonum::encoding::SegmentField<'_>` is not implemented for `chrono::Duration`.
                duration: u64,
            }

            struct MakeBid {
                /// Bidder.
                public_key: &PublicKey,
                /// Auction ID where a bid must be made.
                auction_id: u64,
                /// Bid value.
                value: u64,
            }

            /// Close auction.
            struct CloseAuction {
                /// Auction to close.
                auction_id: u64,
                /// Key of the closing party.
                closing_party: &PublicKey,
                /// Timestamp. Is required to repeat transactions.
                seed: DateTime<Utc>,
            }
        }
    }

    impl Transaction for CreateUser {
        fn verify(&self) -> bool {
            self.verify_signature(self.public_key())
        }

        fn execute(&self, fork: &mut Fork) -> ExecutionResult {
            let ts = current_time(fork).unwrap();

            let state_hash = {
                let info_schema = Schema::new(&fork);
                info_schema.state_hash_aggregator().merkle_root()
            };

            let key = self.public_key();
            let mut schema = schema::CryptoOwlsSchema::new(fork);

            // Reject tx if the user with the same public key is already exists.
            if schema.users().get(key).is_some() {
                Err(ErrorKind::UserAlreadyRegistered)?;
            }

            let user = User::new(key, self.name(), ISSUE_AMOUNT, 0, ts);
            schema.users_mut().put(key, user);

            // New user gets 2 random owls.
            let starter_pack = vec![
                schema.make_uniq_owl((1, 0), &format!("{}'s Adam", self.name()), &state_hash),
                schema.make_uniq_owl((1, 100_042), &format!("{}'s Eve", self.name()), &key.hash()),
            ];
            schema.refresh_owls(key, starter_pack, ts);
            Ok(())
        }
    }

    impl Transaction for MakeOwl {
        fn verify(&self) -> bool {
            self.verify_signature(self.public_key())
        }

        fn execute(&self, fork: &mut Fork) -> ExecutionResult {
            let ts = current_time(fork).unwrap();

            let state_hash = {
                let info_schema = Schema::new(&fork);
                info_schema.state_hash_aggregator().merkle_root()
            };

            let mut schema = schema::CryptoOwlsSchema::new(fork);

            // Find mother and father.
            // If someone is missed will get None response.
            // Reject transaction if mother or father is not found.
            let parents = [self.mother_id(), self.father_id()]
                .iter()
                .map(|i| schema.owls_state().get(i))
                .collect::<Option<Vec<CryptoOwlState>>>()
                .ok_or_else(|| ErrorKind::OwlNotFound)?;

            let user = schema.users().get(self.public_key()).unwrap();

            // Check if user owns these owls.
            if parents.iter().any(|p| p.owner() != user.public_key()) {
                Err(ErrorKind::AccessViolation)?;
            }

            let (mother, father) = (parents[0].owl(), parents[1].owl());
            // Can not use the same owl as mother and father at the same time.
            if mother == father {
                Err(ErrorKind::SelfBreeding)?;
            }

            // Check if user has enough funds for breeding.
            if user.balance() < BREEDING_PRICE {
                Err(ErrorKind::InsufficientFunds)?;
            }

            // Check last breeding time for each owl.
            if parents
                .iter()
                .any(|p| (ts - p.last_breeding()).num_seconds() < BREEDING_TIMEOUT)
            {
                Err(ErrorKind::EarlyBreeding)?;
            }

            // All conditions are fulfilled, start breeding.
            let son = schema.make_uniq_owl((father.dna(), mother.dna()), self.name(), &state_hash);
            let owls_to_update = vec![son, mother, father];
            schema.refresh_owls(user.public_key(), owls_to_update, ts);

            schema.decrease_user_balance(user.public_key(), BREEDING_PRICE);

            Ok(())
        }
    }

    impl Transaction for Issue {
        fn verify(&self) -> bool {
            self.verify_signature(self.public_key())
        }

        fn execute(&self, fork: &mut Fork) -> ExecutionResult {
            let ts = current_time(fork).unwrap();

            let mut schema = schema::CryptoOwlsSchema::new(fork);
            let key = self.public_key();
            let user = schema.users().get(key).unwrap();

            if (ts - user.last_fillup()).num_seconds() < ISSUE_TIMEOUT {
                // Issue timeout is not expired.
                Err(ErrorKind::EarlyIssue)?
            }

            schema.increase_user_balance(user.public_key(), ISSUE_AMOUNT, Some(ts));
            Ok(())
        }
    }

    impl Transaction for CreateAuction {
        fn verify(&self) -> bool {
            self.verify_signature(self.public_key())
        }

        fn execute(&self, fork: &mut Fork) -> ExecutionResult {
            let ts = current_time(fork).unwrap();

            let mut schema = schema::CryptoOwlsSchema::new(fork);
            let auction = Auction::new(
                self.public_key(),
                self.owl_id(),
                self.start_price(),
                self.duration(),
            );

            // Check if the user is registered.
            let user = schema
                .users()
                .get(auction.public_key())
                .ok_or_else(|| ErrorKind::UserIsNotRegistered)?;

            // Check if the owl exists.
            let owl = schema
                .owls_state()
                .get(auction.owl_id())
                .ok_or_else(|| ErrorKind::OwlNotFound)?;

            // Check if the user owns the owl.
            if owl.owner() != user.public_key() {
                Err(ErrorKind::OwlNotOwned)?;
            }

            // Check if the owl isn't auctioned already.
            if schema.owl_auction().get(auction.owl_id()).is_some() {
                Err(ErrorKind::OwlAlreadyAuctioned)?;
            }

            // Establish a new auction.
            let auction_id = schema.auctions().len();
            let owl_id = *auction.owl_id();
            let state = AuctionState::new(auction_id, auction, ts, &Hash::zero(), false);

            schema.auctions_mut().push(state);
            schema.owl_auction_mut().put(&owl_id, auction_id);
            schema.user_auctions_mut(user.public_key()).push(auction_id);

            Ok(())
        }
    }

    impl Transaction for MakeBid {
        fn verify(&self) -> bool {
            self.verify_signature(self.public_key())
        }

        fn execute(&self, fork: &mut Fork) -> ExecutionResult {
            let mut schema = schema::CryptoOwlsSchema::new(fork);

            // Check if the user is registered.
            let user = schema
                .users()
                .get(self.public_key())
                .ok_or_else(|| ErrorKind::UserIsNotRegistered)?;

            // Check if the auction exists.
            let auction_state = schema
                .auctions()
                .get(self.auction_id())
                .ok_or_else(|| ErrorKind::AuctionNotFound)?;

            let auction = auction_state.auction();

            // Check if the auction is open.
            if auction_state.closed() {
                Err(ErrorKind::AuctionClosed)?;
            }

            // Check if the user has enough funds.
            if user.balance() < self.value() {
                Err(ErrorKind::InsufficientFunds)?;
            }

            // Bidding in own auction is prohibited.
            if user.public_key() == auction.public_key() {
                Err(ErrorKind::NoSelfBidding)?;
            }

            // Get the bid to beat.
            let min_bid = match schema.auction_bids(auction_state.id()).last() {
                Some(bid) => bid.value(),
                None => auction.start_price(),
            };

            // Check if the bid is higher than the min bid.
            if min_bid >= self.value() {
                Err(ErrorKind::BidTooLow)?;
            }

            // Release balance of the previous bidder if any.
            if let Some(b) = schema.auction_bids(auction_state.id()).last() {
                let prev_bid_user = schema.users().get(b.public_key()).unwrap();
                schema.release_user_balance(prev_bid_user.public_key(), min_bid);
            }

            // Reserve value in user wallet.
            schema.reserve_user_balance(user.public_key(), self.value());

            // Make a bid.
            let bid = Bid::new(self.public_key(), self.value());
            schema.auction_bids_mut(self.auction_id()).push(bid);

            // Refresh the auction state.
            let bids_merkle_root = schema.auction_bids(self.auction_id()).merkle_root();
            schema.auctions_mut().set(
                auction_state.id(),
                AuctionState::new(
                    auction_state.id(),
                    auction,
                    auction_state.started_at(),
                    &bids_merkle_root,
                    auction_state.closed(),
                ),
            );

            Ok(())
        }
    }

    impl CloseAuction {
        fn check_signed_by_validator(&self, snapshot: &Snapshot) -> ExecutionResult {
            let keys = Schema::new(&snapshot).actual_configuration().validator_keys;
            let signed = keys.iter().any(|k| k.service_key == *self.closing_party());
            if !signed {
                Err(ErrorKind::UnauthorizedTransaction)?
            } else {
                Ok(())
            }
        }
    }

    impl Transaction for CloseAuction {
        fn verify(&self) -> bool {
            true
        }

        fn execute(&self, fork: &mut Fork) -> ExecutionResult {
            // Check that the auction is being closed by one of the validator nodes.
            self.check_signed_by_validator(fork.as_ref())?;

            let ts = current_time(fork).unwrap();

            let mut schema = schema::CryptoOwlsSchema::new(fork);

            // Check if auction exists.
            let auction_state = schema
                .auctions()
                .get(self.auction_id())
                .ok_or_else(|| ErrorKind::AuctionNotFound)?;

            let auction = auction_state.auction();

            assert!(!auction_state.closed());
            let auction_ends_at = auction_state.ends_at();
            assert!(ts >= auction_ends_at);

            if let Some(winner_bid) = schema.auction_bids(auction_state.id()).last() {
                // Decrease winner balance.
                let winner = schema.users().get(winner_bid.public_key()).unwrap();
                schema.confirm_user_bid(winner.public_key(), winner_bid.value());

                // Increase seller balance.
                let seller = schema.users().get(auction.public_key()).unwrap();
                schema.increase_user_balance(seller.public_key(), winner_bid.value(), None);

                // Remove possession from the seller.
                schema
                    .user_owls_mut(seller.public_key())
                    .remove(auction.owl_id());

                // Pass it to the winner.
                schema
                    .user_owls_mut(winner.public_key())
                    .insert(*auction.owl_id());

                // Change owl owner.
                let owl_state = schema.owls_state().get(auction.owl_id()).unwrap();
                schema.owls_state_mut().put(
                    auction.owl_id(),
                    CryptoOwlState::new(
                        owl_state.owl(),
                        winner.public_key(),
                        owl_state.last_breeding(),
                    ),
                );
            };

            schema.owl_auction_mut().remove(auction.owl_id());
            // Close auction
            schema.auctions_mut().set(
                auction_state.id(),
                AuctionState::new(
                    auction_state.id(),
                    auction_state.auction(),
                    auction_state.started_at(),
                    auction_state.bidding_merkle_root(),
                    true,
                ),
            );
            Ok(())
        }
    }

    /// Helper methods.
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
                let newborn = CryptoOwl::new(name, son_dna);
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
            ts: DateTime<Utc>,
        ) {
            for owl in owls {
                self.user_owls_mut(owner_key).insert(owl.hash());
                self.owls_state_mut()
                    .put(&owl.hash(), CryptoOwlState::new(owl, owner_key, ts));
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
            let last_fillup = last_fillup.unwrap_or_else(|| user.last_fillup());
            self.users_mut().put(
                user.public_key(),
                User::new(
                    user.public_key(),
                    user.name(),
                    user.balance() + balance,
                    user.reserved(),
                    last_fillup,
                ),
            );
        }

        /// Helper method to decrease user balance.
        pub fn decrease_user_balance(&mut self, user_id: &PublicKey, balance: u64) {
            let user = self.users().get(user_id).expect("User should be exist.");
            self.users_mut().put(
                user.public_key(),
                User::new(
                    user.public_key(),
                    user.name(),
                    user.balance() - balance,
                    user.reserved(),
                    user.last_fillup(),
                ),
            );
        }

        /// Helper method to decrease user reserved balance.
        pub fn reserve_user_balance(&mut self, user_id: &PublicKey, reserve: u64) {
            let user = self.users().get(user_id).expect("User should be exist.");
            self.users_mut().put(
                user.public_key(),
                User::new(
                    user.public_key(),
                    user.name(),
                    user.balance() - reserve,
                    user.reserved() + reserve,
                    user.last_fillup(),
                ),
            );
        }

        /// Helper method to decrease user reserved balance.
        pub fn release_user_balance(&mut self, user_id: &PublicKey, reserve: u64) {
            let user = self.users().get(user_id).expect("User should be exist.");
            self.users_mut().put(
                user.public_key(),
                User::new(
                    user.public_key(),
                    user.name(),
                    user.balance() + reserve,
                    user.reserved() - reserve,
                    user.last_fillup(),
                ),
            );
        }

        /// Helper method to decrease user bid with value.
        pub fn confirm_user_bid(&mut self, user_id: &PublicKey, bid_value: u64) {
            let user = self.users().get(user_id).expect("User should be exist.");
            self.users_mut().put(
                user.public_key(),
                User::new(
                    user.public_key(),
                    user.name(),
                    user.balance(),
                    user.reserved() - bid_value,
                    user.last_fillup(),
                ),
            );
        }
    }

    impl AuctionState {
        pub fn ends_at(&self) -> DateTime<Utc> {
            self.started_at() + Duration::seconds(self.auction().duration() as i64)
        }
    }

    // A helper function to get current time from the time oracle.
    pub fn current_time(snapshot: &Snapshot) -> Option<DateTime<Utc>> {
        let time_schema = TimeSchema::new(snapshot);
        time_schema.time().get()
    }

    #[derive(Display, Primitive)]
    pub enum ErrorKind {
        #[display(fmt = "Too early for breeding.")]
        EarlyBreeding = 1,
        //
        #[display(fmt = "Too early for balance refill.")]
        EarlyIssue = 2,
        //
        #[display(fmt = "Insufficient funds.")]
        InsufficientFunds = 3,
        //
        #[display(fmt = "Not your property.")]
        AccessViolation = 4,
        //
        #[display(fmt = "You need two different owls.")]
        SelfBreeding = 5,
        //
        #[display(fmt = "User is already registered")]
        UserAlreadyRegistered = 6,
        //
        #[display(fmt = "Participant is not registered")]
        UserIsNotRegistered = 7,
        //
        #[display(fmt = "Owl does not exist")]
        OwlNotFound = 8,
        //
        #[display(fmt = "You do not own of the item")]
        OwlNotOwned = 9,
        //
        #[display(fmt = "Owl is already auctioned")]
        OwlAlreadyAuctioned = 10,
        //
        #[display(fmt = "Auction does not exist")]
        AuctionNotFound = 11,
        //
        #[display(fmt = "Auction is closed")]
        AuctionClosed = 12,
        //
        #[display(fmt = "Bid is below the current highest bid")]
        BidTooLow = 13,
        // CloseAuction can only be performed by the validator nodes.
        #[display(fmt = "Transaction is not authorized.")]
        UnauthorizedTransaction = 14,
        //
        #[display(fmt = "You may not bid on your own item.")]
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
    use exonum::api::{self, ServiceApiBuilder, ServiceApiState};
    use exonum::crypto::{Hash, PublicKey};

    use exonum::blockchain::{Transaction};
    use exonum::node::{TransactionSend};

    use data_layout::{AuctionState, Bid, CryptoOwlState, User};
    use schema;
    use transactions::Transactions;

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
            let schema = schema::CryptoOwlsSchema::new(snapshot);
            Ok(schema.users().get(&query.pub_key))
        }

        /// All users.
        fn get_users(state: &ServiceApiState, _query: ()) -> api::Result<Vec<User>> {
            let snapshot = state.snapshot();
            let schema = schema::CryptoOwlsSchema::new(snapshot);
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
            let schema = schema::CryptoOwlsSchema::new(snapshot);
            Ok(schema.owls_state().get(&query.id))
        }

        /// All owls.
        fn get_owls(state: &ServiceApiState, _query: ()) -> api::Result<Vec<CryptoOwlState>> {
            let snapshot = state.snapshot();
            let schema = schema::CryptoOwlsSchema::new(snapshot);
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
            let schema = schema::CryptoOwlsSchema::new(snapshot);

            Ok(schema.users().get(&query.pub_key).and({
                let idx = schema.user_owls(&query.pub_key);
                // Attention, iterator type is ValueSetIndexIter<'_, Hash> !!!
                let owls = idx.iter()
                    .map(|h| schema.owls_state().get(&h.1))
                    .collect::<Option<Vec<CryptoOwlState>>>()
                    .or_else(|| Some(Vec::new()));
                owls
            }))
        }

        /// Auctions made by user.
        fn get_users_auctions(
            state: &ServiceApiState,
            query: UserQuery,
        ) -> api::Result<Option<Vec<AuctionState>>> {
            let snapshot = state.snapshot();
            let schema = schema::CryptoOwlsSchema::new(snapshot);

            Ok(schema.users().get(&query.pub_key).map(|user| {
                let user_auctions = schema.user_auctions(user.public_key());
                let auctions = user_auctions
                    .into_iter()
                    .map(|auction_id| schema.auctions().get(auction_id).unwrap())
                    .collect();
                auctions
            }))
        }

        /// Auctions and bids by auction identifier.
        fn get_auction_with_bids(
            state: &ServiceApiState,
            query: AuctionQuery,
        ) -> api::Result<Option<(AuctionState, Vec<Bid>)>> {
            let snapshot = state.snapshot();
            let schema = schema::CryptoOwlsSchema::new(snapshot);

            Ok(schema
                .auctions()
                .get(query.id)
                .map(|auction_state| {
                    let auction_bids = schema.auction_bids(auction_state.id());
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
            let schema = schema::CryptoOwlsSchema::new(snapshot);

            Ok(schema
                .auctions()
                .get(query.id)
                .map(|auction_state| {
                    let auction_bids = schema.auction_bids(auction_state.id());
                    let bids = auction_bids.into_iter().collect();
                    bids
                }))
        }

        /// All auctions.
        fn get_auctions(state: &ServiceApiState, _query: ()) -> api::Result<Vec<AuctionState>> {
            let snapshot = state.snapshot();
            let schema = schema::CryptoOwlsSchema::new(snapshot);
            let auctions = schema.auctions();
            let auctions = auctions.into_iter().collect::<Vec<_>>();
            Ok(auctions)
        }

        /// Send new transaction into the blockchain.
        fn post_transaction(
            state: &ServiceApiState,
            transaction: Transactions,
        ) -> api::Result<Hash> {
            let transaction: Box<Transaction> = transaction.into();
            let tx_hash = transaction.hash();
            state.sender().send(transaction)?;
            Ok(tx_hash)
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
                .endpoint("v1/auctions", Self::get_auctions)
                .endpoint_mut("v1/transaction", Self::post_transaction);
        }
    }
}

/// Collecting everything together.
pub mod service {
    use exonum::api::ServiceApiBuilder;
    use exonum::blockchain::{Service, ServiceContext, Transaction, TransactionSet};
    use exonum::crypto::Hash;
    use exonum::encoding;
    use exonum::helpers::fabric::{Context, ServiceFactory};
    use exonum::messages::RawTransaction;
    use exonum::storage::Snapshot;

    use api::CryptoOwlsApi;
    use schema::CryptoOwlsSchema;
    use transactions::{self, CloseAuction, Transactions};

    use {CRYPTOOWLS_SERVICE_ID, CRYPTOOWLS_SERVICE_NAME};

    #[derive(Debug, Default)]
    pub struct CryptoOwlsService;

    #[derive(Debug, Default)]
    pub struct CryptoOwlsServiceFactory;

    impl ServiceFactory for CryptoOwlsServiceFactory {
        fn service_name(&self) -> &str {
            CRYPTOOWLS_SERVICE_NAME
        }

        fn make_service(&mut self, _: &Context) -> Box<Service> {
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
        fn state_hash(&self, snapshot: &Snapshot) -> Vec<Hash> {
            let schema = CryptoOwlsSchema::new(snapshot);
            schema.state_hash()
        }

        // Method to deserialize transactions.
        fn tx_from_raw(&self, raw: RawTransaction) -> Result<Box<Transaction>, encoding::Error> {
            let tx = Transactions::tx_from_raw(raw)?;
            Ok(tx.into())
        }

        // Check open auctions state after each block's commit.
        fn after_commit(&self, ctx: &ServiceContext) {
            let current_time = if let Some(time) = transactions::current_time(ctx.snapshot()) {
                time
            } else {
                return;
            };

            let schema = CryptoOwlsSchema::new(ctx.snapshot());
            let open_auctions = schema.owl_auction();
            // Check open auctions and close them if time expires.
            open_auctions.into_iter().for_each(|(_, auction_id)| {
                let auction_state = schema.auctions().get(auction_id).unwrap();
                let (closing_party, sec_key) = (*ctx.public_key(), ctx.secret_key().clone());
                if auction_state.ends_at() <= current_time {
                    let tx = CloseAuction::new(auction_id, &closing_party, current_time, &sec_key);
                    if let Err(e) = ctx.transaction_sender().send(tx.into()) {
                        error!(
                            "Unable to send `CloseAuction` transaction, an error occurred. {}",
                            e
                        );
                    }
                }
            });
        }

        // Handling requests to a node.
        fn wire_api(&self, builder: &mut ServiceApiBuilder) {
            CryptoOwlsApi::wire(builder)
        }
    }
}

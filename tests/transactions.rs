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
extern crate exonum_testkit;
#[macro_use]
extern crate pretty_assertions;

extern crate chrono;
extern crate exonum;
extern crate exonum_cryptoowls as cryptoowls;
extern crate exonum_time;
extern crate rand;

use chrono::{Duration, Utc};

use exonum_time::{time_provider::MockTimeProvider, TimeService};
use std::collections::{HashMap, HashSet};

use exonum::crypto::{self, CryptoHash};
use exonum::helpers::Height;
use exonum_testkit::{TestKit, TestKitBuilder};

use cryptoowls::schema::CryptoOwlsSchema;
use cryptoowls::service::CryptoOwlsService;
use cryptoowls::transactions::*;
use cryptoowls::ISSUE_AMOUNT;

fn init_testkit() -> (TestKit, MockTimeProvider) {
    let mock_provider = MockTimeProvider::default();
    let mut testkit = TestKitBuilder::validator()
        .with_service(CryptoOwlsService)
        .with_service(TimeService::with_provider(mock_provider.clone()))
        .create();

    // TimeService is None if no blocks were forged
    testkit.create_blocks_until(Height(2));

    (testkit, mock_provider)
}

#[test]
fn test_create_user() {
    let (mut testkit, _) = init_testkit();

    let (pubkey, key) = crypto::gen_keypair();

    testkit.create_block_with_transactions(txvec![CreateUser::new(&pubkey, "Alice", &key),]);
    let snapshot = testkit.snapshot();
    let schema = CryptoOwlsSchema::new(&snapshot);
    let user = schema.users().get(&pubkey).expect("No user persisted");

    let owls_idx = schema.owls_state();
    let user_owls_idx = schema.user_owls(&pubkey);

    let owl_states: Vec<_> = owls_idx.iter().map(|x| x.1).collect();
    let user_owls: HashSet<_> = user_owls_idx.iter().map(|x| x.1).collect();

    assert_eq!(user_owls.len(), 2);

    assert_ne!(owl_states[0].owl().dna(), owl_states[1].owl().dna());

    for state in owl_states {
        assert!(user_owls.contains(&state.owl().hash()));
        assert_eq!(*state.owner(), pubkey);
    }

    assert_eq!(*user.public_key(), pubkey);
    assert_eq!(user.name(), "Alice");
    assert_eq!(user.balance(), 100);
}

#[test]
fn test_issue() {
    let (mut testkit, time_machine) = init_testkit();
    let (pubkey, key) = crypto::gen_keypair();
    testkit.create_block_with_transactions(txvec![CreateUser::new(&pubkey, "Scott", &key),]);

    {
        // should be impossible to issue right after creation of user
        testkit.create_block_with_transactions(txvec![Issue::new(&pubkey, Utc::now(), &key)]);

        let snapshot = testkit.snapshot();
        let schema = CryptoOwlsSchema::new(&snapshot);
        let user = schema.users().get(&pubkey).expect("No user persisted");
        assert_eq!(user.balance(), 100);
    }

    {
        // move us into the future
        time_machine.add_time(Duration::seconds(200));
        testkit.create_blocks_until(Height(8));

        testkit.create_block_with_transactions(txvec![Issue::new(
            &pubkey,
            Utc::now() + Duration::seconds(100),
            &key
        ),]);

        let snapshot = testkit.snapshot();
        let schema = CryptoOwlsSchema::new(&snapshot);

        let user = schema.users().get(&pubkey).expect("No user persisted");

        assert_eq!(user.balance(), 200);
    }
}

#[test]
fn test_breeding() {
    let (mut testkit, time_machine) = init_testkit();

    let (pubkey, key) = crypto::gen_keypair();

    testkit.create_block_with_transactions(txvec![CreateUser::new(&pubkey, "Alice", &key),]);

    let snapshot = testkit.snapshot();
    let schema = CryptoOwlsSchema::new(&snapshot);

    let owls_idx = schema.owls_state();
    let owl_states: HashMap<_, _> = owls_idx.iter().collect();

    let user_owls_idx = schema.user_owls(&pubkey);
    let user_owls: Vec<_> = user_owls_idx.iter().map(|o| o.1).collect();

    testkit.create_block_with_transactions(txvec![MakeOwl::new(
        &pubkey,
        "Abel",
        &user_owls[0],
        &user_owls[1],
        Utc::now(),
        &key,
    ),]);

    let snapshot = testkit.snapshot();
    let schema = CryptoOwlsSchema::new(&snapshot);
    let user_owls_idx = schema.user_owls(&pubkey);

    // Can't breed newborns
    let user_owls_count = user_owls_idx.iter().count();
    assert_eq!(user_owls_count, 2);

    // some time should pass
    time_machine.add_time(Duration::seconds(200));
    testkit.create_blocks_until(Height(8));

    // So, now they grew up enough to breed
    testkit.create_block_with_transactions(txvec![MakeOwl::new(
        &pubkey,
        "Abel",
        &user_owls[0],
        &user_owls[1],
        Utc::now(),
        &key,
    ),]);

    let snapshot = testkit.snapshot();
    let schema = CryptoOwlsSchema::new(&snapshot);
    let user_owls_idx = schema.user_owls(&pubkey);
    let owls_idx = schema.owls_state();

    let user_owls_count = user_owls_idx.iter().count();
    assert_eq!(user_owls_count, 3);

    let new_owl_states: Vec<_> = owls_idx.iter().map(|x| x.1).collect();

    for owl_state in new_owl_states {
        let hash = owl_state.owl().hash();
        if let Some(old_owl_state) = owl_states.get(&hash) {
            assert!(old_owl_state.last_breeding() < owl_state.last_breeding());
        } else {
            assert_eq!(owl_state.owl().name(), "Abel");
            // dna should not be the same as parents have
            assert_ne!(owl_state.owl().dna(), 0u32);
            assert_eq!(owl_state.owner(), &pubkey);
        }
    }

    // some time should pass
    time_machine.add_time(Duration::seconds(200));
    testkit.create_blocks_until(Height(16));

    // Shouldn't be able to make owl from one parent
    testkit.create_block_with_transactions(txvec![MakeOwl::new(
        &pubkey,
        "Bastard",
        &user_owls[0],
        &user_owls[0],
        Utc::now(),
        &key,
    ),]);

    let snapshot = testkit.snapshot();
    let schema = CryptoOwlsSchema::new(&snapshot);
    let user_owls_idx = schema.user_owls(&pubkey);
    let user_owls_count = user_owls_idx.iter().count();
    // same as before
    assert_eq!(user_owls_count, 3);
}

#[test]
fn test_sell_owl() {
    let (mut testkit, time_machine) = init_testkit();
    let alice_keys = crypto::gen_keypair();
    let bob_keys = crypto::gen_keypair();
    let jane_keys = crypto::gen_keypair();

    testkit.create_block_with_transactions(txvec![
        CreateUser::new(&alice_keys.0, "Alice", &alice_keys.1),
        CreateUser::new(&bob_keys.0, "Bob", &bob_keys.1),
        CreateUser::new(&jane_keys.0, "Jane", &jane_keys.1),
    ]);

    let snapshot = testkit.snapshot();
    let schema = CryptoOwlsSchema::new(&snapshot);

    let alice_owls = schema.user_owls(&alice_keys.0);
    let alice_owl = alice_owls.iter().map(|x| x.1).next().unwrap();

    // Create auction
    testkit
        .create_block_with_transactions(txvec![CreateAuction::new(
            &alice_keys.0,
            &alice_owl,
            10,
            1_000,
            &alice_keys.1
        )])
        .transactions
        .into_iter()
        .for_each(|tx| tx.status().unwrap());
    // Make bids
    testkit
        .create_block_with_transactions(txvec![
            MakeBid::new(&bob_keys.0, 0, 20, &bob_keys.1),
            MakeBid::new(&jane_keys.0, 0, 30, &jane_keys.1),
        ])
        .transactions
        .into_iter()
        .for_each(|tx| tx.status().unwrap());
    // Check reserved balances
    {
        let snapshot = testkit.snapshot();
        let schema = CryptoOwlsSchema::new(&snapshot);

        let auction = schema.auctions().get(0).unwrap();
        let bob = schema.users().get(&bob_keys.0).unwrap();
        let jane = schema.users().get(&jane_keys.0).unwrap();

        assert_eq!(bob.balance(), ISSUE_AMOUNT);
        assert_eq!(bob.reserved(), 0);
        assert_eq!(jane.balance(), ISSUE_AMOUNT - 30);
        assert_eq!(jane.reserved(), 30);

        assert!(!auction.closed());
        assert_eq!(
            auction.bidding_merkle_root(),
            &schema.auction_bids(0).merkle_root()
        );
        assert_eq!(schema.owl_auction().get(&alice_owl).unwrap(), 0);
    }
    // Try to close auction immediately
    let validators = testkit.network().validators().to_vec();
    let (closing_party, sec_key) = validators[0].service_keypair();
    testkit
        .create_block_with_transactions(txvec![CloseAuction::new(
            0,
            closing_party,
            Utc::now(),
            sec_key
        ),])
        .transactions
        .into_iter()
        .for_each(|tx| {
            tx.status().unwrap_err();
        });
    // Some time should pass
    time_machine.add_time(Duration::seconds(1_001));
    testkit.create_blocks_until(Height(16));
    // Check results
    {
        let snapshot = testkit.snapshot();
        let schema = CryptoOwlsSchema::new(&snapshot);

        let auction = schema.auctions().get(0).unwrap();
        let alice = schema.users().get(&alice_keys.0).unwrap();
        let bob = schema.users().get(&bob_keys.0).unwrap();
        let jane = schema.users().get(&jane_keys.0).unwrap();

        assert_eq!(bob.balance(), ISSUE_AMOUNT);
        assert_eq!(bob.reserved(), 0);
        assert_eq!(jane.balance(), ISSUE_AMOUNT - 30);
        assert_eq!(jane.reserved(), 0);
        assert_eq!(alice.balance(), ISSUE_AMOUNT + 30);
        assert_eq!(alice.reserved(), 0);

        assert!(auction.closed());
        assert!(schema.owl_auction().get(&alice_owl).is_none());
        assert!(schema.user_owls(&jane_keys.0).contains_by_hash(&alice_owl));
        assert!(!schema.user_owls(&alice_keys.0).contains_by_hash(&alice_owl));
        assert_eq!(
            schema.owls_state().get(&alice_owl).unwrap().owner(),
            &jane_keys.0
        );
    }
}

#[test]
fn test_two_bids_same_user() {
    let (mut testkit, _) = init_testkit();
    let bob_keys = crypto::gen_keypair();
    let jane_keys = crypto::gen_keypair();

    testkit.create_block_with_transactions(txvec![
        CreateUser::new(&bob_keys.0, "Bob", &bob_keys.1),
        CreateUser::new(&jane_keys.0, "Jane", &jane_keys.1),
    ]);

    let snapshot = testkit.snapshot();
    let schema = CryptoOwlsSchema::new(&snapshot);

    let bob_owls = schema.user_owls(&bob_keys.0);
    let bob_owl = bob_owls.iter().map(|x| x.1).next().unwrap();
    // Create auction
    testkit
        .create_block_with_transactions(txvec![CreateAuction::new(
            &bob_keys.0,
            &bob_owl,
            10,
            1_000,
            &bob_keys.1
        )])
        .transactions
        .into_iter()
        .for_each(|tx| tx.status().unwrap());
    // Make bids
    testkit
        .create_block_with_transactions(txvec![
            MakeBid::new(&jane_keys.0, 0, 20, &jane_keys.1),
            MakeBid::new(&jane_keys.0, 0, 30, &jane_keys.1),
        ])
        .transactions
        .into_iter()
        .for_each(|tx| tx.status().unwrap());
    // Check reserved balances
    {
        let snapshot = testkit.snapshot();
        let schema = CryptoOwlsSchema::new(&snapshot);

        let jane = schema.users().get(&jane_keys.0).unwrap();

        assert_eq!(jane.balance(), ISSUE_AMOUNT - 30);
        assert_eq!(jane.reserved(), 30);
    }
}

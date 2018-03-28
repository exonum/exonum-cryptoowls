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

extern crate exonum;
extern crate exonum_time;

extern crate exonum_cryptoowls as cryptoowls;
#[macro_use]
extern crate exonum_testkit;
extern crate rand;

use std::collections::{HashMap, HashSet};
use std::time::{Duration, SystemTime};
use exonum_time::{MockTimeProvider, TimeService};

use exonum::crypto::{self, CryptoHash};
use exonum_testkit::{TestKit, TestKitBuilder};
use exonum::helpers::Height;

use cryptoowls::schema::CryptoOwlsSchema;
use cryptoowls::service::CryptoOwlsService;
use cryptoowls::transactions::*;

fn init_testkit() -> (TestKit, MockTimeProvider) {
    let mock_provider = MockTimeProvider::new(SystemTime::now());
    let mut testkit = TestKitBuilder::validator()
        .with_service(CryptoOwlsService::new())
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
        testkit.create_block_with_transactions(
            txvec![Issue::new(&pubkey, SystemTime::now(), &key)],
        );

        let snapshot = testkit.snapshot();
        let schema = CryptoOwlsSchema::new(&snapshot);
        let user = schema.users().get(&pubkey).expect("No user persisted");
        assert_eq!(user.balance(), 100);
    }

    {
        // move us into the future
        time_machine.add_time(Duration::new(200, 0));
        testkit.create_blocks_until(Height(8));

        testkit.create_block_with_transactions(txvec![
            Issue::new(&pubkey, SystemTime::now() + Duration::new(100, 0), &key),
        ]);

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

    testkit.create_block_with_transactions(txvec![
        MakeOwl::new(
            &pubkey,
            "Abel",
            &user_owls[0],
            &user_owls[1],
            SystemTime::now(),
            &key,
        ),
    ]);

    let snapshot = testkit.snapshot();
    let schema = CryptoOwlsSchema::new(&snapshot);
    let user_owls_idx = schema.user_owls(&pubkey);

    // Can't breed newborns
    let user_owls_count = user_owls_idx.iter().count();
    assert_eq!(user_owls_count, 2);

    // some time should pass
    time_machine.add_time(Duration::new(200, 0));
    testkit.create_blocks_until(Height(8));

    // So, now they grew up enough to breed
    testkit.create_block_with_transactions(txvec![
        MakeOwl::new(
            &pubkey,
            "Abel",
            &user_owls[0],
            &user_owls[1],
            SystemTime::now(),
            &key,
        ),
    ]);

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
}

#[test]
fn test_sell_owl() {
    let (mut testkit, _) = init_testkit();
    let (pubkey, key) = crypto::gen_keypair();
    let (pubkey_1, key_1) = crypto::gen_keypair();
    let (pubkey_2, key_2) = crypto::gen_keypair();

    testkit.create_block_with_transactions(txvec![
        CreateUser::new(&pubkey, "Alice", &key),
        CreateUser::new(&pubkey_1, "Bob", &key_1),
        CreateUser::new(&pubkey_2, "Jane", &key_2),
    ]);

    let snapshot = testkit.snapshot();
    let schema = CryptoOwlsSchema::new(&snapshot);

    let alice_owls = schema.user_owls(&pubkey);
    let alice_owl = alice_owls.iter().map(|x| x.1).next().unwrap();

    let bob_owls = schema.user_owls(&pubkey_1);
    let bob_owl = bob_owls.iter().map(|x| x.1).next().unwrap();

    // Should be impossible to place order on one's own owl
    {
        testkit.create_block_with_transactions(txvec![
            CreateOrder::new(&pubkey, &alice_owl, 0, SystemTime::now(), &key),
        ]);

        let snapshot = testkit.snapshot();
        let schema = CryptoOwlsSchema::new(&snapshot);

        let user_orders = schema.user_orders(&pubkey);
        let user_orders_cnt = user_orders.iter().count();

        let owl_orders = schema.owl_orders(&alice_owl);
        let owl_orders_cnt = owl_orders.iter().count();

        assert_eq!(user_orders_cnt, 0);
        assert_eq!(owl_orders_cnt, 0);
    }

    {
        testkit.create_block_with_transactions(txvec![
            CreateOrder::new(&pubkey_1, &alice_owl, 10, SystemTime::now(), &key_1),
            CreateOrder::new(&pubkey_2, &alice_owl, 90, SystemTime::now(), &key_2),
            CreateOrder::new(&pubkey_2, &alice_owl, 60, SystemTime::now(), &key_2),
            CreateOrder::new(&pubkey_2, &bob_owl, 90, SystemTime::now(), &key_2),
        ]);

        let snapshot = testkit.snapshot();
        let schema = CryptoOwlsSchema::new(&snapshot);

        let alice_orders = schema.user_orders(&pubkey);
        let alice_orders: Vec<_> = alice_orders.iter().collect();

        let bob_orders = schema.user_orders(&pubkey_1);
        let bob_orders: Vec<_> = bob_orders.iter().collect();

        let jane_orders = schema.user_orders(&pubkey_2);
        let jane_orders: Vec<_> = jane_orders.iter().collect();

        let alice_owl_orders = schema.owl_orders(&alice_owl);
        let alice_owl_orders: Vec<_> = alice_owl_orders.iter().collect();

        let bob_owl_orders = schema.owl_orders(&bob_owl);
        let bob_owl_orders: Vec<_> = bob_owl_orders.iter().collect();

        assert_eq!(alice_orders.len(), 0);
        assert_eq!(bob_orders.len(), 1);
        assert_eq!(jane_orders.len(), 3);
        assert_eq!(alice_owl_orders.len(), 3);
        assert_eq!(bob_owl_orders.len(), 1);

        // Bob sells his owl to Alice
        testkit.create_block_with_transactions(txvec![
            AcceptOrder::new(&pubkey_1, &bob_owl_orders[0], &key_1),
        ]);

        let snapshot = testkit.snapshot();
        let schema = CryptoOwlsSchema::new(&snapshot);

        let ex_bob_owl_orders = schema.owl_orders(&bob_owl);
        let ex_bob_owl_orders: Vec<_> = ex_bob_owl_orders.iter().collect();
        let orders = schema.orders();
        let accepted_order = orders.get(&ex_bob_owl_orders[0]).unwrap();
        assert_eq!(accepted_order.status(), "accepted");

        let jane_owls = schema.user_owls(&pubkey_2);
        let jane_owls: Vec<_> = jane_owls.iter().map(|x| x.0).collect();
        assert_eq!(jane_owls.len(), 3);

        let owls = schema.owls_state();
        for owl_hash in jane_owls {
            let owl = owls.get(&owl_hash).unwrap();
            assert_eq!(owl.owner(), &pubkey_2);
        }

        let bob_owls = schema.user_owls(&pubkey_1);
        let bob_owls: Vec<_> = bob_owls.iter().map(|x| x.0).collect();
        assert_eq!(bob_owls.len(), 1);

        let jane = schema.users().get(&pubkey_2).unwrap();
        assert_eq!(jane.balance(), 10);

        let bob = schema.users().get(&pubkey_1).unwrap();
        assert_eq!(bob.balance(), 190);

        // Alice makes attempt to sell her owl to Jane
        let mut jane_orders_iter = jane_orders.iter().filter(|&x| x != &bob_owl_orders[0]);

        let jane_order_id = jane_orders_iter.next().unwrap();
        let jane_order_id_2 = jane_orders_iter.next().unwrap();

        testkit.create_block_with_transactions(txvec![
            AcceptOrder::new(&pubkey, &jane_order_id, &key),
        ]);

        // But unfortunately Jane doesn't have required amount of money now

        let snapshot = testkit.snapshot();
        let schema = CryptoOwlsSchema::new(&snapshot);

        let orders = schema.orders();
        let declined_order = orders.get(&jane_order_id).unwrap();
        assert_eq!(declined_order.status(), "declined");

        // Check if second Jane's order is still in pending state
        let pending_order = orders.get(&jane_order_id_2).unwrap();
        assert_eq!(pending_order.status(), "pending");

        // Jane still has the same amount of owls as before
        let jane_owls = schema.user_owls(&pubkey_2);
        let jane_owls: Vec<_> = jane_owls.iter().map(|x| x.0).collect();
        assert_eq!(jane_owls.len(), 3);

        let alice_owls = schema.user_owls(&pubkey);
        let alice_owls_cnt = alice_owls.iter().count();
        assert_eq!(alice_owls_cnt, 2);

        // Now Alice is selling her owl to Bob
        let bob_order_id = bob_orders[0];

        testkit.create_block_with_transactions(txvec![
            AcceptOrder::new(&pubkey, &bob_order_id, &key),
        ]);

        let snapshot = testkit.snapshot();
        let schema = CryptoOwlsSchema::new(&snapshot);
        let orders = schema.orders();

        // So, second Jane's order should be in declined state now, cause
        // owl has been just sold to Bob
        let declined_order = orders.get(&jane_order_id_2).unwrap();
        assert_eq!(declined_order.status(), "declined");

        // Bob's order should be in accepted state
        let accepted_order = orders.get(&bob_order_id).unwrap();
        assert_eq!(accepted_order.status(), "accepted");

        // And he has 2 owls again
        let bob_owls = schema.user_owls(&pubkey_1);
        let bob_owls_cnt = bob_owls.iter().count();
        assert_eq!(bob_owls_cnt, 2);

        let alice_owls = schema.user_owls(&pubkey);
        let alice_owls_cnt = alice_owls.iter().count();
        assert_eq!(alice_owls_cnt, 1);
    }
}

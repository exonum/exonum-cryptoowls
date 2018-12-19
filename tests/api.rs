// // Copyright 2018 The Exonum Team
// //
// // Licensed under the Apache License, Version 2.0 (the "License");
// // you may not use this file except in compliance with the License.
// // You may obtain a copy of the License at
// //
// //   http://www.apache.org/licenses/LICENSE-2.0
// //
// // Unless required by applicable law or agreed to in writing, software
// // distributed under the License is distributed on an "AS IS" BASIS,
// // WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// // See the License for the specific language governing permissions and
// // limitations under the License.

use chrono::Utc;
use serde_json::json;

use exonum::api::node::public::explorer::TransactionResponse;
use exonum::crypto::{self, PublicKey, SecretKey};
use exonum::helpers::Height;
use exonum::messages::{to_hex_string, Message, ServiceTransaction};
use exonum_testkit::{ApiKind, TestKit, TestKitApi, TestKitBuilder};
use exonum_time::TimeService;

use exonum_cryptoowls::service::CryptoOwlsService;
use exonum_cryptoowls::transactions::*;
use exonum_cryptoowls::CRYPTOOWLS_SERVICE_ID;

fn init_testkit() -> (TestKit, TestKitApi) {
    let mut testkit = TestKitBuilder::validator()
        .with_service(CryptoOwlsService)
        .with_service(TimeService::default())
        .create();

    // TimeService is None if no blocks were forged
    testkit.create_blocks_until(Height(2));

    let api = testkit.api();
    (testkit, api)
}

fn post_transaction(
    api: &TestKitApi,
    tx: impl Into<ServiceTransaction>,
    pk: PublicKey,
    sk: &SecretKey,
) {
    let signed = Message::sign_transaction(tx, CRYPTOOWLS_SERVICE_ID, pk, sk);
    let data = to_hex_string(&signed);
    let response: TransactionResponse = api
        .public(ApiKind::Explorer)
        .query(&json!({ "tx_body": data }))
        .post("v1/transactions")
        .unwrap();
    assert_eq!(response.tx_hash, signed.hash());
}

#[test]
fn test_tx_create_user() {
    let (_testkit, api) = init_testkit();
    let keypair = crypto::gen_keypair();
    post_transaction(
        &api,
        CreateUser {
            name: "Alice".to_owned(),
        },
        keypair.0,
        &keypair.1,
    );
}

#[test]
fn test_tx_make_owl() {
    let (_testkit, api) = init_testkit();
    let keypair = crypto::gen_keypair();
    post_transaction(
        &api,
        MakeOwl {
            name: "owl".to_owned(),
            father_id: crypto::Hash::zero(),
            mother_id: crypto::Hash::zero(),
            seed: Utc::now(),
        },
        keypair.0,
        &keypair.1,
    );
}

#[test]
fn test_tx_issue() {
    let (_testkit, api) = init_testkit();
    let keypair = crypto::gen_keypair();
    post_transaction(&api, Issue { seed: Utc::now() }, keypair.0, &keypair.1);
}

#[test]
fn test_tx_create_auction() {
    let (_testkit, api) = init_testkit();
    let keypair = crypto::gen_keypair();
    post_transaction(
        &api,
        CreateAuction {
            owl_id: crypto::Hash::zero(),
            start_price: 0,
            duration: 10,
        },
        keypair.0,
        &keypair.1,
    );
}

#[test]
fn test_tx_make_bid() {
    let (_testkit, api) = init_testkit();
    let keypair = crypto::gen_keypair();
    post_transaction(
        &api,
        MakeBid {
            auction_id: 0,
            value: 1,
        },
        keypair.0,
        &keypair.1,
    );
}

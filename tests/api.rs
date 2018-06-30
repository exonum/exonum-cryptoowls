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
extern crate pretty_assertions;
#[macro_use]
extern crate serde_json;

extern crate chrono;
extern crate exonum;
extern crate exonum_cryptoowls as cryptoowls;
extern crate exonum_testkit;
extern crate exonum_time;
extern crate rand;

use chrono::Utc;

use exonum_time::TimeService;

use exonum::crypto::{self, CryptoHash};
use exonum::helpers::Height;
use exonum_testkit::{ApiKind, TestKit, TestKitApi, TestKitBuilder};

use cryptoowls::service::CryptoOwlsService;
use cryptoowls::transactions::*;
use cryptoowls::CRYPTOOWLS_SERVICE_NAME;

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

#[test]
fn test_tx_create_user() {
    let (_testkit, api) = init_testkit();
    let keypair = crypto::gen_keypair();
    let tx = CreateUser::new(&keypair.0, "user", &keypair.1);

    let response: serde_json::Value = api.post(
        ApiKind::Service(CRYPTOOWLS_SERVICE_NAME),
        "/v1/transaction",
        &tx,
    );
    assert_eq!(response, json!({ "tx_hash": tx.hash() }));
}

#[test]
fn test_tx_make_owl() {
    let (_testkit, api) = init_testkit();
    let keypair = crypto::gen_keypair();
    let tx = MakeOwl::new(
        &keypair.0,
        "owl",
        &crypto::Hash::zero(),
        &crypto::Hash::zero(),
        Utc::now(),
        &keypair.1,
    );

    let response: serde_json::Value = api.post(
        ApiKind::Service(CRYPTOOWLS_SERVICE_NAME),
        "/v1/transaction",
        &tx,
    );
    assert_eq!(response, json!({ "tx_hash": tx.hash() }));
}

#[test]
fn test_tx_issue() {
    let (_testkit, api) = init_testkit();
    let keypair = crypto::gen_keypair();
    let tx = Issue::new(&keypair.0, Utc::now(), &keypair.1);

    let response: serde_json::Value = api.post(
        ApiKind::Service(CRYPTOOWLS_SERVICE_NAME),
        "/v1/transaction",
        &tx,
    );
    assert_eq!(response, json!({ "tx_hash": tx.hash() }));
}

#[test]
fn test_tx_create_auction() {
    let (_testkit, api) = init_testkit();
    let keypair = crypto::gen_keypair();
    let tx = CreateAuction::new(&keypair.0, &crypto::Hash::zero(), 0, 10, &keypair.1);

    let response: serde_json::Value = api.post(
        ApiKind::Service(CRYPTOOWLS_SERVICE_NAME),
        "/v1/transaction",
        &tx,
    );
    assert_eq!(response, json!({ "tx_hash": tx.hash() }));
}

#[test]
fn test_tx_make_bid() {
    let (_testkit, api) = init_testkit();
    let keypair = crypto::gen_keypair();
    let tx = MakeBid::new(&keypair.0, 0, 1, &keypair.1);

    let response: serde_json::Value = api.post(
        ApiKind::Service(CRYPTOOWLS_SERVICE_NAME),
        "/v1/transaction",
        &tx,
    );
    assert_eq!(response, json!({ "tx_hash": tx.hash() }));
}

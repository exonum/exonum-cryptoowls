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
extern crate serde_json;

extern crate chrono;
extern crate exonum;
extern crate exonum_cryptoowls as cryptoowls;
extern crate exonum_time;
extern crate rand;

use chrono::{Duration, Utc};

use std::collections::{HashMap, HashSet};
use exonum_time::{MockTimeProvider, TimeService};

use exonum::crypto::{self, CryptoHash};
use exonum_testkit::{ApiKind, TestKit, TestKitApi, TestKitBuilder};
use exonum::helpers::Height;

use cryptoowls::{CRYPTOOWLS_SERVICE_NAME, ISSUE_AMOUNT};
use cryptoowls::schema::CryptoOwlsSchema;
use cryptoowls::service::CryptoOwlsService;
use cryptoowls::data_layout::*;
use cryptoowls::transactions::*;

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
fn test_tx_create_auction() {
    let (mut testkit, mut api) = init_testkit();
    let keypair = crypto::gen_keypair();
    let auction = Auction::new(&keypair.0, &crypto::Hash::zero(), 0, 10);
    let tx = CreateAuction::new(auction, &keypair.1);

    let response: serde_json::Value = api.post(
            ApiKind::Service(CRYPTOOWLS_SERVICE_NAME),
            "/v1/transaction",
            &tx
        );
    assert_eq!(
        response,
        json!({ "tx_hash": tx.hash() })
    );
}

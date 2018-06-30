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
extern crate exonum_configuration;
extern crate exonum_time;

extern crate exonum_cryptoowls as cryptoowls;

use cryptoowls::service::CryptoOwlsServiceFactory;
use exonum::helpers;
use exonum::helpers::fabric::NodeBuilder;
use exonum_configuration::ServiceFactory as ConfigurationServiceFactory;
use exonum_time::TimeServiceFactory;

fn main() {
    exonum::crypto::init();
    helpers::init_logger().unwrap();
    let node = NodeBuilder::new()
        .with_service(Box::new(ConfigurationServiceFactory))
        .with_service(Box::new(TimeServiceFactory))
        .with_service(Box::new(CryptoOwlsServiceFactory));
    node.run();
}

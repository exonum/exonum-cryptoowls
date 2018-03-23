extern crate exonum;
extern crate exonum_configuration;
extern crate exonum_time;

extern crate exonum_cryptoowls as cryptoowls;

use exonum::helpers;
use exonum::helpers::fabric::NodeBuilder;
use exonum_time::TimeServiceFactory;
use exonum_configuration::ServiceFactory as ConfigurationServiceFactory;
use cryptoowls::service::CryptoOwlsServiceFactory;

fn main() {
    exonum::crypto::init();
    helpers::init_logger().unwrap();
    let node = NodeBuilder::new()
        .with_service(Box::new(ConfigurationServiceFactory))
        .with_service(Box::new(TimeServiceFactory))
        .with_service(Box::new(CryptoOwlsServiceFactory));
    node.run();
}

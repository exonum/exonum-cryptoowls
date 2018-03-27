extern crate exonum;

#[macro_use]
extern crate log;
extern crate serde_json;

extern crate exonum_cryptoowls as owls;

use exonum::crypto;
use exonum::encoding::serialize::json::ExonumJson;

use serde_json::to_string_pretty;

fn main() {
    let (public_key, secret_key) = crypto::gen_keypair();
    // need possibility to read keys from command line

    println!(
        "Keys are {:#?} {:#?}",
        public_key.to_hex(),
        secret_key.to_hex()
    );
    let tx = owls::transactions::CreateUser::new(&public_key, "Me", &secret_key);
    println!(
        "{}",
        to_string_pretty(&tx.serialize_field().unwrap()).unwrap()
    );
}

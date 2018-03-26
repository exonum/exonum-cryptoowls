# exonum-cryptoowls

Cryptoowls demo created with [Exonum blockchain](https://github.com/exonum/exonum).

## Getting started

Be sure you installed necessary packages:

* [git](https://git-scm.com/downloads)
* [Rust compiler](https://rustup.rs/)

## Run

Below you will find a step-by-step guide to starting the cryptoowls demo
service on 4 nodes on the local machine.

Clone the project:

```sh
git clone https://github.com/exonum/exonum-cryptoowls

```

Generate template:

```sh
cd exonum-cryptoowls

mkdir example

cargo run -- generate-template example/common.toml
```

Generate public and secrets keys for each node:

```sh
cargo run -- generate-config example/common.toml  example/pub_1.toml example/sec_1.toml --peer-addr 127.0.0.1:6331

cargo run -- generate-config example/common.toml  example/pub_2.toml example/sec_2.toml --peer-addr 127.0.0.1:6332

cargo run -- generate-config example/common.toml  example/pub_3.toml example/sec_3.toml --peer-addr 127.0.0.1:6333

cargo run -- generate-config example/common.toml  example/pub_4.toml example/sec_4.toml --peer-addr 127.0.0.1:6334
```

Finalize configs:

```sh
cargo run -- finalize --public-api-address 0.0.0.0:8200 --private-api-address 0.0.0.0:8091 example/sec_1.toml node_1_cfg.toml --public-configs example/pub_1.toml example/pub_2.toml example/pub_3.toml example/pub_4.toml

cargo run -- finalize --public-api-address 0.0.0.0:8201 --private-api-address 0.0.0.0:8092 example/sec_2.toml node_2_cfg.toml --public-configs example/pub_1.toml example/pub_2.toml example/pub_3.toml example/pub_4.toml

cargo run -- finalize --public-api-address 0.0.0.0:8202 --private-api-address 0.0.0.0:8093 example/sec_3.toml node_3_cfg.toml --public-configs example/pub_1.toml example/pub_2.toml example/pub_3.toml example/pub_4.toml

cargo run -- finalize --public-api-address 0.0.0.0:8203 --private-api-address 0.0.0.0:8094 example/sec_4.toml node_4_cfg.toml --public-configs example/pub_1.toml example/pub_2.toml example/pub_3.toml example/pub_4.toml
```

Run nodes:

```sh
cargo run -- run --node-config node_1_cfg.toml --db-path example/db1 --public-api-address 0.0.0.0:8200

cargo run -- run --node-config node_2_cfg.toml --db-path example/db2 --public-api-address 0.0.0.0:8201

cargo run -- run --node-config node_3_cfg.toml --db-path example/db3 --public-api-address 0.0.0.0:8202

cargo run -- run --node-config node_4_cfg.toml --db-path example/db4 --public-api-address 0.0.0.0:8203
```

## License

Cryptoowls demo is licensed under the Apache License (Version 2.0).
See [LICENSE](LICENSE) for details.

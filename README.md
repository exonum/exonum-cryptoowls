# Криптосовы Exonum

Демо Криптосовы, созданное на блокчейне [Exonum blockchain](https://github.com/exonum/exonum).

## Необходимые зависимости

Убедитесь, что Вы установили следующие зависимости:

* [git](https://git-scm.com/downloads)
* [Компилятор Rust](https://rustup.rs/)
* [Node.js & npm](https://nodejs.org/en/download/)

## Сборка и запуск

Пошаговое руководство по запуску демо приложения на 4-х узлах на локальной машине.

Склонируйте директорию с проектом и установите зависимости:

```sh
git clone https://github.com/exonum/exonum-cryptoowls

cd exonum-cryptoowls

cargo install
```

Сгенерируйте конфигурацию блокчейна:

```sh
mkdir example

cargo run -- generate-template example/common.toml
```

Сгенерируйте шаблоны конфигураций узлов:

```sh
cargo run -- generate-config example/common.toml  example/pub_1.toml example/sec_1.toml --peer-address 127.0.0.1:6331

cargo run -- generate-config example/common.toml  example/pub_2.toml example/sec_2.toml --peer-address 127.0.0.1:6332

cargo run -- generate-config example/common.toml  example/pub_3.toml example/sec_3.toml --peer-address 127.0.0.1:6333

cargo run -- generate-config example/common.toml  example/pub_4.toml example/sec_4.toml --peer-address 127.0.0.1:6334
```

Завершите генерацию конфигураций узлов:

```sh
cargo run -- finalize --public-api-address 0.0.0.0:8200 --private-api-address 0.0.0.0:8091 example/sec_1.toml example/node_1_cfg.toml --public-configs example/pub_1.toml example/pub_2.toml example/pub_3.toml example/pub_4.toml

cargo run -- finalize --public-api-address 0.0.0.0:8201 --private-api-address 0.0.0.0:8092 example/sec_2.toml example/node_2_cfg.toml --public-configs example/pub_1.toml example/pub_2.toml example/pub_3.toml example/pub_4.toml

cargo run -- finalize --public-api-address 0.0.0.0:8202 --private-api-address 0.0.0.0:8093 example/sec_3.toml example/node_3_cfg.toml --public-configs example/pub_1.toml example/pub_2.toml example/pub_3.toml example/pub_4.toml

cargo run -- finalize --public-api-address 0.0.0.0:8203 --private-api-address 0.0.0.0:8094 example/sec_4.toml example/node_4_cfg.toml --public-configs example/pub_1.toml example/pub_2.toml example/pub_3.toml example/pub_4.toml
```

Запустите узлы:

```sh
cargo run -- run --node-config example/node_1_cfg.toml --db-path example/db1 --public-api-address 0.0.0.0:8200

cargo run -- run --node-config example/node_2_cfg.toml --db-path example/db2 --public-api-address 0.0.0.0:8201

cargo run -- run --node-config example/node_3_cfg.toml --db-path example/db3 --public-api-address 0.0.0.0:8202

cargo run -- run --node-config example/node_4_cfg.toml --db-path example/db4 --public-api-address 0.0.0.0:8203
```

Установите клиентские зависимости:

```sh
cd frontend

npm install
```

Соберите клиентское приложение:

```sh
npm run build
```

Запустите:

```sh
npm start -- --port=3000 --api-root=http://127.0.0.1:8200
```

`--port` порт для Node.js приложения.

`--api-root` корневой URL публичного API адреса узла.

Готово! Приложение доступно по адресу [http://127.0.0.1:3000](http://127.0.0.1:3000).

## Лицензия

Демоприложение Криптосовы лицензированно под лицензией Apache (Version 2.0).
Смотрите файл [LICENSE](LICENSE) для деталей.

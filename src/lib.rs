extern crate bodyparser;
#[macro_use]
extern crate exonum;
extern crate exonum_time;
extern crate iron;

extern crate router;

extern crate serde;

#[macro_use]
extern crate serde_json;

extern crate byteorder;
extern crate rand;

/// Некоторый уникальный идентификатор сервиса.
pub const CRYPTOOWLS_SERVICE_ID: u16 = 521;
/// Уникальное имя сервиса, которое будет использоваться в апи и конфигурации.
pub const CRYPTOOWLS_SERVICE_NAME: &str = "cryptoowls";

/// Сумма пополнения баланса
pub const ISSUE_AMMOUNT: u64 = 100;

/// Таймаут, после которого разрешено повторное пополнение баланса.
pub const ISSUE_TIMEOUT: u64 = 5 * 60;

/// Таймаут, после которого разрешено повторное размножение.
pub const BREEDING_TIMEOUT: u64 = 5 * 60;

/// Стоимость размножения
pub const BREEDING_PRICE: u64 = 42;

/// Модуль со структурами данных, которые хранятся в блокчейне
mod data_layout {

    use std::time::SystemTime;
    use exonum::crypto::{Hash, PublicKey};

    use byteorder::{BigEndian, ReadBytesExt};

    use std::io::Cursor;
    use rand::{IsaacRng, Rng, SeedableRng};
    use rand::distributions::{Sample, Weighted, WeightedChoice};

    encoding_struct! {
        /// Интересующая нас криптосова, ее уникальный идентифицатор вычисляется как хеш
        /// от этой структуры данных.
        struct CryptoOwl {
            /// Имя совы (должно быть уникальным)
            name: &str,
            /// Генетический код криптосовы.
            dna: u32,
        }
    }

    encoding_struct! {
        /// Текущее состоянии криптосовы
        struct CryptoOwlState {
            /// Сама сова
            owl: CryptoOwl,
            /// Владелец.
            owner: &PublicKey,
            /// Время последнего разведения.
            last_breeding: SystemTime,
        }
    }

    encoding_struct! {
        /// Данные о пользователи и его совах
        struct User {
            /// Его публичный ключ
            public_key: &PublicKey,
            /// Его имя
            name: &str,
            /// Текущий баланс
            balance: u64,
            /// Время последнего пополнения баланса
            last_fillup: SystemTime,
        }
    }

    encoding_struct! {
        /// Ордер на покупку совы
        struct Order {
            /// Тот, кто создал ордер
            public_key: &PublicKey,
            /// Идентификатор совы
            owl_id: &Hash,
            /// pending - в ожидании, accepted - исполнен, declined - отвергнут.
            status: &str,
            /// Цена на сову
            price: u64,
        }
    }

    impl CryptoOwl {
        pub fn breed(&self, other: &CryptoOwl, name: &str, hash_seed: &Hash) -> CryptoOwl {
            // мы можем получить хэш только как [u8:32], чтоб получить что-то,
            // что можно использовать для сидирования генератора случайных чисел,
            // нужно собрать из каждых 4-х u8 один u32

            let hash_seed: &[u8] = hash_seed.as_ref();
            let mut seed = [0u32; 4];
            let mut cursor = Cursor::new(hash_seed);
            for i in 0..4 {
                seed[i] = cursor.read_u32::<BigEndian>().unwrap();
            }
            let mut rng = IsaacRng::from_seed(&seed);
            let mut son_dna = 0u32;

            for i in 0..32 {
                // проходим по всем `генам` и выставляем их в соответствии с генами родителей
                let mask = 2u32.pow(i);
                let (fg, mg) = (self.dna() & mask, other.dna() & mask);
                if fg == mg {
                    // Если биты у родителей совпадают, то с вероятностью
                    // 8/10 бит ребенка будет таким же
                    let mut possible_genes = vec![
                        Weighted {
                            weight: 8,
                            item: fg,
                        },
                        Weighted {
                            weight: 2,
                            item: fg ^ mask,
                        },
                    ];

                    let mut choices = WeightedChoice::new(&mut possible_genes);
                    son_dna |= choices.sample(&mut rng);
                } else {
                    // Если биты различаются, то результирующий бит будет
                    // выбираться с вероятностью 1/2.
                    if rng.gen() {
                        son_dna |= mask;
                    }
                }
            }

            CryptoOwl::new(name, son_dna)
        }
    }

}

// схема данных для базы
mod schema {
    use data_layout::{CryptoOwlState, Order, User};
    use exonum::storage::{Fork, ListIndex, ProofMapIndex, Snapshot, ValueSetIndex};
    use exonum::blockchain::gen_prefix;
    use exonum::crypto::{Hash, PublicKey};

    /// Schema of the key-value storage used by the cryptoowls service.
    pub struct CryptoOwlsSchema<T> {
        view: T,
    }

    impl<T> CryptoOwlsSchema<T>
    where
        T: AsRef<Snapshot>,
    {
        pub fn new(view: T) -> Self {
            CryptoOwlsSchema { view }
        }

        pub fn users_proof(&self) -> ProofMapIndex<&T, PublicKey, User> {
            ProofMapIndex::new("cryptoowls.users", &self.view)
        }

        pub fn owls_state_proof(&self) -> ProofMapIndex<&T, Hash, CryptoOwlState> {
            ProofMapIndex::new("cryptoowls.owls_state", &self.view)
        }

        pub fn orders_proof(&self) -> ProofMapIndex<&T, Hash, Order> {
            ProofMapIndex::new("cryptoowls.orders", &self.view)
        }

        pub fn users_owls_view(&self, public_key: &PublicKey) -> ValueSetIndex<&T, Hash> {
            ValueSetIndex::with_prefix("cryptoowls.users_owls", gen_prefix(public_key), &self.view)
        }

        pub fn user_orders_view(&self, public_key: &PublicKey) -> ListIndex<&T, Hash> {
            ListIndex::with_prefix("cryptoowls.user_orders", gen_prefix(public_key), &self.view)
        }

        pub fn owl_orders_view(&self, owl_id: &Hash) -> ListIndex<&T, Hash> {
            ListIndex::with_prefix("cryptoowls.owl_orders", gen_prefix(owl_id), &self.view)
        }
    }

    impl<'a> CryptoOwlsSchema<&'a mut Fork> {
        pub fn users(&mut self) -> ProofMapIndex<&mut Fork, PublicKey, User> {
            ProofMapIndex::new("cryptoowls.users", self.view)
        }

        pub fn owls_state(&mut self) -> ProofMapIndex<&mut Fork, Hash, CryptoOwlState> {
            ProofMapIndex::new("cryptoowls.owls_state", self.view)
        }

        pub fn orders(&mut self) -> ProofMapIndex<&mut Fork, Hash, Order> {
            ProofMapIndex::new("cryptoowls.orders", self.view)
        }

        pub fn users_owls(&mut self, public_key: &PublicKey) -> ValueSetIndex<&mut Fork, Hash> {
            ValueSetIndex::with_prefix("cryptoowls.users_owls", gen_prefix(public_key), self.view)
        }

        pub fn user_orders(&mut self, public_key: &PublicKey) -> ListIndex<&mut Fork, Hash> {
            ListIndex::with_prefix("cryptoowls.user_orders", gen_prefix(public_key), self.view)
        }

        pub fn owl_orders(&mut self, owl_id: &Hash) -> ListIndex<&mut Fork, Hash> {
            ListIndex::with_prefix("cryptoowls.owl_orders", gen_prefix(owl_id), self.view)
        }
    }
}
/// Модуль с описанием транзакций для демки.
pub mod transactions {
    use exonum::crypto::{CryptoHash, Hash, PublicKey};
    use exonum::blockchain::{ExecutionResult, Schema, Transaction};
    use exonum::storage::Fork;
    use exonum::messages::Message;

    use schema;
    use data_layout::{CryptoOwlState, Order, User};
    use exonum_time::TimeSchema;

    use std::time::SystemTime;

    use {BREEDING_PRICE, BREEDING_TIMEOUT, CRYPTOOWLS_SERVICE_ID, ISSUE_AMMOUNT, ISSUE_TIMEOUT};

    transactions! {
        pub Transactions {
            const SERVICE_ID = CRYPTOOWLS_SERVICE_ID;
            /// Транзакция создания пользователя
            struct CreateUser {
                /// Публичный идентификатор пользователя
                public_key: &PublicKey,
                /// Имя
                name: &str,
            }
            /// Транзакция создания совы. Если идентификаторы отца и матери это нули,
            /// то выводится базовая сова
            struct MakeOwl {
                /// Публичный идентификатор пользователя
                public_key: &PublicKey,
                /// Имя совенка
                name: &str,
                /// Идентификатор отца
                father_id: &Hash,
                /// Идентификатор матери
                mother_id: &Hash,
            }
            /// Транзакция запроса новых средств
            struct Issue {
                /// Публичный идентификатор пользователя
                public_key: &PublicKey,
                /// Текущее время пользователя (нужно только для обхода replay защиты)
                current_time: SystemTime,
            }
            /// Транзакция размещения нового ордера
            struct CreateOrder
            {
                /// Публичный идентификатор пользователя
                public_key: &PublicKey,
                /// Идентификатор совы
                owl_id: &Hash,
                /// Желаемая цена
                price: u64,
                /// Текущее время пользователя
                current_time: SystemTime,
            }
            /// Одобрение ордера на покупку
            struct AcceptOrder
            {
                /// Публичный идентификатор пользователя
                public_key: &PublicKey,
                /// Идентификатор ордера
                order_id: &Hash,
            }
        }
    }

    impl Transaction for CreateUser {
        fn verify(&self) -> bool {
            self.verify_signature(self.public_key())
        }

        fn execute(&self, fork: &mut Fork) -> ExecutionResult {
            let ts = {
                let time_schema = TimeSchema::new(&fork);
                time_schema.time().get().unwrap()
            };

            let key = self.public_key();

            let mut schema = schema::CryptoOwlsSchema::new(fork);
            if schema.users_proof().get(key).is_none() {
                let user = User::new(&key, self.name(), ISSUE_AMMOUNT, ts);
                schema.users().put(key, user);
            }
            Ok(())
        }
    }

    impl Transaction for MakeOwl {
        fn verify(&self) -> bool {
            self.verify_signature(self.public_key())
        }

        fn execute(&self, fork: &mut Fork) -> ExecutionResult {
            let ts = {
                let time_schema = TimeSchema::new(&fork);
                time_schema.time().get().unwrap()
            };

            let state_hash = {
                let info_schema = Schema::new(&fork);
                info_schema.state_hash_aggregator().root_hash()
            };

            let mut schema = schema::CryptoOwlsSchema::new(fork);
            let parents = [self.mother_id(), self.father_id()]
                .iter()
                .map(|&i| schema.owls_state().get(&i))
                .collect::<Option<Vec<CryptoOwlState>>>();

            let user = schema.users().get(self.public_key()).unwrap();
            let key = user.public_key();

            if let Some(parents) = parents {
                if user.balance() >= BREEDING_PRICE && parents.iter().all(|ref p| {
                    ts.duration_since(p.last_breeding()).unwrap().as_secs() >= BREEDING_TIMEOUT
                }) {
                    let (mother, father) = (parents[0].owl(), parents[1].owl());

                    let son = mother.breed(&father, self.name(), &state_hash);

                    let owl_key = son.hash();
                    let sons_state = CryptoOwlState::new(son, &key, ts);

                    //TODO: add renew_breeding_time method

                    let mothers_state = CryptoOwlState::new(mother, &key, ts);

                    let fathers_state = CryptoOwlState::new(father, &key, ts);

                    let user = User::new(&key, user.name(), user.balance() - BREEDING_PRICE, ts);

                    schema.owls_state().put(&owl_key, sons_state);
                    schema.owls_state().put(self.mother_id(), mothers_state);
                    schema.owls_state().put(self.father_id(), fathers_state);
                    schema.users().put(&key, user);
                }
            }

            Ok(())
        }
    }

    impl Transaction for Issue {
        fn verify(&self) -> bool {
            self.verify_signature(self.public_key())
        }

        fn execute(&self, fork: &mut Fork) -> ExecutionResult {
            let ts = {
                let time_schema = TimeSchema::new(&fork);
                time_schema.time().get().unwrap()
            };

            let mut schema = schema::CryptoOwlsSchema::new(fork);
            let key = self.public_key();
            let user = schema.users().get(key).unwrap();

            if ts.duration_since(user.last_fillup()).unwrap().as_secs() >= ISSUE_TIMEOUT {
                let user = User::new(&key, user.name(), user.balance() + ISSUE_AMMOUNT, ts);
                schema.users().put(&key, user);
            }

            Ok(())
        }
    }

    impl Transaction for CreateOrder {
        fn verify(&self) -> bool {
            self.verify_signature(self.public_key())
        }

        fn execute(&self, fork: &mut Fork) -> ExecutionResult {
            let mut schema = schema::CryptoOwlsSchema::new(fork);
            let key = self.public_key();
            let user = schema.users().get(&key).unwrap();
            if let Some(_) = schema.owls_state().get(self.owl_id()) {
                if self.price() <= user.balance() {
                    let order = Order::new(&key, self.owl_id(), "pending", self.price());
                    let order_hash = order.hash();
                    schema.orders().put(&order.hash(), order);
                    schema.user_orders(&key).push(order_hash);
                    schema.owl_orders(&self.owl_id()).push(order_hash);
                }
            }

            Ok(())
        }
    }

    impl Transaction for AcceptOrder {
        fn verify(&self) -> bool {
            self.verify_signature(self.public_key())
        }

        fn execute(&self, fork: &mut Fork) -> ExecutionResult {
            let mut schema = schema::CryptoOwlsSchema::new(fork);
            if let Some(order) = schema.orders().get(self.order_id()) {
                let buyer = schema.users().get(order.public_key()).unwrap();
                if order.status() == "pending" {
                    if buyer.balance() >= order.price()
                        && schema
                            .users_owls(self.public_key())
                            .contains(order.owl_id())
                    {
                        let new_order = Order::new(
                            order.public_key(),
                            order.owl_id(),
                            "accepted",
                            order.price(),
                        );
                        let owl_state = schema.owls_state().get(order.owl_id()).unwrap();

                        let new_owl_state = CryptoOwlState::new(
                            owl_state.owl(),
                            order.public_key(),
                            owl_state.last_breeding(),
                        );

                        schema.users_owls(self.public_key()).remove(order.owl_id());
                        schema
                            .users_owls(order.public_key())
                            .insert(*order.owl_id());

                        schema.orders().put(&order.hash(), new_order);
                        schema.owls_state().put(order.owl_id(), new_owl_state);
                    } else {
                        let new_order = Order::new(
                            order.public_key(),
                            order.owl_id(),
                            "declined",
                            order.price(),
                        );
                        schema.orders().put(&order.hash(), new_order);
                    }
                }
            }
            Ok(())
        }
    }

}

/// Модуль с реализацией api
mod api {
    use serde_json;
    use serde::Deserialize;

    use bodyparser;
    use iron::prelude::*;
    use iron::status::Status;
    use iron::headers::ContentType;
    use iron::modifiers::Header;

    use router::Router;

    use exonum::api::{Api, ApiError};
    use exonum::crypto::{Hash, PublicKey};
    use exonum::encoding::serialize::{FromHex, FromHexError};

    use exonum::node::{ApiSender, TransactionSend};
    use exonum::blockchain::{Blockchain, Transaction};

    use schema;
    use data_layout::{CryptoOwlState, Order, User};
    use transactions::{AcceptOrder, CreateOrder, CreateUser, Issue};

    #[derive(Clone)]
    pub struct CryptoOwlsApi {
        pub channel: ApiSender,
        pub blockchain: Blockchain,
    }

    impl Api for CryptoOwlsApi {
        fn wire(&self, router: &mut Router) {
            let self_ = self.clone();
            let get_user = move |req: &mut Request| self_.get_user(req);

            let self_ = self.clone();
            let get_users = move |req: &mut Request| self_.get_users(req);

            let self_ = self.clone();
            let get_users_orders = move |req: &mut Request| self_.get_users_orders(req);

            let self_ = self.clone();
            let get_owl = move |req: &mut Request| self_.get_owl(req);

            let self_ = self.clone();
            let get_owls = move |req: &mut Request| self_.get_owls(req);

            let self_ = self.clone();
            let get_owls_orders = move |req: &mut Request| self_.get_owls_orders(req);

            let self_ = self.clone();
            let get_users_owls = move |req: &mut Request| self_.get_users_owls(req);

            let self_ = self.clone();
            let post_user = move |req: &mut Request| self_.post_transaction::<CreateUser>(req);

            let self_ = self.clone();
            let post_order = move |req: &mut Request| self_.post_transaction::<CreateOrder>(req);

            let self_ = self.clone();
            let post_issue = move |req: &mut Request| self_.post_transaction::<Issue>(req);

            let self_ = self.clone();
            let post_acceptance =
                move |req: &mut Request| self_.post_transaction::<AcceptOrder>(req);

            // View-only хэндлеры
            router.get("/v1/users", get_users, "get_users");
            router.get("/v1/user/:pub_key", get_user, "get_user");

            router.get(
                "/v1/user/:pub_key/orders",
                get_users_orders,
                "get_users_orders",
            );

            router.get("/v1/user/:pub_key/owls", get_users_owls, "get_users_owls");

            router.get("/v1/owl/:owl_hash", get_owl, "get_owl");
            router.get("/v1/owls", get_owls, "get_owls");

            router.get(
                "/v1/owl/:owl_hash/orders",
                get_owls_orders,
                "get_owls_orders",
            );

            // Транзакции.
            router.post("/v1/users", post_user, "post_user");
            router.post("/v1/users/issue", post_issue, "post_issue");
            router.post("/v1/orders", post_order, "post_order");
            router.post("/v1/orders/accept", post_acceptance, "post_acceptance");
        }
    }

    impl CryptoOwlsApi {
        /// Вычленение хэша совы из url
        fn find_owl_hash(req: &mut Request) -> Result<Hash, FromHexError> {
            let ref owl_hash = req.extensions
                .get::<Router>()
                .unwrap()
                .find("owl_hash")
                .unwrap();
            Hash::from_hex(owl_hash)
        }

        /// Вычленение публичного ключа из url
        fn find_pub_key(req: &mut Request) -> Result<PublicKey, FromHexError> {
            let ref pub_key = req.extensions
                .get::<Router>()
                .unwrap()
                .find("pub_key")
                .unwrap();
            PublicKey::from_hex(pub_key)
        }

        fn bad_request(e: FromHexError, msg: &str) -> IronError {
            IronError::new(e, (Status::BadRequest, Header(ContentType::json()), msg))
        }

        /// Информация о пользователе
        fn get_user(&self, req: &mut Request) -> IronResult<Response> {
            let public_key = CryptoOwlsApi::find_pub_key(req).map_err(|e| {
                CryptoOwlsApi::bad_request(e, "\"Invalid request param: `pub_key`\"")
            })?;

            let user = {
                let snapshot = self.blockchain.snapshot();
                let schema = schema::CryptoOwlsSchema::new(snapshot);
                schema.users_proof().get(&public_key)
            };

            if let Some(user) = user {
                self.ok_response(&serde_json::to_value(user).unwrap())
            } else {
                self.not_found_response(&serde_json::to_value("User not found").unwrap())
            }
        }

        /// Полный список пользователей
        fn get_users(&self, _: &mut Request) -> IronResult<Response> {
            let snapshot = self.blockchain.snapshot();
            let schema = schema::CryptoOwlsSchema::new(snapshot);
            let idx = schema.users_proof();
            let users: Vec<User> = idx.values().collect();

            self.ok_response(&serde_json::to_value(&users).unwrap())
        }

        /// Информация о cове
        fn get_owl(&self, req: &mut Request) -> IronResult<Response> {
            let owl_hash = CryptoOwlsApi::find_owl_hash(req).map_err(|e| {
                CryptoOwlsApi::bad_request(e, "\"Invalid request param: `owl_hash`\"")
            })?;

            let owl = {
                let snapshot = self.blockchain.snapshot();
                let schema = schema::CryptoOwlsSchema::new(snapshot);
                schema.owls_state_proof().get(&owl_hash)
            };

            if let Some(owl) = owl {
                self.ok_response(&serde_json::to_value(owl).unwrap())
            } else {
                self.not_found_response(&serde_json::to_value("Owl not found").unwrap())
            }
        }

        /// Полный список сов
        fn get_owls(&self, _: &mut Request) -> IronResult<Response> {
            let snapshot = self.blockchain.snapshot();
            let schema = schema::CryptoOwlsSchema::new(snapshot);
            let idx = schema.owls_state_proof();
            let owls: Vec<CryptoOwlState> = idx.values().collect();
            self.ok_response(&serde_json::to_value(&owls).unwrap())
        }

        /// Cписок сов для пользователя
        fn get_users_owls(&self, req: &mut Request) -> IronResult<Response> {
            let users_key = CryptoOwlsApi::find_pub_key(req).map_err(|e| {
                CryptoOwlsApi::bad_request(e, "\"Invalid request param: `pub_key`\"")
            })?;

            let snapshot = self.blockchain.snapshot();
            let schema = schema::CryptoOwlsSchema::new(snapshot);

            if let Some(_) = schema.users_proof().get(&users_key) {
                let idx = schema.users_owls_view(&users_key);

                // type of iterator is ValueSetIndexIter<'_, Hash> !!!
                let owls: Vec<CryptoOwlState> = idx.iter()
                    .map(|h| schema.owls_state_proof().get(&h.1))
                    .collect::<Option<Vec<CryptoOwlState>>>()
                    .unwrap();
                self.ok_response(&serde_json::to_value(&owls).unwrap())
            } else {
                self.not_found_response(&serde_json::to_value("User not found").unwrap())
            }
        }

        /// Информация об ордерах на cову
        fn get_owls_orders(&self, req: &mut Request) -> IronResult<Response> {
            let owl_hash = CryptoOwlsApi::find_owl_hash(req).map_err(|e| {
                CryptoOwlsApi::bad_request(e, "\"Invalid request param: `owl_hash`\"")
            })?;
            let snapshot = self.blockchain.snapshot();
            let schema = schema::CryptoOwlsSchema::new(snapshot);

            if let Some(_) = schema.owls_state_proof().get(&owl_hash) {
                let idx = schema.owl_orders_view(&owl_hash);
                let orders: Vec<Order> = idx.iter()
                    .map(|h| schema.orders_proof().get(&h))
                    .collect::<Option<Vec<Order>>>()
                    .unwrap();
                self.ok_response(&serde_json::to_value(&orders).unwrap())
            } else {
                self.not_found_response(&serde_json::to_value("Owl not found").unwrap())
            }
        }

        /// Информация об ордерах выставленных юзером
        fn get_users_orders(&self, req: &mut Request) -> IronResult<Response> {
            let users_key = CryptoOwlsApi::find_pub_key(req).map_err(|e| {
                CryptoOwlsApi::bad_request(e, "\"Invalid request param: `pub_key`\"")
            })?;

            let snapshot = self.blockchain.snapshot();
            let schema = schema::CryptoOwlsSchema::new(snapshot);

            if let Some(_) = schema.users_proof().get(&users_key) {
                let idx = schema.user_orders_view(&users_key);

                let orders: Vec<Order> = idx.iter()
                    .map(|h| schema.orders_proof().get(&h))
                    .collect::<Option<Vec<Order>>>()
                    .unwrap();
                self.ok_response(&serde_json::to_value(&orders).unwrap())
            } else {
                self.not_found_response(&serde_json::to_value("User not found").unwrap())
            }
        }

        /// Общий код для постинга транзакций
        fn post_transaction<T>(&self, req: &mut Request) -> IronResult<Response>
        where
            T: Transaction + Clone + for<'de> Deserialize<'de>,
        {
            match req.get::<bodyparser::Struct<T>>() {
                Ok(Some(transaction)) => {
                    let transaction: Box<Transaction> = Box::new(transaction);
                    let tx_hash = transaction.hash();
                    self.channel.send(transaction).map_err(ApiError::from)?;
                    self.ok_response(&json!({ "tx_hash": tx_hash }))
                }
                Ok(None) => Err(ApiError::BadRequest("Empty request body".into()))?,
                Err(e) => Err(ApiError::InternalError(Box::new(e)))?,
            }
        }
    }
}

pub mod service {
    use iron::Handler;
    use router::Router;

    use exonum::api::Api;
    use exonum::crypto::Hash;
    use exonum::encoding;
    use exonum::storage::Snapshot;
    use exonum::blockchain::{ApiContext, Service, Transaction, TransactionSet};
    use exonum::helpers::fabric::{Context, ServiceFactory};
    use exonum::messages::RawTransaction;

    use api::CryptoOwlsApi;
    use schema::CryptoOwlsSchema;
    use transactions::Transactions;

    use CRYPTOOWLS_SERVICE_ID;

    struct CryptoOwlsService;

    impl CryptoOwlsService {
        fn new() -> Self {
            CryptoOwlsService {}
        }
    }

    pub struct CryptoOwlsServiceFactory;

    impl ServiceFactory for CryptoOwlsServiceFactory {
        fn make_service(&mut self, _: &Context) -> Box<Service> {
            Box::new(CryptoOwlsService::new())
        }
    }

    impl Service for CryptoOwlsService {
        fn service_name(&self) -> &'static str {
            "cryptoowls"
        }

        fn service_id(&self) -> u16 {
            CRYPTOOWLS_SERVICE_ID
        }

        // Метод десериализации для транзакций
        fn tx_from_raw(&self, raw: RawTransaction) -> Result<Box<Transaction>, encoding::Error> {
            let tx = Transactions::tx_from_raw(raw)?;
            Ok(tx.into())
        }

        // Hashes for the service tables that will be included into the state hash.
        // To simplify things, we don't have [Merkelized tables][merkle] in the service storage
        // for now, so we return an empty vector.
        //

        // Хэши таблиц, которые будут включены в общий стейт хэш
        fn state_hash(&self, snapshot: &Snapshot) -> Vec<Hash> {
            let schema = CryptoOwlsSchema::new(snapshot);
            vec![
                schema.users_proof().root_hash(),
                schema.orders_proof().root_hash(),
                schema.owls_state_proof().root_hash(),
            ]
        }

        // Хэндлер для обработки запросов к ноде
        fn public_api_handler(&self, ctx: &ApiContext) -> Option<Box<Handler>> {
            let mut router = Router::new();
            let api = CryptoOwlsApi {
                channel: ctx.node_channel().clone(),
                blockchain: ctx.blockchain().clone(),
            };
            api.wire(&mut router);
            Some(Box::new(router))
        }
    }
}

#[macro_use]
extern crate exonum;

mod schema;

/// Некоторый уникальный идентификатор сервиса.
pub const CRYPTOOWLS_SERVICE_ID: u16 = 521;
/// Уникальное имя сервиса, которое будет использоваться в апи и конфигурации.
pub const CRYPTOOWLS_SERVICE_NAME: &str = "cryptoowls";

/// Модуль со структурами данных, которые хранятся в блокчейне
mod data_layout {
    use std::time::SystemTime;
    use exonum::crypto::{Hash, PublicKey};

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
            /// merkle_root истории сделок с этой совой
            orders_history: &Hash,
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
            // merkle_root таблицы с принадлежащими ему совами
            // owls_root: &Hash,
            /// merkle_root истории ордеров, которые пользователь разместил
            orders_root: &Hash,
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
}


/// Модуль с описанием транзакций для демки.
mod transactions {
    use std::time::SystemTime;
    use exonum::crypto::{Hash, PublicKey};
    use exonum::blockchain::{Transaction, ExecutionResult};
    use exonum::storage::Fork;
    use CRYPTOOWLS_SERVICE_ID;

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
                mather_id: &Hash,
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
            unimplemented!()
        }

        fn execute(&self, fork: &mut Fork) -> ExecutionResult {
            unimplemented!()
        }
    }

    impl Transaction for MakeOwl {
        fn verify(&self) -> bool {
            unimplemented!()
        }

        fn execute(&self, fork: &mut Fork) -> ExecutionResult {
            unimplemented!()
        }
    }

    impl Transaction for Issue {
        fn verify(&self) -> bool {
            unimplemented!()
        }

        fn execute(&self, fork: &mut Fork) -> ExecutionResult {
            unimplemented!()
        }
    }

    impl Transaction for CreateOrder {
        fn verify(&self) -> bool {
            unimplemented!()
        }

        fn execute(&self, fork: &mut Fork) -> ExecutionResult {
            unimplemented!()
        }
    }

    impl Transaction for AcceptOrder {
        fn verify(&self) -> bool {
            unimplemented!()
        }

        fn execute(&self, fork: &mut Fork) -> ExecutionResult {
            unimplemented!()
        }
    }
}

/// Модуль с реализацией api
mod api {}

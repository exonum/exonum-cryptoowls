use super::data_layout::{User, CryptoOwl, CryptoOwlState, Order};
use exonum::storage::{Snapshot, Fork, ProofListIndex, ProofMapIndex, ValueSetIndex};
use exonum::blockchain::gen_prefix;
use exonum::crypto::{PublicKey, Hash};

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

    pub fn users_owls_view(&self, public_key: &PublicKey) -> ValueSetIndex<&T, CryptoOwl> {
        ValueSetIndex::with_prefix("cryptoowls.users_owls", gen_prefix(public_key), &self.view)
    }

    pub fn orders_proof_view(&self) -> ProofMapIndex<&T, Hash, Order> {
        ProofMapIndex::new("cryptoowls.orders", &self.view)
    }

    pub fn user_orders_history_view(&self, public_key: &PublicKey) -> ProofListIndex<&T, Hash> {
        ProofListIndex::with_prefix("cryptoowls.user_orders", gen_prefix(public_key), &self.view)
    }

    pub fn owl_orders_history_view(&self, owl_hash: &Hash) -> ProofListIndex<&T, Hash> {
        ProofListIndex::with_prefix("cryptoowls.owl_orders", gen_prefix(owl_hash), &self.view)
    }
}

impl<'a> CryptoOwlsSchema<&'a mut Fork> {
    pub fn users(&mut self) -> ProofMapIndex<&mut Fork, PublicKey, User> {
        ProofMapIndex::new("cryptoowls.users", self.view)
    }

    pub fn owls_state(&mut self) -> ProofMapIndex<&mut Fork, Hash, CryptoOwlState> {
        ProofMapIndex::new("cryptoowls.owls_state", self.view)
    }

    pub fn users_owls(&mut self, public_key: &PublicKey) -> ValueSetIndex<&mut Fork, Hash> {
        ValueSetIndex::with_prefix("cryptoowls.users_owls", gen_prefix(public_key), self.view)
    }

    pub fn orders(&mut self) -> ProofMapIndex<&mut Fork, Hash, Order> {
        ProofMapIndex::new("cryptoowls.orders", self.view)
    }

    pub fn user_orders_history(
        &mut self,
        public_key: &PublicKey,
    ) -> ProofListIndex<&mut Fork, Hash> {
        ProofListIndex::with_prefix("cryptoowls.user_orders", gen_prefix(public_key), self.view)
    }

    pub fn owl_orders_history(&mut self, owl_hash: &Hash) -> ProofListIndex<&mut Fork, Hash> {
        ProofListIndex::with_prefix("cryptoowls.owl_orders", gen_prefix(owl_hash), self.view)
    }
}

use super::data_layout::{User, CryptoOwl, CryptoOwlState, Order};
use exonum::storage::{Snapshot, Fork, ListIndex, ProofMapIndex, ValueSetIndex};
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

    pub fn users_owls(&mut self, public_key: &PublicKey) -> ValueSetIndex<&mut Fork, Hash> {
        ValueSetIndex::with_prefix("cryptoowls.users_owls", gen_prefix(public_key), self.view)
    }

    pub fn orders(&mut self) -> ProofMapIndex<&mut Fork, Hash, Order> {
        ProofMapIndex::new("cryptoowls.orders", self.view)
    }

    pub fn user_orders(&mut self, public_key: &PublicKey) -> ListIndex<&mut Fork, Hash> {
        ListIndex::with_prefix("cryptoowls.user_orders", gen_prefix(public_key), self.view)
    }

    pub fn owl_orders(&mut self, owl_id: &Hash) -> ListIndex<&mut Fork, Hash> {
        ListIndex::with_prefix("cryptoowls.owl_orders", gen_prefix(owl_id), self.view)
    }
}

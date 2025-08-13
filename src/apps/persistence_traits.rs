use crate::persistence::{Loader, Storer};

pub struct PersistenceTraitsApp<S: Storer, L: Loader> {
    storer: S,
    loader: L,
}

// Problem: There's no guarante that the storer and loader will be of the same family
// (e.g., both json).

impl<S: Storer, L: Loader> PersistenceTraitsApp<S, L> {
    pub fn new(storer: S, loader: L) -> Self {
        Self { storer, loader }
    }

    pub fn store(&self) {
        self.storer.store();
    }

    pub fn load(&self) {
        self.loader.load();
    }
}

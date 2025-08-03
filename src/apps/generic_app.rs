use crate::persistence::{Loader, Storer};

pub struct GenericApp<S: Storer, L: Loader> {
    pub storer: S,
    pub loader: L,
}

impl<S: Storer, L: Loader> GenericApp<S, L> {
    pub fn store(&self) {
        self.storer.store();
    }

    pub fn load(&self) {
        self.loader.load();
    }
}

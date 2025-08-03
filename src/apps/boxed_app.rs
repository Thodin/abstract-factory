use crate::persistence::{Loader, Storer};

pub struct BoxedApp {
    pub storer: Box<dyn Storer>,
    pub loader: Box<dyn Loader>,
}

impl BoxedApp {
    pub fn store(&self) {
        self.storer.store();
    }

    pub fn load(&self) {
        self.loader.load();
    }
}

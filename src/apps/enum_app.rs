use crate::{
    abstract_factories::enum_factories::{LoaderEnum, StorerEnum},
    persistence::{Loader, Storer},
};

pub struct EnumApp {
    pub storer: StorerEnum,
    pub loader: LoaderEnum,
}

impl EnumApp {
    pub fn store(&self) {
        self.storer.store();
    }

    pub fn load(&self) {
        self.loader.load();
    }
}

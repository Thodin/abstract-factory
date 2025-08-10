use crate::abstract_factories::enum_factories::{LoaderEnum, PersistenceFactoryEnum, StorerEnum};

pub struct EnumApp {
    storer: StorerEnum,
    loader: LoaderEnum,
}

impl EnumApp {
    pub fn from_enum_factory(persistence_factory: &PersistenceFactoryEnum) -> Self {
        Self {
            storer: persistence_factory.create_storer(),
            loader: persistence_factory.create_loader(),
        }
    }

    pub fn store(&self) {
        self.storer.store();
    }

    pub fn load(&self) {
        self.loader.load();
    }
}

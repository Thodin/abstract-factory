use crate::{
    abstract_factories::{
        boxed_factories::BoxedPersistenceFactory, impl_factories::ImplPersistenceFactory,
    },
    persistence::{Loader, Storer},
};

pub struct BoxedApp<'a> {
    storer: Box<dyn Storer + 'a>,
    loader: Box<dyn Loader + 'a>,
}

impl<'a> BoxedApp<'a> {
    pub fn from_boxed_factory<F: BoxedPersistenceFactory>(persistence_factory: &F) -> Self {
        Self {
            storer: persistence_factory.create_storer(),
            loader: persistence_factory.create_loader(),
        }
    }

    pub fn from_impl_factory<F: ImplPersistenceFactory<'a>>(persistence_factory: &F) -> Self {
        Self {
            storer: Box::new(persistence_factory.create_storer()),
            loader: Box::new(persistence_factory.create_loader()),
        }
    }

    pub fn store(&self) {
        self.storer.store();
    }

    pub fn load(&self) {
        self.loader.load();
    }
}

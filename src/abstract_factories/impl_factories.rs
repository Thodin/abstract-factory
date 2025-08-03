use crate::persistence::{JsonLoader, JsonStorer, Loader, PostgresLoader, PostgresStorer, Storer};

pub trait ImplPersistenceFactory {
    fn create_storer(&self) -> impl Storer;
    fn create_loader(&self) -> impl Loader;
}

pub struct JsonImplPersistenceFactory {}

impl ImplPersistenceFactory for JsonImplPersistenceFactory {
    fn create_storer(&self) -> impl Storer {
        JsonStorer {}
    }

    fn create_loader(&self) -> impl Loader {
        JsonLoader {}
    }
}

pub struct PostgresImplPersistenceFactory {}

impl ImplPersistenceFactory for PostgresImplPersistenceFactory {
    fn create_storer(&self) -> impl Storer {
        PostgresStorer {}
    }

    fn create_loader(&self) -> impl Loader {
        PostgresLoader {}
    }
}

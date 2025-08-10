use crate::persistence::{JsonLoader, JsonStorer, Loader, PostgresLoader, PostgresStorer, Storer};

pub trait ImplPersistenceFactory<'a> {
    fn create_storer(&self) -> impl Storer + 'a;
    fn create_loader(&self) -> impl Loader + 'a;
}

pub struct JsonImplPersistenceFactory {}

impl<'a> ImplPersistenceFactory<'a> for JsonImplPersistenceFactory {
    fn create_storer(&self) -> impl Storer + 'a {
        JsonStorer {}
    }

    fn create_loader(&self) -> impl Loader + 'a {
        JsonLoader {}
    }
}

pub struct PostgresImplPersistenceFactory {}

impl<'a> ImplPersistenceFactory<'a> for PostgresImplPersistenceFactory {
    fn create_storer(&self) -> impl Storer + 'a {
        PostgresStorer {}
    }

    fn create_loader(&self) -> impl Loader + 'a {
        PostgresLoader {}
    }
}

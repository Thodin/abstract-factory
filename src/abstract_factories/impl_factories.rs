use crate::persistence::{JsonLoader, JsonStorer, Loader, SqlLoader, SqlStorer, Storer};

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

pub struct SqlImplPersistenceFactory {}

impl<'a> ImplPersistenceFactory<'a> for SqlImplPersistenceFactory {
    fn create_storer(&self) -> impl Storer + 'a {
        SqlStorer {}
    }

    fn create_loader(&self) -> impl Loader + 'a {
        SqlLoader {}
    }
}

use crate::persistence::{JsonLoader, JsonStorer, Loader, SqlLoader, SqlStorer, Storer};

pub trait GenericPersistenceFactory<S: Storer, L: Loader> {
    fn create_storer(&self) -> S;
    fn create_loader(&self) -> L;
}

pub struct JsonGenericPersistenceFactory {}

impl GenericPersistenceFactory<JsonStorer, JsonLoader> for JsonGenericPersistenceFactory {
    fn create_storer(&self) -> JsonStorer {
        JsonStorer {}
    }

    fn create_loader(&self) -> JsonLoader {
        JsonLoader {}
    }
}

pub struct SqlGenericPersistenceFactory {}

impl GenericPersistenceFactory<SqlStorer, SqlLoader> for SqlGenericPersistenceFactory {
    fn create_storer(&self) -> SqlStorer {
        SqlStorer {}
    }

    fn create_loader(&self) -> SqlLoader {
        SqlLoader {}
    }
}

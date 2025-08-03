use crate::persistence::{JsonLoader, JsonStorer, Loader, PostgresLoader, PostgresStorer, Storer};

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

pub struct PostgresGenericPersistenceFactory {}

impl GenericPersistenceFactory<PostgresStorer, PostgresLoader>
    for PostgresGenericPersistenceFactory
{
    fn create_storer(&self) -> PostgresStorer {
        PostgresStorer {}
    }

    fn create_loader(&self) -> PostgresLoader {
        PostgresLoader {}
    }
}

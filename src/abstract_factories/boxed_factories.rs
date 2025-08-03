use crate::persistence::{JsonLoader, JsonStorer, Loader, PostgresLoader, PostgresStorer, Storer};

pub trait BoxedPersistenceFactory {
    fn create_storer(&self) -> Box<dyn Storer>;
    fn create_loader(&self) -> Box<dyn Loader>;
}

pub struct JsonBoxedPersistenceFactory {}

impl BoxedPersistenceFactory for JsonBoxedPersistenceFactory {
    fn create_storer(&self) -> Box<dyn Storer> {
        Box::new(JsonStorer {})
    }

    fn create_loader(&self) -> Box<dyn Loader> {
        Box::new(JsonLoader {})
    }
}

pub struct PostgresBoxedPersistenceFactory {}

impl BoxedPersistenceFactory for PostgresBoxedPersistenceFactory {
    fn create_storer(&self) -> Box<dyn Storer> {
        Box::new(PostgresStorer {})
    }

    fn create_loader(&self) -> Box<dyn Loader> {
        Box::new(PostgresLoader {})
    }
}

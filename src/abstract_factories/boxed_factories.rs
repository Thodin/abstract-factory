use crate::persistence::{JsonLoader, JsonStorer, Loader, SqlLoader, SqlStorer, Storer};

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

pub struct SqlBoxedPersistenceFactory {}

impl BoxedPersistenceFactory for SqlBoxedPersistenceFactory {
    fn create_storer(&self) -> Box<dyn Storer> {
        Box::new(SqlStorer {})
    }

    fn create_loader(&self) -> Box<dyn Loader> {
        Box::new(SqlLoader {})
    }
}

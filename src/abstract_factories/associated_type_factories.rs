use crate::persistence::{JsonLoader, JsonStorer, PostgresLoader, PostgresStorer};

pub trait AssociatedTypePersistenceFactory {
    type CreatedStorer;
    type CreatedLoader;

    fn create_storer(&self) -> Self::CreatedStorer;
    fn create_loader(&self) -> Self::CreatedLoader;
}

pub struct JsonAssociatedTypePersistenceFactory {}

impl AssociatedTypePersistenceFactory for JsonAssociatedTypePersistenceFactory {
    type CreatedStorer = JsonStorer;
    type CreatedLoader = JsonLoader;

    fn create_storer(&self) -> Self::CreatedStorer {
        JsonStorer {}
    }

    fn create_loader(&self) -> Self::CreatedLoader {
        JsonLoader {}
    }
}

pub struct PostgresAssociatedTypePersistenceFactory {}

impl AssociatedTypePersistenceFactory for PostgresAssociatedTypePersistenceFactory {
    type CreatedStorer = PostgresStorer;
    type CreatedLoader = PostgresLoader;

    fn create_storer(&self) -> Self::CreatedStorer {
        PostgresStorer {}
    }

    fn create_loader(&self) -> Self::CreatedLoader {
        PostgresLoader {}
    }
}

use crate::persistence::{JsonLoader, JsonStorer, Loader, SqlLoader, SqlStorer, Storer};

pub trait AssociatedTypePersistenceFactory {
    type CreatedStorer: Storer;
    type CreatedLoader: Loader;

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

pub struct SqlAssociatedTypePersistenceFactory {}

impl AssociatedTypePersistenceFactory for SqlAssociatedTypePersistenceFactory {
    type CreatedStorer = SqlStorer;
    type CreatedLoader = SqlLoader;

    fn create_storer(&self) -> Self::CreatedStorer {
        SqlStorer {}
    }

    fn create_loader(&self) -> Self::CreatedLoader {
        SqlLoader {}
    }
}

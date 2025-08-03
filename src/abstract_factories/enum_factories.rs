use crate::persistence::{JsonLoader, JsonStorer, Loader, PostgresLoader, PostgresStorer, Storer};

pub enum StorerEnum {
    Json(JsonStorer),
    Postgres(PostgresStorer),
}

impl StorerEnum {
    pub fn store(&self) {
        match self {
            StorerEnum::Json(json_storer) => json_storer.store(),
            StorerEnum::Postgres(postgres_storer) => postgres_storer.store(),
        }
    }
}

pub enum LoaderEnum {
    Json(JsonLoader),
    Postgres(PostgresLoader),
}

impl LoaderEnum {
    pub fn load(&self) {
        match self {
            LoaderEnum::Json(json_loader) => json_loader.load(),
            LoaderEnum::Postgres(postgres_loader) => postgres_loader.load(),
        }
    }
}

pub enum PersistenceFactoryEnum {
    Json,
    Postgres,
}

impl PersistenceFactoryEnum {
    pub fn create_storer(&self) -> StorerEnum {
        match self {
            PersistenceFactoryEnum::Json => StorerEnum::Json(JsonStorer {}),
            PersistenceFactoryEnum::Postgres => StorerEnum::Postgres(PostgresStorer {}),
        }
    }

    pub fn create_loader(&self) -> LoaderEnum {
        match self {
            PersistenceFactoryEnum::Json => LoaderEnum::Json(JsonLoader {}),
            PersistenceFactoryEnum::Postgres => LoaderEnum::Postgres(PostgresLoader {}),
        }
    }
}

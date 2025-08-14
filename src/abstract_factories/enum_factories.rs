use crate::persistence::{JsonLoader, JsonStorer, Loader, SqlLoader, SqlStorer, Storer};

pub enum StorerEnum {
    Json(JsonStorer),
    Sql(SqlStorer),
}

impl StorerEnum {
    pub fn store(&self) {
        match self {
            StorerEnum::Json(json_storer) => json_storer.store(),
            StorerEnum::Sql(sql_storer) => sql_storer.store(),
        }
    }
}

pub enum LoaderEnum {
    Json(JsonLoader),
    Sql(SqlLoader),
}

impl LoaderEnum {
    pub fn load(&self) {
        match self {
            LoaderEnum::Json(json_loader) => json_loader.load(),
            LoaderEnum::Sql(sql_loader) => sql_loader.load(),
        }
    }
}

pub enum PersistenceFactoryEnum {
    Json,
    Sql,
}

impl PersistenceFactoryEnum {
    pub fn create_storer(&self) -> StorerEnum {
        match self {
            PersistenceFactoryEnum::Json => StorerEnum::Json(JsonStorer {}),
            PersistenceFactoryEnum::Sql => StorerEnum::Sql(SqlStorer {}),
        }
    }

    pub fn create_loader(&self) -> LoaderEnum {
        match self {
            PersistenceFactoryEnum::Json => LoaderEnum::Json(JsonLoader {}),
            PersistenceFactoryEnum::Sql => LoaderEnum::Sql(SqlLoader {}),
        }
    }
}

use crate::{
    abstract_factories::widget_factory::BoxedExporterFactory,
    persistence::{
        JsonLoader, JsonStorer, LinuxButton, LinuxText, Loader, PostgresLoader, PostgresStorer,
        Storer, Widget, WindowsButton, WindowsText,
    },
};

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

impl BoxedExporterFactory for JsonBoxedPersistenceFactory {
    fn create_button(&self) -> Box<dyn Widget> {
        Box::new(WindowsButton {})
    }

    fn create_text(&self) -> Box<dyn Widget> {
        Box::new(WindowsText {})
    }
}

pub struct CsvBoxedExporterFactory {}

impl BoxedExporterFactory for CsvBoxedExporterFactory {
    fn create_button(&self) -> Box<dyn Widget> {
        Box::new(LinuxButton {})
    }

    fn create_text(&self) -> Box<dyn Widget> {
        Box::new(LinuxText {})
    }
}

use crate::persistence::{JsonLoader, JsonStorer, Loader, Storer};

pub struct HardcodedApp {
    pub storer: JsonStorer,
    pub loader: JsonLoader,
}

// Problem: Changing to a postgres storer/loader requires changing the app completely.
// There is no guarantee that the postgres objects will satisfy the same `Loader`/`Storer`
// traits, so the code in `store`/`load` might need to change as well.

impl HardcodedApp {
    pub fn new() -> Self {
        Self {
            storer: JsonStorer {},
            loader: JsonLoader {},
        }
    }

    pub fn store(&self) {
        self.storer.store();
    }

    pub fn load(&self) {
        self.loader.load();
    }
}

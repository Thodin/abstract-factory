pub trait Storer {
    fn store(&self);
}

pub trait Loader {
    fn load(&self);
}

pub struct JsonStorer {}

impl Storer for JsonStorer {
    fn store(&self) {
        println!("Storing in json file ...");
    }
}

pub struct SqlStorer {}

impl Storer for SqlStorer {
    fn store(&self) {
        println!("Storing in SQL db ...");
    }
}

pub struct JsonLoader {}

impl Loader for JsonLoader {
    fn load(&self) {
        println!("Loading from json file ...");
    }
}

pub struct SqlLoader {}

impl Loader for SqlLoader {
    fn load(&self) {
        println!("Loading from SQL db ...");
    }
}

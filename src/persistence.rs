pub trait Widget {
    fn render(&self);
}

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

pub struct PostgresStorer {}

impl Storer for PostgresStorer {
    fn store(&self) {
        println!("Storing in Postgres db ...");
    }
}

pub struct JsonLoader {}

impl Loader for JsonLoader {
    fn load(&self) {
        println!("Loading from json file ...");
    }
}

pub struct PostgresLoader {}

impl Loader for PostgresLoader {
    fn load(&self) {
        println!("Loading from Postgres db ...");
    }
}
pub struct WindowsButton {}

impl Widget for WindowsButton {
    fn render(&self) {
        println!("Windows Button");
    }
}

pub struct LinuxButton {}

impl Widget for LinuxButton {
    fn render(&self) {
        println!("Linux Button");
    }
}

pub struct WindowsText {}

impl Widget for WindowsText {
    fn render(&self) {
        println!("Windows Text");
    }
}

pub struct LinuxText {}

impl Widget for LinuxText {
    fn render(&self) {
        println!("Linux Text");
    }
}

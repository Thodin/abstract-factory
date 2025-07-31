pub trait Widget {
    fn render(&self);
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

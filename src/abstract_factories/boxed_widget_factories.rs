use crate::{
    abstract_factories::widget_factory::BoxedWidgetFactory,
    widgets::{LinuxButton, LinuxText, Widget, WindowsButton, WindowsText},
};

pub struct WindowsBoxedWidgetFactory {}

impl BoxedWidgetFactory for WindowsBoxedWidgetFactory {
    fn create_button(&self) -> Box<dyn Widget> {
        Box::new(WindowsButton {})
    }

    fn create_text(&self) -> Box<dyn Widget> {
        Box::new(WindowsText {})
    }
}

pub struct LinuxBoxedWidgetFactory {}

impl BoxedWidgetFactory for LinuxBoxedWidgetFactory {
    fn create_button(&self) -> Box<dyn Widget> {
        Box::new(LinuxButton {})
    }

    fn create_text(&self) -> Box<dyn Widget> {
        Box::new(LinuxText {})
    }
}

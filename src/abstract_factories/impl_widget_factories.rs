use crate::{
    abstract_factories::widget_factory::ImplWidgetFactory,
    widgets::{LinuxButton, LinuxText, Widget, WindowsButton, WindowsText},
};

pub struct WindowsImplWidgetFactory {}

impl ImplWidgetFactory for WindowsImplWidgetFactory {
    fn create_button(&self) -> impl Widget {
        WindowsButton {}
    }

    fn create_text(&self) -> impl Widget {
        WindowsText {}
    }
}

pub struct LinuxImplWidgetFactory {}

impl ImplWidgetFactory for LinuxImplWidgetFactory {
    fn create_button(&self) -> impl Widget {
        LinuxButton {}
    }

    fn create_text(&self) -> impl Widget {
        LinuxText {}
    }
}

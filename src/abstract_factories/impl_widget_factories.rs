use crate::{
    abstract_factories::widget_factory::ImplWidgetFactory,
    persistence::{LinuxButton, LinuxText, Widget, WindowsButton, WindowsText},
};

pub struct WindowsImplWidgetFactory {}

impl<'a> ImplWidgetFactory<'a> for WindowsImplWidgetFactory {
    fn create_button(&self) -> impl Widget + 'a {
        WindowsButton {}
    }

    fn create_text(&self) -> impl Widget + 'a {
        WindowsText {}
    }
}

pub struct LinuxImplWidgetFactory {}

impl<'a> ImplWidgetFactory<'a> for LinuxImplWidgetFactory {
    fn create_button(&self) -> impl Widget + 'a {
        LinuxButton {}
    }

    fn create_text(&self) -> impl Widget + 'a {
        LinuxText {}
    }
}

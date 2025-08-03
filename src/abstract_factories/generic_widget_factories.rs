use crate::{
    abstract_factories::widget_factory::GenericWidgetFactory,
    persistence::{LinuxButton, LinuxText, WindowsButton, WindowsText},
};

pub struct WindowsGenericWidgetFactory {}

impl GenericWidgetFactory<WindowsButton, WindowsText> for WindowsGenericWidgetFactory {
    fn create_button(&self) -> WindowsButton {
        WindowsButton {}
    }

    fn create_text(&self) -> WindowsText {
        WindowsText {}
    }
}

pub struct LinuxGenericWidgetFactory {}

impl GenericWidgetFactory<LinuxButton, LinuxText> for LinuxGenericWidgetFactory {
    fn create_button(&self) -> LinuxButton {
        LinuxButton {}
    }

    fn create_text(&self) -> LinuxText {
        LinuxText {}
    }
}

use crate::{
    abstract_factories::widget_factory::AssociatedTypeWidgetFactory,
    widgets::{LinuxButton, LinuxText, WindowsButton, WindowsText},
};

pub struct WindowsAssociatedTypeWidgetFactory {}

impl AssociatedTypeWidgetFactory for WindowsAssociatedTypeWidgetFactory {
    type CreatedButton = WindowsButton;
    type CreatedText = WindowsText;

    fn create_button(&self) -> Self::CreatedButton {
        WindowsButton {}
    }

    fn create_text(&self) -> Self::CreatedText {
        WindowsText {}
    }
}

pub struct LinuxAssociatedTypeWidgetFactory {}

impl AssociatedTypeWidgetFactory for LinuxAssociatedTypeWidgetFactory {
    type CreatedButton = LinuxButton;
    type CreatedText = LinuxText;

    fn create_button(&self) -> Self::CreatedButton {
        LinuxButton {}
    }

    fn create_text(&self) -> Self::CreatedText {
        LinuxText {}
    }
}

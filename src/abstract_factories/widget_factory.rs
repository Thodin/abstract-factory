use crate::widgets::Widget;

pub trait BoxedWidgetFactory {
    fn create_button(&self) -> Box<dyn Widget>;
    fn create_text(&self) -> Box<dyn Widget>;
}

pub trait AssociatedTypeWidgetFactory {
    type CreatedButton;
    type CreatedText;

    fn create_button(&self) -> Self::CreatedButton;
    fn create_text(&self) -> Self::CreatedText;
}

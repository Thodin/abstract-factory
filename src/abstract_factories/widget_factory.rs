use crate::widgets::Widget;

pub trait BoxedWidgetFactory {
    fn create_button(&self) -> Box<dyn Widget>;
    fn create_text(&self) -> Box<dyn Widget>;
}

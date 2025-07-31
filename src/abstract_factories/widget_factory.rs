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

// Note: this trait isn't dyn compatible because the `impl Widget` return types are monomorphized
// into different types at compile time.
// Methods in a dyn compatible trait need to return objects of the same size every time though!
pub trait ImplWidgetFactory {
    fn create_button(&self) -> impl Widget;
    fn create_text(&self) -> impl Widget;
}

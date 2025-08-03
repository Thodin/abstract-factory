use crate::persistence::Widget;

pub trait BoxedExporterFactory {
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

// For some cases, it might be required to add a lifetime specifier (e.g., `+ 'a` or `+ 'static`) to the return impl type,
// especially when the created widgets are being move inside Boxes (and handed around to other functions).
pub trait ImplWidgetFactory<'a> {
    fn create_button(&self) -> impl Widget + 'a;
    fn create_text(&self) -> impl Widget + 'a;
}

pub trait GenericWidgetFactory<B: Widget, T: Widget> {
    fn create_button(&self) -> B;
    fn create_text(&self) -> T;
}

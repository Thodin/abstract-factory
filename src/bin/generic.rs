use abstract_factory::{
    abstract_factories::{
        generic_widget_factories::{LinuxGenericWidgetFactory, WindowsGenericWidgetFactory},
        widget_factory::GenericWidgetFactory,
    },
    widgets::Widget,
};

fn main() {
    let windows_factory = WindowsGenericWidgetFactory {};
    let linux_factory = LinuxGenericWidgetFactory {};

    // Can't store them in a common container, they don't have the same type (due to generic arguments).
    // let factories = [
    //     &windows_factory as &dyn GenericWidgetFactory<WindowsButton, WindowsText>,
    //     &linux_factory as &dyn GenericWidgetFactory<LinuxButton, LinuxText>,
    // ];

    let mut buttons = vec![];
    let mut texts = vec![];

    // Also can't store created buttons in a common collection, as the monomorphized concrete
    // button types don't need to match.
    // buttons.push(windows_factory.create_button());
    // buttons.push(linux_factory.create_button());

    // So therefore, we're back to trait objects again.
    // But at least, we can use references for trait objects here, instead of boxes.
    // However, the use cases for this are rare I assume...
    let windows_button = windows_factory.create_button();
    let linux_button = linux_factory.create_button();

    buttons.push(&windows_button as &dyn Widget);
    buttons.push(&linux_button as &dyn Widget);

    // Do the texts via boxes.
    texts.push(Box::new(windows_factory.create_text()) as Box<dyn Widget>);
    texts.push(Box::new(linux_factory.create_text()) as Box<dyn Widget>);

    println!("BUTTONS");
    for b in &buttons {
        b.render();
    }
    println!();

    println!("TEXTS");
    for t in &texts {
        t.render();
    }

    // Creation of the app here requires the lifetime specifiers on the return types of the factory.
    let app = App::new(texts);

    app.render();
}

struct App {
    widgets: Vec<Box<dyn Widget>>,
}

impl App {
    pub fn new(widgets: Vec<Box<dyn Widget>>) -> Self {
        App { widgets }
    }

    pub fn render(&self) {
        for w in &self.widgets {
            w.render();
        }
    }
}

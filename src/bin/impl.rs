use abstract_factory::{
    abstract_factories::{
        impl_widget_factories::{LinuxImplWidgetFactory, WindowsImplWidgetFactory},
        widget_factory::ImplWidgetFactory,
    },
    widgets::Widget,
};

fn main() {
    let windows_factory = WindowsImplWidgetFactory {};
    let linux_factory = LinuxImplWidgetFactory {};

    // Can't store them in a common container, not dyn compatible!
    // let factories = [
    //     &windows_factory as &dyn ImplWidgetFactory,
    //     &linux_factory as &dyn ImplWidgetFactory,
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
}

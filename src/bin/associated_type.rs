use abstract_factory::{
    abstract_factories::{
        associated_type_widget_factories::{
            LinuxAssociatedTypeWidgetFactory, WindowsAssociatedTypeWidgetFactory,
        },
        widget_factory::AssociatedTypeWidgetFactory,
    },
    persistence::Widget,
};

fn main() {
    let windows_factory = WindowsAssociatedTypeWidgetFactory {};
    let linux_factory = LinuxAssociatedTypeWidgetFactory {};

    // First drawback: Can't store all the factories in a common collection, because the associated
    // types are part of the trait type. Probably not a big problem though, it is not so common
    // to have multiple factories of the same trait.
    // let factories = [
    //     &windows_factory
    //         as &dyn AssociatedTypeWidgetFactory<
    //             CreatedButton = WindowsButton,
    //             CreatedText = WindowsText,
    //         >,
    //     &linux_factory
    //         as &dyn AssociatedTypeWidgetFactory<CreatedButton = LinuxButton, CreatedText = LinuxText>,
    // ];

    // This problem can be circumvented if the struct using the factory is generic over this factory type,
    // restricted by the trait (F: AssociatedTypeWidgetFactory)

    // However, because these types are differnt, they can't be stored in a single variable of the same type.
    // => Can't handle them polymorphically!
    // https://github.com/rust-lang/rfcs/pull/2515: This might fix that problem in the future.

    // Also, the buttons the factory create are of the concrete types. Therefore, it's not possible to use
    // them interchangeably, unless we store them as trait objects (either behind references or boxed).
    let mut buttons: Vec<Box<dyn Widget>> = Vec::new();
    let mut texts: Vec<Box<dyn Widget>> = Vec::new();

    buttons.push(Box::new(windows_factory.create_button()));
    buttons.push(Box::new(linux_factory.create_button()));

    texts.push(Box::new(windows_factory.create_text()));
    texts.push(Box::new(linux_factory.create_text()));

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

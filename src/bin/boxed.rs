use abstract_factory::abstract_factories::{
    boxed_widget_factories::{LinuxBoxedWidgetFactory, WindowsBoxedWidgetFactory},
    widget_factory::BoxedWidgetFactory,
};

fn main() {
    let windows_factory = WindowsBoxedWidgetFactory {};
    let linux_factory = LinuxBoxedWidgetFactory {};

    let factories = [
        &windows_factory as &dyn BoxedWidgetFactory,
        &linux_factory as &dyn BoxedWidgetFactory,
    ];

    let mut buttons = vec![];
    let mut texts = vec![];

    for f in factories {
        buttons.push(f.create_button());
        texts.push(f.create_text());
    }

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

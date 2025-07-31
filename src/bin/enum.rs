use abstract_factory::{
    abstract_factories::enum_widget_factories::{AnyWidget, EnumWidgetFactory},
    widgets::Widget,
};

fn main() {
    let windows_factory = EnumWidgetFactory::WindowsWidgetFactory {};
    let linux_factory = EnumWidgetFactory::LinuxWidgetFactory {};

    let mut buttons = vec![];
    buttons.push(windows_factory.create_button());
    buttons.push(linux_factory.create_button());

    let mut texts = vec![];
    texts.push(windows_factory.create_text());
    texts.push(linux_factory.create_text());

    let mut all_widgets: Vec<AnyWidget> = texts.into_iter().map(|t| AnyWidget::Text(t)).collect();
    all_widgets.append(&mut buttons.into_iter().map(|b| AnyWidget::Button(b)).collect());

    let app = App::new(all_widgets);

    app.render();
}

pub struct App {
    widgets: Vec<AnyWidget>,
}

impl App {
    pub fn new(widgets: Vec<AnyWidget>) -> Self {
        App { widgets }
    }

    pub fn render(&self) {
        for w in &self.widgets {
            w.render();
        }
    }
}

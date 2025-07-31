use crate::widgets::{LinuxButton, LinuxText, Widget, WindowsButton, WindowsText};

// The enum approach has better performance than the boxed approach, and is probably
// a bit more idiomatic. There's a lot of boiler plate due to the wrapping though,
// especially when a lot of different types of widgets exist. Every step of abstraction
// requires a new wrapping enum (one for buttons, one for widgets in general).
// Cutting out the Button or Text enum would hurt readability badly though, as
// the AnyWidget enum would hold the OS-specific buttons directly. This approach would even
// break down completely when buttons have different functionality than texts (beyond the
// common widget funcitonality).

pub enum EnumWidgetFactory {
    WindowsWidgetFactory,
    LinuxWidgetFactory,
}

// Enum wrapper for buttons - no boxing needed!
// Alternatively, you could return a Box<Widget> of course.
pub enum Button {
    Windows(WindowsButton),
    Linux(LinuxButton),
}

impl Widget for Button {
    fn render(&self) {
        match self {
            Button::Windows(btn) => btn.render(),
            Button::Linux(btn) => btn.render(),
        }
    }
}

pub enum Text {
    Windows(WindowsText),
    Linux(LinuxText),
}

impl Widget for Text {
    fn render(&self) {
        match self {
            Text::Windows(text) => text.render(),
            Text::Linux(text) => text.render(),
        }
    }
}

pub enum AnyWidget {
    Button(Button),
    Text(Text),
}

impl Widget for AnyWidget {
    fn render(&self) {
        match self {
            AnyWidget::Button(button) => button.render(),
            AnyWidget::Text(text) => text.render(),
        }
    }
}

impl EnumWidgetFactory {
    pub fn create_button(&self) -> Button {
        match self {
            EnumWidgetFactory::WindowsWidgetFactory => Button::Windows(WindowsButton {}),
            EnumWidgetFactory::LinuxWidgetFactory => Button::Linux(LinuxButton {}),
        }
    }

    pub fn create_text(&self) -> Text {
        match self {
            EnumWidgetFactory::WindowsWidgetFactory => Text::Windows(WindowsText {}),
            EnumWidgetFactory::LinuxWidgetFactory => Text::Linux(LinuxText {}),
        }
    }
}

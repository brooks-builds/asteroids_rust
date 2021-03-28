use ggez::graphics::{Font, Scale, Text};

pub fn create_message(message: &str) -> Text {
    let mut text = Text::new(message);
    text.set_font(Font::default(), Scale::uniform(75.0));
    text
}

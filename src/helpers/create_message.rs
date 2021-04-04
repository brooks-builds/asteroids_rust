use ggez::graphics::{Font, Scale, Text};

pub fn create_message(message: &str, scale: f32) -> Text {
    let mut text = Text::new(message);
    text.set_font(Font::default(), Scale::uniform(scale));
    text
}

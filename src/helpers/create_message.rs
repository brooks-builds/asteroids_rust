use ggez::graphics::{Font, Scale, Text};
use ggez::GameResult;

pub fn create_message(message: &str) -> Text {
    let mut text = Text::new(message);
    text.set_font(Font::default(), Scale::uniform(15.0));
    text
}

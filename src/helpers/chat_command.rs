#[derive(Clone, Copy)]
pub enum ChatCommand {
    Asteroid,
    UFO,
    Random,
    None,
}

impl ChatCommand {
    pub fn from_message(message: &str) -> ChatCommand {
        let message = message.to_lowercase();
        if message.find("asteroid").is_some() {
            return Self::Asteroid;
        }
        if message.find("ufo").is_some() {
            return Self::UFO;
        }
        if message.find("random").is_some() {
            return Self::Random;
        }

        ChatCommand::None
    }
}

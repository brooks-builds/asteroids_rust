use super::chat_command::ChatCommand;

#[derive(Debug, strum_macros::ToString)]
pub enum PlatformFiringStrategy {
    Random,
    ClosestAsteroid,
    Ufo,
}

impl PlatformFiringStrategy {
    pub fn from_string(string: &str) -> Self {
        match string {
            "Random" => Self::Random,
            "ClosestAsteroid" => Self::ClosestAsteroid,
            "Ufo" => Self::Ufo,
            _ => Self::Random,
        }
    }

    pub fn from_chat_command(command: ChatCommand) -> Option<Self> {
        match command {
            ChatCommand::Asteroid => Some(Self::ClosestAsteroid),
            ChatCommand::UFO => Some(Self::Ufo),
            ChatCommand::Random => Some(Self::Random),
            ChatCommand::None => None,
        }
    }
}

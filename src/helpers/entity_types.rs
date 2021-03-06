#[derive(strum_macros::ToString)]
pub enum EntityTypes {
    Player,
    Asteroid,
    Message,
    Bullet,
    LevelText,
    Score,
    LivesMesh,
    UFO,
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn turn_entity_types_into_string() {
        assert_eq!(EntityTypes::Player.to_string(), "Player".to_owned());
    }
}

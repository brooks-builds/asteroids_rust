pub enum Names {
    BackgroundColor,
    ArenaSize,
    Location,
    Mesh,
    PlayerSize,
}

impl Into<String> for Names {
    fn into(self) -> String {
        match self {
            Names::BackgroundColor => "BackgroundColor".to_owned(),
            Names::ArenaSize => "ArenaSize".to_owned(),
            Names::Location => "Location".to_owned(),
            Names::Mesh => "Mesh".to_owned(),
            Names::PlayerSize => "PlayerSize".to_owned(),
        }
    }
}

pub enum Names {
    BackgroundColor,
    ArenaSize,
    Location,
    Mesh,
    PlayerSize,
    Thrusting,
    ThrusterColor,
    ThrustKeyCode,
}

impl Into<String> for Names {
    fn into(self) -> String {
        match self {
            Names::BackgroundColor => "BackgroundColor".to_owned(),
            Names::ArenaSize => "ArenaSize".to_owned(),
            Names::Location => "Location".to_owned(),
            Names::Mesh => "Mesh".to_owned(),
            Names::PlayerSize => "PlayerSize".to_owned(),
            Names::Thrusting => "Thrusting".to_owned(),
            Names::ThrusterColor => "ThrusterColor".to_owned(),
            Names::ThrustKeyCode => "ThrustKeyCode".to_owned(),
        }
    }
}

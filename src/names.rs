pub enum Names {
    Acceleration,
    ArenaSize,
    BackgroundColor,
    Location,
    PlayerMesh,
    PlayerShipColor,
    PlayerSize,
    Rotation,
    ThrusterColor,
    Thrusting,
    ThrustKeyCode,
    ThrustSpeed,
    Velocity,
}

impl Into<String> for Names {
    fn into(self) -> String {
        match self {
            Names::BackgroundColor => "BackgroundColor".to_owned(),
            Names::ArenaSize => "ArenaSize".to_owned(),
            Names::Location => "Location".to_owned(),
            Names::PlayerMesh => "PlayerMesh".to_owned(),
            Names::PlayerSize => "PlayerSize".to_owned(),
            Names::Thrusting => "Thrusting".to_owned(),
            Names::ThrusterColor => "ThrusterColor".to_owned(),
            Names::ThrustKeyCode => "ThrustKeyCode".to_owned(),
            Names::PlayerShipColor => "PlayerShipColor".to_owned(),
            Names::Rotation => "Rotation".to_owned(),
            Names::Acceleration => "Acceleration".to_owned(),
            Names::Velocity => "Velocity".to_owned(),
            Names::ThrustSpeed => "ThrustSpeed".to_owned(),
        }
    }
}

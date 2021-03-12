pub enum Names {
    Acceleration,
    ArenaSize,
    BackgroundColor,
    Location,
    Marker,
    Mesh,
    PlayerShipColor,
    PlayerSize,
    RotateRightKeyCode,
    RotateLeftKeyCode,
    Rotation,
    RotationSpeed,
    ThrusterColor,
    Thrusting,
    ThrustKeyCode,
    ThrustSpeed,
    UpdateFps,
    Velocity,
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
            Names::PlayerShipColor => "PlayerShipColor".to_owned(),
            Names::Rotation => "Rotation".to_owned(),
            Names::Acceleration => "Acceleration".to_owned(),
            Names::Velocity => "Velocity".to_owned(),
            Names::ThrustSpeed => "ThrustSpeed".to_owned(),
            Names::Marker => "Marker".to_owned(),
            Names::RotateRightKeyCode => "RotateRightKeyCode".to_owned(),
            Names::RotateLeftKeyCode => "RotateLeftKeyCode".to_owned(),
            Names::RotationSpeed => "RotationSpeed".to_owned(),
            Names::UpdateFps => "UpdateFps".to_owned(),
        }
    }
}

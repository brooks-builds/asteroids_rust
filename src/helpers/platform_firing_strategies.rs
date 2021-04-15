#[derive(strum_macros::ToString)]
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
}

use eyre::Result;

#[derive(strum_macros::ToString)]
pub enum PlatformFiringStrategy {
    Random,
    ClosestAsteroid,
}

impl PlatformFiringStrategy {
    pub fn from_string(string: &str) -> Self {
        match string {
            "Random" => Self::Random,
            "ClosestAsteroid" => Self::ClosestAsteroid,
            _ => Self::Random,
        }
    }
}

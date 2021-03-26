use thiserror::Error;

#[derive(Debug, Error)]
pub enum Errors {
    #[error("Could not find the player")]
    PlayerDoesNotExist,
}

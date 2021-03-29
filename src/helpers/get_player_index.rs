use bbecs::components::{CastComponents, ComponentData};
use bbecs::world::DataWrapper;
use eyre::Result;

use crate::errors::Errors;

pub fn get_player_index(player_id: u32, ids_from_query: &[&ComponentData]) -> Result<usize> {
    for (index, id) in ids_from_query.iter().enumerate() {
        let id: &DataWrapper<u32> = id.cast()?;
        let id = id.borrow();
        if player_id == *id {
            return Ok(index);
        }
    }
    Err(Errors::PlayerDoesNotExist.into())
}

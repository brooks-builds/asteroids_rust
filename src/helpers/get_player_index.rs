use super::entity_types::EntityTypes;
use super::names::Names;
use bbecs::components::CastComponents;
use bbecs::world::World;
use eyre::Result;

pub fn get_player_index(world: &World) -> Result<Option<usize>> {
    let wrapped_markers = world.query_one(Names::Marker.to_string()).unwrap().borrow();
    let entity_types: &Vec<String> = wrapped_markers.cast()?;
    for (index, entity_type) in entity_types.iter().enumerate() {
        if *entity_type == EntityTypes::Player.to_string() {
            return Ok(Some(index));
        }
    }
    Ok(None)
}

use bbecs::components::CastComponents;
use bbecs::world::{DataWrapper, World};
use eyre::Result;

use super::{entity_types::EntityTypes, names::Names};

pub fn count_entities_with_marker(world: &World, entity_type: EntityTypes) -> Result<u32> {
    let query = world.query(vec![&Names::Marker.to_string()])?;
    let markers = query.get(&Names::Marker.to_string()).unwrap();

    Ok(markers.iter().fold(0, |count, marker| {
        let wrapped_marker: &DataWrapper<String> = marker.cast().unwrap();
        let borrowed_marker = wrapped_marker.borrow();

        if *borrowed_marker == entity_type.to_string() {
            count + 1
        } else {
            count
        }
    }))
}

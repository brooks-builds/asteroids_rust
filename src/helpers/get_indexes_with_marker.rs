use bbecs::components::{CastComponents, ComponentData};
use bbecs::world::DataWrapper;
use eyre::Result;

use super::entity_types::EntityTypes;

pub fn get_indexes_with_marker(
    marker_components: &[&ComponentData],
    entity_type: EntityTypes,
) -> Result<Vec<usize>> {
    let mut indexes = vec![];

    for (index, marker_component) in marker_components.iter().enumerate() {
        let wrapped_marker: &DataWrapper<String> = marker_component.cast()?;
        let borrowed_marker = wrapped_marker.borrow();

        if *borrowed_marker == entity_type.to_string() {
            indexes.push(index);
        }
    }

    Ok(indexes)
}

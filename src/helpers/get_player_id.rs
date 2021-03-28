use std::cell::RefCell;
use std::rc::Rc;

use super::entity_types::EntityTypes;
use super::names::Names;
use bbecs::components::CastComponents;
use bbecs::world::{World, ENTITY_ID};
use eyre::Result;

pub fn get_player_id(world: &World) -> Result<Option<u32>> {
    let queries = world.query(vec![&Names::Marker.to_string(), ENTITY_ID])?;
    let marker_query = queries.get(&Names::Marker.to_string()).unwrap();
    let id_query = queries.get(ENTITY_ID).unwrap();

    for (index, component) in marker_query.iter().enumerate() {
        let wrapped_marker: &Rc<RefCell<String>> = component.cast()?;
        let marker = wrapped_marker.borrow();

        if *marker == EntityTypes::Player.to_string() {
            let id = id_query[index];
            let id: &Rc<RefCell<u32>> = id.cast()?;
            let id = id.borrow();
            return Ok(Some(*id));
        }
    }
    Ok(None)
}

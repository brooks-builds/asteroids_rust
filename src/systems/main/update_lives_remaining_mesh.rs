use bbecs::components::{CastComponents, ComponentData};
use bbecs::world::{DataWrapper, World, ENTITY_ID};
use eyre::{bail, Result};
use ggez::graphics::Mesh;
use ggez::Context;

use crate::helpers::entity_types::EntityTypes;
use crate::helpers::get_lives_left::get_lives_left;
use crate::helpers::{create_lives_left_mesh::create_lives_left_mesh, names::Names};

pub fn update_lives_remaining_mesh(world: &World, context: &mut Context) -> Result<()> {
    let lives_left = get_lives_left(world)?;

    let query = world.query(vec![
        &Names::Marker.to_string(),
        &Names::Mesh.to_string(),
        ENTITY_ID,
    ])?;
    let markers = query.get(&Names::Marker.to_string()).unwrap();
    let meshes = query.get(&Names::Mesh.to_string()).unwrap();
    let entity_ids = query.get(ENTITY_ID).unwrap();
    assert_eq!(markers.len(), meshes.len());

    let index = if let Some(index) = find_index_of_lives_remaining_mesh(markers)? {
        index
    } else {
        return Ok(());
    };

    if lives_left == 0 {
        let id: &DataWrapper<u32> = entity_ids[index].cast()?;
        world.delete_by_id(*id.borrow())?;
        return Ok(());
    }

    let new_mesh = create_lives_left_mesh(context, lives_left)?;

    let mesh: &DataWrapper<Mesh> = meshes[index].cast()?;
    let mut mesh = mesh.borrow_mut();

    *mesh = new_mesh;

    Ok(())
}

fn find_index_of_lives_remaining_mesh(markers: &[&ComponentData]) -> Result<Option<usize>> {
    for (index, marker) in markers.iter().enumerate() {
        let marker: &DataWrapper<String> = marker.cast()?;
        if *marker.borrow() == EntityTypes::LivesMesh.to_string() {
            return Ok(Some(index));
        }
    }

    Ok(None)
}

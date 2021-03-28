use bbecs::components::CastComponents;
use bbecs::resources::resource::ResourceCast;
use bbecs::world::{DataWrapper, World, ENTITY_ID};
use eyre::Result;
use ggez::graphics::{Color, Mesh};
use ggez::Context;

use crate::helpers::create_player_ship::create_player_ship;
use crate::helpers::get_player_id::get_player_id;
use crate::helpers::get_player_index::get_player_index;
use crate::helpers::names::Names;

pub fn update_mesh_system(context: &mut Context, world: &World) -> Result<()> {
    if let Some(player_id) = get_player_id(&world)? {
        let wrapped_is_thrusting = world.get_resource(Names::Thrusting.to_string())?.borrow();
        let is_thrusting: &bool = wrapped_is_thrusting.cast()?;
        let wrapped_thruster_color = world
            .get_resource(Names::ThrusterColor.to_string())?
            .borrow();
        let thruster_color: &Color = wrapped_thruster_color.cast()?;
        let wrapped_player_ship_color = world
            .get_resource(Names::PlayerShipColor.to_string())?
            .borrow();
        let player_ship_color: &Color = wrapped_player_ship_color.cast()?;
        let queries = world.query(vec![
            &Names::Size.to_string(),
            &Names::Mesh.to_string(),
            ENTITY_ID,
        ])?;
        let sizes = queries.get(&Names::Size.to_string()).unwrap();
        let meshes = queries.get(&Names::Mesh.to_string()).unwrap();
        let ids = queries.get(ENTITY_ID).unwrap();
        let player_index = get_player_index(player_id, &ids)?;
        let player_size: &DataWrapper<f32> = sizes[player_index].cast()?;
        let player_mesh: &DataWrapper<Mesh> = meshes[player_index].cast()?;
        let mut player_mesh = player_mesh.borrow_mut();

        let new_player_mesh = create_player_ship(
            context,
            *player_size.borrow(),
            *player_ship_color,
            *is_thrusting,
            *thruster_color,
        )
        .unwrap();

        *player_mesh = new_player_mesh;
    }

    Ok(())
}

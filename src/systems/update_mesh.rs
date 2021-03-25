use bbecs::components::CastComponents;
use bbecs::resources::resource::ResourceCast;
use bbecs::world::World;
use eyre::Result;
use ggez::graphics::{Color, Mesh};
use ggez::Context;

use crate::helpers::create_player_ship::create_player_ship;
use crate::helpers::get_player_id::get_player_index;
use crate::helpers::names::Names;

pub fn update_mesh_system(context: &mut Context, world: &World) -> Result<()> {
    if let Some(player_index) = get_player_index(&world)? {
        let wrapped_is_thrusting = world.get_resource(Names::Thrusting.to_string())?.borrow();
        let is_thrusting: &bool = wrapped_is_thrusting.cast()?;
        let wrapped_sizes = world.query_one(Names::Size.to_string())?.borrow();
        let sizes: &Vec<f32> = wrapped_sizes.cast()?;
        let wrapped_thruster_color = world
            .get_resource(Names::ThrusterColor.to_string())?
            .borrow();
        let thruster_color: &Color = wrapped_thruster_color.cast()?;
        let wrapped_player_ship_color = world
            .get_resource(Names::PlayerShipColor.to_string())?
            .borrow();
        let player_ship_color: &Color = wrapped_player_ship_color.cast()?;
        let mut wrapped_meshes = world
            .query_one(Names::Mesh.to_string())
            .unwrap()
            .borrow_mut();
        let meshes: &mut Vec<Mesh> = wrapped_meshes.cast_mut()?;

        let player_mesh = create_player_ship(
            context,
            sizes[player_index],
            *player_ship_color,
            *is_thrusting,
            *thruster_color,
        )
        .unwrap();

        meshes[player_index] = player_mesh;
    }

    Ok(())
}

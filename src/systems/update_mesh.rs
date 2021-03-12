use bbecs::components::CastComponents;
use bbecs::world::{World, WorldMethods};
use eyre::Result;
use ggez::graphics::{Color, Mesh};
use ggez::Context;

use crate::helpers::create_player_ship::create_player_ship;
use crate::helpers::get_player_index::get_player_index;
use crate::helpers::names::Names;

pub fn update_mesh_system(context: &mut Context, world: &mut World) -> Result<()> {
    if let Some(player_index) = get_player_index(&world)? {
        let is_thrusting: &bool = world.get_resource(Names::Thrusting)?;
        let player_size: &f32 = world.get_resource(Names::PlayerSize)?;
        let thruster_color: &Color = world.get_resource(Names::ThrusterColor)?;
        let player_ship_color: &Color = world.get_resource(Names::PlayerShipColor)?;
        let mut wrapped_meshes = world.query_one(Names::Mesh).unwrap().borrow_mut();
        let meshes: &mut Vec<Mesh> = wrapped_meshes.cast_mut()?;

        let player_mesh = create_player_ship(
            context,
            *player_size,
            *player_ship_color,
            *is_thrusting,
            *thruster_color,
        )
        .unwrap();

        meshes[player_index] = player_mesh;
    }

    Ok(())
}

use bbecs::components::CastComponents;
use bbecs::data_types::point::Point;
use bbecs::world::{World, WorldMethods};
use eyre::Result;
use ggez::graphics::{DrawParam, Mesh};
use ggez::{graphics, Context, GameResult};

use crate::names::Names;

pub fn draw_system(context: &mut Context, world: &mut World) -> Result<()> {
    let player_mesh: &Mesh = world.get_resource(Names::PlayerMesh)?;
    let wrapped_locations = world.query_one(Names::Location).unwrap().borrow();
    let locations: &Vec<Point> = wrapped_locations.cast()?;
    let wrapped_rotations = world.query_one(Names::Rotation).unwrap().borrow();
    let rotations: &Vec<f32> = wrapped_rotations.cast()?;

    graphics::draw(
        context,
        player_mesh,
        DrawParam::new()
            .dest(locations[0].to_array())
            .rotation(rotations[0]),
    )
    .unwrap();
    Ok(())
}

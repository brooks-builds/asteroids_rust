use bbecs::components::CastComponents;
use bbecs::data_types::point::Point;
use bbecs::world::World;
use eyre::Result;
use ggez::graphics::{DrawParam, Mesh};
use ggez::{graphics, Context};

use crate::helpers::names::Names;

pub fn draw_system(context: &mut Context, world: &World) -> Result<()> {
    let wrapped_locations = world.query_one(Names::Location).unwrap().borrow();
    let locations: &Vec<Point> = wrapped_locations.cast()?;
    let wrapped_rotations = world.query_one(Names::Rotation).unwrap().borrow();
    let rotations: &Vec<f32> = wrapped_rotations.cast()?;
    let wrapped_meshes = world
        .query_one(Names::Mesh)
        .expect("querying for meshes")
        .borrow();
    let meshes: &Vec<Mesh> = wrapped_meshes.cast()?;

    meshes.iter().enumerate().for_each(|(index, mesh)| {
        graphics::draw(
            context,
            mesh,
            DrawParam::new()
                .dest(locations[index].to_array())
                .rotation(rotations[index]),
        )
        .unwrap();
    });

    Ok(())
}

use bbecs::components::CastComponents;
use bbecs::data_types::point::Point;
use bbecs::world::World;
use eyre::Result;
use ggez::graphics::{self, DrawParam, Mesh};
use ggez::Context;

use crate::helpers::names::Names;

pub fn draw_system(particles_world: &World, context: &mut Context) -> Result<()> {
    let wrapped_meshes = particles_world.query_one(Names::Mesh.to_string())?.borrow();
    let wrapped_locations = particles_world
        .query_one(Names::Location.to_string())?
        .borrow();

    let meshes: &Vec<Mesh> = wrapped_meshes.cast()?;
    let locations: &Vec<Point> = wrapped_locations.cast()?;

    locations.iter().enumerate().for_each(|(index, location)| {
        graphics::draw(
            context,
            &meshes[index],
            DrawParam::new().dest(location.to_array()),
        )
        .unwrap();
    });
    Ok(())
}

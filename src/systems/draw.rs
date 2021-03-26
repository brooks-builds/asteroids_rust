use bbecs::components::CastComponents;
use bbecs::data_types::point::Point;
use bbecs::world::{DataWrapper, World};
use eyre::Result;
use ggez::graphics::{DrawParam, Mesh};
use ggez::{graphics, Context};

use crate::helpers::names::Names;

pub fn draw_system(context: &mut Context, world: &World) -> Result<()> {
    let query = world.query(vec![
        &Names::Location.to_string(),
        &Names::Rotation.to_string(),
        &Names::Mesh.to_string(),
    ])?;
    let locations = &query[0];
    let rotations = &query[1];
    let meshes = &query[2];

    for (index, location) in locations.iter().enumerate() {
        let location: &DataWrapper<Point> = location.cast()?;
        let rotation: &DataWrapper<f32> = rotations[index].cast()?;
        let mesh: &DataWrapper<Mesh> = meshes[index].cast()?;

        graphics::draw(
            context,
            &*mesh.borrow(),
            DrawParam::new()
                .rotation(*rotation.borrow())
                .dest(location.borrow().to_array()),
        )?;
    }
    Ok(())
}

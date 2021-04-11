use crate::helpers::names::Names;
use bbecs::components::CastComponents;
use bbecs::data_types::point::Point;
use bbecs::world::DataWrapper;
use bbecs::world::World;
use eyre::Result;
use ggez::graphics;
use ggez::graphics::{Color, DrawMode, DrawParam, MeshBuilder};
use ggez::Context;

#[allow(dead_code)]
pub fn render_hitboxes_system(world: &World, context: &mut Context) -> Result<()> {
    let query = world.query(vec![&Names::Location.to_string(), &Names::Size.to_string()])?;
    let locations = query.get(&Names::Location.to_string()).unwrap();
    let sizes = query.get(&Names::Size.to_string()).unwrap();
    let color = Color::new(1.0, 0.0, 0.0, 1.0);

    for (index, location) in locations.iter().enumerate() {
        let location: &DataWrapper<Point> = location.cast()?;
        let size: &DataWrapper<f32> = sizes[index].cast()?;
        let mesh = MeshBuilder::new()
            .circle(
                DrawMode::stroke(2.0),
                location.borrow().to_array(),
                *size.borrow(),
                0.1,
                color,
            )
            .build(context)?;

        graphics::draw(context, &mesh, DrawParam::new())?;
    }

    Ok(())
}

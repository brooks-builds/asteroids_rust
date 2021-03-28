use std::cell::RefCell;
use std::rc::Rc;

use bbecs::components::CastComponents;
use bbecs::data_types::point::Point;
use bbecs::world::World;
use eyre::Result;
use ggez::graphics::{self, Color, DrawParam, Mesh};
use ggez::Context;

use crate::helpers::names::Names;

pub fn draw_system(particles_world: &World, context: &mut Context) -> Result<()> {
    let query = particles_world.query(vec![
        &Names::Mesh.to_string(),
        &Names::Location.to_string(),
        &Names::DebrisColor.to_string(),
    ])?;
    let mesh_query = query.get(&Names::Mesh.to_string()).unwrap();
    let location_query = query.get(&Names::Location.to_string()).unwrap();
    let color_query = query.get(&Names::DebrisColor.to_string()).unwrap();

    for (index, location) in location_query.iter().enumerate() {
        let location: &Rc<RefCell<Point>> = location.cast()?;
        let location = location.borrow();
        let mesh: &Rc<RefCell<Mesh>> = mesh_query[index].cast()?;
        let mesh = mesh.borrow();
        let color: &Rc<RefCell<Color>> = color_query[index].cast()?;
        let color = color.borrow();

        graphics::draw(
            context,
            &*mesh,
            DrawParam::new().color(*color).dest(location.to_array()),
        )?;
    }

    Ok(())
}

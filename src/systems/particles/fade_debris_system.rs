use std::cell::RefCell;
use std::rc::Rc;

use bbecs::components::CastComponents;
use bbecs::resources::resource::ResourceCast;
use bbecs::world::World;
use eyre::Result;
use ggez::graphics::Color;

use crate::helpers::names::Names;

pub fn fade_debris_system(particles_world: &World) -> Result<()> {
    let query = particles_world.query(vec![&Names::DebrisColor.to_string()])?;
    let debris_color_query = query.get(&Names::DebrisColor.to_string()).unwrap();
    let wrapped_ticks_to_live = particles_world
        .get_resource(Names::DebrisTicksToLive.to_string())?
        .borrow();
    let ticks_to_live: &usize = wrapped_ticks_to_live.cast()?;

    for debris_color in debris_color_query {
        let debris_color: &Rc<RefCell<Color>> = debris_color.cast()?;
        let mut debris_color = debris_color.borrow_mut();

        debris_color.a -= 1.0 / *ticks_to_live as f32;
    }

    Ok(())
}

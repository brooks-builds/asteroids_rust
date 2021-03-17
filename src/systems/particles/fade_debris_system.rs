use bbecs::components::CastComponents;
use bbecs::resources::resource::ResourceCast;
use bbecs::world::World;
use eyre::Result;
use ggez::graphics::Color;

use crate::helpers::names::Names;

pub fn fade_debris_system(particles_world: &World) -> Result<()> {
    let mut wrapped_debris_colors = particles_world
        .query_one(Names::DebrisColor.to_string())?
        .borrow_mut();
    let debris_color: &mut Vec<Color> = wrapped_debris_colors.cast_mut()?;
    let wrapped_ticks_to_live = particles_world
        .get_resource(Names::DebrisTicksToLive.to_string())?
        .borrow();
    let ticks_to_live: &usize = wrapped_ticks_to_live.cast()?;

    debris_color.iter_mut().for_each(|color| {
        color.a -= 1.0 / *ticks_to_live as f32;
    });

    Ok(())
}

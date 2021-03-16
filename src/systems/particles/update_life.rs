use bbecs::components::CastComponents;
use bbecs::world::World;
use eyre::Result;

use crate::helpers::names::Names;

pub fn update_life_system(particles_world: &World) -> Result<()> {
    let mut wrapped_ticks_to_live = particles_world
        .query_one(Names::TicksToLive.to_string())?
        .borrow_mut();

    let ticks_to_live: &mut Vec<usize> = wrapped_ticks_to_live.cast_mut()?;

    ticks_to_live
        .iter_mut()
        .enumerate()
        .for_each(|(index, tick_to_live)| {
            *tick_to_live -= 1;

            if *tick_to_live == 0 {
                particles_world.delete_entity_by_index(index).unwrap();
            }
        });

    Ok(())
}

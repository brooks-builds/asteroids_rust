use bbecs::components::CastComponents;
use bbecs::world::{DataWrapper, World, ENTITY_ID};
use eyre::Result;

use crate::helpers::names::Names;

pub fn update_life_system(particles_world: &World) -> Result<()> {
    let query = particles_world.query(vec![&Names::TicksToLive.to_string(), ENTITY_ID])?;
    let ticks_to_live = query[0];
    let ids = query[1];

    ticks_to_live
        .iter()
        .enumerate()
        .try_for_each(|(index, tick_to_live)| {
            let tick_to_live: &DataWrapper<usize> = tick_to_live.cast()?;
            let mut tick_to_live = tick_to_live.borrow_mut();
            *tick_to_live -= 1;

            if *tick_to_live == 0 {
                let id: &DataWrapper<u32> = ids[index].cast()?;
                let id = id.borrow();
                particles_world.delete_by_id(*id).unwrap();
            }
            Ok(())
        });

    Ok(())
}

use bbecs::components::CastComponents;
use bbecs::world::{DataWrapper, World, ENTITY_ID};
use eyre::Result;

use crate::helpers::names::Names;

pub fn update_life_system(particles_world: &World) -> Result<()> {
    let query = particles_world.query(vec![&Names::TicksToLive.to_string(), ENTITY_ID])?;
    let ticks_to_live = query.get(&Names::TicksToLive.to_string()).unwrap();
    let ids = query.get(ENTITY_ID).unwrap();

    ticks_to_live
        .iter()
        .enumerate()
        .for_each(|(index, tick_to_live)| {
            let tick_to_live: &DataWrapper<usize> = tick_to_live.cast().unwrap();
            let mut tick_to_live = tick_to_live.borrow_mut();
            *tick_to_live -= 1;

            if *tick_to_live == 0 {
                let id: &DataWrapper<u32> = ids[index].cast().unwrap();
                let id = id.borrow();
                particles_world.delete_by_id(*id).unwrap();
            }
        });

    Ok(())
}

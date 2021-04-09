use crate::helpers::names::Names;
use bbecs::components::CastComponents;
use bbecs::world::DataWrapper;
use bbecs::world::World;
use eyre::Result;

pub fn increment_ticks_lived_system(world: &World) -> Result<()> {
    let query = world.query(vec![&Names::TicksLived.to_string()])?;
    let ticks_lived = query.get(&Names::TicksLived.to_string()).unwrap();

    for ticks in ticks_lived {
        let ticks: &DataWrapper<usize> = ticks.cast()?;
        *ticks.borrow_mut() += 1;
    }

    Ok(())
}

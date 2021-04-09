use crate::helpers::bitmask::MAIN_LAYER;
use crate::helpers::names::Names;
use bbecs::components::CastComponents;
use bbecs::world::DataWrapper;
use bbecs::world::World;
use eyre::Result;

pub fn activate_safe_bullets_system(world: &World) -> Result<()> {
    let query = world.query(vec![
        &Names::TicksLived.to_string(),
        &Names::CollisionBitMask.to_string(),
    ])?;
    let ticks_lived = query.get(&Names::TicksLived.to_string()).unwrap();
    let collision_bitmasks = query.get(&Names::CollisionBitMask.to_string()).unwrap();
    assert!(ticks_lived.len() == collision_bitmasks.len());

    for (index, ticks) in ticks_lived.iter().enumerate() {
        let ticks: &DataWrapper<usize> = ticks.cast()?;
        if *ticks.borrow() == 10 {
            let collision_bitmask: &DataWrapper<u32> = collision_bitmasks[index].cast()?;
            *collision_bitmask.borrow_mut() = MAIN_LAYER;
        }
    }

    Ok(())
}

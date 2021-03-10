use bbecs::components::CastComponents;
use bbecs::data_types::point::Point;
use bbecs::world::World;
use eyre::Result;

use crate::names::Names;

pub fn update_movement_system(world: &World) -> Result<()> {
    let mut wrapped_accelerations = world.query_one(Names::Acceleration).unwrap().borrow_mut();
    let mut wrapped_velocities = world.query_one(Names::Velocity).unwrap().borrow_mut();
    let mut wrapped_locations = world.query_one(Names::Location).unwrap().borrow_mut();
    let accelerations: &mut Vec<Point> = wrapped_accelerations.cast_mut()?;
    let velocities: &mut Vec<Point> = wrapped_velocities.cast_mut()?;
    let locations: &mut Vec<Point> = wrapped_locations.cast_mut()?;

    velocities[0].add(&accelerations[0]);
    locations[0].add(&velocities[0]);
    accelerations[0].multiply_scalar(0.0);
    Ok(())
}

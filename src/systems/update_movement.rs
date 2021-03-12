use bbecs::components::CastComponents;
use bbecs::data_types::point::Point;
use bbecs::world::World;
use eyre::Result;

use crate::helpers::names::Names;

pub fn update_movement_system(world: &World) -> Result<()> {
    let mut wrapped_accelerations = world.query_one(Names::Acceleration).unwrap().borrow_mut();
    let mut wrapped_velocities = world.query_one(Names::Velocity).unwrap().borrow_mut();
    let mut wrapped_locations = world.query_one(Names::Location).unwrap().borrow_mut();
    let accelerations: &mut Vec<Point> = wrapped_accelerations.cast_mut()?;
    let velocities: &mut Vec<Point> = wrapped_velocities.cast_mut()?;
    let locations: &mut Vec<Point> = wrapped_locations.cast_mut()?;

    locations
        .iter_mut()
        .enumerate()
        .for_each(|(index, location)| {
            velocities[index].add(&accelerations[index]);
            location.add(&velocities[index]);
            accelerations[index].multiply_scalar(0.0);
        });

    Ok(())
}

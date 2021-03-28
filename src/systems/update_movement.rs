use bbecs::components::CastComponents;
use bbecs::data_types::point::Point;
use bbecs::world::{DataWrapper, World};
use eyre::Result;

use crate::helpers::names::Names;

pub fn update_movement_system(world: &World) -> Result<()> {
    let queries = world.query(vec![
        &Names::Acceleration.to_string(),
        &Names::Velocity.to_string(),
        &Names::Location.to_string(),
    ])?;
    let accelerations = queries.get(&Names::Acceleration.to_string()).unwrap();
    let velocities = queries.get(&Names::Velocity.to_string()).unwrap();
    let locations = queries.get(&Names::Location.to_string()).unwrap();

    for (index, location) in locations.iter().enumerate() {
        let location: &DataWrapper<Point> = location.cast()?;
        let mut location = location.borrow_mut();
        let acceleration: &DataWrapper<Point> = accelerations[index].cast()?;
        let mut acceleration = acceleration.borrow_mut();
        let velocity: &DataWrapper<Point> = velocities[index].cast()?;
        let mut velocity = velocity.borrow_mut();
        *velocity += *acceleration;
        *location += *velocity;
        acceleration.multiply_scalar(0.0);
    }

    Ok(())
}

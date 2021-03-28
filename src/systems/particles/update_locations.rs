use bbecs::components::CastComponents;
use bbecs::data_types::point::Point;
use bbecs::world::{DataWrapper, World};
use eyre::Result;

use crate::helpers::names::Names;

pub fn update_locations_system(particles_world: &World) -> Result<()> {
    let query = particles_world.query(vec![
        &Names::Location.to_string(),
        &Names::Velocity.to_string(),
    ])?;
    let locations = query.get(&Names::Location.to_string()).unwrap();
    let velocities = query.get(&Names::Velocity.to_string()).unwrap();

    locations
        .iter()
        .enumerate()
        .try_for_each(|(index, location)| {
            let location: &DataWrapper<Point> = location.cast()?;
            let mut location = location.borrow_mut();
            let velocity: &DataWrapper<Point> = velocities[index].cast()?;
            let velocity = velocity.borrow();

            *location += *velocity;
            Ok(())
        })
}

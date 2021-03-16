use bbecs::components::CastComponents;
use bbecs::data_types::point::Point;
use bbecs::world::World;
use eyre::Result;

use crate::helpers::names::Names;

pub fn update_locations_system(particles_world: &World) -> Result<()> {
    let mut wrapped_locations = particles_world
        .query_one(Names::Location.to_string())?
        .borrow_mut();
    let wrapped_velocities = particles_world
        .query_one(Names::Velocity.to_string())?
        .borrow();

    let locations: &mut Vec<Point> = wrapped_locations.cast_mut()?;
    let velocities: &Vec<Point> = wrapped_velocities.cast()?;

    locations
        .iter_mut()
        .enumerate()
        .for_each(|(index, location)| {
            *location += velocities[index];
        });

    Ok(())
}

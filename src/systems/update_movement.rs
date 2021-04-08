use bbecs::components::{CastComponents, ComponentData};
use bbecs::data_types::point::Point;
use bbecs::world::{DataWrapper, World};
use eyre::Result;

use crate::helpers::entity_types::EntityTypes;
use crate::helpers::names::Names;

pub fn update_movement_system(world: &World) -> Result<()> {
    let queries = world.query(vec![
        &Names::Acceleration.to_string(),
        &Names::Velocity.to_string(),
        &Names::Location.to_string(),
        &Names::Marker.to_string(),
    ])?;
    let accelerations = queries.get(&Names::Acceleration.to_string()).unwrap();
    let velocities = queries.get(&Names::Velocity.to_string()).unwrap();
    let locations = queries.get(&Names::Location.to_string()).unwrap();
    let markers = queries.get(&Names::Marker.to_string()).unwrap();

    for (index, location) in locations.iter().enumerate() {
        let location: &DataWrapper<Point> = location.cast()?;
        let mut location = location.borrow_mut();
        let acceleration: &DataWrapper<Point> = accelerations[index].cast()?;
        let mut acceleration = acceleration.borrow_mut();
        let velocity: &DataWrapper<Point> = velocities[index].cast()?;
        let mut velocity = velocity.borrow_mut();
        *velocity += *acceleration;
        if is_ufo(markers, index)? {
            velocity.clamp(5.0, -5.0);
        }
        *location += *velocity;
        acceleration.multiply_scalar(0.0);
    }

    Ok(())
}

fn is_ufo(markers: &[&ComponentData], index: usize) -> Result<bool> {
    let marker: &DataWrapper<String> = markers[index].cast()?;
    Ok(*marker.borrow() == EntityTypes::UFO.to_string())
}

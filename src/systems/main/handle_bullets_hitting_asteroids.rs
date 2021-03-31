use crate::helpers::entity_types::EntityTypes;
use crate::helpers::get_indexes_with_marker::get_indexes_with_marker;
use crate::helpers::names::Names;
use bbecs::components::CastComponents;
use bbecs::data_types::point::Point;
use bbecs::world::{DataWrapper, World, ENTITY_ID};
use eyre::Result;
use ggez::Context;

/// Have the bullets hit the asteroids, which will break them up into smaller asteroids. Each time they are hit the
/// asteroid duplicates and flies off in a random direction, just a tiny bit faster.
///
/// 1. query for bullets and asteroids
/// 2. for each bullet
///   1. is bullet colliding with any of the asteroids?
///     1. if yes, then
///       1. destroy the bullet
///       2. create two new asteroids with half the size of the original if the size is above a threshold
///       3. destroy the asteroid
///       4. insert new asteroids into the world
pub fn handle_bullets_hitting_asteroids_system(
    world: &mut World,
    _context: &mut Context,
) -> Result<()> {
    let query = world.query(vec![
        &Names::Location.to_string(),
        &Names::Marker.to_string(),
        ENTITY_ID,
        &Names::Size.to_string(),
    ])?;

    let location_components = query.get(&Names::Location.to_string()).unwrap();
    let marker_components = query.get(&Names::Marker.to_string()).unwrap();
    let id_components = query.get(ENTITY_ID).unwrap();
    let size_components = query.get(&Names::Size.to_string()).unwrap();
    let bullet_indexes = get_indexes_with_marker(marker_components, EntityTypes::Bullet)?;
    let asteroid_indexes = get_indexes_with_marker(marker_components, EntityTypes::Asteroid)?;

    for bullet_index in bullet_indexes {
        let wrapped_bullet_location: &DataWrapper<Point> =
            location_components[bullet_index].cast()?;
        let borrowed_bullet_location = wrapped_bullet_location.borrow();

        for asteroid_index in &asteroid_indexes {
            let wrapped_asteroid_location: &DataWrapper<Point> =
                location_components[*asteroid_index].cast()?;
            let borrowed_asteroid_location = wrapped_asteroid_location.borrow();
            let wrapped_asteroid_size: &DataWrapper<f32> =
                size_components[*asteroid_index].cast()?;
            let borrowed_asteroid_size = wrapped_asteroid_size.borrow();

            if borrowed_bullet_location.distance_to(&borrowed_asteroid_location)
                < *borrowed_asteroid_size
            {
                let wrapped_bullet_id: &DataWrapper<u32> = id_components[bullet_index].cast()?;
                let borrowed_bullet_id = wrapped_bullet_id.borrow();
                let wrapped_asteroid_id: &DataWrapper<u32> =
                    id_components[*asteroid_index].cast()?;
                let borrowed_asteroid_id = wrapped_asteroid_id.borrow();

                world.delete_by_id(*borrowed_bullet_id)?;
                world.delete_by_id(*borrowed_asteroid_id)?;
            }
        }
    }

    Ok(())
}

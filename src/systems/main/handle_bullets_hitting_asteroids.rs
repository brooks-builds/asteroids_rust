use std::vec;

use crate::helpers::asteroid_data::AsteroidData;
use crate::helpers::entity_types::EntityTypes;
use crate::helpers::get_indexes_with_marker::get_indexes_with_marker;
use crate::helpers::names::Names;
use bbecs::components::CastComponents;
use bbecs::data_types::point::Point;
use bbecs::resources::resource::ResourceCast;
use bbecs::world::{DataWrapper, World, ENTITY_ID};
use eyre::Result;

pub fn handle_bullets_hitting_asteroids_system(world: &World) -> Result<Vec<AsteroidData>> {
    let query = world.query(vec![
        &Names::Location.to_string(),
        &Names::Marker.to_string(),
        ENTITY_ID,
        &Names::Size.to_string(),
        &Names::Velocity.to_string(),
    ])?;

    let location_components = query.get(&Names::Location.to_string()).unwrap();
    let marker_components = query.get(&Names::Marker.to_string()).unwrap();
    let id_components = query.get(ENTITY_ID).unwrap();
    let size_components = query.get(&Names::Size.to_string()).unwrap();
    let velocity_components = query.get(&Names::Velocity.to_string()).unwrap();
    let bullet_indexes = get_indexes_with_marker(marker_components, EntityTypes::Bullet)?;
    let asteroid_indexes = get_indexes_with_marker(marker_components, EntityTypes::Asteroid)?;
    let mut destroyed_asteroids = vec![];

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

                let wrapped_asteroid_velocity: &DataWrapper<Point> =
                    velocity_components[*asteroid_index].cast()?;
                let borrowed_asteroid_velocity = wrapped_asteroid_velocity.borrow();

                update_score(world)?;

                destroyed_asteroids.push(AsteroidData::new(
                    *borrowed_asteroid_size,
                    borrowed_asteroid_velocity.length(),
                    *borrowed_asteroid_location,
                ))
            }
        }
    }

    Ok(destroyed_asteroids)
}

fn update_score(world: &World) -> Result<()> {
    let level = get_level(world)?;
    let query = world.query(vec![&Names::Score.to_string()])?;
    let scores = query.get(&Names::Score.to_string()).unwrap();
    assert!(scores.len() == 1);
    let score: &DataWrapper<u32> = scores[0].cast()?;
    let mut score = score.borrow_mut();
    *score += level;
    Ok(())
}

fn get_level(world: &World) -> Result<u32> {
    let level = world.get_resource(&Names::Level.to_string())?.borrow();
    Ok(*level.cast()?)
}

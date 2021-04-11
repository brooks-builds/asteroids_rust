use crate::helpers::{
    create_ship_debris::create_ship_debris, entity_types::EntityTypes, names::Names,
};
use bbecs::components::{CastComponents, ComponentData};
use bbecs::data_types::point::Point;
use bbecs::resources::resource::ResourceCast;
use bbecs::world::{DataWrapper, World, ENTITY_ID};
use eyre::Result;
use ggez::Context;
use rand::prelude::ThreadRng;

pub fn handle_bullets_hitting_ships(
    world: &World,
    particles_world: &mut World,
    context: &mut Context,
    rng: &mut ThreadRng,
) -> Result<()> {
    let query = world.query(vec![
        &Names::Location.to_string(),
        &Names::Marker.to_string(),
        &Names::Size.to_string(),
        &Names::CollisionBitMask.to_string(),
        ENTITY_ID,
    ])?;
    let locations = query.get(&Names::Location.to_string()).unwrap();
    let markers = query.get(&Names::Marker.to_string()).unwrap();
    let sizes = query.get(&Names::Size.to_string()).unwrap();
    let collision_bitmask = query.get(&Names::CollisionBitMask.to_string()).unwrap();
    let entity_ids = query.get(ENTITY_ID).unwrap();

    let bullet_locations = get_bullet_location(locations, markers)?;
    let ship_locations = get_ship_locations(locations, markers)?;

    if ship_locations.is_empty() {
        return Ok(());
    }

    for ship_location_wrapper in ship_locations {
        let ship_location: &DataWrapper<Point> = ship_location_wrapper.location.cast()?;
        let ship_layer: &DataWrapper<u32> =
            collision_bitmask[ship_location_wrapper.index].cast()?;
        for bullet_location_wrapper in &bullet_locations {
            let bullet_location: &DataWrapper<Point> = bullet_location_wrapper.location.cast()?;
            let bullet_layer: &DataWrapper<u32> =
                collision_bitmask[bullet_location_wrapper.index].cast()?;

            if *ship_layer.borrow() & *bullet_layer.borrow() == 0 {
                continue;
            }

            let distance_to_ship = bullet_location
                .borrow()
                .distance_to(&*ship_location.borrow());
            let ship_size: &DataWrapper<f32> = sizes[ship_location_wrapper.index].cast()?;
            let ship_size = ship_size.borrow();
            if distance_to_ship < *ship_size {
                let ship_id: &DataWrapper<u32> = entity_ids[ship_location_wrapper.index].cast()?;
                world.delete_by_id(*ship_id.borrow())?;
                let bullet_id: &DataWrapper<u32> =
                    entity_ids[bullet_location_wrapper.index].cast()?;
                world.delete_by_id(*bullet_id.borrow())?;
                create_ship_debris(particles_world, context, rng, *ship_location.borrow())?;
                if is_ufo(markers[ship_location_wrapper.index])? {
                    increment_ufo_level_killed_on(world)?;
                    increment_score(world)?;
                }
            }
        }
    }

    Ok(())
}

fn get_bullet_location<'a>(
    locations: &[&'a ComponentData],
    markers: &[&ComponentData],
) -> Result<Vec<LocationWrapper<'a>>> {
    let mut bullet_locations = vec![];

    for (index, marker) in markers.iter().enumerate() {
        let marker: &DataWrapper<String> = marker.cast()?;
        if *marker.borrow() == EntityTypes::Bullet.to_string() {
            bullet_locations.push(LocationWrapper {
                location: locations[index],
                index,
            });
        }
    }

    Ok(bullet_locations)
}

fn get_ship_locations<'a>(
    locations: &[&'a ComponentData],
    markers: &[&ComponentData],
) -> Result<Vec<LocationWrapper<'a>>> {
    let mut ship_locations = vec![];

    for (index, marker) in markers.iter().enumerate() {
        let marker: &DataWrapper<String> = marker.cast()?;
        let marker = marker.borrow();
        if *marker == EntityTypes::Player.to_string() || *marker == EntityTypes::UFO.to_string() {
            ship_locations.push(LocationWrapper {
                location: locations[index],
                index,
            });
        }
    }

    Ok(ship_locations)
}

fn is_ufo(marker: &ComponentData) -> Result<bool> {
    let marker: &DataWrapper<String> = marker.cast()?;
    Ok(*marker.borrow() == EntityTypes::UFO.to_string())
}

fn increment_ufo_level_killed_on(world: &World) -> Result<()> {
    let level = world.get_resource(&Names::Level.to_string())?.borrow();
    let level: u32 = *level.cast()?;
    let mut ufo_level_killed_on = world
        .get_resource(&Names::UFOKilledOnLevel.to_string())?
        .borrow_mut();
    let ufo_level_killed_on: &mut u32 = ufo_level_killed_on.cast_mut()?;

    *ufo_level_killed_on = level;
    Ok(())
}

fn increment_score(world: &World) -> Result<()> {
    let query = world.query(vec![&Names::Score.to_string()])?;
    let scores = query.get(&Names::Score.to_string()).unwrap();
    assert!(scores.len() == 1);

    let score: &DataWrapper<u32> = scores[0].cast()?;
    *score.borrow_mut() += 100;
    Ok(())
}

struct LocationWrapper<'a> {
    pub location: &'a ComponentData,
    pub index: usize,
}

use crate::helpers::{
    entity_types::EntityTypes, get_player_id::get_player_id, get_player_index::get_player_index,
    names::Names,
};
use bbecs::components::{CastComponents, ComponentData};
use bbecs::data_types::point::Point;
use bbecs::world::{DataWrapper, World};
use eyre::Result;

pub fn handle_bullets_hitting_ships(world: &World) -> Result<()> {
    let query = world.query(vec![
        &Names::Location.to_string(),
        &Names::Marker.to_string(),
        &Names::Size.to_string(),
    ])?;
    let locations = query.get(&Names::Location.to_string()).unwrap();
    let markers = query.get(&Names::Marker.to_string()).unwrap();
    let sizes = query.get(&Names::Size.to_string()).unwrap();

    let bullet_locations = get_bullet_location(locations, markers)?;
    let ship_locations = get_ship_locations(locations, markers)?;

    if ship_locations.is_empty() {
        return Ok(());
    }

    for ship_location in ship_locations {
        let ship_location: &DataWrapper<Point> = ship_location.cast()?;
        for bullet_location in bullet_locations.iter() {
            let bullet_location: &DataWrapper<Point> = bullet_location.cast()?;
            let distance_to_ship = bullet_location
                .borrow()
                .distance_to(&ship_location.borrow());
            if distance_to_ship < 25.0 {
                dbg!("hit!");
            }
        }
    }
    Ok(())
}

fn get_bullet_location<'a>(
    locations: &[&'a ComponentData],
    markers: &[&ComponentData],
) -> Result<Vec<&'a ComponentData>> {
    let mut bullet_locations = vec![];

    for (index, marker) in markers.iter().enumerate() {
        let marker: &DataWrapper<String> = marker.cast()?;
        if *marker.borrow() == EntityTypes::Bullet.to_string() {
            bullet_locations.push(locations[index]);
        }
    }

    Ok(bullet_locations)
}

fn get_ship_locations<'a>(
    locations: &[&'a ComponentData],
    markers: &[&ComponentData],
) -> Result<Vec<&'a ComponentData>> {
    let mut ship_locations = vec![];

    for (index, marker) in markers.iter().enumerate() {
        let marker: &DataWrapper<String> = marker.cast()?;
        let marker = marker.borrow();
        if *marker == EntityTypes::Player.to_string() || *marker == EntityTypes::UFO.to_string() {
            ship_locations.push(locations[index]);
        }
    }

    Ok(ship_locations)
}

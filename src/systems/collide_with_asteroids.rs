use bbecs::components::CastComponents;
use bbecs::data_types::point::Point;
use bbecs::resources::resource::ResourceCast;
use bbecs::world::{DataWrapper, World, ENTITY_ID};
use eyre::Result;
use ggez::Context;
use rand::prelude::ThreadRng;

use crate::helpers::create_ship_debris::create_ship_debris;
use crate::helpers::entity_types::EntityTypes;
use crate::helpers::get_player_id;
use crate::helpers::get_player_index::get_player_index;
use crate::helpers::names::Names;

pub fn collide_with_asteroids_system(
    world: &World,
    particles_world: &mut World,
    context: &mut Context,
    rng: &mut ThreadRng,
) -> Result<()> {
    if let Some(player_id) = get_player_id::get_player_id(world)? {
        let query = world.query(vec![
            &Names::Location.to_string(),
            &Names::Marker.to_string(),
            &Names::Size.to_string(),
            ENTITY_ID,
        ])?;
        let locations = query.get(&Names::Location.to_string()).unwrap();
        let markers = query.get(&Names::Marker.to_string()).unwrap();
        let sizes = query.get(&Names::Size.to_string()).unwrap();
        let ids = query.get(ENTITY_ID).unwrap();
        let player_index = get_player_index(player_id, &ids)?;
        let player_location: &DataWrapper<Point> = locations[player_index].cast()?;
        let player_location = *player_location.borrow();

        locations.iter().enumerate().for_each(|(index, location)| {
            let location: &DataWrapper<Point> = location.cast().unwrap();
            let location = location.borrow();
            let marker: &DataWrapper<String> = markers[index].cast().unwrap();
            let marker = marker.borrow();
            let size: &DataWrapper<f32> = sizes[index].cast().unwrap();
            let size = *size.borrow();
            if *marker == EntityTypes::Asteroid.to_string()
                && location.distance_to(&&player_location) < size
            {
                world.delete_by_id(player_id).unwrap();
                create_ship_debris(particles_world, context, rng, player_location).unwrap();
                remove_player_life(world).unwrap();
            }
        });
    }

    Ok(())
}

fn remove_player_life(world: &World) -> Result<()> {
    let wrapped_player_lives_left = world.get_resource(&Names::LivesRemaining.to_string())?;
    let mut borrowed_player_lives_left = wrapped_player_lives_left.borrow_mut();
    let player_lives_left: &mut u32 = borrowed_player_lives_left.cast_mut()?;
    *player_lives_left -= 1;
    Ok(())
}

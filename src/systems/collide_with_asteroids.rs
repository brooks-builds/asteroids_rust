use bbecs::components::CastComponents;
use bbecs::data_types::point::Point;
use bbecs::world::{DataWrapper, World, ENTITY_ID};
use eyre::Result;
use ggez::{graphics, Context};
use rand::prelude::ThreadRng;

use crate::helpers::create_message::create_message;
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
                let screen_size = graphics::drawable_size(context);
                // We cannot mutate the world here, so we need to do this somewhere else
                crate::GameState::insert_message_into_world(
                    "You died :(...Press Return to start again",
                    &mut world,
                    screen_size,
                    context,
                )
                .unwrap();
            }
        });
    }

    Ok(())
}

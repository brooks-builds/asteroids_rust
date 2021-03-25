use bbecs::components::{CastComponents, ComponentData};
use bbecs::data_types::point::Point;
use bbecs::resources::resource::ResourceCast;
use bbecs::world::{DataWrapper, World, ENTITY_ID};
use eyre::Result;
use ggez::Context;
use rand::prelude::ThreadRng;

use crate::helpers::create_ship_debris::create_ship_debris;
use crate::helpers::entity_types::EntityTypes;
use crate::helpers::get_player_id;
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
        let locations = query[0];
        let markers = query[1];
        let sizes = query[2];
        let ids = query[3];
        let player_index = get_player_index(player_id, ids)?;
        let player_location: &DataWrapper<Point> = locations[player_index].cast()?;
        let player_location = *player_location.borrow();

        locations
            .iter()
            .enumerate()
            .try_for_each(|(index, location)| {
                let location: &DataWrapper<Point> = location.cast()?;
                let location = location.borrow();
                let marker: &DataWrapper<String> = markers[index].cast()?;
                let marker = *marker.borrow();
                let size: &DataWrapper<f32> = sizes[index].cast()?;
                let size = *size.borrow();
                if marker == EntityTypes::Asteroid.to_string()
                    && location.distance_to(&&player_location) < size
                {
                    world.delete_by_id(player_id)?;
                    create_ship_debris(particles_world, context, rng, player_location)?;
                }
                Ok(())
            });
    }

    Ok(())
}

fn get_player_index(player_id: u32, ids: Vec<&ComponentData>) -> Result<usize> {
    let mut player_index = 0;

    for (index, id) in ids.iter().enumerate() {
        let id: &DataWrapper<u32> = id.cast()?;
        let id = id.borrow();
        if player_id == *id {
            player_index = index;
            break;
        }
    }

    Ok(player_index)
}

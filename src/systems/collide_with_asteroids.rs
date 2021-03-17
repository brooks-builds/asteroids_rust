use bbecs::components::CastComponents;
use bbecs::data_types::point::Point;
use bbecs::resources::resource::ResourceCast;
use bbecs::world::World;
use eyre::Result;
use ggez::Context;
use rand::prelude::ThreadRng;

use crate::helpers::create_ship_debris::create_ship_debris;
use crate::helpers::entity_types::EntityTypes;
use crate::helpers::get_player_index::get_player_index;
use crate::helpers::names::Names;

pub fn collide_with_asteroids_system(
    world: &World,
    particles_world: &mut World,
    context: &mut Context,
    rng: &mut ThreadRng,
) -> Result<()> {
    if let Some(player_index) = get_player_index(world)? {
        let wrapped_locations = world.query_one(Names::Location.to_string())?.borrow();
        let wrapped_markers = world.query_one(Names::Marker.to_string())?.borrow();
        let wrapped_sizes = world.query_one(Names::Size.to_string())?.borrow();
        let mut wrapped_respawn_in = world
            .get_resource(Names::SpawnPlayerIn.to_string())?
            .borrow_mut();
        let wrapped_spawn_time = world.get_resource(Names::SpawnTime.to_string())?.borrow();

        let locations: &Vec<Point> = wrapped_locations.cast()?;
        let markers: &Vec<String> = wrapped_markers.cast()?;
        let sizes: &Vec<f32> = wrapped_sizes.cast()?;
        let respawn_in: &mut usize = wrapped_respawn_in.cast_mut()?;
        let spawn_time: &usize = wrapped_spawn_time.cast()?;

        let player_location = locations[player_index];
        locations.iter().enumerate().for_each(|(index, location)| {
            if markers[index] == EntityTypes::Asteroid.to_string()
                && location.distance_to(&player_location) < sizes[index]
            {
                world.delete_entity_by_index(player_index).unwrap();
                create_ship_debris(particles_world, context, rng, player_location).unwrap();
                *respawn_in = *spawn_time;
            }
        });
    }

    Ok(())
}

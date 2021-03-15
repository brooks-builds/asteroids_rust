use bbecs::components::CastComponents;
use bbecs::data_types::point::Point;
use bbecs::resources::resource::ResourceCast;
use bbecs::world::World;
use eyre::Result;

use crate::helpers::entity_types::EntityTypes;
use crate::helpers::get_player_index::get_player_index;
use crate::helpers::names::Names;

pub fn collide_with_asteroids_system(world: &World) -> Result<()> {
    if let Some(player_index) = get_player_index(world)? {
        let wrapped_locations = world.query_one(Names::Location.to_string())?.borrow();
        let wrapped_markers = world.query_one(Names::Marker.to_string())?.borrow();
        let wrapped_sizes = world.query_one(Names::Size.to_string())?.borrow();
        let mut wrapped_ship_destroyed = world
            .get_resource(Names::ShipDestroyed.to_string())?
            .borrow_mut();
        let locations: &Vec<Point> = wrapped_locations.cast()?;
        let markers: &Vec<String> = wrapped_markers.cast()?;
        let sizes: &Vec<f32> = wrapped_sizes.cast()?;
        let ship_destroyed: &mut bool = wrapped_ship_destroyed.cast_mut()?;

        let player_location = locations[player_index];
        locations.iter().enumerate().for_each(|(index, location)| {
            if markers[index] == EntityTypes::Asteroid.to_string()
                && location.distance_to(&player_location) < sizes[index]
            {
                let mut wrapped_last_known_position = world
                    .get_resource(Names::LastKnownPlayerLocation.to_string())
                    .unwrap()
                    .borrow_mut();
                let last_known_position: &mut Point =
                    wrapped_last_known_position.cast_mut().unwrap();
                world.delete_entity_by_index(player_index).unwrap();
                *ship_destroyed = true;
                *last_known_position = player_location;
            }
        });
    }

    Ok(())
}

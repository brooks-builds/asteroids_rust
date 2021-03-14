use bbecs::components::CastComponents;
use bbecs::data_types::point::Point;
use bbecs::world::World;
use eyre::Result;

use crate::helpers::entity_types::EntityTypes;
use crate::helpers::get_player_index::get_player_index;
use crate::helpers::names::Names;

pub fn collide_with_asteroids_system(world: &World) -> Result<()> {
    if let Some(player_index) = get_player_index(world)? {
        let wrapped_locations = world.query_one(Names::Location)?.borrow();
        let wrapped_markers = world.query_one(Names::Marker)?.borrow();
        let wrapped_sizes = world.query_one(Names::Size)?.borrow();
        let locations: &Vec<Point> = wrapped_locations.cast()?;
        let markers: &Vec<String> = wrapped_markers.cast()?;
        let sizes: &Vec<f32> = wrapped_sizes.cast()?;

        let player_location = locations[player_index];
        locations.iter().enumerate().for_each(|(index, location)| {
            if markers[index] == EntityTypes::Asteroid.to_string()
                && location.distance_to(&player_location) < sizes[index]
            {
                world.delete_entity_by_index(player_index).unwrap();
            }
        });
    }

    Ok(())
}

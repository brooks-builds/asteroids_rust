use bbecs::components::CastComponents;
use bbecs::data_types::point::Point;
use bbecs::resources::resource::ResourceCast;
use bbecs::world::World;
use eyre::Result;

use crate::helpers::names::Names;

pub fn handle_screen_edges_system(world: &World) -> Result<()> {
    let wrapped_arena_size = world.get_resource(Names::ArenaSize)?.borrow();
    let arena_size: &Point = wrapped_arena_size.cast()?;
    let mut wrapped_locations = world.query_one(Names::Location)?.borrow_mut();
    let locations: &mut Vec<Point> = wrapped_locations.cast_mut()?;

    locations.iter_mut().for_each(|location| {
        if location.x > arena_size.x {
            location.x = 0.0;
        } else if location.x < 0.0 {
            location.x = arena_size.x;
        }

        if location.y > arena_size.y {
            location.y = 0.0;
        } else if location.y < 0.0 {
            location.y = arena_size.y;
        }
    });
    Ok(())
}
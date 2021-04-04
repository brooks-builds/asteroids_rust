use bbecs::world::World;
use eyre::Result;
use ggez::Context;

use crate::helpers::asteroid_data::AsteroidData;
use crate::helpers::insert_asteroid_into_world::insert_asteroid_into_world;

pub fn insert_asteroids_system(
    world: &mut World,
    context: &mut Context,
    destroyed_asteroids: Vec<AsteroidData>,
) -> Result<()> {
    for mut destroyed_asteroid in destroyed_asteroids {
        if destroyed_asteroid.size < 20.0 {
            continue;
        }
        destroyed_asteroid.update_for_destroyed();
        for _ in 0..2 {
            insert_asteroid_into_world(
                world,
                destroyed_asteroid.size,
                context,
                destroyed_asteroid.speed,
                destroyed_asteroid.location,
            )?;
        }
    }
    Ok(())
}

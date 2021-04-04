use super::insert_asteroid_into_world::insert_asteroid_into_world;
use bbecs::data_types::point::Point;
use bbecs::world::World;
use eyre::Result;
use ggez::Context;
use rand::{thread_rng, Rng};

pub fn insert_asteroids_into_world(
    (width, height): (f32, f32),
    world: &mut World,
    asteroid_radius: f32,
    context: &mut Context,
    asteroid_speed: f32,
) -> Result<()> {
    let mut rng = thread_rng();
    let asteroid_location = Point::new(rng.gen_range(0.0..width), rng.gen_range(0.0..height));
    insert_asteroid_into_world(
        world,
        asteroid_radius,
        context,
        asteroid_speed,
        asteroid_location,
    )
}

use bbecs::data_types::point::Point;
use bbecs::resources::resource::ResourceCast;
use bbecs::world::{World, WorldMethods};
use eyre::Result;
use ggez::graphics::WHITE;
use ggez::Context;
use rand::prelude::ThreadRng;
use rand::Rng;

use super::create_debris_mesh::create_debris_mesh;
use super::names::Names;

pub fn create_ship_debris(
    particles_world: &mut World,
    context: &mut Context,
    rng: &mut ThreadRng,
    location: Point,
) -> Result<()> {
    let wrapped_speed = particles_world
        .get_resource(Names::DebrisParticleSpeed.to_string())?
        .borrow();
    let wrapped_count = particles_world
        .get_resource(Names::DebrisParticleCount.to_string())?
        .borrow();
    let wrapped_ticks_to_live = particles_world
        .get_resource(Names::DebrisTicksToLive.to_string())?
        .borrow();

    let speed: f32 = *wrapped_speed.cast()?;
    let count: u32 = *wrapped_count.cast()?;
    let ticks_to_live: usize = *wrapped_ticks_to_live.cast()?;
    let debris_mesh = create_debris_mesh(context, 2.0, WHITE).unwrap();

    drop(wrapped_count);
    drop(wrapped_speed);
    drop(wrapped_ticks_to_live);

    for _ in 0..count {
        let mut velocity = Point::new(rng.gen_range(-1.0..1.0), rng.gen_range(-1.0..1.0));
        velocity.normalize();
        velocity.multiply_scalar(speed);

        particles_world
            .spawn_entity()?
            .with_component(Names::Mesh.to_string(), debris_mesh.clone())?
            .with_component(Names::Velocity.to_string(), velocity)?
            .with_component(Names::Location.to_string(), location)?
            .with_component(Names::TicksToLive.to_string(), ticks_to_live)?;
    }

    Ok(())
}

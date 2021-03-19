use bbecs::data_types::point::Point;
use bbecs::resources::resource::ResourceCast;
use bbecs::world::{World, WorldMethods};
use eyre::Result;
use ggez::graphics::Color;
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
    let wrapped_debris_size = particles_world
        .get_resource(Names::DebrisSize.to_string())?
        .borrow();

    let speed: f32 = *wrapped_speed.cast()?;
    let count: u32 = *wrapped_count.cast()?;
    let ticks_to_live: usize = *wrapped_ticks_to_live.cast()?;
    let debris_size: f32 = *wrapped_debris_size.cast()?;
    let debris_mesh = create_debris_mesh(context, debris_size).unwrap();

    drop(wrapped_count);
    drop(wrapped_speed);
    drop(wrapped_ticks_to_live);
    drop(wrapped_debris_size);

    for _ in 0..count {
        let velocity = Point::new(rng.gen_range(-speed..speed), rng.gen_range(-speed..speed));

        particles_world
            .spawn_entity()?
            .with_component(&Names::Mesh.to_string(), debris_mesh.clone())?
            .with_component(&Names::Velocity.to_string(), velocity)?
            .with_component(&Names::Location.to_string(), location)?
            .with_component(&Names::TicksToLive.to_string(), ticks_to_live)?
            .with_component(
                &Names::DebrisColor.to_string(),
                Color::new(1.0, 1.0, 1.0, 1.0),
            )?;
    }

    Ok(())
}

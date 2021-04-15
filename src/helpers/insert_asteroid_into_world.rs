use bbecs::data_types::point::Point;
use bbecs::world::{World, WorldMethods};
use create_asteroid_mesh::create_asteroid_mesh;
use eyre::Result;
use ggez::Context;
use rand::random;

use super::bitmask::EnemyLayer;
use super::create_asteroid_mesh;
use super::entity_types::EntityTypes;
use super::names::Names;

pub fn insert_asteroid_into_world(
    world: &mut World,
    radius: f32,
    context: &mut Context,
    speed: f32,
    location: Point,
) -> Result<()> {
    let mesh = create_asteroid_mesh(context, radius).unwrap();
    let mut acceleration = Point::new(random::<f32>() - 0.5, random::<f32>() - 0.5);
    acceleration.normalize();
    acceleration.multiply_scalar(speed);

    world
        .spawn_entity()?
        .with_component(&Names::Location.to_string(), location)?
        .with_component(&Names::Velocity.to_string(), Point::new(0.0, 0.0))?
        .with_component(&Names::Acceleration.to_string(), acceleration)?
        .with_component(&Names::Mesh.to_string(), mesh)?
        .with_component(&Names::Rotation.to_string(), 0.0_f32)?
        .with_component(
            &Names::Marker.to_string(),
            EntityTypes::Asteroid.to_string(),
        )?
        .with_component(&Names::Size.to_string(), radius)?
        .with_component(&Names::CollisionBitMask.to_string(), EnemyLayer)?
        .with_component(&Names::Asteroid.to_string(), true)?;
    Ok(())
}

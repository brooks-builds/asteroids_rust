use bbecs::data_types::point::Point;
use bbecs::world::{World, WorldMethods};
use eyre::Result;
use ggez::Context;

use super::create_bullet_mesh;
use super::entity_types::EntityTypes;
use super::names::Names;

pub fn insert_bullet_into_world(
    context: &mut Context,
    world: &mut World,
    location: Point,
    acceleration: Point,
) -> Result<()> {
    let bullet_mesh = create_bullet_mesh::create_bullet_mesh(context)?;
    world
        .spawn_entity()?
        .with_component(&Names::Mesh.to_string(), bullet_mesh)?
        .with_component(&Names::Location.to_string(), location)?
        .with_component(&Names::Rotation.to_string(), 0.0_f32)?
        .with_component(&Names::Velocity.to_string(), Point::new(0.0, 0.0))?
        .with_component(&Names::Acceleration.to_string(), acceleration)?
        .with_component(&Names::Marker.to_string(), EntityTypes::Bullet.to_string())?
        .with_component(&Names::TicksToLive.to_string(), 125_usize)?;

    Ok(())
}

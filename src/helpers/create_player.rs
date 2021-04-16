use bbecs::data_types::point::Point;
use bbecs::world::{World, WorldMethods};
use eyre::Result;
use ggez::graphics::Mesh;

use super::bitmask::PLAYER_LAYER;
use super::entity_types::EntityTypes;
use super::names::Names;

pub fn create_player(
    world: &mut World,
    player_ship: Mesh,
    size: f32,
    location: Point,
) -> Result<()> {
    world
        .spawn_entity()?
        .with_component(&Names::Location.to_string(), location)?
        .with_component(&Names::Rotation.to_string(), 0.0_f32)?
        .with_component(&Names::Velocity.to_string(), Point::new(0.0, 0.0))?
        .with_component(&Names::Acceleration.to_string(), Point::new(0.0, 0.0))?
        .with_component(&Names::Mesh.to_string(), player_ship)?
        .with_component(&Names::Marker.to_string(), EntityTypes::Player.to_string())?
        .with_component(&Names::Size.to_string(), size)?
        .with_component(&Names::CollisionBitMask.to_string(), PLAYER_LAYER)?;
    Ok(())
}

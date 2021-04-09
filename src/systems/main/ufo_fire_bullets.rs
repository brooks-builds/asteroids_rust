use crate::helpers::{
    bitmask::SAFE_BULLETS_LAYER, create_bullet_mesh::create_bullet_mesh, entity_types::EntityTypes,
    names::Names,
};
use bbecs::components::{CastComponents, ComponentData};
use bbecs::data_types::point::Point;
use bbecs::world::{DataWrapper, World, WorldMethods};
use eyre::Result;
use ggez::Context;
use rand::random;

pub fn ufo_fire_bullets(world: &mut World, context: &mut Context) -> Result<()> {
    let query = world.query(vec![
        &Names::Location.to_string(),
        &Names::Marker.to_string(),
    ])?;
    let locations = query.get(&Names::Location.to_string()).unwrap();
    let markers = query.get(&Names::Marker.to_string()).unwrap();

    let ufo_index = if let Some(index) = get_index_of_ufo(markers)? {
        index
    } else {
        return Ok(());
    };

    if should_fire_bullet() {
        let bullet_mesh = create_bullet_mesh(context)?;
        let bullet_velocity = Point::new(0.0, 0.0);
        let mut bullet_acceleration = Point::new(random::<f32>() - 0.5, random::<f32>() - 0.5);
        bullet_acceleration.normalize();
        bullet_acceleration.multiply_scalar(10.0);
        let bullet_location = get_ufo_location(locations, ufo_index)?;
        let bullet_marker = EntityTypes::Bullet.to_string();
        let bullet_rotation = 0.0_f32;
        let bullet_ticks_to_live = 125_usize;
        let bullet_size = 2.0_f32;

        world
            .spawn_entity()?
            .with_component(&Names::Mesh.to_string(), bullet_mesh)?
            .with_component(&Names::Velocity.to_string(), bullet_velocity)?
            .with_component(&Names::Acceleration.to_string(), bullet_acceleration)?
            .with_component(&Names::Location.to_string(), bullet_location)?
            .with_component(&Names::Marker.to_string(), bullet_marker)?
            .with_component(&Names::Rotation.to_string(), bullet_rotation)?
            .with_component(&Names::TicksToLive.to_string(), bullet_ticks_to_live)?
            .with_component(&Names::Size.to_string(), bullet_size)?
            .with_component(&Names::CollisionBitMask.to_string(), SAFE_BULLETS_LAYER)?
            .with_component(&Names::TicksLived.to_string(), 0_usize)?;
    }

    Ok(())
}

fn get_index_of_ufo(markers: &[&ComponentData]) -> Result<Option<usize>> {
    for (index, marker) in markers.iter().enumerate() {
        let marker: &DataWrapper<String> = marker.cast()?;
        if *marker.borrow() == EntityTypes::UFO.to_string() {
            return Ok(Some(index));
        }
    }

    Ok(None)
}

fn should_fire_bullet() -> bool {
    let random_number = random::<f32>();
    random_number < 0.01
}

fn get_ufo_location(locations: &[&ComponentData], ufo_index: usize) -> Result<Point> {
    let ufo_location: &DataWrapper<Point> = locations[ufo_index].cast()?;
    Ok(*ufo_location.borrow())
}

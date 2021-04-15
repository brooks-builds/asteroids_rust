use crate::helpers::bitmask::EnemyLayer;
use crate::helpers::create_ufo_mesh::create_ufo_mesh;
use crate::helpers::{entity_types::EntityTypes, names::Names};
use bbecs::components::CastComponents;
use bbecs::data_types::point::Point;
use bbecs::resources::resource::ResourceCast;
use bbecs::world::{DataWrapper, World, WorldMethods};
use eyre::Result;
use ggez::Context;
use rand::prelude::ThreadRng;
use rand::Rng;

pub fn spawn_ufo_system(
    world: &mut World,
    context: &mut Context,
    rng: &mut ThreadRng,
) -> Result<()> {
    if does_ufo_exist(&world)? {
        return Ok(());
    }

    let level_wrapper = world.get_resource(&Names::Level.to_string())?.borrow();
    let level: u32 = *level_wrapper.cast()?;
    drop(level_wrapper);
    let ufo_killed_on_level_wrapper = world
        .get_resource(&Names::UFOKilledOnLevel.to_string())?
        .borrow();
    let ufo_killed_on_level: u32 = *ufo_killed_on_level_wrapper.cast()?;
    drop(ufo_killed_on_level_wrapper);

    if level <= ufo_killed_on_level {
        return Ok(());
    }

    let (width, height) = ggez::graphics::drawable_size(context);
    let ufo_size = get_ufo_size(&world)?;
    let ufo_mesh = create_ufo_mesh(context, ufo_size)?;
    let location = Point::new(rng.gen_range(0.0..width), rng.gen_range(0.0..height));
    let acceleration = Point::new(0.0, 0.0);
    let velocity = Point::new(0.0, 0.0);
    let rotation = 0.0;

    world
        .spawn_entity()?
        .with_component(&Names::Mesh.to_string(), ufo_mesh)?
        .with_component(&Names::Marker.to_string(), EntityTypes::UFO.to_string())?
        .with_component(&Names::Location.to_string(), location)?
        .with_component(&Names::Rotation.to_string(), rotation)?
        .with_component(&Names::Acceleration.to_string(), acceleration)?
        .with_component(&Names::Velocity.to_string(), velocity)?
        .with_component(&Names::CollisionBitMask.to_string(), EnemyLayer)?
        .with_component(&Names::Size.to_string(), ufo_size)?
        .with_component(&Names::UFO.to_string(), true)?;
    Ok(())
}

fn does_ufo_exist(world: &World) -> Result<bool> {
    let query = world.query(vec![&Names::Marker.to_string()])?;
    let markers = query.get(&Names::Marker.to_string()).unwrap();

    for marker in markers {
        let marker: &DataWrapper<String> = marker.cast()?;
        if *marker.borrow() == EntityTypes::UFO.to_string() {
            return Ok(true);
        }
    }

    Ok(false)
}

fn get_ufo_size(world: &World) -> Result<f32> {
    let size = world.get_resource(&Names::UFOSize.to_string())?.borrow();
    Ok(*size.cast()?)
}

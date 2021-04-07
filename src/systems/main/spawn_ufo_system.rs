use crate::helpers::create_ufo_mesh::create_ufo_mesh;
use crate::helpers::{entity_types::EntityTypes, names::Names};
use bbecs::components::CastComponents;
use bbecs::data_types::point::Point;
use bbecs::resources::resource::ResourceCast;
use bbecs::world::{DataWrapper, World, WorldMethods};
use eyre::Result;
use ggez::Context;

pub fn spawn_ufo_system(world: &mut World, context: &mut Context) -> Result<()> {
    if does_ufo_exist(&world)? {
        return Ok(());
    }

    let ufo_size = get_ufo_size(&world)?;
    let ufo_mesh = create_ufo_mesh(context, ufo_size)?;
    let location = Point::new(500.0, 500.0);
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
        .with_component(&Names::Velocity.to_string(), velocity)?;
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

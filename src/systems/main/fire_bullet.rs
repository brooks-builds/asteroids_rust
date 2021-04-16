use crate::helpers;
use crate::helpers::names::Names;
use bbecs::components::CastComponents;
use bbecs::data_types::point::Point;
use bbecs::resources::resource::ResourceCast;
use bbecs::world::{DataWrapper, World};
use eyre::{bail, Result};
use ggez::input::keyboard::KeyCode;
use ggez::{input, Context};
use helpers::bitmask::ENEMY_LAYER;

use crate::helpers::entity_types::EntityTypes;
use crate::helpers::get_player_id::get_player_id;
use crate::helpers::insert_bullet_into_world::insert_bullet_into_world;

pub fn fire_bullet_system(world: &mut World, context: &mut Context) -> Result<()> {
    let reloading_ticks_left = decrement_reloading(&world)?;
    if get_player_id(&world)?.is_none()
        || !input::keyboard::is_key_pressed(context, KeyCode::Space)
        || reloading_ticks_left > 0
    {
        return Ok(());
    }

    let (location, rotation) = get_player_location(&world)?;
    let x = rotation.cos();
    let y = rotation.sin();
    let mut acceleration = Point::new(x, y);
    acceleration.normalize();
    acceleration.multiply_scalar(13.0);

    insert_bullet_into_world(context, world, location, acceleration, ENEMY_LAYER)?;
    set_reloading_ticks_left(&world)?;

    Ok(())
}

fn get_player_location(world: &World) -> Result<(Point, f32)> {
    let query = world.query(vec![
        &Names::Location.to_string(),
        &Names::Marker.to_string(),
        &Names::Rotation.to_string(),
    ])?;
    let locations = query.get(&Names::Location.to_string()).unwrap();
    let markers = query.get(&Names::Marker.to_string()).unwrap();
    let rotations = query.get(&Names::Rotation.to_string()).unwrap();

    for (index, marker) in markers.iter().enumerate() {
        let marker: &DataWrapper<String> = marker.cast()?;
        let marker = marker.borrow();

        if *marker == EntityTypes::Player.to_string() {
            let location: &DataWrapper<Point> = locations[index].cast()?;
            let rotation: &DataWrapper<f32> = rotations[index].cast()?;
            return Ok((*location.borrow(), *rotation.borrow()));
        }
    }
    bail!("Could not find player location")
}

fn decrement_reloading(world: &World) -> Result<u32> {
    let mut wrapped_reloading_ticks_left = world
        .get_resource(&Names::ReloadingTicksLeft.to_string())?
        .borrow_mut();
    let reloading_ticks_left: &mut u32 = wrapped_reloading_ticks_left.cast_mut()?;
    if *reloading_ticks_left > 0 {
        *reloading_ticks_left -= 1;
    }
    Ok(*reloading_ticks_left)
}

fn set_reloading_ticks_left(world: &World) -> Result<()> {
    let mut wrapped_reloading_ticks_left = world
        .get_resource(&Names::ReloadingTicksLeft.to_string())?
        .borrow_mut();
    let reloading_ticks_left: &mut u32 = wrapped_reloading_ticks_left.cast_mut()?;
    *reloading_ticks_left = 10;
    Ok(())
}

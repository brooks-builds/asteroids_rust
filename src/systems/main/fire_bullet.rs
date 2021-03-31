use crate::helpers::names::Names;
use bbecs::components::CastComponents;
use bbecs::data_types::point::Point;
use bbecs::world::{DataWrapper, World};
use eyre::{bail, Result};
use ggez::input::keyboard::KeyCode;
use ggez::Context;

use crate::helpers::entity_types::EntityTypes;
use crate::helpers::get_player_id::get_player_id;
use crate::helpers::insert_bullet_into_world::insert_bullet_into_world;

pub fn fire_bullet_system(
    world: &mut World,
    keycode: KeyCode,
    context: &mut Context,
) -> Result<()> {
    if get_player_id(&world)?.is_none() || keycode != KeyCode::Space {
        return Ok(());
    }

    let (location, rotation) = get_player_location(&world)?;
    let x = rotation.cos();
    let y = rotation.sin();
    let mut acceleration = Point::new(x, y);
    acceleration.normalize();
    acceleration.multiply_scalar(13.0);

    insert_bullet_into_world(context, world, location, acceleration)?;

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

use crate::helpers::{
    create_player::create_player, create_player_ship::create_player_ship,
    entity_types::EntityTypes, get_player_id::get_player_id,
};
use bbecs::components::CastComponents;
use bbecs::data_types::point::Point;
use bbecs::resources::resource::ResourceCast;
use bbecs::world::{DataWrapper, World, ENTITY_ID};
use eyre::Result;
use ggez::graphics::{self, Color};
use ggez::Context;

use crate::helpers::names::Names;

pub fn handle_respawn_system(world: &mut World, context: &mut Context) -> Result<()> {
    let borrowed_lives_remaining = world
        .get_resource(&Names::LivesRemaining.to_string())?
        .borrow();
    let lives_remaining: u32 = *borrowed_lives_remaining.cast()?;
    drop(borrowed_lives_remaining);

    if get_player_id(&world)?.is_none()
        && lives_remaining > 0
        && is_player_safe_to_respawn(&world, context)?
    {
        let wrapped_player_size = world.get_resource(&Names::PlayerSize.to_string())?.borrow();
        let player_size: f32 = *wrapped_player_size.cast()?;
        let wrapped_player_ship_color = world
            .get_resource(&Names::PlayerShipColor.to_string())?
            .borrow();
        let player_ship_color: &Color = wrapped_player_ship_color.cast()?;
        let wrapped_thruster_color = world
            .get_resource(&Names::ThrusterColor.to_string())?
            .borrow();
        let thruster_color: &Color = wrapped_thruster_color.cast()?;

        remove_messages(&world)?;

        let player_ship = create_player_ship(
            context,
            player_size,
            *player_ship_color,
            false,
            *thruster_color,
        )?;
        let (width, height) = graphics::drawable_size(context);
        let player_location = Point::new(width / 2.0, height / 2.0);
        drop(wrapped_player_ship_color);
        drop(wrapped_player_size);
        drop(wrapped_thruster_color);
        create_player(world, player_ship, player_size, player_location)?;
    }
    Ok(())
}

fn remove_messages(world: &World) -> Result<()> {
    let query = world.query(vec![&Names::Marker.to_string(), ENTITY_ID])?;
    let ids = query.get(ENTITY_ID).unwrap();
    let markers = query.get(&Names::Marker.to_string()).unwrap();
    for (index, marker) in markers.iter().enumerate() {
        let marker: &DataWrapper<String> = marker.cast()?;
        if *marker.borrow() == EntityTypes::Message.to_string() {
            let id: &DataWrapper<u32> = ids[index].cast()?;
            world.delete_by_id(*id.borrow())?;
        }
    }
    Ok(())
}

fn is_player_safe_to_respawn(world: &World, context: &mut Context) -> Result<bool> {
    let query = world.query(vec![
        &Names::Location.to_string(),
        &Names::Size.to_string(),
        &Names::Asteroid.to_string(),
    ])?;
    let locations = query.get(&Names::Location.to_string()).unwrap();
    let sizes = query.get(&Names::Size.to_string()).unwrap();
    let (width, height) = graphics::drawable_size(context);
    let center = Point::new(width / 2.0, height / 2.0);

    for (index, location) in locations.iter().enumerate() {
        let location: &DataWrapper<Point> = location.cast()?;
        let size: &DataWrapper<f32> = sizes[index].cast()?;
        let distance_point = center - *location.borrow();

        if distance_point.length() < *size.borrow() * 2.0 {
            return Ok(false);
        }
    }

    Ok(true)
}

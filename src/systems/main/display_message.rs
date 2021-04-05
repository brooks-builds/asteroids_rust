use bbecs::components::{CastComponents, ComponentData};
use bbecs::data_types::point::Point;
use bbecs::resources::resource::ResourceCast;
use bbecs::world::{DataWrapper, World, WorldMethods};
use eyre::Result;
use ggez::{graphics, Context};

use crate::helpers::entity_types::EntityTypes;
use crate::helpers::names::Names;
use crate::helpers::{create_message, get_player_id};

pub fn handle_message_system(world: &mut World, context: &mut Context) -> Result<()> {
    if get_player_id::get_player_id(&world)?.is_none() {
        let query = world.query(vec![&Names::Marker.to_string()])?;
        let markers = query.get(&Names::Marker.to_string()).unwrap();
        if is_message_in_query(markers)? {
            return Ok(());
        }

        let wrapped_lives_remaining = world.get_resource(&Names::LivesRemaining.to_string())?;
        let lives_remaining: u32 = *wrapped_lives_remaining.borrow().cast()?;

        let message = match lives_remaining {
            3 => "Press Enter to start the game",
            2 => "2 lives left, press enter to re-spawn",
            1 => "last life! Press enter to re-spawn",
            0 => "Game over",
            _ => "What happened?",
        };

        insert_message_into_world(message, world, context)?;
    }

    Ok(())
}

fn insert_message_into_world(
    message: &str,
    world: &mut World,
    context: &mut Context,
) -> Result<()> {
    let screen_size = graphics::drawable_size(context);
    let text = create_message::create_message(message, 75.0);
    let location = Point::new(
        screen_size.0 / 2.0 - text.width(context) as f32 / 2.0,
        screen_size.1 / 2.0 - text.height(context) as f32 / 2.0,
    );
    world
        .spawn_entity()?
        .with_component(&Names::Location.to_string(), location)?
        .with_component(&Names::Marker.to_string(), EntityTypes::Message.to_string())?
        .with_component(&Names::Message.to_string(), text)?;
    Ok(())
}

fn is_message_in_query(markers: &[&ComponentData]) -> Result<bool> {
    for marker in markers {
        let marker: &DataWrapper<String> = marker.cast()?;
        if *marker.borrow() == EntityTypes::Message.to_string() {
            return Ok(true);
        }
    }

    Ok(false)
}

use crate::helpers::{entity_types::EntityTypes, names::Names};
use bbecs::components::CastComponents;
use bbecs::resources::resource::ResourceCast;
use bbecs::world::{DataWrapper, World};
use eyre::Result;
use ggez::graphics::Text;

use crate::helpers::create_message::create_message;

pub fn update_display_level_system(world: &World) -> Result<()> {
    let query = world.query(vec![
        &Names::Marker.to_string(),
        &Names::Message.to_string(),
    ])?;
    let markers = query.get(&Names::Marker.to_string()).unwrap();
    let messages = query.get(&Names::Message.to_string()).unwrap();
    let level = get_level(world)?;

    for (index, marker) in markers.iter().enumerate() {
        let marker: &DataWrapper<String> = marker.cast()?;
        let marker = marker.borrow();

        if *marker == EntityTypes::LevelText.to_string() {
            let level_text: &DataWrapper<Text> = messages[index].cast()?;
            let mut level_text = level_text.borrow_mut();
            *level_text = create_message(&format!("Level: {}", level), 25.0);
        }
    }

    Ok(())
}

fn get_level(world: &World) -> Result<u32> {
    let level = world.get_resource(&Names::Level.to_string())?.borrow();
    let level: &u32 = level.cast()?;
    Ok(*level)
}

use crate::helpers::chat_command::{self, ChatCommand};
use crate::helpers::names::Names;
use crate::helpers::platform_firing_strategies::PlatformFiringStrategy;
use bbecs::components::CastComponents;
use bbecs::data_types::point::Point;
use bbecs::world::DataWrapper;
use bbecs::world::{World, WorldMethods};
use eyre::Result;
use ggez::graphics::{
    self, Color, DrawMode, Font, Mesh, MeshBuilder, Rect, Scale, Text, BLACK, WHITE,
};
use ggez::Context;
use rand::prelude::ThreadRng;
use rand::Rng;
use std::sync::mpsc::Receiver;
use twitch_chat_wrapper::ChatMessage;

pub fn handle_chat_message_system(
    receive_from_chat: &Receiver<ChatMessage>,
    world: &mut World,
    context: &mut Context,
    rng: &mut ThreadRng,
) -> Result<()> {
    let message = if let Ok(message) = receive_from_chat.try_recv() {
        message
    } else {
        return Ok(());
    };
    let chat_command = chat_command::ChatCommand::from_message(&message.message);
    let chatter_name = if let Some(name) = message.display_name {
        name
    } else {
        message.name
    };

    if update_platform_firing_strategy(&world, &chatter_name, &chat_command)?.is_some() {
        return Ok(());
    }

    let location = choose_location(rng, context)?;
    let size = 50.0;
    let mesh = create_platform_mesh(context, size, message.color_rgb)?;
    let label = create_label(&chatter_name);

    // platform doesn't exist, create platform with provided firing strategy
    insert_platform_into_world(
        world,
        location,
        mesh,
        size,
        chatter_name,
        label,
        chat_command,
    )?;

    Ok(())
}

fn insert_platform_into_world(
    world: &mut World,
    location: Point,
    mesh: Mesh,
    size: f32,
    chatter_name: String,
    label: Text,
    command: ChatCommand,
) -> Result<()> {
    let rotation = 0.0_f32;
    let firing_strategy = if let Some(strategy) = PlatformFiringStrategy::from_chat_command(command)
    {
        strategy
    } else {
        PlatformFiringStrategy::Random
    };
    world
        .spawn_entity()?
        .with_component(&Names::Location.to_string(), location)?
        .with_component(&Names::Rotation.to_string(), rotation)?
        .with_component(&Names::Mesh.to_string(), mesh)?
        .with_component(&Names::Size.to_string(), size)?
        .with_component(&Names::ChatterName.to_string(), chatter_name)?
        .with_component(&Names::Label.to_string(), label)?
        .with_component(
            &Names::PlatformFiringStrategy.to_string(),
            firing_strategy.to_string(),
        )?
        .with_component(&Names::TicksLived.to_string(), 0_usize)?;
    Ok(())
}

fn choose_location(rng: &mut ThreadRng, context: &mut Context) -> Result<Point> {
    let (width, height) = graphics::drawable_size(context);
    let x = rng.gen_range(0.0..width);
    let y = rng.gen_range(0.0..height);
    let location = Point::new(x, y);
    Ok(location)
}

fn create_platform_mesh(context: &mut Context, size: f32, color: (u8, u8, u8)) -> Result<Mesh> {
    let rect = Rect::new(-size / 2.0, -size / 2.0, size, size);
    let color = Color::from_rgb(color.0, color.1, color.2);
    Ok(MeshBuilder::new()
        .rectangle(DrawMode::fill(), rect, color)
        .rectangle(DrawMode::stroke(2.0), rect, WHITE)
        .circle(DrawMode::fill(), [0.0, 0.0], size / 8.0, 1.5, BLACK)
        .build(context)?)
}

fn update_platform_firing_strategy(
    world: &World,
    message_name: &str,
    command: &ChatCommand,
) -> Result<Option<()>> {
    let query = world.query(vec![
        &Names::ChatterName.to_string(),
        &Names::PlatformFiringStrategy.to_string(),
    ])?;
    let chatter_names = query.get(&Names::ChatterName.to_string()).unwrap();
    let firing_strategies = query
        .get(&Names::PlatformFiringStrategy.to_string())
        .unwrap();
    for (index, chatter_name) in chatter_names.iter().enumerate() {
        let chatter_name: &DataWrapper<String> = chatter_name.cast()?;
        if *chatter_name.borrow() == message_name {
            let new_firing_strategy = if let Some(firing_strategy) =
                PlatformFiringStrategy::from_chat_command(*command)
            {
                firing_strategy
            } else {
                return Ok(Some(()));
            };

            dbg!(&new_firing_strategy);
            let firing_strategy: &DataWrapper<String> = firing_strategies[index].cast()?;
            *firing_strategy.borrow_mut() = new_firing_strategy.to_string();
            return Ok(Some(()));
        }
    }
    Ok(None)
}

fn create_label(name: &str) -> Text {
    let mut text = Text::new(name);
    text.set_font(Font::default(), Scale::uniform(25.0));

    text
}

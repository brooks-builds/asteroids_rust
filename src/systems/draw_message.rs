use crate::helpers::names::Names;
use bbecs::components::CastComponents;
use bbecs::data_types::point::Point;
use bbecs::world::{DataWrapper, World};
use eyre::Result;
use ggez::graphics::{self, DrawParam, Text};
use ggez::Context;

pub fn draw_message_system(world: &World, context: &mut Context) -> Result<()> {
    let queries = world.query(vec![
        &Names::Message.to_string(),
        &Names::Location.to_string(),
    ])?;
    let messages = queries.get(&Names::Message.to_string()).unwrap();
    let locations = queries.get(&Names::Location.to_string()).unwrap();

    for (index, message) in messages.iter().enumerate() {
        let message: &DataWrapper<Text> = message.cast()?;
        let location: &DataWrapper<Point> = locations[index].cast()?;

        graphics::draw(
            context,
            &*message.borrow(),
            DrawParam::new().dest(location.borrow().to_array()),
        )?;
    }
    Ok(())
}

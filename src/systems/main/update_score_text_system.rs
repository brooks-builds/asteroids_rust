use crate::helpers::{create_message::create_message, names::Names};
use bbecs::components::CastComponents;
use bbecs::world::{DataWrapper, World};
use eyre::Result;
use ggez::graphics::Text;

pub fn update_score_text_system(world: &World) -> Result<()> {
    let query = world.query(vec![&Names::Score.to_string(), &Names::Message.to_string()])?;
    let scores = query.get(&Names::Score.to_string()).unwrap();
    let messages = query.get(&Names::Message.to_string()).unwrap();
    assert!(scores.len() == 1 && messages.len() == 1);
    let score: &DataWrapper<u32> = scores[0].cast()?;
    let messages: &DataWrapper<Text> = messages[0].cast()?;
    let new_score_text = create_message(&format!("Score: {}", score.borrow()), 25.0);
    *messages.borrow_mut() = new_score_text;
    Ok(())
}

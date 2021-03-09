use bbecs::world::{World, WorldMethods};
use ggez::event::KeyCode;
use ggez::input::keyboard;
use ggez::{Context, GameResult};

use crate::names::Names;

pub fn handle_input_system(world: &mut World, context: &mut Context) -> GameResult {
    let thrust_keycode: &KeyCode = world.get_resource(Names::ThrustKeyCode).unwrap();
    let thrust_keycode = *thrust_keycode;
    let is_thrusting: &mut bool = world.get_resource_mut(Names::Thrusting).unwrap();
    if keyboard::is_key_pressed(context, thrust_keycode) {
        *is_thrusting = true;
    } else {
        *is_thrusting = false;
    }
    Ok(())
}

use bbecs::components::CastComponents;
use bbecs::world::{World, WorldMethods};
use ggez::event::KeyCode;
use ggez::input::keyboard;
use ggez::{Context, GameResult};

use crate::helpers::get_player_index::get_player_index;
use crate::helpers::names::Names;

pub fn handle_input_system(world: &mut World, context: &mut Context) -> GameResult {
    if let Some(player_index) = get_player_index(&world).unwrap() {
        let thrust_keycode: &KeyCode = world.get_resource(Names::ThrustKeyCode).unwrap();
        let thrust_keycode = *thrust_keycode;
        let is_thrusting: &mut bool = world.get_resource_mut(Names::Thrusting).unwrap();
        if keyboard::is_key_pressed(context, thrust_keycode) {
            *is_thrusting = true;
        } else {
            *is_thrusting = false;
        }

        handle_rotation(world, context, player_index)?;
    }
    Ok(())
}

fn handle_rotation(world: &mut World, context: &mut Context, player_index: usize) -> GameResult {
    let rotate_left_keycode: &KeyCode = world.get_resource(Names::RotateLeftKeyCode).unwrap();
    let rotate_right_keycode: &KeyCode = world.get_resource(Names::RotateRightKeyCode).unwrap();
    let mut wrapped_rotations = world.query_one(Names::Rotation).unwrap().borrow_mut();
    let rotations: &mut Vec<f32> = wrapped_rotations.cast_mut().unwrap();
    let rotation_speed: &f32 = world.get_resource(Names::RotationSpeed).unwrap();

    if keyboard::is_key_pressed(context, *rotate_left_keycode) {
        rotations[player_index] -= *rotation_speed;
    } else if keyboard::is_key_pressed(context, *rotate_right_keycode) {
        rotations[player_index] += *rotation_speed;
    }

    Ok(())
}

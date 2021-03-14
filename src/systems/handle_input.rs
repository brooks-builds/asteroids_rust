use bbecs::components::CastComponents;
use bbecs::resources::resource::ResourceCast;
use bbecs::world::World;
use ggez::event::KeyCode;
use ggez::input::keyboard;
use ggez::{Context, GameResult};

use crate::helpers::get_player_index::get_player_index;
use crate::helpers::names::Names;

pub fn handle_input_system(world: &World, context: &mut Context) -> GameResult {
    if let Some(player_index) = get_player_index(&world).unwrap() {
        let wrapped_thrust_keycode = world.get_resource(Names::ThrustKeyCode).unwrap().borrow();
        let thrust_keycode: &KeyCode = wrapped_thrust_keycode.cast().unwrap();
        let mut wrapped_is_thrusting = world.get_resource(Names::Thrusting).unwrap().borrow_mut();
        let is_thrusting: &mut bool = wrapped_is_thrusting.cast_mut().unwrap();
        if keyboard::is_key_pressed(context, *thrust_keycode) {
            *is_thrusting = true;
        } else {
            *is_thrusting = false;
        }

        handle_rotation(world, context, player_index)?;
    }
    Ok(())
}

fn handle_rotation(world: &World, context: &mut Context, player_index: usize) -> GameResult {
    let wrapped_rotate_left_keycode = world
        .get_resource(Names::RotateLeftKeyCode)
        .unwrap()
        .borrow();
    let wrapped_rotate_right_keycode = world
        .get_resource(Names::RotateRightKeyCode)
        .unwrap()
        .borrow();
    let rotate_left_keycode: &KeyCode = wrapped_rotate_left_keycode.cast().unwrap();
    let rotate_right_keycode: &KeyCode = wrapped_rotate_right_keycode.cast().unwrap();
    let mut wrapped_rotations = world.query_one(Names::Rotation).unwrap().borrow_mut();
    let rotations: &mut Vec<f32> = wrapped_rotations.cast_mut().unwrap();
    let wrapped_rotation_speed = world.get_resource(Names::RotationSpeed).unwrap().borrow();
    let rotation_speed: &f32 = wrapped_rotation_speed.cast().unwrap();

    if keyboard::is_key_pressed(context, *rotate_left_keycode) {
        rotations[player_index] -= *rotation_speed;
    } else if keyboard::is_key_pressed(context, *rotate_right_keycode) {
        rotations[player_index] += *rotation_speed;
    }

    Ok(())
}

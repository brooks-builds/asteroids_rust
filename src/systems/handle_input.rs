use bbecs::components::CastComponents;
use bbecs::resources::resource::ResourceCast;
use bbecs::world::{DataWrapper, World, ENTITY_ID};
use eyre::Result;
use ggez::event::KeyCode;
use ggez::input::keyboard;
use ggez::Context;

use crate::helpers::get_player_id::get_player_id;
use crate::helpers::get_player_index::get_player_index;
use crate::helpers::names::Names;

pub fn handle_input_system(world: &World, context: &mut Context) -> Result<()> {
    if let Some(player_id) = get_player_id(&world)? {
        let wrapped_thrust_keycode = world
            .get_resource(Names::ThrustKeyCode.to_string())
            .unwrap()
            .borrow();
        let thrust_keycode: &KeyCode = wrapped_thrust_keycode.cast().unwrap();
        let mut wrapped_is_thrusting = world
            .get_resource(Names::Thrusting.to_string())
            .unwrap()
            .borrow_mut();
        let is_thrusting: &mut bool = wrapped_is_thrusting.cast_mut().unwrap();
        if keyboard::is_key_pressed(context, *thrust_keycode) {
            *is_thrusting = true;
        } else {
            *is_thrusting = false;
        }

        handle_rotation(world, context, player_id)?;
    }
    Ok(())
}

fn handle_rotation(world: &World, context: &mut Context, player_id: u32) -> Result<()> {
    let wrapped_rotate_left_keycode = world
        .get_resource(Names::RotateLeftKeyCode.to_string())
        .unwrap()
        .borrow();
    let wrapped_rotate_right_keycode = world
        .get_resource(Names::RotateRightKeyCode.to_string())
        .unwrap()
        .borrow();
    let rotate_left_keycode: &KeyCode = wrapped_rotate_left_keycode.cast().unwrap();
    let rotate_right_keycode: &KeyCode = wrapped_rotate_right_keycode.cast().unwrap();
    let wrapped_rotation_speed = world
        .get_resource(Names::RotationSpeed.to_string())
        .unwrap()
        .borrow();
    let rotation_speed: &f32 = wrapped_rotation_speed.cast().unwrap();
    let queries = world.query(vec![&Names::Rotation.to_string(), ENTITY_ID])?;
    let rotations = queries.get(&Names::Rotation.to_string()).unwrap();
    let ids = queries.get(ENTITY_ID).unwrap();
    let player_index = get_player_index(player_id, ids)?;

    if keyboard::is_key_pressed(context, *rotate_left_keycode) {
        let rotation: &DataWrapper<f32> = rotations[player_index].cast()?;
        let mut rotation = rotation.borrow_mut();
        *rotation -= *rotation_speed;
    } else if keyboard::is_key_pressed(context, *rotate_right_keycode) {
        let rotation: &DataWrapper<f32> = rotations[player_index].cast()?;
        let mut rotation = rotation.borrow_mut();
        *rotation += *rotation_speed;
    }

    Ok(())
}

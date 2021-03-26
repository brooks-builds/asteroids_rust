use bbecs::components::CastComponents;
use bbecs::data_types::point::Point;
use bbecs::resources::resource::ResourceCast;
use bbecs::world::{DataWrapper, World, ENTITY_ID};
use eyre::Result;

use crate::helpers::get_player_id::get_player_id;
use crate::helpers::get_player_index::get_player_index;
use crate::helpers::names::Names;

pub fn update_acceleration_system(world: &World) -> Result<()> {
    if let Some(player_id) = get_player_id(world)? {
        let wrapped_is_thrusting = world.get_resource(Names::Thrusting.to_string())?.borrow();
        let is_thrusting: &bool = wrapped_is_thrusting.cast()?;
        if *is_thrusting {
            let wrapped_thrust_speed = world.get_resource(Names::ThrustSpeed.to_string())?.borrow();
            let thrust_speed: &f32 = wrapped_thrust_speed.cast()?;
            let queries = world.query(vec![
                &Names::Acceleration.to_string(),
                &Names::Rotation.to_string(),
                ENTITY_ID,
            ])?;
            let accelerations = &queries[0];
            let rotations = &queries[1];
            let ids = &queries[2];
            let player_index = get_player_index(player_id, &ids)?;

            let player_rotation: &DataWrapper<f32> = rotations[player_index].cast()?;
            let player_rotation = player_rotation.borrow();

            let x = player_rotation.cos();
            let y = player_rotation.sin();
            let mut rotation = Point::new(x, y);
            rotation.multiply_scalar(*thrust_speed);

            let player_acceleration: &DataWrapper<Point> = accelerations[player_index].cast()?;
            let mut player_acceleration = player_acceleration.borrow_mut();
            player_acceleration.add(&rotation);
        }
    }
    Ok(())
}

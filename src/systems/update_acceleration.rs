use bbecs::components::CastComponents;
use bbecs::data_types::point::Point;
use bbecs::resources::resource::ResourceCast;
use bbecs::world::World;
use eyre::Result;

use crate::helpers::get_player_id::get_player_index;
use crate::helpers::names::Names;

pub fn update_acceleration_system(world: &World) -> Result<()> {
    if let Some(player_index) = get_player_index(world)? {
        let wrapped_is_thrusting = world.get_resource(Names::Thrusting.to_string())?.borrow();
        let is_thrusting: &bool = wrapped_is_thrusting.cast()?;
        if *is_thrusting {
            let wrapped_thrust_speed = world.get_resource(Names::ThrustSpeed.to_string())?.borrow();
            let thrust_speed: &f32 = wrapped_thrust_speed.cast()?;
            let mut wrapped_accelerations = world
                .query_one(Names::Acceleration.to_string())
                .unwrap()
                .borrow_mut();
            let accelerations: &mut Vec<Point> = wrapped_accelerations.cast_mut()?;
            let wrapped_rotations = world
                .query_one(Names::Rotation.to_string())
                .unwrap()
                .borrow();
            let rotations: &Vec<f32> = wrapped_rotations.cast()?;
            let x = rotations[player_index].cos();
            let y = rotations[player_index].sin();
            let mut rotation = Point::new(x, y);
            rotation.multiply_scalar(*thrust_speed);
            accelerations[player_index].add(&rotation);
        }
    }
    Ok(())
}

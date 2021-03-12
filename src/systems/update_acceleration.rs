use bbecs::components::CastComponents;
use bbecs::data_types::point::Point;
use bbecs::world::{World, WorldMethods};
use eyre::Result;

use crate::helpers::get_player_index::get_player_index;
use crate::helpers::names::Names;

pub fn update_acceleration_system(world: &World) -> Result<()> {
    if let Some(player_index) = get_player_index(world)? {
        let is_thrusting: &bool = world.get_resource(Names::Thrusting)?;
        if *is_thrusting {
            let thrust_speed: &f32 = world.get_resource(Names::ThrustSpeed)?;
            let mut wrapped_accelerations =
                world.query_one(Names::Acceleration).unwrap().borrow_mut();
            let accelerations: &mut Vec<Point> = wrapped_accelerations.cast_mut()?;
            let wrapped_rotations = world.query_one(Names::Rotation).unwrap().borrow();
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

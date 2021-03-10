use bbecs::components::CastComponents;
use bbecs::data_types::point::Point;
use bbecs::world::{self, World, WorldMethods};
use eyre::Result;

use crate::names::Names;

pub fn update_acceleration_system(world: &World) -> Result<()> {
    let is_thrusting: &bool = world.get_resource(Names::Thrusting)?;
    let thrust_speed: &f32 = world.get_resource(Names::ThrustSpeed)?;
    if *is_thrusting {
        let mut wrapped_accelerations = world.query_one(Names::Acceleration).unwrap().borrow_mut();
        let accelerations: &mut Vec<Point> = wrapped_accelerations.cast_mut()?;
        let wrapped_rotations = world.query_one(Names::Rotation).unwrap().borrow();
        let rotations: &Vec<f32> = wrapped_rotations.cast()?;
        let x = rotations[0].cos();
        let y = rotations[0].sin();
        let mut rotation = Point::new(x, y);
        rotation.multiply_scalar(*thrust_speed);
        accelerations[0].add(&rotation);
    }
    Ok(())
}

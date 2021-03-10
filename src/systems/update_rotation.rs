use bbecs::components::CastComponents;
use bbecs::data_types::point::Point;
use bbecs::world::World;
use eyre::Result;

use crate::names::Names;

pub fn update_rotation_system(world: &World) -> Result<()> {
    let mut wrapped_rotations = world.query_one(Names::Rotation).unwrap().borrow_mut();
    let rotations: &mut Vec<f32> = wrapped_rotations.cast_mut()?;
    Ok(())
}

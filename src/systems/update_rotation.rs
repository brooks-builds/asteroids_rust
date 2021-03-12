use bbecs::components::CastComponents;
use bbecs::world::World;
use eyre::Result;

use crate::helpers::names::Names;

pub fn update_rotation_system(world: &World) -> Result<()> {
    let mut wrapped_rotations = world.query_one(Names::Rotation).unwrap().borrow_mut();
    let _rotations: &mut Vec<f32> = wrapped_rotations.cast_mut()?;
    Ok(())
}

use super::names::Names;
use bbecs::resources::resource::ResourceCast;
use bbecs::world::World;
use eyre::Result;

#[allow(dead_code)]
pub fn get_lives_left(world: &World) -> Result<u32> {
    let lives_left = world
        .get_resource(&Names::LivesRemaining.to_string())?
        .borrow();
    Ok(*lives_left.cast()?)
}

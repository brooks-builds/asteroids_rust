use super::names::Names;
use bbecs::resources::resource::ResourceCast;
use bbecs::world::World;
use eyre::Result;

pub fn get_asteroid_radius(world: &World) -> Result<f32> {
    let wrapped_asteroid_radius = world
        .get_resource(&Names::AsteroidRadius.to_string())?
        .borrow();
    Ok(*wrapped_asteroid_radius.cast()?)
}

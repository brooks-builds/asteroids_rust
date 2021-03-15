use bbecs::data_types::point::Point;
use bbecs::resources::resource::ResourceCast;
use bbecs::world::{World, WorldMethods};
use eyre::Result;
use ggez::Context;
use rand::prelude::ThreadRng;
use rand::Rng;

use crate::helpers::create_debris_mesh::create_debris_mesh;
use crate::helpers::entity_types::EntityTypes;
use crate::helpers::names::Names;

pub fn create_ship_debris_system(
    world: &mut World,
    context: &mut Context,
    rng: &mut ThreadRng,
) -> Result<()> {
    {
        let mut wrapped_ship_destroyed = world
            .get_resource(Names::ShipDestroyed.to_string())?
            .borrow_mut();
        let ship_destroyed: &mut bool = wrapped_ship_destroyed.cast_mut()?;

        if !*ship_destroyed {
            return Ok(());
        }

        *ship_destroyed = false;
    }
    let debris_location = get_debris_location(&world)?;

    for _ in 0..5 {
        world
            .spawn_entity()?
            .with_component(Names::Location.to_string(), debris_location)?
            .with_component(Names::Rotation.to_string(), 0.0_f32)?
            .with_component(Names::Velocity.to_string(), Point::new(0.0, 0.0))?
            .with_component(
                Names::Acceleration.to_string(),
                Point::new(rng.gen_range(-2.0..2.0), rng.gen_range(-2.0..2.0)),
            )?
            .with_component(
                Names::Mesh.to_string(),
                create_debris_mesh(context).unwrap(),
            )?
            .with_component(Names::Marker.to_string(), EntityTypes::Debris.to_string())?
            .with_component(Names::Size.to_string(), 2.0)?;
    }

    Ok(())
}

fn get_debris_location(world: &World) -> Result<Point> {
    let wrapped_locations = world
        .get_resource(Names::LastKnownPlayerLocation.to_string())?
        .borrow();
    let location: &Point = wrapped_locations.cast()?;
    Ok(*location)
}

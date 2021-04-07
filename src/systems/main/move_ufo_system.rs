use crate::helpers::{entity_types::EntityTypes, names::Names};
use bbecs::components::{CastComponents, ComponentData};
use bbecs::data_types::point::Point;
use bbecs::world::{DataWrapper, World};
use eyre::Result;
use noise::{NoiseFn, Perlin};

pub fn move_ufo_system(world: &World, noise_offsets: (f64, f64), noise: &Perlin) -> Result<()> {
    let query = world.query(vec![
        &Names::Marker.to_string(),
        &Names::Acceleration.to_string(),
    ])?;
    let markers = query.get(&Names::Marker.to_string()).unwrap();
    let accelerations = query.get(&Names::Acceleration.to_string()).unwrap();
    let ufo_index = if let Some(index) = get_ufo_index(markers)? {
        index
    } else {
        return Ok(());
    };

    let mut random_acceleration = Point::new(
        noise.get([noise_offsets.0, noise_offsets.1]) as f32,
        noise.get([noise_offsets.0 + 10000.0, noise_offsets.1 + 10000.0]) as f32,
    );

    random_acceleration.multiply_scalar(0.5);

    let acceleration: &DataWrapper<Point> = accelerations[ufo_index].cast()?;
    *acceleration.borrow_mut() += random_acceleration;

    Ok(())
}

fn get_ufo_index(markers: &[&ComponentData]) -> Result<Option<usize>> {
    for (index, marker) in markers.iter().enumerate() {
        let marker: &DataWrapper<String> = marker.cast()?;
        if *marker.borrow() == EntityTypes::UFO.to_string() {
            return Ok(Some(index));
        }
    }
    Ok(None)
}

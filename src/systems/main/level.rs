use crate::helpers::count_entities_with_marker::count_entities_with_marker;
use crate::helpers::{
    entity_types::EntityTypes, get_asteroid_radius::get_asteroid_radius,
    insert_asteroid_into_world::insert_asteroid_into_world, names::Names,
};
use bbecs::components::CastComponents;
use bbecs::data_types::point::Point;
use bbecs::resources::resource::ResourceCast;
use bbecs::world::{DataWrapper, World};
use eyre::{bail, Result};
use ggez::{graphics, Context};
use rand::{thread_rng, Rng};

pub fn level_system(world: &mut World, context: &mut Context) -> Result<()> {
    if count_entities_with_marker(&world, EntityTypes::Asteroid)? > 0 {
        return Ok(());
    }

    let level = increment_level(&world)?;
    let asteroid_radius = get_asteroid_radius(&world)?;
    let asteroid_speed = increment_asteroid_speed(&world)?;
    let player_location = get_player_location(&world)?;
    for _ in 0..level {
        let location = create_random_asteroid_location_away_from_player(
            asteroid_radius,
            player_location,
            context,
        )?;
        insert_asteroid_into_world(world, asteroid_radius, context, asteroid_speed, location)?;
    }

    Ok(())
}

fn increment_level(world: &World) -> Result<u32> {
    let mut wrapped_level = world.get_resource(&Names::Level.to_string())?.borrow_mut();
    let level: &mut u32 = wrapped_level.cast_mut()?;
    *level += 1;
    Ok(*level)
}

fn increment_asteroid_speed(world: &World) -> Result<f32> {
    let mut wrapped_asteroid_speed = world
        .get_resource(&Names::AsteroidSpeed.to_string())?
        .borrow_mut();
    let asteroid_speed: &mut f32 = wrapped_asteroid_speed.cast_mut()?;
    *asteroid_speed += 1.0;
    Ok(*asteroid_speed)
}

fn get_player_location(world: &World) -> Result<Point> {
    let query = world.query(vec![
        &Names::Location.to_string(),
        &Names::Marker.to_string(),
    ])?;
    let locations = query.get(&Names::Location.to_string()).unwrap();
    let markers = query.get(&Names::Marker.to_string()).unwrap();

    for (index, marker) in markers.iter().enumerate() {
        let wrapped_marker: &DataWrapper<String> = marker.cast()?;
        let marker = wrapped_marker.borrow();

        if *marker == EntityTypes::Player.to_string() {
            let wrapped_location: &DataWrapper<Point> = locations[index].cast()?;
            return Ok(*wrapped_location.borrow());
        }
    }

    bail!("Could not fin the player location");
}

fn create_random_asteroid_location_away_from_player(
    asteroid_radius: f32,
    player_location: Point,
    context: &mut Context,
) -> Result<Point> {
    let mut rng = thread_rng();
    let (arena_width, arena_height) = graphics::drawable_size(context);

    for _ in 0..1000 {
        let random_location = Point::new(
            rng.gen_range(0.0..arena_width),
            rng.gen_range(0.0..arena_height),
        );
        let distance_from_player = player_location.distance_to(&random_location);
        if distance_from_player > asteroid_radius * 2.0 {
            return Ok(random_location);
        }
    }

    bail!("could not find a safe place to spawn a asteroid without killing the player instantly");
}

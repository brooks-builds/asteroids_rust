use crate::helpers::bitmask::ENEMY_LAYER;
use crate::helpers::get_player_id::get_player_id;
use crate::helpers::insert_bullet_into_world;
use crate::helpers::names::Names;
use crate::helpers::platform_firing_strategies::PlatformFiringStrategy::{self};
use bbecs::components::CastComponents;
use bbecs::data_types::point::Point;
use bbecs::world::{DataWrapper, World};
use eyre::Result;
use ggez::Context;
use insert_bullet_into_world::insert_bullet_into_world;
use rand::random;

const FIRE_BULLET_EVERY_MIN: usize = 15;
const FIRE_BULLET_EVERY_MAX: usize = 200;

pub fn fire_bullets_from_platforms_system(world: &mut World, context: &mut Context) -> Result<()> {
    if !does_player_exist(&world)? {
        return Ok(());
    }

    let query = world.query(vec![
        &Names::Location.to_string(),
        &Names::PlatformFiringStrategy.to_string(),
        &Names::TicksLived.to_string(),
    ])?;
    let locations = query.get(&Names::Location.to_string()).unwrap();
    let platform_firing_strategies = query
        .get(&Names::PlatformFiringStrategy.to_string())
        .unwrap();
    let ticks_lived = query.get(&Names::TicksLived.to_string()).unwrap();
    assert!(locations.len() == platform_firing_strategies.len());
    let fire_bullets_every = map(
        locations.len(),
        0,
        10,
        FIRE_BULLET_EVERY_MIN,
        FIRE_BULLET_EVERY_MAX,
    );

    let mut bullet_infos = vec![];

    for (index, location) in locations.iter().enumerate() {
        let location: &DataWrapper<Point> = location.cast()?;
        let firing_strategy: &DataWrapper<String> = platform_firing_strategies[index].cast()?;
        let bullet_acceleration = if let Some(bullet_acceleration) = get_bullet_acceleration(
            PlatformFiringStrategy::from_string(&*firing_strategy.borrow()),
            &world,
            &*location.borrow(),
        )? {
            bullet_acceleration
        } else {
            continue;
        };
        let tick_lived: &DataWrapper<usize> = ticks_lived[index].cast()?;
        if *tick_lived.borrow() % fire_bullets_every == 0 {
            bullet_infos.push((*location.borrow(), bullet_acceleration));
        }
    }

    for (location, acceleration) in bullet_infos {
        insert_bullet_into_world(context, world, location, acceleration, ENEMY_LAYER)?;
    }
    Ok(())
}

fn get_bullet_acceleration(
    firing_strategy: PlatformFiringStrategy,
    world: &World,
    platform_location: &Point,
) -> Result<Option<Point>> {
    match firing_strategy {
        PlatformFiringStrategy::Random => Ok(Some(random_firing_strategy())),
        PlatformFiringStrategy::ClosestAsteroid => Ok(Some(closest_asteroid_firing_strategy(
            world,
            platform_location,
        )?)),
        PlatformFiringStrategy::Ufo => Ok(ufo_firing_strategy(world, platform_location)?),
    }
}

fn ufo_firing_strategy(world: &World, platform_location: &Point) -> Result<Option<Point>> {
    let query = world.query(vec![&Names::Location.to_string(), &Names::UFO.to_string()])?;
    let locations = query.get(&Names::Location.to_string()).unwrap();
    assert!(locations.len() <= 1);

    if locations.is_empty() {
        return Ok(None);
    };

    let location: &DataWrapper<Point> = locations[0].cast()?;
    let mut direction = *location.borrow() - *platform_location;
    direction.normalize();
    direction.multiply_scalar(15.0);
    Ok(Some(direction))
}

fn closest_asteroid_firing_strategy(world: &World, platform_location: &Point) -> Result<Point> {
    let query = world.query(vec![
        &Names::Location.to_string(),
        &Names::Velocity.to_string(),
        &Names::Asteroid.to_string(),
    ])?;
    let locations = query.get(&Names::Location.to_string()).unwrap();
    let velocities = query.get(&Names::Velocity.to_string()).unwrap();
    assert!(!locations.is_empty());
    let mut closest_asteroid_location = Point::default();
    let mut closest_asteroid_distance = 99999.0;

    for (index, location) in locations.iter().enumerate() {
        let location: &DataWrapper<Point> = location.cast()?;
        let velocity: &DataWrapper<Point> = velocities[index].cast()?;
        let distance = platform_location.distance_to(&*location.borrow());

        if distance < closest_asteroid_distance {
            closest_asteroid_location = *location.borrow() - *platform_location;
            let mut closest_asteroid_velocity = *velocity.borrow();
            closest_asteroid_velocity.normalize();
            closest_asteroid_velocity.multiply_scalar(50.0);
            closest_asteroid_location += closest_asteroid_velocity;
            closest_asteroid_distance = distance;
        }
    }
    closest_asteroid_location.normalize();
    closest_asteroid_location.multiply_scalar(15.0);

    Ok(closest_asteroid_location)
}

fn random_firing_strategy() -> Point {
    let x = random::<f32>() - 0.5;
    let y = random::<f32>() - 0.5;
    let mut acceleration = Point::new(x, y);
    acceleration.normalize();
    acceleration.multiply_scalar(15.0);
    acceleration
}

fn does_player_exist(world: &World) -> Result<bool> {
    let player_id = get_player_id(world)?;

    if player_id.is_some() {
        Ok(true)
    } else {
        Ok(false)
    }
}

fn map(n: usize, start1: usize, stop1: usize, start2: usize, stop2: usize) -> usize {
    (n - start1) / (stop1 - start1) * (stop2 - start2) + start2
}

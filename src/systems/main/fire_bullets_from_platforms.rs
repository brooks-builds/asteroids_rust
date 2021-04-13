use crate::helpers::get_player_id::get_player_id;
use crate::helpers::insert_bullet_into_world;
use crate::helpers::names::Names;
use crate::helpers::platform_firing_strategies::PlatformFiringStrategy::Random;
use bbecs::components::CastComponents;
use bbecs::data_types::point::Point;
use bbecs::world::{DataWrapper, World};
use eyre::Result;
use ggez::Context;
use insert_bullet_into_world::insert_bullet_into_world;
use rand::random;

const FIRE_BULLET_EVERY: usize = 60;

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

    let mut bullet_infos = vec![];

    for (index, location) in locations.iter().enumerate() {
        let location: &DataWrapper<Point> = location.cast()?;
        let firing_strategy: &DataWrapper<String> = platform_firing_strategies[index].cast()?;
        let bullet_acceleration = get_bullet_acceleration(firing_strategy.borrow().clone());
        let tick_lived: &DataWrapper<usize> = ticks_lived[index].cast()?;
        if *tick_lived.borrow() % FIRE_BULLET_EVERY == 0 {
            bullet_infos.push((*location.borrow(), bullet_acceleration));
        }
    }

    for (location, acceleration) in bullet_infos {
        insert_bullet_into_world(context, world, location, acceleration)?;
    }
    Ok(())
}

fn get_bullet_acceleration(firing_strategy: String) -> Point {
    let platform_firing_strategy = Random.to_string();
    match firing_strategy {
        platform_firing_strategy => random_firing_strategy(),
        _ => unimplemented!(),
    }
}

fn random_firing_strategy() -> Point {
    let x = random::<f32>() - 0.5;
    let y = random::<f32>() - 0.5;
    let mut acceleration = Point::new(x, y);
    acceleration.normalize();
    acceleration.multiply_scalar(8.0);
    acceleration
}

fn does_player_exist(world: &World) -> Result<bool> {
    let player_id = get_player_id(world)?;

    if let Some(_) = player_id {
        Ok(true)
    } else {
        Ok(false)
    }
}

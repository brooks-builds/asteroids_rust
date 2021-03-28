mod errors;
mod helpers;
mod systems;

use std::cell::RefCell;
use std::rc::Rc;

use bbecs::components::CastComponents;
use bbecs::data_types::point::Point;
use bbecs::resources::resource::ResourceCast;
use bbecs::world::{DataWrapper, World, WorldMethods, ENTITY_ID};
use eyre::Result;
use ggez::event::{EventHandler, KeyCode};
use ggez::graphics::pipe::Data;
use ggez::graphics::{Color, Mesh, Rect, WHITE};
use ggez::{graphics, timer, Context, GameResult};
use helpers::create_asteroid::create_asteroid_mesh;
use helpers::create_message::create_message;
use helpers::create_player_ship::create_player_ship;
use helpers::entity_types::EntityTypes;
use helpers::get_player_id::get_player_id;
use helpers::names::Names;
use rand::prelude::ThreadRng;
use rand::{random, thread_rng};
use systems::collide_with_asteroids::collide_with_asteroids_system;
use systems::draw::draw_system;
use systems::draw_message::draw_message_system;
use systems::handle_input::handle_input_system;
use systems::handle_screen_edges::handle_screen_edges_system;
use systems::particles;
use systems::update_acceleration::update_acceleration_system;
use systems::update_mesh::update_mesh_system;
use systems::update_movement::update_movement_system;

pub struct GameState {
    world: World,
    rng: ThreadRng,
    particles_world: World,
}

impl GameState {
    pub fn new(context: &mut Context) -> GameResult<Self> {
        let mut world = World::new();
        let mut particles_world = World::new();
        let (width, height) = graphics::drawable_size(context);
        let player_size = 25.0_f32;
        let player_ship_color = WHITE;
        let is_thrusting = false;
        let thruster_color = Color::new(1.0, 0.0, 0.0, 1.0);
        let asteroid_speed = 1.0_f32;
        let player_location = Point::new(width / 2.0, height / 2.0);
        let asteroid_radius = 100.0;
        let update_fps = 60_u32;
        let seconds_to_respawn = 3_usize;
        let debris_seconds_to_live = seconds_to_respawn / 2;

        world.register(&Names::Location.to_string()).unwrap();
        world.register(&Names::Thrusting.to_string()).unwrap();
        world.register(&Names::Rotation.to_string()).unwrap();
        world.register(&Names::Acceleration.to_string()).unwrap();
        world.register(&Names::Velocity.to_string()).unwrap();
        world.register(&Names::Mesh.to_string()).unwrap();
        world.register(&Names::Marker.to_string()).unwrap();
        world.register(&Names::Size.to_string()).unwrap();
        world.register(&Names::Message.to_string()).unwrap();

        particles_world.register(&Names::Mesh.to_string()).unwrap();
        particles_world
            .register(&Names::Velocity.to_string())
            .unwrap();
        particles_world
            .register(&Names::Location.to_string())
            .unwrap();
        particles_world
            .register(&Names::TicksToLive.to_string())
            .unwrap();
        particles_world
            .register(&Names::DebrisColor.to_string())
            .unwrap();

        world.add_resource(
            Names::BackgroundColor.to_string(),
            Color::new(0.1, 0.1, 0.1, 1.0),
        );
        world.add_resource(Names::ArenaSize.to_string(), Point::new(width, height));
        world.add_resource(Names::ThrusterColor.to_string(), thruster_color);
        world.add_resource(Names::ThrustKeyCode.to_string(), KeyCode::Up);
        world.add_resource(Names::Thrusting.to_string(), is_thrusting);
        world.add_resource(Names::PlayerShipColor.to_string(), player_ship_color);
        world.add_resource(Names::ThrustSpeed.to_string(), 0.2_f32);
        world.add_resource(Names::RotateLeftKeyCode.to_string(), KeyCode::Left);
        world.add_resource(Names::RotateRightKeyCode.to_string(), KeyCode::Right);
        world.add_resource(Names::RotationSpeed.to_string(), 0.1_f32);
        world.add_resource(Names::UpdateFps.to_string(), update_fps);
        world.add_resource(Names::AsteroidSpeed.to_string(), asteroid_speed);
        world.add_resource(Names::SpawnPlayerIn.to_string(), 0_usize);
        world.add_resource(
            Names::SpawnTime.to_string(),
            seconds_to_respawn * update_fps as usize,
        );
        world.add_resource(Names::LivesRemaining.to_string(), 3_u32);
        world.add_resource(Names::PlayerSize.to_string(), player_size);

        particles_world.add_resource(Names::DebrisParticleSpeed.to_string(), 2.0_f32);
        particles_world.add_resource(Names::DebrisParticleCount.to_string(), 40_u32);
        particles_world.add_resource(
            Names::DebrisTicksToLive.to_string(),
            debris_seconds_to_live * update_fps as usize,
        );
        particles_world.add_resource(Names::DebrisSize.to_string(), 3.0_f32);

        // Self::create_player(
        //     &mut world,
        //     create_player_ship(
        //         context,
        //         player_size,
        //         player_ship_color,
        //         is_thrusting,
        //         thruster_color,
        //     )?,
        //     player_size,
        //     player_location,
        // )
        // .expect("error creating player");

        Self::insert_message_into_world(
            "Press Enter to start game",
            &mut world,
            (width, height),
            context,
        )
        .unwrap();

        for _ in 0..1 {
            Self::create_asteroid(
                &mut world,
                asteroid_radius,
                context,
                asteroid_speed,
                player_size,
                &player_location,
                (width, height),
            )
            .unwrap();
        }

        Ok(Self {
            world,
            rng: thread_rng(),
            particles_world,
        })
    }

    fn create_player(
        world: &mut World,
        player_ship: Mesh,
        size: f32,
        location: Point,
    ) -> Result<()> {
        world
            .spawn_entity()?
            .with_component(&Names::Location.to_string(), location)?
            .with_component(&Names::Rotation.to_string(), 0.0_f32)?
            .with_component(&Names::Velocity.to_string(), Point::new(0.0, 0.0))?
            .with_component(&Names::Acceleration.to_string(), Point::new(0.0, 0.0))?
            .with_component(&Names::Mesh.to_string(), player_ship)?
            .with_component(&Names::Marker.to_string(), EntityTypes::Player.to_string())?
            .with_component(&Names::Size.to_string(), size)?;
        Ok(())
    }

    fn create_asteroid(
        world: &mut World,
        radius: f32,
        context: &mut Context,
        speed: f32,
        player_size: f32,
        player_location: &Point,
        arena_size: (f32, f32),
    ) -> Result<()> {
        let mesh = create_asteroid_mesh(context, radius).unwrap();
        let location =
            Self::generate_asteroid_location(player_size, player_location, radius, arena_size);
        let mut acceleration = Point::new(random::<f32>() - 0.5, random::<f32>() - 0.5);
        acceleration.normalize();
        acceleration.multiply_scalar(speed);

        world
            .spawn_entity()?
            .with_component(&Names::Location.to_string(), location)?
            .with_component(&Names::Velocity.to_string(), Point::new(0.0, 0.0))?
            .with_component(&Names::Acceleration.to_string(), acceleration)?
            .with_component(&Names::Mesh.to_string(), mesh)?
            .with_component(&Names::Rotation.to_string(), 0.0_f32)?
            .with_component(
                &Names::Marker.to_string(),
                EntityTypes::Asteroid.to_string(),
            )?
            .with_component(&Names::Size.to_string(), radius)?;
        Ok(())
    }

    fn generate_asteroid_location(
        player_size: f32,
        player_location: &Point,
        asteroid_size: f32,
        (arena_width, arena_height): (f32, f32),
    ) -> Point {
        let mut location = Point::new(0.0, 0.0);

        loop {
            location.x = random::<f32>() * arena_width;
            location.y = random::<f32>() * arena_height;

            if location.distance_to(player_location) > player_size * 2.0 + asteroid_size * 2.0 {
                break;
            }
        }

        location
    }

    fn insert_message_into_world(
        message: &str,
        world: &mut World,
        screen_size: (f32, f32),
        context: &mut Context,
    ) -> Result<()> {
        let text = create_message(message);
        let location = Point::new(
            screen_size.0 / 2.0 - text.width(context) as f32 / 2.0,
            screen_size.1 / 2.0 - text.height(context) as f32 / 2.0,
        );
        world
            .spawn_entity()?
            .with_component(&Names::Location.to_string(), location)?
            .with_component(&Names::Marker.to_string(), EntityTypes::Message.to_string())?
            .with_component(&Names::Message.to_string(), text)?;
        Ok(())
    }
}

impl EventHandler for GameState {
    fn update(&mut self, context: &mut Context) -> GameResult {
        let wrapped_update_fps = self
            .world
            .get_resource(&Names::UpdateFps.to_string())
            .unwrap()
            .borrow();
        let update_fps: &u32 = wrapped_update_fps.cast().unwrap();
        let update_fps = *update_fps;
        drop(wrapped_update_fps);
        while timer::check_update_time(context, update_fps) {
            handle_input_system(&self.world, context).unwrap();
            update_acceleration_system(&self.world).unwrap();
            update_movement_system(&self.world).unwrap();
            handle_screen_edges_system(&self.world).unwrap();
            update_mesh_system(context, &self.world).unwrap();
            collide_with_asteroids_system(
                &mut self.world,
                &mut self.particles_world,
                context,
                &mut self.rng,
            )
            .unwrap();
            particles::update_locations::update_locations_system(&self.particles_world).unwrap();
            particles::update_life::update_life_system(&self.particles_world).unwrap();
            particles::fade_debris_system::fade_debris_system(&self.particles_world).unwrap();
            self.world.update().unwrap();
            self.particles_world.update().unwrap();
        }
        Ok(())
    }

    fn draw(&mut self, context: &mut Context) -> GameResult {
        let wrapped_background_color = self
            .world
            .get_resource(&Names::BackgroundColor.to_string())
            .unwrap()
            .borrow();
        let background_color = wrapped_background_color.cast().unwrap();
        graphics::clear(context, *background_color);
        draw_system(context, &self.world).unwrap();
        particles::draw::draw_system(&self.particles_world, context).unwrap();
        draw_message_system(&self.world, context).unwrap();
        graphics::present(context)
    }

    fn resize_event(&mut self, context: &mut Context, width: f32, height: f32) {
        self.world
            .add_resource(Names::ArenaSize.to_string(), Point::new(width, height));
        let screen_size = Rect::new(0.0, 0.0, width, height);
        graphics::set_screen_coordinates(context, screen_size).unwrap()
    }

    fn key_down_event(
        &mut self,
        context: &mut Context,
        keycode: KeyCode,
        _keymods: ggez::event::KeyMods,
        _repeat: bool,
    ) {
        if let None = get_player_id(&self.world).unwrap() {
            if keycode == KeyCode::Return {
                let wrapped_player_size = self
                    .world
                    .get_resource(&Names::PlayerSize.to_string())
                    .unwrap()
                    .borrow();
                let player_size: f32 = *wrapped_player_size.cast().unwrap();
                let wrapped_player_ship_color = self
                    .world
                    .get_resource(&Names::PlayerShipColor.to_string())
                    .unwrap()
                    .borrow();
                let player_ship_color: &Color = wrapped_player_ship_color.cast().unwrap();
                let wrapped_thruster_color = self
                    .world
                    .get_resource(&Names::ThrusterColor.to_string())
                    .unwrap()
                    .borrow();
                let thruster_color: &Color = wrapped_thruster_color.cast().unwrap();
                let query = self
                    .world
                    .query(vec![&Names::Message.to_string(), ENTITY_ID])
                    .unwrap();
                let ids = query.get(ENTITY_ID).unwrap();
                for id in ids {
                    let id: &DataWrapper<u32> = id.cast().unwrap();
                    self.world.delete_by_id(*id.borrow()).unwrap();
                }

                let player_ship = create_player_ship(
                    context,
                    player_size,
                    *player_ship_color,
                    false,
                    *thruster_color,
                )
                .unwrap();
                let (width, height) = graphics::drawable_size(context);
                let player_location = Point::new(width / 2.0, height / 2.0);
                drop(wrapped_player_ship_color);
                drop(wrapped_player_size);
                drop(wrapped_thruster_color);
                Self::create_player(&mut self.world, player_ship, player_size, player_location)
                    .unwrap();
            }
        }
    }
}

mod errors;
mod helpers;
mod systems;

use bbecs::data_types::point::Point;
use bbecs::resources::resource::ResourceCast;
use bbecs::world::{World, WorldMethods};
use ggez::event::{EventHandler, KeyCode};
use ggez::graphics::{Color, Rect, WHITE};
use ggez::{graphics, timer, Context, GameResult};
use helpers::insert_asteroid_into_world;
use helpers::names::Names;
use insert_asteroid_into_world::insert_asteroid_into_world;
use rand::prelude::ThreadRng;
use rand::{thread_rng, Rng};
use systems::collide_with_asteroids::collide_with_asteroids_system;
use systems::draw::draw_system;
use systems::draw_message::draw_message_system;
use systems::handle_input::handle_input_system;
use systems::handle_screen_edges::handle_screen_edges_system;
use systems::main::display_message::handle_message_system;
use systems::main::fire_bullet::fire_bullet_system;
use systems::main::handle_bullets_hitting_asteroids::handle_bullets_hitting_asteroids_system;
use systems::main::handle_respawn::handle_respawn_system;
use systems::main::insert_asteroids::insert_asteroids_system;
use systems::particles;
use systems::particles::update_life::update_life_system;
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
        let mut rng = thread_rng();
        let mut world = World::new();
        let mut particles_world = World::new();
        let (width, height) = graphics::drawable_size(context);
        let player_size = 25.0_f32;
        let player_ship_color = WHITE;
        let is_thrusting = false;
        let thruster_color = Color::new(1.0, 0.0, 0.0, 1.0);
        let asteroid_speed = 1.0_f32;
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
        world.register(&Names::TicksToLive.to_string()).unwrap();

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

        for _ in 0..1 {
            let asteroid_location =
                Point::new(rng.gen_range(0.0..width), rng.gen_range(0.0..height));
            insert_asteroid_into_world(
                &mut world,
                asteroid_radius,
                context,
                asteroid_speed,
                asteroid_location,
            )
            .unwrap();
        }

        Ok(Self {
            world,
            rng,
            particles_world,
        })
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
                &self.world,
                &mut self.particles_world,
                context,
                &mut self.rng,
            )
            .unwrap();
            handle_message_system(&mut self.world, context).unwrap();
            update_life_system(&self.world).unwrap();
            let destroyed_asteroids = handle_bullets_hitting_asteroids_system(&self.world).unwrap();
            insert_asteroids_system(&mut self.world, context, destroyed_asteroids).unwrap();
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
        handle_respawn_system(&mut self.world, keycode, context).unwrap();
        fire_bullet_system(&mut self.world, keycode, context).unwrap();
    }
}

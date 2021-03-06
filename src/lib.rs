mod errors;
mod helpers;
mod systems;

use std::sync::mpsc::{Receiver, Sender};

use bbecs::data_types::point::Point;
use bbecs::resources::resource::ResourceCast;
use bbecs::world::{World, WorldMethods};
use eyre::Result;
use ggez::audio::SoundData;
use ggez::event::{EventHandler, KeyCode};
use ggez::graphics::{Color, Rect, WHITE};
use ggez::{graphics, timer, Context, GameResult};
use helpers::create_lives_left_mesh::create_lives_left_mesh;
use helpers::create_message::create_message;
use helpers::entity_types::EntityTypes;
use helpers::insert_asteroids_into_world::insert_asteroids_into_world;
use helpers::names::Names;
use noise::Perlin;
use rand::prelude::ThreadRng;
use rand::thread_rng;
use systems::collide_with_asteroids::collide_with_asteroids_system;
use systems::draw::draw_system;
use systems::draw_message::draw_message_system;
use systems::handle_input::handle_input_system;
use systems::handle_screen_edges::handle_screen_edges_system;
use systems::main::display_message::handle_message_system;
use systems::main::draw_labels::draw_labels_system;
use systems::main::fire_bullet::fire_bullet_system;
use systems::main::fire_bullets_from_platforms::fire_bullets_from_platforms_system;
use systems::main::handle_bullets_hitting_asteroids::handle_bullets_hitting_asteroids_system;
use systems::main::handle_bullets_hitting_ships::handle_bullets_hitting_ships;
use systems::main::handle_chat_message_system::handle_chat_message_system;
use systems::main::handle_respawn::handle_respawn_system;
use systems::main::increment_ticks_lived_system::increment_ticks_lived_system;
use systems::main::insert_asteroids::insert_asteroids_system;
use systems::main::level::level_system;
use systems::main::move_ufo_system::move_ufo_system;
use systems::main::play_asteroid_strategy_set_system::play_asteroid_strategy_set_system;
use systems::main::play_explosion_sound_system::play_explosion_sound_system;
use systems::main::play_gun_system::play_gun_system;
use systems::main::play_random_firing_strategy_set_system::play_random_firing_strategy_set_system;
use systems::main::play_reinforcements_sound_system::play_reinforcements_sound_system;
use systems::main::play_ufo_firing_strategy_set_system::play_ufo_firing_strategy_set_system;
#[allow(unused_imports)]
use systems::main::render_hitboxes::render_hitboxes_system;
use systems::main::spawn_ufo_system::spawn_ufo_system;
use systems::main::ufo_fire_bullets::ufo_fire_bullets;
use systems::main::update_display_level::update_display_level_system;
use systems::main::update_lives_remaining_mesh::update_lives_remaining_mesh;
use systems::main::update_score_text_system::update_score_text_system;
use systems::particles;
use systems::particles::update_life::update_life_system;
use systems::update_acceleration::update_acceleration_system;
use systems::update_mesh::update_mesh_system;
use systems::update_movement::update_movement_system;
use twitch_chat_wrapper::ChatMessage;

pub struct GameState {
    world: World,
    rng: ThreadRng,
    particles_world: World,
    noise: Perlin,
    noise_offsets: (f64, f64),
    receive_from_chat: Receiver<ChatMessage>,
}

impl GameState {
    pub fn new(
        context: &mut Context,
        receive_from_chat: Receiver<ChatMessage>,
        send_to_chat: Sender<String>,
    ) -> GameResult<Self> {
        let rng = thread_rng();
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
        let level = 1_u32;
        let lives_remaining = 5_u32;
        let ufo_killed_on_level = 1_u32;
        let laser_fire_sound = SoundData::new(context, "/air_woosh_underwater.mp3")?;
        let explosion_sound = SoundData::new(context, "/big_explosion_distant.mp3")?;
        let reinforcements_sound = SoundData::new(context, "/reinforcements.mp3")?;
        let random_firing_strategy_set =
            SoundData::new(context, "/random_firing_strategy_set.mp3")?;
        let target_nearest_asteroid = SoundData::new(context, "/target_nearest_asteroid.mp3")?;
        let ufo_firing_strategy_set = SoundData::new(context, "/ufo_firing_strategy_set.mp3")?;

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
        world.register(&Names::Score.to_string()).unwrap();
        world
            .register(&Names::CollisionBitMask.to_string())
            .unwrap();
        world.register(&Names::TicksLived.to_string()).unwrap();
        world.register(&Names::ChatterName.to_string()).unwrap();
        world.register(&Names::Label.to_string()).unwrap();
        world
            .register(&Names::PlatformFiringStrategy.to_string())
            .unwrap();
        world.register(&Names::Asteroid.to_string()).unwrap();
        world.register(&Names::UFO.to_string()).unwrap();

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
        world.add_resource(Names::RotationSpeed.to_string(), 0.08_f32);
        world.add_resource(Names::UpdateFps.to_string(), update_fps);
        world.add_resource(Names::AsteroidSpeed.to_string(), asteroid_speed);
        world.add_resource(Names::SpawnPlayerIn.to_string(), 0_usize);
        world.add_resource(
            Names::SpawnTime.to_string(),
            seconds_to_respawn * update_fps as usize,
        );
        world.add_resource(Names::LivesRemaining.to_string(), lives_remaining);
        world.add_resource(Names::PlayerSize.to_string(), player_size);
        world.add_resource(Names::Level.to_string(), level);
        world.add_resource(Names::AsteroidRadius.to_string(), asteroid_radius);
        world.add_resource(Names::ReloadingTicksLeft.to_string(), 0_u32);
        world.add_resource(Names::UFOSize.to_string(), 75.0_f32);
        world.add_resource(Names::UFOKilledOnLevel.to_string(), ufo_killed_on_level);
        world.add_resource(Names::NeedToPlayGunSound.to_string(), false);
        world.add_resource(Names::GunSound.to_string(), laser_fire_sound);
        world.add_resource(Names::NeedToPlayExplosionSound.to_string(), false);
        world.add_resource(Names::ExplosionSound.to_string(), explosion_sound);
        world.add_resource(Names::NeedtoPlayReinforcementsSound.to_string(), false);
        world.add_resource(Names::ReinforcementsSound.to_string(), reinforcements_sound);
        world.add_resource(Names::NeedToPlayAsteroidStrategySet.to_string(), false);
        world.add_resource(
            Names::AsteroidStrategySetSound.to_string(),
            target_nearest_asteroid,
        );
        world.add_resource(Names::NeedToPlayRandomFiringStrategySet.to_string(), false);
        world.add_resource(
            Names::RandomFiringStrategySetSound.to_string(),
            random_firing_strategy_set,
        );
        world.add_resource(Names::NeedToPlayUfoFiringStrategySet.to_string(), false);
        world.add_resource(
            Names::UFOStrategySetSound.to_string(),
            ufo_firing_strategy_set,
        );

        particles_world.add_resource(Names::DebrisParticleSpeed.to_string(), 2.0_f32);
        particles_world.add_resource(Names::DebrisParticleCount.to_string(), 40_u32);
        particles_world.add_resource(
            Names::DebrisTicksToLive.to_string(),
            debris_seconds_to_live * update_fps as usize,
        );
        particles_world.add_resource(Names::DebrisSize.to_string(), 3.0_f32);

        for _ in 0..level {
            insert_asteroids_into_world(
                (width, height),
                &mut world,
                asteroid_radius,
                context,
                asteroid_speed,
            )
            .unwrap();
        }

        Self::create_level_text(&mut world).unwrap();
        Self::create_score(&mut world).unwrap();
        Self::create_lives_left_text(&mut world, context, lives_remaining).unwrap();

        send_to_chat
            .send("Asteroids Game Online: Join in by chatting.".to_string())
            .unwrap();
        send_to_chat
            .send("Available strategies: #asteroid, #ufo, #random".to_string())
            .unwrap();

        Ok(Self {
            world,
            rng,
            particles_world,
            noise: Perlin::new(),
            noise_offsets: (0.0, 0.0),
            receive_from_chat,
        })
    }

    fn create_level_text(world: &mut World) -> Result<()> {
        let level_text = create_message("Level: 0", 25.0);
        world
            .spawn_entity()?
            .with_component(&Names::Message.to_string(), level_text)?
            .with_component(&Names::Location.to_string(), Point::new(5.0, 5.0))?
            .with_component(
                &Names::Marker.to_string(),
                EntityTypes::LevelText.to_string(),
            )?;
        Ok(())
    }

    fn create_score(world: &mut World) -> Result<()> {
        world
            .spawn_entity()?
            .with_component(&Names::Location.to_string(), Point::new(5.0, 30.0))?
            .with_component(
                &Names::Message.to_string(),
                create_message("Score: 0", 25.0),
            )?
            .with_component(&Names::Score.to_string(), 0_u32)?
            .with_component(&Names::Marker.to_string(), EntityTypes::Score.to_string())?;
        Ok(())
    }

    fn create_lives_left_text(
        world: &mut World,
        context: &mut Context,
        lives_left: u32,
    ) -> Result<()> {
        world
            .spawn_entity()?
            .with_component(&Names::Location.to_string(), Point::new(5.0, 60.0))?
            .with_component(&Names::Rotation.to_string(), 0.0_f32)?
            .with_component(
                &Names::Mesh.to_string(),
                create_lives_left_mesh(context, lives_left)?,
            )?
            .with_component(
                &Names::Marker.to_string(),
                EntityTypes::LivesMesh.to_string(),
            )?;
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
            handle_chat_message_system(
                &self.receive_from_chat,
                &mut self.world,
                context,
                &mut self.rng,
            )
            .unwrap();
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
            let destroyed_asteroids = handle_bullets_hitting_asteroids_system(
                &self.world,
                &mut self.particles_world,
                context,
                &mut self.rng,
            )
            .unwrap();
            insert_asteroids_system(&mut self.world, context, destroyed_asteroids).unwrap();
            level_system(&mut self.world, context).unwrap();
            fire_bullet_system(&mut self.world, context).unwrap();
            update_display_level_system(&self.world).unwrap();
            update_score_text_system(&self.world).unwrap();
            update_lives_remaining_mesh(&self.world, context).unwrap();
            spawn_ufo_system(&mut self.world, context, &mut self.rng).unwrap();
            move_ufo_system(&self.world, self.noise_offsets, &self.noise).unwrap();
            ufo_fire_bullets(&mut self.world, context).unwrap();
            handle_bullets_hitting_ships(
                &self.world,
                &mut self.particles_world,
                context,
                &mut self.rng,
            )
            .unwrap(); // use bitmask for the collisions
            increment_ticks_lived_system(&self.world).unwrap();
            fire_bullets_from_platforms_system(&mut self.world, context).unwrap();
            handle_respawn_system(&mut self.world, context).unwrap();
            play_gun_system(&self.world, context).unwrap();
            play_explosion_sound_system(&self.world, context).unwrap();
            play_reinforcements_sound_system(&self.world, context).unwrap();
            play_random_firing_strategy_set_system(&self.world, context).unwrap();
            play_asteroid_strategy_set_system(&self.world, context).unwrap();
            play_ufo_firing_strategy_set_system(&self.world, context).unwrap();
            particles::update_locations::update_locations_system(&self.particles_world).unwrap();
            particles::update_life::update_life_system(&self.particles_world).unwrap();
            particles::fade_debris_system::fade_debris_system(&self.particles_world).unwrap();
            self.world.update().unwrap();
            self.particles_world.update().unwrap();

            self.noise_offsets.0 += 0.01;
            self.noise_offsets.1 += 0.01;
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
        // render_hitboxes_system(&self.world, context).unwrap();
        draw_labels_system(&self.world, context).unwrap();
        graphics::present(context)
    }

    fn resize_event(&mut self, context: &mut Context, width: f32, height: f32) {
        self.world
            .add_resource(Names::ArenaSize.to_string(), Point::new(width, height));
        let screen_size = Rect::new(0.0, 0.0, width, height);
        graphics::set_screen_coordinates(context, screen_size).unwrap()
    }
}

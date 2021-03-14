mod helpers;
mod systems;

use bbecs::components::Component;
use bbecs::data_types::point::Point;
use bbecs::resources::resource::ResourceCast;
use bbecs::world::{World, WorldMethods};
use eyre::Result;
use ggez::event::{EventHandler, KeyCode};
use ggez::graphics::{Color, Mesh, Rect, WHITE};
use ggez::{graphics, timer, Context, GameResult};
use helpers::create_asteroid::create_asteroid_mesh;
use helpers::create_player_ship::create_player_ship;
use helpers::entity_types::EntityTypes;
use helpers::names::Names;
use rand::random;
use systems::collide_with_asteroids::collide_with_asteroids_system;
use systems::draw::draw_system;
use systems::handle_input::handle_input_system;
use systems::handle_screen_edges::handle_screen_edges_system;
use systems::update_acceleration::update_acceleration_system;
use systems::update_mesh::update_mesh_system;
use systems::update_movement::update_movement_system;
use systems::update_rotation::update_rotation_system;

pub struct GameState {
    world: World,
}

impl GameState {
    pub fn new(context: &mut Context) -> GameResult<Self> {
        let mut world = World::new();
        let (width, height) = graphics::drawable_size(context);
        let player_size = 25.0_f32;
        let player_ship_color = WHITE;
        let is_thrusting = false;
        let thruster_color = Color::new(1.0, 0.0, 0.0, 1.0);
        let asteroid_speed = 1.0_f32;
        let player_location = Point::new(width / 2.0, height / 2.0);
        let asteroid_radius = 100.0;

        world.register(Names::Location, bbecs::components::Component::Point);
        world.register(Names::Thrusting, bbecs::components::Component::Bool);
        world.register(Names::Rotation, Component::F32);
        world.register(Names::Acceleration, Component::Point);
        world.register(Names::Velocity, Component::Point);
        world.register(Names::Mesh, Component::Mesh);
        world.register(Names::Marker, Component::Marker);
        world.register(Names::Size, Component::F32);

        world.add_resource::<Names>(Names::BackgroundColor, Color::new(0.1, 0.1, 0.1, 1.0));
        world.add_resource(Names::ArenaSize, Point::new(width, height));
        world.add_resource(Names::ThrusterColor, thruster_color);
        world.add_resource(Names::ThrustKeyCode, KeyCode::Up);
        world.add_resource(Names::Thrusting, is_thrusting);
        world.add_resource(Names::PlayerShipColor, player_ship_color);
        world.add_resource(Names::ThrustSpeed, 0.2_f32);
        world.add_resource(Names::RotateLeftKeyCode, KeyCode::Left);
        world.add_resource(Names::RotateRightKeyCode, KeyCode::Right);
        world.add_resource(Names::RotationSpeed, 0.1_f32);
        world.add_resource(Names::UpdateFps, 60_u32);
        world.add_resource(Names::AsteroidSpeed, asteroid_speed);

        Self::create_player(
            &mut world,
            create_player_ship(
                context,
                player_size,
                player_ship_color,
                is_thrusting,
                thruster_color,
            )?,
            player_size,
            player_location,
        )
        .expect("error creating player");

        for _ in 0..5 {
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

        Ok(Self { world })
    }

    fn create_player(
        world: &mut World,
        player_ship: Mesh,
        size: f32,
        location: Point,
    ) -> Result<()> {
        world
            .spawn_entity()?
            .with_component(Names::Location, location)?
            .with_component(Names::Rotation, 0.0_f32)?
            .with_component(Names::Velocity, Point::new(0.0, 0.0))?
            .with_component(Names::Acceleration, Point::new(0.0, 0.0))?
            .with_component(Names::Mesh, player_ship)?
            .with_component(Names::Marker, EntityTypes::Player.to_string())?
            .with_component(Names::Size, size)?;
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
            .with_component(Names::Location, location)?
            .with_component(Names::Rotation, 0.0_f32)?
            .with_component(Names::Velocity, Point::new(0.0, 0.0))?
            .with_component(Names::Acceleration, acceleration)?
            .with_component(Names::Mesh, mesh)?
            .with_component(Names::Marker, EntityTypes::Asteroid.to_string())?
            .with_component(Names::Size, radius)?;
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
}

impl EventHandler for GameState {
    fn update(&mut self, context: &mut Context) -> GameResult {
        let wrapped_update_fps = self.world.get_resource(Names::UpdateFps).unwrap().borrow();
        let update_fps: &u32 = wrapped_update_fps.cast().unwrap();
        while timer::check_update_time(context, *update_fps) {
            handle_input_system(&self.world, context)?;
            update_rotation_system(&self.world).unwrap();
            update_acceleration_system(&self.world).unwrap();
            update_movement_system(&self.world).unwrap();
            handle_screen_edges_system(&self.world).unwrap();
            collide_with_asteroids_system(&self.world).unwrap();
            update_mesh_system(context, &self.world).unwrap();
            self.world.update().unwrap();
        }
        Ok(())
    }

    fn draw(&mut self, context: &mut Context) -> GameResult {
        let wrapped_background_color = self
            .world
            .get_resource(Names::BackgroundColor)
            .unwrap()
            .borrow();
        let background_color = wrapped_background_color.cast().unwrap();
        graphics::clear(context, *background_color);
        draw_system(context, &self.world).unwrap();
        graphics::present(context)
    }

    fn resize_event(&mut self, context: &mut Context, width: f32, height: f32) {
        self.world
            .add_resource(Names::ArenaSize, Point::new(width, height));
        let screen_size = Rect::new(0.0, 0.0, width, height);
        graphics::set_screen_coordinates(context, screen_size).unwrap()
    }
}

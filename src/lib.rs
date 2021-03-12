mod helpers;
mod systems;

use bbecs::components::Component;
use bbecs::data_types::point::Point;
use bbecs::world::{World, WorldMethods};
use eyre::Result;
use ggez::event::{EventHandler, KeyCode};
use ggez::graphics::{Color, Mesh, Rect, WHITE};
use ggez::{graphics, timer, Context, GameResult};
use helpers::create_player_ship::create_player_ship;
use helpers::entity_types::EntityTypes;
use helpers::names::Names;
use systems::draw::draw_system;
use systems::handle_input::handle_input_system;
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

        world.register(Names::Location, bbecs::components::Component::Point);
        world.register(Names::Thrusting, bbecs::components::Component::Bool);
        world.register(Names::Rotation, Component::F32);
        world.register(Names::Acceleration, Component::Point);
        world.register(Names::Velocity, Component::Point);
        world.register(Names::Mesh, Component::Mesh);
        world.register(Names::Marker, Component::Marker);

        world.add_resource::<Names>(Names::BackgroundColor, Color::new(0.1, 0.1, 0.1, 1.0));
        world.add_resource(Names::ArenaSize, Point::new(width, height));
        world.add_resource(Names::PlayerSize, player_size);
        world.add_resource(Names::ThrusterColor, thruster_color);
        world.add_resource(Names::ThrustKeyCode, KeyCode::Up);
        world.add_resource(Names::Thrusting, is_thrusting);
        world.add_resource(Names::PlayerShipColor, player_ship_color);
        world.add_resource(Names::ThrustSpeed, 0.2_f32);
        world.add_resource(Names::RotateLeftKeyCode, KeyCode::Left);
        world.add_resource(Names::RotateRightKeyCode, KeyCode::Right);
        world.add_resource(Names::RotationSpeed, 5.0_f32);
        world.add_resource(Names::UpdateFps, 60_u32);

        Self::create_player(
            &mut world,
            width,
            height,
            create_player_ship(
                context,
                player_size,
                player_ship_color,
                is_thrusting,
                thruster_color,
            )?,
        )
        .expect("error creating player");

        Ok(Self { world })
    }

    fn create_player(world: &mut World, width: f32, height: f32, player_ship: Mesh) -> Result<()> {
        let location = Point::new(width / 2.0, height / 2.0);
        world
            .spawn_entity()
            .with_component(Names::Location, location)?
            .with_component(Names::Rotation, 0.0_f32)?
            .with_component(Names::Velocity, Point::new(0.0, 0.0))?
            .with_component(Names::Acceleration, Point::new(0.0, 0.0))?
            .with_component(Names::Mesh, player_ship)?
            .with_component(Names::Marker, EntityTypes::Player.to_string())?;
        Ok(())
    }
}

impl EventHandler for GameState {
    fn update(&mut self, context: &mut Context) -> GameResult {
        let update_fps: &u32 = self.world.get_resource(Names::UpdateFps).unwrap();
        while timer::check_update_time(context, *update_fps) {
            handle_input_system(&mut self.world, context)?;
            update_rotation_system(&self.world).unwrap();
            update_acceleration_system(&self.world).unwrap();
            update_movement_system(&self.world).unwrap();
            update_mesh_system(context, &mut self.world).unwrap();
        }
        Ok(())
    }

    fn draw(&mut self, context: &mut Context) -> GameResult {
        let background_color = self.world.get_resource(Names::BackgroundColor).unwrap();
        graphics::clear(context, *background_color);
        draw_system(context, &mut self.world).unwrap();
        graphics::present(context)
    }

    fn resize_event(&mut self, context: &mut Context, width: f32, height: f32) {
        self.world
            .add_resource(Names::ArenaSize, Point::new(width, height));
        let screen_size = Rect::new(0.0, 0.0, width, height);
        graphics::set_screen_coordinates(context, screen_size).unwrap()
    }
}

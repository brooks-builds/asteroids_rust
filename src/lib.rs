mod names;
mod systems;

use bbecs::components::Component;
use bbecs::data_types::point::Point;
use bbecs::world::{World, WorldMethods};
use eyre::Result;
use ggez::event::{EventHandler, KeyCode};
use ggez::graphics::{Color, Mesh, MeshBuilder, Rect, WHITE};
use ggez::mint::Point2;
use ggez::{graphics, Context, GameResult};
use names::Names;
use systems::create_player_mesh::create_player_mesh_system;
use systems::draw::draw_system;
use systems::handle_input::handle_input_system;
use systems::update_acceleration::update_acceleration_system;
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

        world.register(Names::Location, bbecs::components::Component::Point);
        world.register(Names::Thrusting, bbecs::components::Component::Bool);
        world.register(Names::Rotation, Component::F32);
        world.register(Names::Acceleration, Component::Point);
        world.register(Names::Velocity, Component::Point);

        world.add_resource::<Names>(Names::BackgroundColor, Color::new(0.1, 0.1, 0.1, 1.0));
        world.add_resource(Names::ArenaSize, Point::new(width, height));
        world.add_resource(Names::PlayerSize, player_size);
        world.add_resource(Names::ThrusterColor, Color::new(1.0, 0.0, 0.0, 1.0));
        world.add_resource(Names::ThrustKeyCode, KeyCode::Up);
        world.add_resource(Names::Thrusting, false);
        world.add_resource(Names::PlayerShipColor, WHITE);
        world.add_resource(Names::ThrustSpeed, 0.2_f32);

        Self::create_player(&mut world, width, height).expect("error creating player");

        Ok(Self { world })
    }

    fn create_player(world: &mut World, width: f32, height: f32) -> Result<()> {
        let location = Point::new(width / 2.0, height / 2.0);
        world
            .spawn_entity()
            .with_component(Names::Location, location)?
            .with_component(Names::Rotation, 0.0_f32)?
            .with_component(Names::Velocity, Point::new(0.0, 0.0))?
            .with_component(Names::Acceleration, Point::new(0.0, 0.0))?;
        Ok(())
    }
}

impl EventHandler for GameState {
    fn update(&mut self, context: &mut Context) -> GameResult {
        handle_input_system(&mut self.world, context)?;
        update_rotation_system(&self.world).unwrap();
        update_acceleration_system(&self.world).unwrap();
        update_movement_system(&self.world);
        create_player_mesh_system(context, &mut self.world).unwrap();
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

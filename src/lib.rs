mod names;
mod systems;

use bbecs::data_types::point::Point;
use bbecs::world::{World, WorldMethods};
use eyre::Result;
use ggez::event::EventHandler;
use ggez::graphics::{Color, Rect};
use ggez::{graphics, Context, GameResult};
use names::Names;
use systems::create_mesh::create_mesh;
use systems::draw::draw_system;

pub struct GameState {
    world: World,
}

impl GameState {
    pub fn new(context: &mut Context) -> GameResult<Self> {
        let mut world = World::new();
        let (width, height) = graphics::drawable_size(context);

        // world.register(Names::Location, bbecs::components::Component::Point);

        world.add_resource::<Names>(Names::BackgroundColor, Color::new(0.1, 0.1, 0.1, 1.0));
        world.add_resource(Names::ArenaSize, Point::new(width, height));
        world.add_resource(Names::PlayerSize, 25.0_f32);

        Self::create_player(&mut world, width, height).expect("error creating player");

        Ok(Self { world })
    }

    fn create_player(world: &mut World, width: f32, height: f32) -> Result<()> {
        let location = Point::new(width / 2.0, height / 2.0);
        world
            .spawn_entity()
            .with_component(Names::Location, location)?;
        Ok(())
    }
}

impl EventHandler for GameState {
    fn update(&mut self, context: &mut Context) -> GameResult {
        create_mesh(context, &mut self.world)?;
        Ok(())
    }

    fn draw(&mut self, context: &mut Context) -> GameResult {
        let background_color = self.world.get_resource(Names::BackgroundColor).unwrap();
        graphics::clear(context, *background_color);
        draw_system(context, &mut self.world)?;
        graphics::present(context)
    }

    fn resize_event(&mut self, context: &mut Context, width: f32, height: f32) {
        self.world
            .add_resource(Names::ArenaSize, Point::new(width, height));
        let screen_size = Rect::new(0.0, 0.0, width, height);
        graphics::set_screen_coordinates(context, screen_size).unwrap()
    }
}

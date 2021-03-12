use asteroids_rust::GameState;
use ggez::conf::WindowMode;
use ggez::event::run;
use ggez::{ContextBuilder, GameResult};

fn main() -> GameResult {
    let window_mode = WindowMode::default()
        .dimensions(1920.0, 1080.0)
        .resizable(true);
    let (context, event_loop) = &mut ContextBuilder::new("rusty_asteroids", "Brookzerker")
        .window_mode(window_mode)
        .build()?;

    let game_state = &mut GameState::new(context)?;

    run(context, event_loop, game_state)
}

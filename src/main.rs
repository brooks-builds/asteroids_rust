use std::sync::mpsc::channel;
use std::thread::spawn;

use asteroids_rust::GameState;
use ggez::conf::{WindowMode, WindowSetup};
use ggez::event::run;
use ggez::{ContextBuilder, GameResult};
use twitch_chat_wrapper::ChatMessage;

fn main() -> GameResult {
    let window_mode = WindowMode::default()
        .dimensions(1920.0, 1080.0)
        .resizable(true);
    let window_setup = WindowSetup::default().title("rusty_asteroids");
    let (context, event_loop) = &mut ContextBuilder::new("rusty_asteroids", "Brookzerker")
        .window_mode(window_mode)
        .window_setup(window_setup)
        .build()?;

    let (send_to_chat, receive_from_game) = channel::<String>();
    let (send_to_game, receive_from_chat) = channel::<ChatMessage>();

    let _twitchchat_wrapper_thread = spawn(|| {
        twitch_chat_wrapper::run(receive_from_game, send_to_game).unwrap();
    });

    let game_state = &mut GameState::new(context, receive_from_chat, send_to_chat)?;

    run(context, event_loop, game_state)
}

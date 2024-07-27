use ggez::{conf::WindowMode, conf::WindowSetup, ContextBuilder, GameResult};
use ggez::event;
use ggez::conf::Conf;


mod camera;
mod draw;
mod game_state;
mod input;
mod tiles;

use game_state::GameState;

fn main() -> GameResult {
    let (mut ctx, event_loop) = ContextBuilder::new("race_sim", "Author")
        .window_mode(WindowMode::default())
        .window_setup(WindowSetup::default().title("Race Simulation"))
        .add_resource_path("./resources")
        .build()?;

    let state = GameState::new(&mut ctx)?;
    event::run(ctx, event_loop, state)
}

fn clear_screen() {
    // Example implementation for clearing the screen
    println!("Clearing the screen");
    // Replace with your graphics library call
}

fn present_frame() {
    // Example implementation for presenting the frame
    println!("Presenting the frame");
    // Replace with your graphics library call
}
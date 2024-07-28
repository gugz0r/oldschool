use env_logger;
use ggez::conf::{WindowMode, WindowSetup};
use ggez::event;
use ggez::{ContextBuilder, GameResult};
use log::{error, info};

mod camera;
mod draw_grass;
mod draw_road;
mod game_state;
mod input;
mod tiles;

use game_state::GameState;

fn main() -> GameResult {
    env_logger::init();

    info!("Starting the game");

    let (mut ctx, event_loop) = match ContextBuilder::new("race_sim", "gugz0r")
        .window_mode(WindowMode::default().dimensions(800.0, 600.0))
        .window_setup(WindowSetup::default().title("Race Simulation"))
        //.add_resource_path("./resources")
        .build()
    {
        Ok(result) => result,
        Err(e) => {
            error!("Failed to build context: {}", e);
            return Err(e);
        }
    };

    info!("Game context created");

    let state = match GameState::new(&mut ctx) {
        Ok(state) => state,
        Err(e) => {
            error!("Failed to initialize game state: {}", e);
            return Err(e);
        }
    };

    info!("Game state initialized");

    event::run(ctx, event_loop, state)
}

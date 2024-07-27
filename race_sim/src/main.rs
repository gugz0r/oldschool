mod camera;
mod tiles;
mod game_state;
mod input;
mod draw;

use ggez::conf::Conf;
use ggez::event;
use ggez::{ContextBuilder, GameResult};
use crate::game_state::GameState;

fn main() -> GameResult {
    let mut conf = Conf::new();
    conf.modules.audio = false;

    let (mut ctx, event_loop) = ContextBuilder::new("tilemap_engine", "gugz0r")
        .default_conf(conf)
        .build()?;

    let state = GameState::new(&mut ctx)?;
    event::run(ctx, event_loop, state)
}

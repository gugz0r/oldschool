use ggez::input::keyboard::KeyCode;
use ggez::Context;

use crate::game_state::GameState;

pub fn handle_input(ctx: &mut Context, state: &mut GameState) {
    state.direction = 0.0;

    if ctx.keyboard.is_key_pressed(KeyCode::Left) {
        state.direction = -1.0;
    } else if ctx.keyboard.is_key_pressed(KeyCode::Right) {
        state.direction = 1.0;
    }

    if ctx.keyboard.is_key_pressed(KeyCode::Up) {
        state.speed += 1.0;
    }
    if ctx.keyboard.is_key_pressed(KeyCode::Down) {
        state.speed -= 1.0;
        if state.speed < 0.0 {
            state.speed = 0.0;
        }
    }
}

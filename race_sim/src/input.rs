// src/input.rs

use ggez::input::keyboard::{self, KeyCode};
use ggez::{Context};
use crate::game_state::GameState;

pub fn handle_input(state: &mut GameState, ctx: &mut Context) {
    state.direction = 0.0; // Reset direction

    if keyboard::is_key_pressed(ctx, KeyCode::Left) {
        state.direction = -1.0;
    } else if keyboard::is_key_pressed(ctx, KeyCode::Right) {
        state.direction = 1.0;
    }

    // Separate speed controls outside of direction checks
    if keyboard::is_key_pressed(ctx, KeyCode::Up) {
        state.speed += 1.0; // Increase speed
    }
    if keyboard::is_key_pressed(ctx, KeyCode::Down) {
        state.speed -= 1.0;
        if (state.speed < 0.0) {
            state.speed = 0.0; // Clamp speed to zero
        }
    }
}

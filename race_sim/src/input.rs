use ggez::input::keyboard;
use ggez::{Context};
use ggez::event::KeyCode;
use crate::game_state::GameState;

pub fn handle_input(state: &mut GameState, ctx: &mut Context) {
    state.direction = 0.0;

    if keyboard::is_key_pressed(ctx, KeyCode::Left) {
        state.camera.x -= 5.0;
    } else if keyboard::is_key_pressed(ctx, KeyCode::Right) {
        state.camera.x += 5.0;
    }

    if keyboard::is_key_pressed(ctx, KeyCode::Up) {
        state.speed += 1.0; // Increase speed
    }
    if keyboard::is_key_pressed(ctx, KeyCode::Down) {
        state.speed -= 1.0;
        if state.speed < 0.0 {
            state.speed = 0.0; // Clamp speed to zero
        }
    }
}

use ggez::input::keyboard;
use ggez::{Context, GameResult};
use ggez::event::KeyCode;
use crate::game_state::GameState;

impl GameState {
    pub fn handle_input(&mut self, ctx: &mut Context) {
        self.direction = 0.0;

        if keyboard::is_key_pressed(ctx, KeyCode::Left) {
            self.direction = -1.0;
        } else if keyboard::is_key_pressed(ctx, KeyCode::Right) {
            self.direction = 1.0;
        }

        if keyboard::is_key_pressed(ctx, KeyCode::Up) {
            self.speed += 1.0;
        }
        if keyboard::is_key_pressed(ctx, KeyCode::Down) {
            self.speed -= 1.0;
            if self.speed < 0.0 {
                self.speed = 0.0;
            }
        }
    }
}
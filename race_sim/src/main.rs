mod camera;
mod tiles;
mod game_state;
mod draw;
mod input;

use ggez::conf::Conf;
use ggez::event::{self, EventHandler};
use ggez::graphics::{self, DrawParam};
use ggez::{Context, ContextBuilder, GameResult};
use game_state::GameState;

impl EventHandler<ggez::GameError> for GameState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        self.handle_input(ctx);

        let dt = ggez::timer::duration_to_f64(ggez::timer::delta(ctx)) as f32;
        self.scroll += self.speed * self.direction * dt;
        if self.scroll >= self.tiles[0].width {
            self.scroll -= self.tiles[0].width;
        }
        if self.scroll < 0.0 {
            self.scroll += self.tiles[0].width;
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, [0.0, 0.5, 1.0, 1.0].into());

        self.draw_grass(ctx)?;
        self.draw_perspective_road(ctx)?;

        let tile = &self.tiles[0];
        let num_repeats = (800.0 / tile.width).ceil() as i32 + 1;

        for i in 0..num_repeats {
            let dest_x = i as f32 * tile.width - self.scroll;
            graphics::draw(
                ctx,
                &tile.image,
                DrawParam::default().dest([dest_x, 0.0])
            )?;
        }

        graphics::present(ctx)?;
        Ok(())
    }
}

fn main() -> GameResult {
    let mut conf = Conf::new();
    conf.modules.audio = false;

    let (mut ctx, event_loop) = ContextBuilder::new("tilemap_engine", "gugz0r")
        .default_conf(conf)
        .build()?;

    let state = GameState::new(&mut ctx)?;
    event::run(ctx, event_loop, state)
}
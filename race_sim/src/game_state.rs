use ggez::event::EventHandler;
use ggez::graphics::{self, Color, DrawParam, Image};
use ggez::{Context, GameResult};

use crate::camera::Camera;
use crate::input;
use crate::tiles::{Tile, Tilemap};
use crate::draw;

pub struct GameState {
    pub tilemap: Tilemap,
    pub camera: Camera,
    pub tiles: Vec<Tile>,
    pub speed: f32,
    pub scroll: f32,
    pub direction: f32,
}

impl GameState {
    pub fn new(ctx: &mut Context) -> GameResult<Self> {
        let tile_image_bytes: Vec<Vec<u8>> = vec![
            include_bytes!("tiles/mountains.png").to_vec(),
        ];

        let mut tiles = Vec::new();
        for bytes in tile_image_bytes {
            let image = Image::from_bytes(ctx, &bytes)?;
            let width = image.width() as f32;
            let height = image.height() as f32;
            tiles.push(Tile { image, width, height });
        }

        let tilemap = Tilemap::new(); // Load your tilemap data here
        let camera = Camera::new(0.0, 0.0, 800.0, 600.0);
        let speed = 100.0; // Speed of the scrolling
        let scroll = 0.0;
        let direction = 0.0;

        Ok(GameState {
            tilemap,
            camera,
            tiles,
            speed,
            scroll,
            direction,
        })
    }
}

impl EventHandler<ggez::GameError> for GameState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        input::handle_input(self, ctx);

        let dt = ggez::timer::duration_to_f64(ggez::timer::delta(ctx)) as f32;
        self.scroll += self.speed * dt;
        if self.scroll >= self.tiles[0].height {
            self.scroll -= self.tiles[0].height;
        }
        if self.scroll < 0.0 {
            self.scroll += self.tiles[0].height;
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, [0.0, 0.5, 1.0, 1.0].into());

        draw::draw_background(ctx, self)?;

        let tile = &self.tiles[0];
        let num_repeats = (800.0 / tile.width).ceil() as i32 + 1;

        for i in 0..num_repeats {
            let dest_x = i as f32 * tile.width - self.camera.x;
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

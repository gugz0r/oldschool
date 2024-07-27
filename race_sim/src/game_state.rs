// src/game_state.rs

use ggez::graphics::Image;
use ggez::{Context, GameResult};
use ggez::event::EventHandler;

use crate::camera::Camera;
use crate::tiles::{Tile, Tilemap};
use crate::input::handle_input;
use crate::draw::{draw_road, draw_grass, draw_background};

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

        let tilemap = Tilemap::new();
        let camera = Camera::new(0.0, 0.0, 800.0, 600.0);
        let speed = 100.0; 
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
        handle_input(self, ctx);

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
        ggez::graphics::clear(ctx, [0.0, 0.5, 1.0, 1.0].into()); 

        draw_background(ctx, self)?;
        draw_grass(ctx, self)?;
        draw_road(ctx, self)?;

        ggez::graphics::present(ctx)?;
        Ok(())
    }
}

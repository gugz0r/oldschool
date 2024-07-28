use ggez::event::EventHandler;
use ggez::graphics::{self, Canvas, Color, Image};
use ggez::input::keyboard::KeyCode;
use ggez::Context;
use ggez::GameResult;
use log::info;

use crate::camera::Camera;
use crate::draw_grass;
use crate::draw_road;
use crate::tiles::{Tile, Tilemap};

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
        info!("Initializing game state");

        let tile_image_bytes: Vec<Vec<u8>> = vec![include_bytes!("tiles/mountains.png").to_vec()];

        let mut tiles = Vec::new();
        for bytes in tile_image_bytes {
            let image = Image::from_bytes(ctx, &bytes)?;
            let width = image.width() as f32;
            let height = image.height() as f32;
            tiles.push(Tile {
                image,
                width,
                height,
            });
        }

        let tilemap = Tilemap::new();
        let camera = Camera::new(0.0, 0.0, 800.0, 600.0);
        let speed = 100.0;
        let scroll = 0.0;
        let direction = 0.0;

        info!("Game state initialized successfully");

        Ok(GameState {
            tilemap,
            camera,
            tiles,
            speed,
            scroll,
            direction,
        })
    }

    pub fn handle_input(&mut self, ctx: &mut Context) {
        self.direction = 0.0;

        if ctx.keyboard.is_key_pressed(KeyCode::Left) {
            self.direction = -1.0;
        } else if ctx.keyboard.is_key_pressed(KeyCode::Right) {
            self.direction = 1.0;
        }

        if ctx.keyboard.is_key_pressed(KeyCode::Up) {
            self.speed += 1.0;
        }
        if ctx.keyboard.is_key_pressed(KeyCode::Down) {
            self.speed -= 1.0;
            if self.speed < 0.0 {
                self.speed = 0.0;
            }
        }
    }
}

impl EventHandler for GameState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        info!("Updating game state");
        self.handle_input(ctx);

        let dt = ctx.time.delta().as_secs_f32();
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
        info!("Drawing frame");

        let mut canvas = Canvas::from_frame(ctx, Color::new(0.0, 0.5, 1.0, 1.0));

        self.camera.x = -self.scroll;

        draw_grass::draw_grass(ctx, &mut canvas, self)?;
        draw_road::draw_perspective_road(ctx, &mut canvas, self)?;

        let tile = &self.tiles[0];
        let num_repeats = (800.0 / tile.width).ceil() as i32 + 1;

        for i in 0..num_repeats {
            let dest_x = i as f32 * tile.width - self.scroll;
            canvas.draw(
                &tile.image,
                graphics::DrawParam::default().dest([dest_x, 0.0]),
            );
        }

        canvas.finish(ctx)?;
        Ok(())
    }
}

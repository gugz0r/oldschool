use ggez::graphics::{Image};
use ggez::{Context, GameResult};
use crate::camera::Camera;
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
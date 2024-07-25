// src/tiles.rs

use ggez::graphics::Image;
use ggez::{Context, GameResult};

// Struct to represent a single tile
pub struct Tile {
    pub image: Image,
    pub width: f32,
    pub height: f32,
}

impl Tile {
    pub fn new(ctx: &mut Context, path: &str) -> GameResult<Self> {
        let image = Image::new(ctx, path)?;
        let width = image.width() as f32;
        let height = image.height() as f32;

        Ok(Tile {
            image,
            width,
            height,
        })
    }
}

// Struct to represent the entire tilemap
pub struct Tilemap {
    pub layers: Vec<Vec<Vec<u8>>>, // A 3D vector to hold multiple layers of tiles
}

impl Tilemap {
    pub fn new() -> Self {
        let layer = vec![
            vec![0, 1, 0, 1, 0], // Example data
            vec![1, 0, 1, 0, 1],
            vec![0, 1, 0, 1, 0],
        ];

        Tilemap {
            layers: vec![layer], // Single layer for simplicity
        }
    }

    // Add a method to get a tile ID at a specific position
    pub fn get_tile_id(&self, layer: usize, x: usize, y: usize) -> Option<u8> {
        self.layers.get(layer)?.get(y)?.get(x).copied()
    }

    // Add a method to set a tile ID at a specific position
    pub fn set_tile_id(&mut self, layer: usize, x: usize, y: usize, id: u8) {
        if let Some(layer) = self.layers.get_mut(layer) {
            if let Some(row) = layer.get_mut(y) {
                if let Some(tile) = row.get_mut(x) {
                    *tile = id;
                }
            }
        }
    }
}

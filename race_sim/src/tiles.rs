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
        let image = Image::from_path(ctx, path)?;
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
    pub layers: Vec<Vec<Vec<u8>>>,
}

impl Tilemap {
    pub fn new() -> Self {
        Tilemap {
            layers: vec![],
        }
    }

    pub fn get_tile_id(&self, layer: usize, x: usize, y: usize) -> Option<u8> {
        self.layers.get(layer)?.get(y)?.get(x).copied()
    }

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

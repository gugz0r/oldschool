// src/camera.rs

pub struct Camera {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

impl Camera {
    pub fn new(x: f32, y: f32, width: f32, height: f32) -> Self {
        Camera {
            x,
            y,
            width,
            height,
        }
    }
}
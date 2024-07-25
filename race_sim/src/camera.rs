// src/camera.rs

/// Represents a camera used for rendering.
pub struct Camera {
    /// The x-coordinate of the camera's position.
    pub x: f32,
    /// The y-coordinate of the camera's position.
    pub y: f32,
    /// The width of the camera's viewport.
    pub width: f32,
    /// The height of the camera's viewport.
    pub height: f32,
}

impl Camera {
    /// Creates a new camera with the specified position and viewport size.
    ///
    /// # Arguments
    ///
    /// * `x` - The x-coordinate of the camera's position.
    /// * `y` - The y-coordinate of the camera's position.
    /// * `width` - The width of the camera's viewport.
    /// * `height` - The height of the camera's viewport.
    ///
    /// # Returns
    ///
    /// A new `Camera` instance.
    pub fn new(x: f32, y: f32, width: f32, height: f32) -> Self {
        Camera {
            x,
            y,
            width,
            height,
        }
    }
}

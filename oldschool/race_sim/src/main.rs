// src/main.rs

use ggez::conf::Conf;
use ggez::event::{self, EventHandler, KeyCode};
use ggez::graphics::{self, Color, DrawMode, DrawParam, Image, Mesh, Rect};
use ggez::input::keyboard;
use ggez::mint::Point2;
use ggez::{Context, ContextBuilder, GameResult};

mod camera;
mod tiles;
use camera::Camera;
use tiles::{Tile, Tilemap};

struct GameState {
    tilemap: Tilemap,
    camera: Camera,
    tiles: Vec<Tile>,
    speed: f32,
    scroll: f32,
    direction: f32,
}

impl GameState {
    fn new(ctx: &mut Context) -> GameResult<Self> {
        // Embed tile images
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

    fn handle_input(&mut self, ctx: &mut Context) {
        self.direction = 0.0; // Reset direction

        if keyboard::is_key_pressed(ctx, KeyCode::Left) {
            self.direction = -1.0;
        } else if keyboard::is_key_pressed(ctx, KeyCode::Right) {
            self.direction = 1.0;
        }

        // Separate speed controls outside of direction checks
        if keyboard::is_key_pressed(ctx, KeyCode::Up) {
            self.speed += 1.0; // Increase speed
        }
        if keyboard::is_key_pressed(ctx, KeyCode::Down) {
            self.speed -= 1.0;
            if self.speed < 0.0 {
                self.speed = 0.0; // Clamp speed to zero
            }
        }
    }

    fn draw_perspective_road(&self, ctx: &mut Context) -> GameResult {
        let road_color = Color::from_rgb(128, 128, 128); // Gray color
        let road_top_width = 50.0;
        let road_bottom_width = 800.0;
        let road_top_y = 300.0; // Adjusted to touch the mountains
        let road_bottom_y = 600.0;

        let road_mesh = Mesh::new_polygon(
            ctx,
            DrawMode::fill(),
            &[
                Point2 {
                    x: 400.0 - road_top_width / 2.0,
                    y: road_top_y,
                },
                Point2 {
                    x: 400.0 + road_top_width / 2.0,
                    y: road_top_y,
                },
                Point2 {
                    x: 400.0 + road_bottom_width / 2.0,
                    y: road_bottom_y,
                },
                Point2 {
                    x: 400.0 - road_bottom_width / 2.0,
                    y: road_bottom_y,
                },
            ],
            road_color,
        )?;

        graphics::draw(ctx, &road_mesh, DrawParam::default())?;

        // Draw white line in the middle of the road
        for i in 0..10 {
            let y_start = road_top_y + (i as f32 * 30.0);
            let y_end = y_start + 20.0;
            let width_start = road_top_width / 10.0 * i as f32;
            let width_end = road_top_width / 10.0 * (i as f32 + 1.0);

            let line_mesh = Mesh::new_polygon(
                ctx,
                DrawMode::fill(),
                &[
                    Point2 {
                        x: 400.0 - width_start / 2.0,
                        y: y_start,
                    },
                    Point2 {
                        x: 400.0 + width_start / 2.0,
                        y: y_start,
                    },
                    Point2 {
                        x: 400.0 + width_end / 2.0,
                        y: y_end,
                    },
                    Point2 {
                        x: 400.0 - width_end / 2.0,
                        y: y_end,
                    },
                ],
                Color::from_rgb(255, 255, 255),
            )?;
            graphics::draw(ctx, &line_mesh, DrawParam::default())?;
        }

        Ok(())
    }

    fn draw_grass(&self, ctx: &mut Context) -> GameResult {
        let grass_color1 = Color::from_rgb(34, 139, 34); // Green color
        let grass_color2 = Color::from_rgb(0, 100, 0); // Darker green color
        let grass_top_y = 300.0; // Adjusted to touch the road
        let grass_bottom_y = 600.0;

        // Draw alternating grass sections
        for i in 0..10 {
            let y_start = grass_top_y + (i as f32 * (grass_bottom_y - grass_top_y) / 10.0);
            let y_end = grass_top_y + ((i + 1) as f32 * (grass_bottom_y - grass_top_y) / 10.0);
            let width_start = 800.0 / 10.0 * i as f32;
            let width_end = 800.0 / 10.0 * (i as f32 + 1.0);
            let color = if i % 2 == 0 {
                grass_color1
            } else {
                grass_color2
            };

            let grass_mesh = Mesh::new_polygon(
                ctx,
                DrawMode::fill(),
                &[
                    Point2 { x: 0.0, y: y_start },
                    Point2 {
                        x: 800.0,
                        y: y_start,
                    },
                    Point2 { x: 800.0, y: y_end },
                    Point2 { x: 0.0, y: y_end },
                ],
                color,
            )?;
            graphics::draw(ctx, &grass_mesh, DrawParam::default())?;
        }

        Ok(())
    }
}

impl EventHandler<ggez::GameError> for GameState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        // Handle input
        self.handle_input(ctx);

        // Update the scroll position based on speed, direction, and elapsed time
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

        // Apply the scroll to the camera position
        self.camera.x = -self.scroll;

        // Draw the background with the camera
        self.draw_grass(ctx)?;
        self.draw_perspective_road(ctx)?;

        let tile = &self.tiles[0];
        let num_repeats = (800.0 / tile.width).ceil() as i32 + 1;

        for i in 0..num_repeats {
            let dest_x = i as f32 * tile.width - self.scroll;
            graphics::draw(ctx, &tile.image, DrawParam::default().dest([dest_x, 0.0]))?;
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

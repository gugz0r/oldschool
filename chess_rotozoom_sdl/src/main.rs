extern crate sdl2;

use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};
use sdl2::render::{Texture, TextureCreator};
use sdl2::video::WindowContext;
use sdl2::Sdl;
use std::time::Duration;

const BOARD_SIZE: u32 = 8;
const TILE_SIZE: u32 = 64;
const WINDOW_SIZE: u32 = BOARD_SIZE * TILE_SIZE;
const PI: f64 = std::f64::consts::PI;

fn draw_chessboard<'a>(
    texture_creator: &'a TextureCreator<WindowContext>,
    colors: &[Color],
) -> Texture<'a> {
    let mut texture = texture_creator
        .create_texture_streaming(None, WINDOW_SIZE, WINDOW_SIZE)
        .unwrap();

    texture
        .with_lock(None, |buffer: &mut [u8], pitch: usize| {
            for y in 0..WINDOW_SIZE {
                for x in 0..WINDOW_SIZE {
                    let offset = y as usize * pitch + x as usize * 4;
                    let tile_index = ((x / TILE_SIZE) + (y / TILE_SIZE)) as usize % colors.len();
                    let color = colors[tile_index];
                    buffer[offset] = color.r;
                    buffer[offset + 1] = color.g;
                    buffer[offset + 2] = color.b;
                    buffer[offset + 3] = 255;
                }
            }
        })
        .unwrap();

    texture
}

fn main() {
    let sdl_context: Sdl = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window(
            "Chess Rotozoom - Use UP or DOWN to play with speed: 1.0",
            WINDOW_SIZE,
            WINDOW_SIZE,
        )
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();
    let texture_creator = canvas.texture_creator();

    let mut colors = vec![
        Color::RGB(255, 0, 0),     // Red
        Color::RGB(0, 255, 0),     // Green
        Color::RGB(0, 0, 255),     // Blue
        Color::RGB(255, 255, 0),   // Yellow
        Color::RGB(255, 0, 255),   // Magenta
        Color::RGB(0, 255, 255),   // Cyan
        Color::RGB(255, 255, 255), // White
    ];

    let mut angle: f64 = 0.0;
    let mut scale: f64 = 1.0;
    let mut growing = true;
    let mut speed = 1.0; // Added speed variable

    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut color_offset: f64 = 0.0;

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                sdl2::event::Event::Quit { .. }
                | sdl2::event::Event::KeyDown {
                    keycode: Some(sdl2::keyboard::Keycode::Escape),
                    ..
                } => break 'running,
                sdl2::event::Event::KeyDown {
                    keycode: Some(sdl2::keyboard::Keycode::Up),
                    ..
                } => {
                    speed += 0.1;
                    canvas
                        .window_mut()
                        .set_title(&format!(
                            "Chess Rotozoom - Use UP or DOWN to play with speed: {:.1}",
                            speed
                        ))
                        .unwrap();
                }
                sdl2::event::Event::KeyDown {
                    keycode: Some(sdl2::keyboard::Keycode::Down),
                    ..
                } => {
                    speed -= 0.1;
                    canvas
                        .window_mut()
                        .set_title(&format!(
                            "Chess Rotozoom - Use UP or DOWN to play with speed: {:.1}",
                            speed
                        ))
                        .unwrap();
                }
                _ => {}
            }
        }

        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();

        // Smooth color transition effect
        color_offset += 0.01 * speed;
        for (i, color) in colors.iter_mut().enumerate() {
            let offset = (i as f64 + color_offset) * 0.1;
            color.r = ((offset.sin() * 127.0 + 128.0) as u8).max(0).min(255);
            color.g = (((offset + 2.0 * PI / 3.0).sin() * 127.0 + 128.0) as u8)
                .max(0)
                .min(255);
            color.b = (((offset + 4.0 * PI / 3.0).sin() * 127.0 + 128.0) as u8)
                .max(0)
                .min(255);
        }

        let chessboard_texture = draw_chessboard(&texture_creator, &colors);

        let center = Point::new((WINDOW_SIZE / 2) as i32, (WINDOW_SIZE / 2) as i32);
        let dst = Rect::new(
            ((WINDOW_SIZE as f64 * (1.0 - scale)) / 2.0) as i32,
            ((WINDOW_SIZE as f64 * (1.0 - scale)) / 2.0) as i32,
            (WINDOW_SIZE as f64 * scale) as u32,
            (WINDOW_SIZE as f64 * scale) as u32,
        );

        canvas
            .copy_ex(
                &chessboard_texture,
                None,
                Some(dst),
                angle, // Apply angle directly
                Some(center),
                false,
                false,
            )
            .unwrap();

        canvas.present();

        angle += 1.0 * speed; // Update angle based on speed
        if growing {
            scale += 0.05 * speed; // Increased zoom speed
            if scale > 4.0 {
                // Increased maximum zoom level
                growing = false;
            }
        } else {
            scale -= 0.05 * speed; // Increased zoom speed
            if scale < 0.2 {
                // Decreased minimum zoom level
                growing = true;
            }
        }

        ::std::thread::sleep(Duration::from_millis(16));
    }
}

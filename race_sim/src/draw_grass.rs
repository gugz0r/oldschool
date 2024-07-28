use ggez::graphics::{self, Canvas, Color, Mesh};
use ggez::mint::Point2;
use ggez::{Context, GameResult};
use log::info;

use crate::game_state::GameState;

pub fn draw_grass(ctx: &mut Context, canvas: &mut Canvas, _state: &GameState) -> GameResult {
    info!("Drawing grass");

    let grass_color1 = Color::from_rgb(34, 139, 34);
    let grass_color2 = Color::from_rgb(0, 100, 0);
    let grass_top_y = 200.0;
    let grass_bottom_y = 600.0;

    let num_stripes = 20;
    for i in 0..num_stripes {
        let y_start =
            grass_top_y + (i as f32 * (grass_bottom_y - grass_top_y) / num_stripes as f32);
        let y_end =
            grass_top_y + ((i + 1) as f32 * (grass_bottom_y - grass_top_y) / num_stripes as f32);
        let width_start = 800.0 / num_stripes as f32 * i as f32;
        let width_end = 800.0 / num_stripes as f32 * (i + 1) as f32;
        let color = if i % 2 == 0 {
            grass_color1
        } else {
            grass_color2
        };

        let grass_mesh = Mesh::new_polygon(
            ctx,
            graphics::DrawMode::fill(),
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
        canvas.draw(&grass_mesh, graphics::DrawParam::default());
    }

    info!("Finished drawing grass");
    Ok(())
}

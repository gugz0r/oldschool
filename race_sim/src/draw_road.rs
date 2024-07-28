use ggez::graphics::{self, Canvas, Color, Mesh};
use ggez::mint::Point2;
use ggez::{Context, GameResult};
use log::info;

use crate::game_state::GameState;

pub fn draw_perspective_road(
    ctx: &mut Context,
    canvas: &mut Canvas,
    _state: &GameState,
) -> GameResult {
    info!("Drawing perspective road");

    let road_color = Color::from_rgb(128, 128, 128);
    let road_top_width = 50.0;
    let road_bottom_width = 800.0;
    let road_top_y = 200.0;
    let road_bottom_y = 600.0;

    let road_mesh = Mesh::new_polygon(
        ctx,
        graphics::DrawMode::fill(),
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

    canvas.draw(&road_mesh, graphics::DrawParam::default());

    let num_dashes = 20;
    for i in 0..num_dashes {
        let y_start = road_top_y + (i as f32 * (road_bottom_y - road_top_y) / num_dashes as f32);
        let y_end =
            road_top_y + ((i + 1) as f32 * (road_bottom_y - road_top_y) / num_dashes as f32);
        let width_start =
            road_top_width + (i as f32 * (road_bottom_width - road_top_width) / num_dashes as f32);
        let width_end = road_top_width
            + ((i + 1) as f32 * (road_bottom_width - road_top_width) / num_dashes as f32);

        let dash_mesh = Mesh::new_polygon(
            ctx,
            graphics::DrawMode::fill(),
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
            Color::WHITE,
        )?;
        canvas.draw(&dash_mesh, graphics::DrawParam::default());
    }

    info!("Finished drawing perspective road");
    Ok(())
}

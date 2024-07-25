use ggez::graphics::{self, Color, DrawMode, Mesh};
use ggez::{Context, GameResult};
use ggez::mint::Point2;
use crate::game_state::GameState;

pub fn draw_background(ctx: &mut Context, state: &mut GameState) -> GameResult {
    let road_color = Color::from_rgb(128, 128, 128);
    let road_top_width = 50.0;
    let road_bottom_width = 800.0;
    let road_top_y = 250.0;
    let road_bottom_y = 600.0;

    let road_mesh = Mesh::new_polygon(
        ctx,
        DrawMode::fill(),
        &[
            Point2 { x: 400.0 - road_top_width / 2.0, y: road_top_y },
            Point2 { x: 400.0 + road_top_width / 2.0, y: road_top_y },
            Point2 { x: 400.0 + road_bottom_width / 2.0, y: road_bottom_y },
            Point2 { x: 400.0 - road_bottom_width / 2.0, y: road_bottom_y },
        ],
        road_color,
    )?;

    graphics::draw(ctx, &road_mesh, graphics::DrawParam::default())?;

    for i in 0..10 {
        let y_start = road_top_y + (i as f32 * 30.0) - state.scroll % 30.0;
        let y_end = y_start + 20.0;
        let width_start = road_top_width + (i as f32 * (road_bottom_width - road_top_width) / 10.0);
        let width_end = road_top_width + ((i + 1) as f32 * (road_bottom_width - road_top_width) / 10.0);

        let line_mesh = Mesh::new_polygon(
            ctx,
            DrawMode::fill(),
            &[
                Point2 { x: 400.0 - width_start / 2.0, y: y_start },
                Point2 { x: 400.0 + width_start / 2.0, y: y_start },
                Point2 { x: 400.0 + width_end / 2.0, y: y_end },
                Point2 { x: 400.0 - width_end / 2.0, y: y_end },
            ],
            Color::from_rgb(255, 255, 255),
        )?;
        graphics::draw(ctx, &line_mesh, graphics::DrawParam::default())?;
    }

    let grass_color1 = Color::from_rgb(34, 139, 34);
    let grass_color2 = Color::from_rgb(0, 100, 0);
    let grass_top_y = 250.0;
    let grass_bottom_y = 600.0;

    for i in 0..10 {
        let y_start = grass_top_y + (i as f32 * (grass_bottom_y - grass_top_y) / 10.0);
        let y_end = grass_top_y + ((i + 1) as f32 * (grass_bottom_y - grass_top_y) / 10.0);

        let color = if i % 2 == 0 { grass_color1 } else { grass_color2 };

        let grass_mesh = Mesh::new_polygon(
            ctx,
            DrawMode::fill(),
            &[
                Point2 { x: 0.0, y: y_start },
                Point2 { x: 800.0, y: y_start },
                Point2 { x: 800.0, y: y_end },
                Point2 { x: 0.0, y: y_end },
            ],
            color,
        )?;
        graphics::draw(ctx, &grass_mesh, graphics::DrawParam::default())?;
    }

    Ok(())
}

use sdl2::rect::{Rect, Point};
use sdl2::pixels::Color;
use sdl2::render::WindowCanvas;
use crate::game::GameState;
use crate::game::map::CellType;
use crate::rendering::{shoot_ray, FOV_DEGREES};

pub fn render_overhead_map(canvas: &mut WindowCanvas, game_state: &GameState) {
    let zoom = game_state.map_zoom_level as f32;

    for row in 0..game_state.map.height as i32 {
        for col in 0..game_state.map.width as i32 {
            let x1 = (col as f32 * game_state.map.units_per_cell as f32 * zoom) as i32;
            let y1 = (row as f32 * game_state.map.units_per_cell as f32 * zoom) as i32;
            let width = (game_state.map.units_per_cell as f32 * zoom) as u32;
            let height = (game_state.map.units_per_cell as f32 * zoom) as u32;

            let rect = Rect::new(x1, y1, width, height);
            let cell = game_state.map.cell_at(row as usize, col as usize);

            match cell {
                None => unreachable!(),
                Some(CellType::BrickWall) => canvas.set_draw_color(Color::RED),
                Some(CellType::BlueWall) => canvas.set_draw_color(Color::BLUE),
                Some(CellType::WoodWall) => canvas.set_draw_color(Color::YELLOW),
                Some(CellType::Empty) => canvas.set_draw_color(Color::WHITE)
            }

            canvas.fill_rect(rect).unwrap();
        }
    }

    {
        // Adjust the player's position based on the scale of the map
        let player_size = game_state.player.collision_size as f32 * zoom;
        let pos_x = game_state.player.position.x * zoom;
        let pos_y = game_state.player.position.y * zoom;

        let x1 = pos_x - (player_size / 2.0);
        let y1 = pos_y - (player_size / 2.0);

        let rect = Rect::new(x1 as i32, y1 as i32, player_size as u32, player_size as u32);

        canvas.set_draw_color(Color::GREEN);
        canvas.fill_rect(rect).unwrap();

        // Show rays for all pixel columns
        canvas.set_draw_color(Color::BLACK);

        let first_ray_at = game_state.player.facing - FOV_DEGREES.to_radians() / 2.0;
        let ray_count = canvas.window().size().0;
        let radians_per_ray = FOV_DEGREES.to_radians() / ray_count as f32;
        for x in 0..ray_count {
            let angle = first_ray_at + (radians_per_ray * x as f32);
            let ray = shoot_ray(game_state, angle);

            let line_end_x = (angle.0.cos() * ray.distance * zoom) + pos_x;
            let line_end_y = (angle.0.sin() * ray.distance * zoom) + pos_y;

            canvas.draw_line(Point::new(pos_x as i32, pos_y as i32),
                             Point::new(line_end_x as i32, line_end_y as i32)).unwrap();
        }
    }
}
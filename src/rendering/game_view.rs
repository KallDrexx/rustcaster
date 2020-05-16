use std::cmp::max;
use sdl2::rect::{Rect, Point};
use sdl2::render::WindowCanvas;
use sdl2::pixels::Color;
use crate::game::GameState;
use crate::{SCREEN_HEIGHT, SCREEN_WIDTH};
use crate::rendering::{shoot_ray, FOV_DEGREES};

pub fn render_game_view(canvas: &mut WindowCanvas, game_state: &GameState) {
    canvas.set_draw_color(Color::GRAY);
    canvas.clear();

    canvas.set_draw_color(Color::WHITE);
    canvas.fill_rect(Rect::new(0, SCREEN_HEIGHT as i32 / 2, SCREEN_WIDTH, SCREEN_HEIGHT / 2)).unwrap();

    let first_ray_at = game_state.player.facing - FOV_DEGREES.to_radians() / 2.0;
    let ray_count = canvas.window().size().0;
    let radians_per_ray = FOV_DEGREES.to_radians() / ray_count as f32;

    for x in 0..ray_count {
        let angle = first_ray_at + (radians_per_ray * x as f32);
        let ray = shoot_ray(game_state, angle);

        let mut distance = ray.distance;
        if distance <= 0.0 {
            distance = 0.1;
        }

        // Adjust the distance to prevent fish-eye distortion
        let angle_from_facing = game_state.player.facing - angle;
        let mut adjusted_distance = distance * angle_from_facing.0.cos();
        if adjusted_distance < 1.0 {
            adjusted_distance = 1.0;
        }


        const COLOR_GRADIENT_VALUE: f32 = 5.0;
        const MIN_COLOR: u8 = 50;

        let color_denominator = if distance < COLOR_GRADIENT_VALUE { 1.0 } else { distance / COLOR_GRADIENT_VALUE };
        let color_value = (255.0 / color_denominator) as u8;
        let color = Color::RGB(max(color_value, MIN_COLOR), 0, 0);

        canvas.set_draw_color(color);

        let mut height = SCREEN_HEIGHT as f32 / (adjusted_distance / 1.5);
        if height > SCREEN_HEIGHT as f32 {
            height = SCREEN_HEIGHT as f32;
        }

        let start_y = SCREEN_HEIGHT as f32 / 2.0 - height / 2.0;
        for y in (start_y as u32)..(start_y as u32 + height as u32) {
            canvas.draw_point(Point::new(x as i32, y as i32)).unwrap();
        }
    }
}
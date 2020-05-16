use sdl2::render::WindowCanvas;
use sdl2::pixels::Color;
use sdl2::rect::{Rect, Point};
use crate::game::map::CellType;
use crate::game::GameState;
use crate::core::radians::Radians;
use crate::core::vector::Vector;
use crate::core::degrees::Degrees;
use crate::{SCREEN_HEIGHT, SCREEN_WIDTH};
use std::cmp::max;

const FOV_DEGREES: Degrees = Degrees(90.0);

pub fn render(canvas: &mut WindowCanvas, game_state: &GameState) {
    canvas.set_draw_color(Color::BLACK);
    canvas.clear();

    render_game_view(canvas, game_state);
    if game_state.display_map {
        render_overhead_map(canvas, game_state);
    }

    canvas.present();
}

struct RayResult {
    distance: f32,
}

fn render_overhead_map(canvas: &mut WindowCanvas, game_state: &GameState) {
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
                Some(CellType::Wall) => canvas.set_draw_color(Color::RED),
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
            canvas.draw_line(Point::new(pos_x as i32, pos_y as i32), Point::new(line_end_x as i32, line_end_y as i32)).unwrap();
        }
    }
}

fn render_game_view(canvas: &mut WindowCanvas, game_state: &GameState) {
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

        let height = SCREEN_HEIGHT as f32 / (adjusted_distance / 1.5);
        let start_y = SCREEN_HEIGHT as f32 / 2.0 - height / 2.0;

        const COLOR_GRADIENT_VALUE: f32 = 5.0;
        const MIN_COLOR: u8 = 50;

        let color_denominator = if distance < COLOR_GRADIENT_VALUE { 1.0 } else { distance / COLOR_GRADIENT_VALUE };
        let color_value = (255.0 / color_denominator) as u8;
        let color = Color::RGB(max(color_value, MIN_COLOR), 0, 0);
        canvas.set_draw_color(color);
        canvas.draw_line(Point::new(x as i32, start_y as i32), Point::new(x as i32, start_y as i32 + height as i32)).unwrap();
    }
}

fn shoot_ray(game_state: &GameState, angle: Radians) -> RayResult {
    // If the player is off the map in the negative region, don't shoot the array
    if game_state.player.position.x < 0.0 || game_state.player.position.y < 0.0 {
        return RayResult {distance: 0.0};
    }

    let rise = angle.0.sin();
    let run = angle.0.cos();
    let slope = rise / run;

    // compute y intercept for full y=mx+b equation
    let y_offset = (-slope * game_state.player.position.x) + game_state.player.position.y;

    let y_adjustment = if rise < 0.0 { 0 } else { game_state.map.units_per_cell };
    let x_adjustment = if run < 0.0 { 0 } else { game_state.map.units_per_cell };
    let current_row = game_state.player.position.y as u32 / game_state.map.units_per_cell;
    let current_col = game_state.player.position.x as u32 / game_state.map.units_per_cell;

    let mut y_value = (current_row * game_state.map.units_per_cell + y_adjustment) as f32;
    let mut x_value = (current_col * game_state.map.units_per_cell + x_adjustment) as f32;

    loop {
        let horizontally_out_of_map = x_value < 0.0 || x_value > game_state.map.width as f32 * game_state.map.units_per_cell as f32;
        let vertically_out_of_map = y_value < 0.0 || y_value > game_state.map.height as f32 * game_state.map.units_per_cell as f32;

        if horizontally_out_of_map && vertically_out_of_map {
            return RayResult { distance: 0.0 };
        }

        let x_at_y_value_intercept = (y_value - y_offset) / slope;
        let y_at_x_value_intercept = slope * x_value + y_offset;

        let distance_to_y_value = calc_distance(&game_state.player.position, x_at_y_value_intercept, y_value);
        let distance_to_x_value = calc_distance(&game_state.player.position, x_value, y_at_x_value_intercept);

        if distance_to_y_value.is_finite() && distance_to_y_value < distance_to_x_value {
            // since we are on the border of two cells we need to look up the next cell over, depending on if we are going up or down
            let adjustment = if rise > 0.0 { 0.1 } else { -0.1 };
            let adjusted_value = y_value + adjustment;

            let row = adjusted_value as u32 / game_state.map.units_per_cell;
            let col = x_at_y_value_intercept as u32 / game_state.map.units_per_cell;

            match game_state.map.cell_at(row as usize, col as usize) {
                None => (),
                Some(CellType::Wall) => return RayResult {distance: distance_to_y_value},
                Some(_) => (),
            }

            // hit empty space
            y_value += game_state.map.units_per_cell as f32 * if rise < 0.0 { -1.0 } else { 1.0 };
        } else {
            // since we are on the border of two cells we need to look up the next cell over, depending on if we are going right or left
            let adjustment = if run > 0.0 { 0.1 } else { -0.1 };
            let adjusted_value = x_value + adjustment;

            let row = y_at_x_value_intercept as u32 / game_state.map.units_per_cell;
            let col = adjusted_value as u32 / game_state.map.units_per_cell;

            match game_state.map.cell_at(row as usize, col as usize) {
                None => (),
                Some(CellType::Wall) => return RayResult {distance: distance_to_x_value},
                Some(_) => (),
            }

            // hit empty space
            x_value += game_state.map.units_per_cell as f32 * if run < 0.0 { -1.0 } else { 1.0 };
        }
    }
}
fn calc_distance(first: &Vector, second_x: f32, second_y: f32) -> f32 {
    let a = (first.y - second_y).abs();
    let b = (first.x - second_x).abs();

    (a * a + b * b).sqrt()
}
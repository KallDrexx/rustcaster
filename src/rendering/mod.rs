pub mod atlas;
mod map;
mod game_view;

use sdl2::render::WindowCanvas;
use sdl2::pixels::Color;
use crate::game::GameState;
use crate::core::radians::Radians;
use crate::game::map::CellType;
use crate::core::vector::Vector;
use crate::core::degrees::Degrees;
use crate::rendering::atlas::Atlas;
use map::render_overhead_map;
use game_view::render_game_view;

pub fn render(canvas: &mut WindowCanvas, game_state: &GameState, wall_atlas: &Atlas) {
    canvas.set_draw_color(Color::BLACK);
    canvas.clear();

    render_game_view(canvas, game_state, wall_atlas);
    if game_state.display_map {
        render_overhead_map(canvas, game_state);
    }

    canvas.present();
}

const FOV_DEGREES: Degrees = Degrees(90.0);

struct RayResult {
    distance: f32,
    units_from_cell_start: f32,
    cell_type: CellType,
}

fn shoot_ray(game_state: &GameState, angle: Radians) -> RayResult {
    // If the player is off the map in the negative region, don't shoot the array
    if game_state.player.position.x < 0.0 || game_state.player.position.y < 0.0 {
        return RayResult {distance: 0.0, units_from_cell_start: 0.0, cell_type: CellType::Empty};
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
            return RayResult { distance: 0.0, units_from_cell_start: 0.0, cell_type: CellType::Empty };
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

            match get_ray_result(game_state, row as usize, col as usize, distance_to_y_value, x_at_y_value_intercept) {
                Some(x) => return x,
                _ => (),
            }

            // hit empty space
            y_value += game_state.map.units_per_cell as f32 * if rise < 0.0 { -1.0 } else { 1.0 };
        } else {
            // since we are on the border of two cells we need to look up the next cell over, depending on if we are going right or left
            let adjustment = if run > 0.0 { 0.1 } else { -0.1 };
            let adjusted_value = x_value + adjustment;

            let row = y_at_x_value_intercept as u32 / game_state.map.units_per_cell;
            let col = adjusted_value as u32 / game_state.map.units_per_cell;

            match get_ray_result(game_state, row as usize, col as usize, distance_to_x_value, y_at_x_value_intercept) {
                Some(x) => return x,
                _ => (),
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

fn get_ray_result(game_state: &GameState, row: usize, col: usize, distance: f32, position: f32) -> Option<RayResult> {
    match game_state.map.cell_at(row, col) {
        None => None,
        Some(CellType::Empty) => None,
        Some(cell_type) => Some(RayResult {
            distance,
            units_from_cell_start: position % game_state.map.units_per_cell as f32,
            cell_type,
        }),
    }
}
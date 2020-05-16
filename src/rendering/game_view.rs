use sdl2::rect::{Rect, Point};
use sdl2::render::WindowCanvas;
use sdl2::pixels::Color;
use crate::game::GameState;
use crate::{SCREEN_HEIGHT, SCREEN_WIDTH, BRICK_WALL_SECTION, WOOD_WALL_SECTION, BLUE_WALL_SECTION};
use crate::rendering::{shoot_ray, FOV_DEGREES};
use crate::rendering::atlas::Atlas;
use crate::game::map::CellType;

pub fn render_game_view(canvas: &mut WindowCanvas, game_state: &GameState, wall_atlas: &Atlas) {
    canvas.set_draw_color(Color::GRAY);
    canvas.clear();

    canvas.set_draw_color(Color::WHITE);
    canvas.fill_rect(Rect::new(0, SCREEN_HEIGHT as i32 / 2, SCREEN_WIDTH, SCREEN_HEIGHT / 2)).unwrap();

    let first_ray_at = game_state.player.facing - FOV_DEGREES.to_radians() / 2.0;
    let ray_count = canvas.window().size().0;
    let radians_per_ray = FOV_DEGREES.to_radians() / ray_count as f32;

    let (section_width, section_height) = wall_atlas.get_section_width_and_height(BRICK_WALL_SECTION).unwrap();

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

        let mut height = SCREEN_HEIGHT as f32 / (adjusted_distance / 1.5);
        if height > SCREEN_HEIGHT as f32 {
            height = SCREEN_HEIGHT as f32;
        }

        let start_y = SCREEN_HEIGHT as f32 / 2.0 - height / 2.0;
        for y in (start_y as u32)..(start_y as u32 + height as u32) {
            let image_x = (ray.units_from_cell_start / game_state.map.units_per_cell as f32) * section_width as f32;
            let image_y = ((y - start_y as u32) as f32 / height) * section_height as f32;

            let section_name = match ray.cell_type {
                CellType::BrickWall => BRICK_WALL_SECTION,
                CellType::WoodWall => WOOD_WALL_SECTION,
                CellType::BlueWall => BLUE_WALL_SECTION,
                x => panic!("Can't handle cell type {:?}", x),
            };

            let rgb = wall_atlas.get_rgb_at(section_name, image_x as u32, image_y as u32).unwrap();

            canvas.set_draw_color(Color::RGB(rgb.0, rgb.1, rgb.2));
            canvas.draw_point(Point::new(x as i32, y as i32)).unwrap();
        }
    }
}
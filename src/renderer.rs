use sdl2::render::WindowCanvas;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use crate::game::map::CellType;
use crate::game::GameState;

pub fn render(canvas: &mut WindowCanvas, game_state: &GameState) {
    canvas.set_draw_color(Color::BLACK);
    canvas.clear();

    render_overhead_map(canvas, game_state);

    canvas.present();
}

fn render_overhead_map(canvas: &mut WindowCanvas, game_state: &GameState) {
    let zoom = game_state.map_zoom_level;

    for row in 0..game_state.map.height as i32 {
        for col in 0..game_state.map.width as i32 {
            let x1 = col * game_state.map.units_per_cell as i32 * zoom as i32;
            let y1 = row * game_state.map.units_per_cell as i32 * zoom as i32;
            let width = game_state.map.units_per_cell * zoom as u32;
            let height = game_state.map.units_per_cell * zoom as u32;

            let rect = Rect::new(x1, y1, width, height);
            let cell = game_state.map.cell_at(row as usize, col as usize);

            match &*cell {
                CellType::Wall => canvas.set_draw_color(Color::RED),
                CellType::Empty => canvas.set_draw_color(Color::WHITE)
            }

            canvas.fill_rect(rect).unwrap();
        }
    }

    {
        // Adjust the player's position based on the scale of the map
        let player_size = game_state.player.collision_size * zoom as u16;
        let pos_x = game_state.player.position.x as i32 * zoom as i32;
        let pos_y = game_state.player.position.y as i32 * zoom as i32;

        let x1 = pos_x - (player_size / 2) as i32;
        let y1 = pos_y - (player_size / 2) as i32;

        let rect = Rect::new(x1, y1, player_size as u32, player_size as u32);

        canvas.set_draw_color(Color::GREEN);
        canvas.fill_rect(rect).unwrap();
    }
}
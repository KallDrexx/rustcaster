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
    for row in 0..game_state.map.height as i32 {
        for col in 0..game_state.map.width as i32 {
            let x1 = col * game_state.map.units_per_cell as i32;
            let y1 = row * game_state.map.units_per_cell as i32;

            let rect = Rect::new(x1, y1, game_state.map.units_per_cell, game_state.map.units_per_cell);
            let cell = game_state.map.cell_at(row as usize, col as usize);

            match &*cell {
                CellType::Wall => canvas.set_draw_color(Color::RED),
                CellType::Empty => canvas.set_draw_color(Color::WHITE)
            }

            canvas.fill_rect(rect).unwrap();
        }
    }

    {
        let x1 = game_state.player.position.x as i32 - (game_state.player.collision_size as i32 / 2);
        let y1 = game_state.player.position.x as i32 - (game_state.player.collision_size as i32 / 2);
        let rect = Rect::new(x1, y1, game_state.player.collision_size as u32, game_state.player.collision_size as u32);

        canvas.set_draw_color(Color::GREEN);
        canvas.fill_rect(rect).unwrap();
    }
}
extern crate sdl2;
mod map;
mod core;
mod entities;

use std::time::Duration;
use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::render::{WindowCanvas};
use sdl2::rect::Rect;
use crate::map::{Map, CellType};
use crate::core::Vector;
use crate::entities::Player;

static SCREEN_WIDTH: u32 = 800;
static SCREEN_HEIGHT: u32 = 600;

pub fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem.window("Rust Raycaster", SCREEN_WIDTH, SCREEN_HEIGHT)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    let map = Map::new();

    // Start the player in the middle of the 2nd diagonal cell
    let pos_value = map.units_per_cell as f32 * 1.5_f32;
    let initial_pos = Vector {x: pos_value, y: pos_value};
    let player = Player::new(initial_pos);

    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut frame_count = 0_u32;
    'main_loop: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'main_loop
                },
                _ => {}
            }
        }

        render(&mut canvas, &map, &player);

        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
        frame_count = frame_count.wrapping_add(1_u32);
    }
}

fn render(canvas: &mut WindowCanvas, map: &Map, player: &Player) {
    canvas.set_draw_color(Color::BLACK);
    canvas.clear();

    for row in 0..map.height as i32 {
        for col in 0..map.width as i32 {
            let x1 = col * map.units_per_cell as i32;
            let y1 = row * map.units_per_cell as i32;

            let rect = Rect::new(x1, y1, map.units_per_cell, map.units_per_cell);
            let cell = map.cell_at(row as usize, col as usize);

            match &*cell {
                CellType::Wall => canvas.set_draw_color(Color::RED),
                CellType::Empty => canvas.set_draw_color(Color::WHITE)
            }

            canvas.fill_rect(rect).unwrap();
        }
    }

    {
        let x1 = player.position.x as i32 - (player.collision_size as i32 / 2);
        let y1 = player.position.x as i32 - (player.collision_size as i32 / 2);
        let rect = Rect::new(x1, y1, player.collision_size as u32, player.collision_size as u32);

        canvas.set_draw_color(Color::GREEN);
        canvas.fill_rect(rect).unwrap();
    }

    canvas.present();
}
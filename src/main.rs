extern crate sdl2;
mod core;
mod game;
mod renderer;

use std::time::{Instant};
use sdl2::event::Event;
use sdl2::keyboard::Keycode;

use crate::game::{GameState, ActiveInputs};

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
    let mut game_state = GameState::new();
    let mut frame_count = 0_u32;
    let mut last_frame_at = Instant::now();

    let mut event_pump = sdl_context.event_pump().unwrap();
    'main_loop: loop {
        let frame_start = Instant::now();
        let time_since_last_frame = frame_start - last_frame_at;

        let mut inputs = ActiveInputs::new();
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} => break 'main_loop,
                Event::KeyDown { keycode: Some(key), .. } => {
                    apply_key_to_inputs(&mut inputs, key);
                },
                _ => {}
            }
        }

        if inputs.exit_game {
            break;
        }

        game_state.tick(&time_since_last_frame, inputs);

        renderer::render(&mut canvas, &game_state);

        frame_count = frame_count.wrapping_add(1_u32);
        last_frame_at = frame_start;
    }
}

fn apply_key_to_inputs(inputs: &mut ActiveInputs, key: Keycode) {
    match key {
        Keycode::Escape => inputs.exit_game = true,
        Keycode::W => inputs.move_forward = true,
        Keycode::S => inputs.move_back = true,
        Keycode::A => inputs.turn_left = true,
        Keycode::D => inputs.turn_right = true,
        _ => (),
    }
}
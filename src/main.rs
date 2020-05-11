extern crate sdl2;
mod core;
mod game;
mod renderer;

use std::time::{Instant};
use sdl2::event::Event;
use sdl2::keyboard::{Keycode, KeyboardState, Scancode};

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
    let mut inputs = ActiveInputs::new();

    let mut event_pump = sdl_context.event_pump().unwrap();
    'main_loop: loop {
        let frame_start = Instant::now();
        let time_since_last_frame = frame_start - last_frame_at;

        inputs.zoom_out = false;
        inputs.zoom_in = false;

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} => break 'main_loop,
                Event::KeyUp { keycode: Some(key), .. } => {

                    apply_key_up_to_inputs(&mut inputs, key);
                }
                _ => {}
            }
        }

        process_inputs(&mut inputs, &event_pump.keyboard_state());

        if inputs.exit_game {
            break;
        }

        game_state.tick(&time_since_last_frame, &inputs);

        renderer::render(&mut canvas, &game_state);

        frame_count = frame_count.wrapping_add(1_u32);
        last_frame_at = frame_start;
    }
}

fn process_inputs(inputs: &mut ActiveInputs, keyboard_state: &KeyboardState) {
    inputs.exit_game = keyboard_state.is_scancode_pressed(Scancode::Escape);
    inputs.move_forward = keyboard_state.is_scancode_pressed(Scancode::W);
    inputs.move_back = keyboard_state.is_scancode_pressed(Scancode::S);
    inputs.turn_right = keyboard_state.is_scancode_pressed(Scancode::D);
    inputs.turn_left = keyboard_state.is_scancode_pressed(Scancode::A);

}

fn apply_key_up_to_inputs(inputs: &mut ActiveInputs, key: Keycode) {
    match key {
        Keycode::KpPlus => inputs.zoom_in = true,
        Keycode::KpMinus => inputs.zoom_out = true,
        _ => (),
    }
}
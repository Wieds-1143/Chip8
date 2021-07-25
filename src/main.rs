mod interperter;
mod render;

use crate::interperter::*;
//use crate::render::*;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
//use std::thread::sleep;
use std::time::{Duration, Instant};

fn main() {
    let screen_width = 64;
    let screen_height = 32;

    // load game
    let mut chip8 = Chip8::init();
    &chip8.load_game("games\\snake.ch8".to_string());

    //setup window
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let window = video_subsystem
        .window(
            "Chip8",
            (screen_width as f32 * 10.0) as u32,
            (screen_height as f32 * 10.0) as u32,
        )
        .position_centered()
        .build()
        .unwrap();
    let mut canvas = window.into_canvas().present_vsync().build().unwrap();
    canvas.set_scale(10.0, 10.0).unwrap();

    let mut pixels = render::make_pixels(screen_width, screen_height);
    render::color_pixels(&[0; 32 * 64], &mut pixels, &mut canvas);
    let run_speed = 16666666; // normal run speed of the emulator
    let slow: u32 = 1000000000;
    let mut speed: u32 = run_speed; // used to speed up or slow down the run time to take a look at the opcode

    'emulator_loop: loop {
        let time = Instant::now();
        let mut opcode_count = 0;
        while (Instant::now() - time) < Duration::from_millis(16) {
            let next_opcode = &chip8.fetch_opcode();
            let decoded_opcode = decode_opcode(*next_opcode);

            if decoded_opcode != None {
                &chip8.execute_opcode(decoded_opcode.unwrap());
            }

            let mut event_pump = sdl_context.event_pump().unwrap();
            for event in event_pump.poll_iter() {
                match event {
                    Event::Quit { .. } => break 'emulator_loop,
                    Event::KeyDown { keycode: x, .. } => match x {
                        Some(Keycode::P) => {
                            if speed == slow {
                                speed = run_speed;
                            } else {
                                speed = slow
                            }
                        }
                        Some(Keycode::Escape) => break 'emulator_loop,
                        _ => chip8.check_key_state(event),
                    },

                    Event::KeyUp { keycode: x, .. } => {
                        chip8.check_key_state(event);
                    }
                    _ => {}
                }
            }
            opcode_count += 1;
            if opcode_count == 9 {
                break;
            }
        }

        &canvas.clear();
        render::color_pixels(&chip8.gfx, &mut pixels, &mut canvas);
        canvas.present();

        chip8.decrease_timers();
    }
}

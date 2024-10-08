use sdl2::keyboard::Keycode;
use sdl2::keyboard::Scancode;
use sdl2::event::Event;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use std::time::Duration;

pub fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let window = video_subsystem
        .window("Rust-SDL2", 800, 600)
        .position_centered()
        .build()
        .unwrap();
    let mut canvas = window.into_canvas().build().unwrap();
    let mut event_pump = sdl_context.event_pump().unwrap();
    let (mut x, mut y) = (350, 250);

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                _ => {}
            }
        }
        let state = event_pump.keyboard_state();
        if state.is_scancode_pressed(Scancode::Up) {
            y -= 5;
        }
        if state.is_scancode_pressed(Scancode::Down) {
            y += 5;
        }
        if state.is_scancode_pressed(Scancode::Left) {
            x -= 5;
        }
        if state.is_scancode_pressed(Scancode::Right) {
            x += 5;
        }

        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();

        canvas.set_draw_color(Color::RGB(255, 255, 255));
        //canvas.fill_rect(Rect::new(350, 250, 100, 100)).unwrap();
        canvas.fill_rect(Rect::new(x, y, 100, 100)).unwrap();

        canvas.present();
        std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}

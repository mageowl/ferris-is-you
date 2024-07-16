use gfx::Assets;
use input::Input;
use macroquad::{
    color::Color,
    time::get_frame_time,
    window::{clear_background, next_frame, Conf},
};

use crate::frame::Frame;

mod frame;
mod gfx;
mod input;
mod math;

fn window_config() -> Conf {
    Conf {
        window_title: String::from("Ferris is You"),
        window_width: 1440,
        window_height: 960,
        ..Default::default()
    }
}

#[macroquad::main(window_config)]
async fn main() {
    let mut frame = Frame::from_file("assets/levels/test.dat");
    let mut input_timer = 0.0;

    let assets = Assets::load().await;

    loop {
        clear_background(Color::from_rgba(0, 0, 0, 255));

        if input_timer <= 0.0 {
            let input = Input::new();
            if input.should_step() {
                println!("input");
                frame = frame.step(input);
                input_timer = input::INPUT_DELAY;
            } else if input.undo && frame.prev.is_some() {
                frame = *frame.prev.unwrap();
                input_timer = input::INPUT_DELAY;
            }
        } else {
            input_timer -= get_frame_time();
        }

        frame.draw(&assets);

        next_frame().await
    }
}

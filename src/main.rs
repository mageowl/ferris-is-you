use input::Input;
use macroquad::{
    color::Color,
    text::draw_text,
    time::get_frame_time,
    window::{clear_background, next_frame, Conf},
};

use crate::frame::Frame;

mod frame;
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

    loop {
        clear_background(Color::from_rgba(0, 0, 0, 255));
        draw_text(
            "Hello World",
            10.0,
            20.0,
            24.0,
            Color::from_rgba(255, 255, 255, 255),
        );

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

        next_frame().await
    }
}

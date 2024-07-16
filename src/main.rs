use gfx::Assets;
use input::Input;
use macroquad::{
    color::Color,
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
    let mut frame = Frame::from_file("assets/levels/baba_00.dat");
    let assets = Assets::load().await;

    loop {
        clear_background(Color::from_rgba(0, 0, 0, 255));

        let input = Input::new();
        if input.should_step() {
            frame = frame.step(input);
        } else if input.undo && frame.prev.is_some() {
            frame = *frame.prev.unwrap();
        } else if input.restart {
            frame = frame.get_oldest();
        }

        frame.draw(&assets);

        next_frame().await
    }
}

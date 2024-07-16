use frame::GameState;
use gfx::Assets;
use input::Input;
use macroquad::{
    color::Color,
    text::{draw_text_ex, load_ttf_font, measure_text, TextParams},
    texture::FilterMode,
    window::{clear_background, next_frame, screen_height, screen_width, Conf},
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

        if *frame.state.borrow() == GameState::Playing {
            let input = Input::new();
            if input.should_step() {
                frame = frame.step(input);
            } else if input.undo && frame.prev.is_some() {
                frame = *frame.prev.unwrap();
            } else if input.restart {
                frame = frame.get_oldest();
            }
        } else {
            const WIN_MESSAGE: &'static str = "CONGRADULATIONS";
            const FONT_SIZE: u16 = 30;
            let size = measure_text(WIN_MESSAGE, Some(&assets.font), FONT_SIZE, 1.0);
            draw_text_ex(
                WIN_MESSAGE,
                screen_width() / 2.0 - size.width / 2.0,
                screen_height() / 2.0 - size.height / 2.0,
                TextParams {
                    font: Some(&assets.font),
                    font_size: FONT_SIZE,
                    ..Default::default()
                },
            )
        }

        frame.draw(&assets);

        next_frame().await
    }
}

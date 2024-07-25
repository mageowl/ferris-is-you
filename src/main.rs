use frame::GameState;
use gfx::Assets;
use input::Input;
use level_select::get_level;
use macroquad::{
    color::Color,
    text::{draw_text_ex, measure_text, TextParams},
    time::get_frame_time,
    window::{clear_background, next_frame, screen_height, screen_width, Conf},
};

use crate::frame::Frame;

mod frame;
mod gfx;
mod input;
mod level_select;
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
    let assets = Assets::load().await;

    let mut frame = Frame::from_file(&get_level(&assets).await);
    let mut win_timer = 1.;

    while win_timer > 0. {
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
        }

        frame.draw(&assets);

        if *frame.state.borrow() == GameState::Win {
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
            );

            win_timer -= get_frame_time();
        }

        next_frame().await
    }
}

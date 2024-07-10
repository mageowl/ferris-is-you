use macroquad::{
    color::Color,
    text::draw_text,
    window::{clear_background, next_frame},
};

#[macroquad::main("Ferris is You")]
async fn main() {
    loop {
        clear_background(Color::from_rgba(0, 0, 0, 255));
        draw_text(
            "Hello World",
            10.0,
            20.0,
            24.0,
            Color::from_rgba(255, 255, 255, 255),
        );

        next_frame().await
    }
}

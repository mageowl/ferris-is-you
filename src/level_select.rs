use std::{ffi::OsStr, fs};

use macroquad::{
    color::{Color, WHITE},
    input::{is_key_down, KeyCode},
    math::vec2,
    text::draw_text_ex,
    ui::{root_ui, widgets, Skin},
    window::{clear_background, next_frame},
};

use crate::gfx::Assets;

pub async fn get_level(assets: &Assets) -> Vec<u8> {
    let files = {
        let mut files = Vec::new();

        for file in fs::read_dir("assets/levels").expect("No levels found.") {
            let file = file.unwrap().path();
            if file.extension() == Some(OsStr::new("dat")) {
                files.push(file)
            }
        }

        files
    };
    let mut selected = None;

    let label_style = root_ui()
        .style_builder()
        .font(include_bytes!("../assets/jumbledFont.ttf"))
        .unwrap()
        .font_size(20)
        .text_color(WHITE)
        .build();
    let skin = &Skin {
        label_style,
        ..root_ui().default_skin()
    };

    root_ui().push_skin(&skin);

    while selected.is_none() {
        clear_background(Color::from_rgba(0, 0, 0, 255));

        root_ui().label(None, "select a level");

        for file in &files {
            if root_ui().button(None, file.to_str().unwrap()) {
                selected = Some(file);
            }
        }

        next_frame().await
    }

    fs::read(selected.unwrap()).expect("level not found.")
}

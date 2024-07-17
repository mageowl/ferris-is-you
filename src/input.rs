use macroquad::input::{is_key_pressed, KeyCode};

use crate::math::Direction;

#[derive(Clone, Copy)]
pub struct Input {
    pub direction: Option<Direction>,
    pub undo: bool,
    pub wait: bool,
    pub restart: bool,
}

impl Input {
    pub fn new() -> Self {
        Self {
            direction: if is_key_pressed(KeyCode::Left) {
                Some(Direction::Left)
            } else if is_key_pressed(KeyCode::Right) {
                Some(Direction::Right)
            } else if is_key_pressed(KeyCode::Up) {
                Some(Direction::Up)
            } else if is_key_pressed(KeyCode::Down) {
                Some(Direction::Down)
            } else {
                None
            },
            undo: is_key_pressed(KeyCode::Backspace) || is_key_pressed(KeyCode::Z),
            wait: is_key_pressed(KeyCode::Space),
            restart: is_key_pressed(KeyCode::R),
        }
    }

    pub fn should_step(&self) -> bool {
        self.direction.is_some() || self.wait
    }
}

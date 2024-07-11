use macroquad::input::{is_key_down, KeyCode};

use crate::math::Direction;

pub const INPUT_DELAY: f32 = 0.5;

#[derive(Clone, Copy)]
pub struct Input {
    pub direction: Option<Direction>,
    pub undo: bool,
    pub wait: bool,
}

impl Input {
    pub fn new() -> Self {
        Self {
            direction: if is_key_down(KeyCode::Left) {
                Some(Direction::Left)
            } else if is_key_down(KeyCode::Right) {
                Some(Direction::Right)
            } else if is_key_down(KeyCode::Up) {
                Some(Direction::Up)
            } else if is_key_down(KeyCode::Down) {
                Some(Direction::Down)
            } else {
                None
            },
            undo: is_key_down(KeyCode::Backspace),
            wait: is_key_down(KeyCode::Space),
        }
    }

    pub fn should_step(&self) -> bool {
        self.direction.is_some() || self.wait
    }
}

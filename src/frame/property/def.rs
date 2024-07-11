use crate::{
    frame::{object::Object, Frame, GameState},
    math::Direction,
};

use super::Property;

#[derive(PartialEq, Eq)]
pub struct Stop;

impl Property for Stop {
    fn can_move_onto(&self, _f: &mut Frame, _o: &mut Object, _d: Direction) -> bool {
        return false;
    }
}

#[derive(PartialEq, Eq)]
pub struct Push;

impl Property for Push {
    fn can_move_onto(&self, frame: &mut Frame, object: &mut Object, direction: Direction) -> bool {
        return frame.try_move(object, direction);
    }
}

#[derive(PartialEq, Eq)]
pub struct You;

impl Property for You {
    fn on_step(&self, frame: &mut Frame, object: &mut Object) {
        if let Some(direction) = frame.get_input_direction() {
            frame.try_move(object, direction);
        }
    }
}

#[derive(PartialEq, Eq)]
pub struct Move;

impl Property for Move {
    fn on_step(&self, frame: &mut Frame, object: &mut Object) {
        frame.try_move(object, object.facing);
    }
}

#[derive(PartialEq, Eq)]
pub struct Win;

impl Property for Win {
    fn on_step_end(&self, frame: &mut Frame, object: &Object) {
        let mut should_win = false;
        for object in frame.get_overlapping(object) {
            if frame.has_property(object, You) {
                should_win = true;
                break;
            }
        }

        if should_win {
            frame.state = GameState::Win;
        }
    }
}

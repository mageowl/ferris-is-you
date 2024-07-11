use crate::{
    frame::{Frame, GameState, ObjectRef},
    math::Direction,
};

use super::Property;

pub struct Stop;

impl Property for Stop {
    fn is_solid(&self) -> bool {
        true
    }
}

pub struct Push;

impl Property for Push {
    fn is_solid(&self) -> bool {
        true
    }

    fn can_move_onto(
        &self,
        frame: &Frame,
        object: ObjectRef,
        _m: ObjectRef,
        direction: Direction,
    ) -> bool {
        frame.try_move(object, direction)
    }
}

pub struct You;

impl Property for You {
    fn on_step(&self, frame: &Frame, object: ObjectRef) {
        if let Some(direction) = frame.get_input_direction() {
            frame.try_move(object, direction);
        }
    }
}

pub struct Move;

impl Property for Move {
    fn on_step(&self, frame: &Frame, object: ObjectRef) {
        frame.try_move(object, frame[object].facing);
    }
}

pub struct Win;

impl Property for Win {
    fn on_step_end(&self, frame: &Frame, object: ObjectRef) {
        let mut should_win = false;
        for object in frame.get_overlapping(object) {
            if frame.has_property(object, You) {
                should_win = true;
                break;
            }
        }

        if should_win {
            frame.set_state(GameState::Win);
        }
    }
}

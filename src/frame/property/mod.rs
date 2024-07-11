use std::any::Any;

use super::{object::Object, Frame};
use crate::math::Direction;

mod def;

#[allow(unused_variables)]
pub trait Property: Any {
    fn can_move_onto(&self, frame: &mut Frame, object: &mut Object, direction: Direction) -> bool {
        true
    }

    fn can_move(&self, frame: &mut Frame, object: &mut Object, direction: Direction) -> bool {
        true
    }

    fn on_move_onto(&self, frame: &mut Frame, object: &mut Object, mover: &Object) {}
    fn on_step(&self, frame: &mut Frame, object: &mut Object) {}
    fn on_step_end(&self, frame: &mut Frame, object: &Object) {}
}

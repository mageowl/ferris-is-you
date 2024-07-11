use std::any::Any;

use super::{Frame, ObjectRef};
use crate::math::Direction;

mod def;

#[allow(unused_variables)]
pub trait Property: Any {
    fn is_solid(&self) -> bool {
        false
    }

    fn can_move_onto(
        &self,
        frame: &Frame,
        object: ObjectRef,
        mover: ObjectRef,
        direction: Direction,
    ) -> bool {
        false
    }
    fn can_move(&self, frame: &Frame, object: ObjectRef, direction: Direction) -> bool {
        true
    }

    fn on_move_onto(&self, frame: &Frame, object: ObjectRef, mover: ObjectRef) {}
    fn on_step(&self, frame: &Frame, object: ObjectRef) {}
    fn on_step_end(&self, frame: &Frame, object: ObjectRef) {}
}

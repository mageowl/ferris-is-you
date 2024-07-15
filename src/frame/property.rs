use crate::{frame::GameState, math::Direction};

use super::{Frame, ObjectRef};

macro_rules! inline_fn{
    [($($i:ident: $t:ty),*) $b:block] => {{
        fn temp($($i: $t),*) {$b}
        Some(temp)
    }};
    [($($i:ident: $t:ty),*) -> $r:ty $b:block] => {{
        fn temp($($i: $t),*) -> $r {$b}
        Some(temp)
    }};
}

pub type Id = u64;

#[derive(Clone, Copy, Default)]
pub struct Property {
    pub id: Id,
    pub on_step: Option<fn(&Frame, ObjectRef)>,
    pub can_move_onto: Option<fn(&Frame, ObjectRef, ObjectRef, Direction) -> bool>,
    pub on_move_onto: Option<fn(&Frame, ObjectRef, ObjectRef)>,
}

impl Property {
    const YOU: Id = 01;
    const PUSH: Id = 02;
    const STOP: Id = 03;
    const WIN: Id = 04;

    pub fn get(id: u64) -> Option<Self> {
        match id {
            Property::YOU => Some(Self {
                id,
                on_step: inline_fn! [(frame: &Frame, object: ObjectRef) {
                    if let Some(direction) = frame.get_input_direction() {
                        frame.try_move(object, direction);
                    }
                }],
                ..Default::default()
            }),
            Property::PUSH => Some(Self {
                id,
                can_move_onto: inline_fn! [(frame: &Frame, object: ObjectRef, _mover: ObjectRef, direction: Direction) -> bool {
                    frame.try_move(object, direction)
                }],
                ..Default::default()
            }),
            Property::STOP => Some(Self {
                id,
                can_move_onto: inline_fn! [(_f: &Frame, _o: ObjectRef, _m: ObjectRef, _d: Direction) -> bool {
                    false
                }],
                ..Default::default()
            }),
            Property::WIN => Some(Self {
                id,
                on_move_onto: inline_fn! [(frame: &Frame, _o: ObjectRef, mover: ObjectRef) {
                    if frame.has_property(frame.get_object(mover).id(), Property::YOU) {
                        frame.set_state(GameState::Win)
                    }
                }],
                ..Default::default()
            }),
            _ => None,
        }
    }

    pub fn on_step(&self, frame: &Frame, object: ObjectRef) {
        if let Some(cb) = self.on_step {
            cb(frame, object)
        }
    }
}

impl PartialEq for Property {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl PartialEq<Id> for Property {
    fn eq(&self, other: &Id) -> bool {
        self.id == *other
    }
}

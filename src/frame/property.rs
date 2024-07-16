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
    pub priority: isize,
    pub on_step: Option<fn(&Frame, ObjectRef)>,
    pub on_step_end: Option<fn(&Frame, ObjectRef)>,
    pub can_move_onto: Option<fn(&Frame, ObjectRef, ObjectRef, Direction) -> bool>,
}

impl Property {
    pub const YOU: Id = 01;
    pub const PUSH: Id = 02;
    pub const STOP: Id = 03;
    pub const WIN: Id = 04;

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
                priority: 5,
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
                on_step_end: inline_fn! [(frame: &Frame, object: ObjectRef) {
                    for object in frame.get_overlapping(object) {
                        if frame.has_property(object.id(), Property::YOU) {
                            *frame.state.borrow_mut() = GameState::Win;
                        }
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

    pub fn on_step_end(&self, frame: &Frame, object: ObjectRef) {
        if let Some(cb) = self.on_step_end {
            cb(frame, object)
        }
    }
}

impl PartialEq for Property {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for Property {}

impl PartialEq<Id> for Property {
    fn eq(&self, other: &Id) -> bool {
        self.id == *other
    }
}

impl Ord for Property {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.priority.cmp(&other.priority)
    }
}

impl PartialOrd for Property {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

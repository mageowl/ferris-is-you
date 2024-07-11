use std::any::TypeId;

use crate::math::{Direction, UPt};

pub type ObjectId = u64;

#[derive(Clone, Copy)]
pub enum ObjectClass {
    Generic(ObjectId),
    TextProperty(TypeId),
    TextIs,
    TextNoun(ObjectId),
}

#[derive(Clone, Copy)]
pub struct Object {
    pub class: ObjectClass,
    pub pos: UPt,
    pub facing: Direction,
}

impl Object {
    pub fn id(&self) -> ObjectId {
        match self.class {
            ObjectClass::Generic(id) => id,
            _ => OBJ_TEXT,
        }
    }
}

const OBJ_TEXT: ObjectId = 0;
const OBJ_FERRIS: ObjectId = 1;
const OBJ_WALL: ObjectId = 2;

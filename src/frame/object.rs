use crate::math::{Direction, UPt};

use super::property;

pub type Id = u64;

#[derive(Clone, Copy)]
pub enum ObjectClass {
    Generic(Id),
    TextProperty(property::Id),
    TextIs,
    TextNoun(Id),
}

#[derive(Clone, Copy)]
pub struct Object {
    pub class: ObjectClass,
    pub pos: UPt,
    pub facing: Direction,
}

impl Object {
    pub fn id(&self) -> Id {
        match self.class {
            ObjectClass::Generic(id) => id,
            _ => OBJ_TEXT,
        }
    }
}

const OBJ_TEXT: Id = 0;
const OBJ_FERRIS: Id = 1;
const OBJ_WALL: Id = 2;

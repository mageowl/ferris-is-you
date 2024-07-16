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
    pub const TEXT: Id = 00;
    pub const FERRIS: Id = 01;
    pub const WALL: Id = 02;
    pub const FLAG: Id = 03;

    pub fn id(&self) -> Id {
        match self.class {
            ObjectClass::Generic(id) => id,
            _ => Self::TEXT,
        }
    }
}

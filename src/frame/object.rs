use crate::math::{Direction, UPt};

use super::property;

pub type Id = u64;

#[derive(Clone, Copy, Debug)]
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
    pub const TILE: Id = 04;
    pub const ROCK: Id = 05;

    pub fn id(&self) -> Id {
        match self.class {
            ObjectClass::Generic(id) => id,
            _ => Self::TEXT,
        }
    }

    pub fn z_index(&self) -> Option<i8> {
        match self.id() {
            Object::TEXT => Some(15),
            Object::FERRIS => Some(10),
            Object::ROCK => Some(8),
            Object::WALL => Some(3),
            Object::FLAG => Some(5),
            Object::TILE => Some(0),
            _ => None,
        }
    }
}

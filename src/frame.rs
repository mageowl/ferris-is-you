use std::{array, collections::HashMap};

use object::{Object, ObjectId};
use property::Property;

use crate::{input::Input, math::Direction};

mod object;
mod property;

#[derive(Clone)]
pub struct Tile {
    objects: Vec<Object>,
}

impl Tile {
    fn empty() -> Self {
        Self {
            objects: Vec::new(),
        }
    }

    fn is_empty(&self) -> bool {
        self.objects.is_empty()
    }
}

impl<'a> IntoIterator for &'a Tile {
    type Item = &'a Object;
    type IntoIter = std::slice::Iter<'a, Object>;

    fn into_iter(self) -> Self::IntoIter {
        self.objects.iter()
    }
}

pub enum GameState {
    Playing,
    Win,
    Lose,
}

pub struct Frame {
    grid: [[Tile; 40]; 60],
    input: Option<Input>,
    rules: HashMap<ObjectId, Vec<Box<dyn Property>>>,
    pub state: GameState,
    pub prev: Option<Box<Frame>>,
}

impl Frame {
    pub fn from_file(_path: &str) -> Self {
        Self {
            grid: array::from_fn(|_| array::from_fn(|_| Tile::empty())),
            input: None,
            rules: HashMap::new(),
            state: GameState::Playing,
            prev: None,
        }
    }

    pub fn step(self) -> Self {
        for col in &self.grid {
            for tile in col {
                if tile.is_empty() {}
            }
        }

        self
    }

    pub fn try_move(&mut self, object: &mut Object, direction: Direction) -> bool {
        false
    }

    pub fn has_property(&self, object: &Object, property: impl Property) -> bool {
        self.rules
            .get(&object.id())
            .is_some_and(|vec| vec.iter().any(|p| property.type_id() == p.type_id()))
    }

    pub fn get_overlapping(&self, object: &Object) -> &Tile {
        &self.grid[object.pos.0][object.pos.1]
    }

    pub fn get_input_direction(&self) -> Option<Direction> {
        self.input.map(|i| i.direction).unwrap_or(None)
    }
}

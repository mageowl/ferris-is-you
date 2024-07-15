use std::{array, cell::RefCell, collections::HashMap, ops::Index};

use object::Object;
use property::Property;

use crate::{
    input::Input,
    math::{Direction, Pt, UPt},
};

mod object;
mod property;

pub type ObjectRef = (UPt, u8);

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

    fn add(&mut self, object: Object) {
        self.objects.push(object)
    }

    fn remove(&mut self, index: u8) -> Object {
        self.objects.remove(index as usize)
    }
}

impl<'a> IntoIterator for &'a Tile {
    type Item = &'a Object;
    type IntoIter = std::slice::Iter<'a, Object>;

    fn into_iter(self) -> Self::IntoIter {
        self.objects.iter()
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum GameState {
    Playing,
    Win,
    Lose,
}

pub struct Frame {
    grid: [[Tile; 40]; 60],
    input: Option<Input>,
    rules: HashMap<object::Id, Vec<Property>>,
    pub state: GameState,
    pub prev: Option<Box<Frame>>,
    next: Option<Box<RefCell<Frame>>>,
}

impl Frame {
    pub fn from_file(_path: &str) -> Self {
        let s = Self {
            grid: array::from_fn(|_| array::from_fn(|_| Tile::empty())),
            input: None,
            rules: HashMap::new(),
            state: GameState::Playing,
            prev: None,
            next: None,
        };

        s
    }

    pub fn step(mut self, input: Input) -> Self {
        self.next = Some(Box::new(RefCell::new(Self {
            input: Some(input),
            grid: self.grid.clone(),
            rules: HashMap::new(),
            state: self.state,
            prev: None,
            next: None,
        })));

        for col in &self.grid {
            for tile in col {
                for (i, object) in tile.into_iter().enumerate() {
                    let Some(properties) = self.rules.get(&object.id()) else {
                        continue;
                    };
                    for property in properties {
                        property.on_step(&self, (object.pos, i as u8))
                    }
                }
            }
        }

        let mut next = self.next.take().unwrap().into_inner();
        next.prev = Some(Box::new(self));
        if next.state == GameState::Playing {
            next.compile_rules();
        }

        next
    }

    pub fn try_move(&self, mover: ObjectRef, direction: Direction) -> bool {
        let pos = (
            (mover.0 .0 as isize + Into::<Pt>::into(direction).0).clamp(0, 60) as usize,
            (mover.0 .1 as isize + Into::<Pt>::into(direction).1).clamp(0, 40) as usize,
        );
        let target = &self.grid[pos.0][pos.1];

        let mut can_move = true;
        for (i, object) in target.into_iter().enumerate() {
            let Some(properties) = self.rules.get(&object.id()) else {
                continue;
            };

            for property in properties {
                if let Some(cb) = property.can_move_onto {
                    can_move = cb(self, (object.pos, i as u8), mover, direction);
                }
            }
        }

        if can_move {
            let mut object = self.next.as_ref().unwrap().borrow_mut().grid[mover.0 .0][mover.0 .1]
                .remove(mover.1);
            object.pos = pos;
            self.next.as_ref().unwrap().borrow_mut().grid
                [(mover.0 .0 as isize + Into::<Pt>::into(direction).0).clamp(0, 60) as usize]
                [(mover.0 .1 as isize + Into::<Pt>::into(direction).1).clamp(0, 40) as usize]
                .add(object);
        }

        can_move
    }

    pub fn set_state(&self, state: GameState) {
        self.next.as_ref().unwrap().borrow_mut().state = state;
    }

    pub fn get_object(&self, object: ObjectRef) -> &Object {
        &self.grid[object.0 .0][object.0 .1].objects[object.1 as usize]
    }

    pub fn has_property(&self, object: object::Id, property: property::Id) -> bool {
        self.rules
            .get(&object)
            .is_some_and(|vec| vec.iter().any(|p| *p == property))
    }

    pub fn get_overlapping(&self, object: ObjectRef) -> &Tile {
        &self.grid[object.0 .0][object.0 .1]
    }

    pub fn get_input_direction(&self) -> Option<Direction> {
        self.input.map(|i| i.direction).unwrap_or(None)
    }

    fn compile_rules(&mut self) {}
}

impl Index<ObjectRef> for Frame {
    type Output = Object;

    fn index(&self, index: ObjectRef) -> &Self::Output {
        &self.grid[index.0 .0][index.0 .1].objects[index.1 as usize]
    }
}

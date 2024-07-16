use std::{
    cell::RefCell,
    collections::HashMap,
    fs::File,
    io::{BufReader, Read},
    ops::Index,
};

use grid::{Grid, ObjectRef, Tile};
use object::{Object, ObjectClass};
use property::Property;

use crate::{
    input::Input,
    math::{upt, Direction},
};

pub mod grid;
pub mod object;
pub mod property;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum GameState {
    Playing,
    Win,
}

pub struct Frame {
    pub grid: Grid,
    input: Option<Input>,
    rules: HashMap<object::Id, Vec<Property>>,
    pub state: RefCell<GameState>,
    pub prev: Option<Box<Frame>>,
    next: Option<Box<RefCell<Frame>>>,
}

impl Frame {
    pub fn from_file(path: &str) -> Self {
        let mut s = Self {
            grid: Grid::empty(),
            input: None,
            rules: HashMap::new(),
            state: RefCell::new(GameState::Playing),
            prev: None,
            next: None,
        };
        let mut file = BufReader::new(File::open(path).unwrap());

        for x in 0..60 {
            for y in 0..40 {
                let mut class = [0; 1];
                file.read_exact(&mut class).unwrap();
                dbg!(&class);

                let mut id = [0; 8];
                file.read_exact(&mut id).unwrap();
                let id = u64::from_be_bytes(id);
                dbg!(&id);

                let class = match class[0] {
                    0 => ObjectClass::Generic(id),
                    1 => ObjectClass::TextNoun(id),
                    2 => ObjectClass::TextIs,
                    3 => ObjectClass::TextProperty(id),

                    _ => panic!("Error reading level: unknown object class {}.", class[0]),
                };

                let object = Object {
                    class,
                    pos: upt(x, y),
                    facing: Direction::Down,
                };

                s.grid[object.pos].add(object);
            }
        }

        s
    }

    pub fn step(mut self, input: Input) -> Self {
        self.next = Some(Box::new(RefCell::new(Self {
            input: Some(input),
            grid: self.grid.clone(),
            rules: HashMap::new(),
            state: self.state.clone(),
            prev: None,
            next: None,
        })));

        for tile in &self.grid {
            for (i, object) in tile.into_iter().enumerate() {
                let Some(properties) = self.rules.get(&object.id()) else {
                    continue;
                };
                let obj_ref = (object.pos, i as u8);

                for property in properties {
                    property.on_step(&self, obj_ref);
                }
            }
        }

        if let Some(ref next) = self.next {
            let mut next = next.borrow_mut();

            for tile in &next.grid.clone() {
                for (i, object) in tile.into_iter().enumerate() {
                    let Some(properties) = self.rules.get(&object.id()) else {
                        continue;
                    };
                    let obj_ref = (object.pos, i as u8);

                    for property in properties {
                        property.on_step_end(&mut next, obj_ref);
                    }
                }
            }
        }

        let mut next = self.next.take().unwrap().into_inner();
        next.prev = Some(Box::new(self));
        if *next.state.borrow() == GameState::Playing {
            next.compile_rules();
        }

        next
    }

    pub fn try_move(&self, mover: ObjectRef, direction: Direction) -> bool {
        if let None = self.next {
            return false;
        }

        let pos = mover.0 + direction;
        let target = &self.grid[pos];

        let mut can_move = true;
        for (i, object) in target.into_iter().enumerate() {
            let Some(properties) = self.rules.get(&object.id()) else {
                continue;
            };

            for property in properties {
                if let Some(cb) = property.can_move_onto {
                    can_move &= cb(self, (object.pos, i as u8), mover, direction);
                    break;
                }
            }
        }

        if can_move {
            let mut object = self.next.as_ref().unwrap().borrow_mut().grid[mover.0].remove(mover.1);
            object.pos = pos;
            self.next.as_ref().unwrap().borrow_mut().grid[pos].add(object);
        }

        can_move
    }

    pub fn has_property(&self, object: object::Id, property: property::Id) -> bool {
        self.rules
            .get(&object)
            .is_some_and(|vec| vec.iter().any(|p| *p == property))
    }

    pub fn get_overlapping(&self, object: ObjectRef) -> &Tile {
        &self.grid[object.0]
    }

    pub fn get_input_direction(&self) -> Option<Direction> {
        self.input.map(|i| i.direction).unwrap_or(None)
    }

    fn compile_rules(&mut self) {
        let mut rules: HashMap<u64, Vec<Property>> = HashMap::new();

        for tile in &self.grid {
            for object_is in tile {
                if let ObjectClass::TextIs = object_is.class {
                    let mut check_direction = |d: Direction| {
                        for object_noun in &self.grid[object_is.pos + d] {
                            if let ObjectClass::TextNoun(noun_id) = object_noun.class {
                                for object_prop in &self.grid[object_is.pos + d.opposite()] {
                                    if let ObjectClass::TextProperty(property_id) =
                                        object_prop.class
                                    {
                                        if let Some(vec) = rules.get_mut(&noun_id) {
                                            vec.push(
                                                Property::get(property_id)
                                                    .expect("Unknown property."),
                                            );
                                        } else {
                                            rules.insert(
                                                noun_id,
                                                vec![Property::get(property_id)
                                                    .expect("Unknown property.")],
                                            );
                                        }
                                    }
                                }
                            }
                        }
                    };

                    check_direction(Direction::Up);
                    check_direction(Direction::Left);
                }
            }
        }

        rules.iter_mut().for_each(|(_, v)| v.sort());
        self.rules = rules;
    }
}

impl Index<ObjectRef> for Frame {
    type Output = Object;

    fn index(&self, index: ObjectRef) -> &Self::Output {
        &self.grid[index.0][index.1]
    }
}

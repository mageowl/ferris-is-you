use std::{
    array,
    ops::{Index, IndexMut},
};

use crate::math::{upt, UPt};

use super::object::Object;

pub type ObjectRef = (UPt, u8);

#[derive(Clone)]
pub struct Tile {
    objects: Vec<Object>,
}

impl Tile {
    pub fn empty() -> Self {
        Self {
            objects: Vec::new(),
        }
    }

    pub fn add(&mut self, object: Object) {
        self.objects.push(object)
    }

    pub fn remove(&mut self, index: u8) -> Object {
        self.objects.remove(index as usize)
    }

    pub fn is_empty(&self) -> bool {
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

impl<'a> IntoIterator for &'a mut Tile {
    type Item = &'a mut Object;
    type IntoIter = std::slice::IterMut<'a, Object>;

    fn into_iter(self) -> Self::IntoIter {
        self.objects.iter_mut()
    }
}

impl Index<u8> for Tile {
    type Output = Object;

    fn index(&self, index: u8) -> &Self::Output {
        &self.objects[index as usize]
    }
}

#[derive(Clone)]
pub struct Grid {
    inner: [[Tile; 20]; 30],
}

impl Grid {
    pub fn empty() -> Self {
        Self {
            inner: array::from_fn(|_| array::from_fn(|_| Tile::empty())),
        }
    }
}

impl<'a> IntoIterator for &'a Grid {
    type Item = &'a Tile;
    type IntoIter = GridIter<'a>;

    fn into_iter(self) -> Self::IntoIter {
        GridIter {
            i: upt(0, 0),
            grid: self,
        }
    }
}

impl Index<UPt> for Grid {
    type Output = Tile;

    fn index(&self, index: UPt) -> &Self::Output {
        &self.inner[index.x][index.y]
    }
}

impl IndexMut<UPt> for Grid {
    fn index_mut(&mut self, index: UPt) -> &mut Self::Output {
        &mut self.inner[index.x][index.y]
    }
}

pub struct GridIter<'a> {
    i: UPt,
    grid: &'a Grid,
}

impl<'a> Iterator for GridIter<'a> {
    type Item = &'a Tile;

    fn next(&mut self) -> Option<Self::Item> {
        let item = if self.i.x < 30 && self.i.y < 20 {
            Some(&self.grid[self.i])
        } else {
            None
        };

        self.i.x += 1;
        if self.i.x >= 30 {
            self.i.x = 0;
            self.i.y += 1;
        }

        item
    }
}

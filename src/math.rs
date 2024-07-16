use std::{isize, ops::Add};

#[inline]
pub fn pt(x: isize, y: isize) -> Pt {
    Pt { x, y }
}

#[inline]
pub fn upt(x: usize, y: usize) -> UPt {
    UPt { x, y }
}

#[derive(Clone, Copy)]
pub struct Pt {
    pub x: isize,
    pub y: isize,
}
#[derive(Clone, Copy)]
pub struct UPt {
    pub x: usize,
    pub y: usize,
}

impl Add for Pt {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Add<Pt> for UPt {
    type Output = Self;

    fn add(self, rhs: Pt) -> Self::Output {
        Self {
            x: (self.x as isize + rhs.x).clamp(0, 60) as usize,
            y: (self.y as isize + rhs.y).clamp(0, 60) as usize,
        }
    }
}

impl Add for UPt {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Add<Direction> for Pt {
    type Output = Self;

    fn add(self, rhs: Direction) -> Self::Output {
        self + Into::<Pt>::into(rhs)
    }
}

impl Add<Direction> for UPt {
    type Output = Self;

    fn add(self, rhs: Direction) -> Self::Output {
        self + Into::<Pt>::into(rhs)
    }
}

#[derive(Clone, Copy)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    pub fn opposite(&self) -> Self {
        match self {
            Self::Up => Self::Down,
            Self::Down => Self::Up,
            Self::Left => Self::Right,
            Self::Right => Self::Left,
        }
    }
}

impl Into<Pt> for Direction {
    fn into(self) -> Pt {
        match self {
            Self::Up => pt(0, -1),
            Self::Down => pt(0, 1),
            Self::Left => pt(-1, 0),
            Self::Right => pt(1, 0),
        }
    }
}

pub type Pt = (isize, isize);
pub type UPt = (usize, usize);

#[derive(Clone, Copy)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Into<Pt> for Direction {
    fn into(self) -> Pt {
        match self {
            Self::Up => (0, -1),
            Self::Down => (0, 1),
            Self::Left => (-1, 0),
            Self::Right => (1, 0),
        }
    }
}

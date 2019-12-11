#[derive(Copy, Clone, Debug, PartialEq, Hash, Eq)]
pub struct Point2D(pub isize, pub isize);



impl Point2D {
    pub fn new(x: isize, y: isize) -> Point2D {
        Point2D(x, y)
    }
    
    pub fn into_index(&self) -> Option<(usize, usize)> {
        if self.0 >= 0 && self.1 >= 0 {
            Some((self.0 as usize, self.1 as usize))
        } else {
            None
        }
    }

    pub fn add(&self, x: isize, y: isize) -> Point2D {
        return Point2D(self.0 + x, self.1 + y)
    }
}

pub enum Dir {
    North,
    South,
    West,
    East
}

impl Dir {
    pub fn right(&self) -> Dir {
        use Dir::*;
        match *self {
            North => East,
            East => South,
            South => West,
            West => North
        }
    }

    pub fn left(&self) -> Dir {
        use Dir::*;
        match *self {
            South => East,
            West => South,
            North => West,
            East => North
        }
    }

    pub fn next_pos(&self, pos: &Point2D) -> Point2D {
        let off = match self {
            Dir::North => (0, -1),
            Dir::East => (1, 0),
            Dir::West => (0, -1),
            Dir::South => (0, 1)
        };

        pos.add(off.0, off.1)
    }
}
use geo::Point;

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

    pub fn offset(&self) -> (isize, isize) {
        match self {
            Dir::North => (0, -1),
            Dir::East => (1, 0),
            Dir::West => (-1, 0),
            Dir::South => (0, 1)
        }
    }

    pub fn next_pos(&self, pos: Point<usize>) -> Point<usize> {
        let off = self.offset();
        offset_pos(pos, off)
    }
}

pub fn offset_pos(pt: Point<usize>, offset: (isize, isize)) -> Point<usize> {
    let x = (pt.0.x as isize) +  offset.0;
    let y = (pt.0.y as isize) +  offset.1;

    Point::new(x as usize, y as usize)
}
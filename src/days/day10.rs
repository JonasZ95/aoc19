#![allow(dead_code)]

use crate::*;
use num::Integer;
use std::collections::HashSet;
use std::str::FromStr;

const DAY: usize = 10;

#[derive(Clone, PartialEq)]
pub enum GridField {
    Asteroid,
    Empty,
}

impl FromStr for Grid<GridField> {
    type Err = AocErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let width = s.lines().next().map(|l| l.chars().count()).unwrap_or(0);

        let height = s.lines().count();

        let mut grid = Grid::new(width, height, GridField::Empty);
        for (x, y, c) in s
            .lines()
            .enumerate()
            .flat_map(|(y, l)| l.chars().enumerate().map(move |(x, c)| (x, y, c)))
        {
            let pt = Point2(x, y);
            let field = grid
                .get_mut(pt)
                .ok_or_else(|| custom_err("Invalid point"))?;

            *field = match c {
                '#' => GridField::Asteroid,
                '.' => GridField::Empty,
                _ => return Err(custom_err(format!("Invalid Grid field: {}", c))),
            }
        }

        Ok(grid)
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Hash, Eq)]
pub struct Point2(usize, usize);

impl Point2 {
    fn add(&self, x_off: isize, y_off: isize) -> Option<Point2> {
        fn add(n: usize, a: isize) -> Option<usize> {
            let n = (n as isize) + a;
            if n >= 0 {
                Some(n as usize)
            } else {
                None
            }
        }

        match (add(self.0, x_off), add(self.1, y_off)) {
            (Some(x), Some(y)) => Some(Point2(x, y)),
            _ => None,
        }
    }
}

pub struct Grid<T> {
    data: Vec<T>,
    width: usize,
    height: usize,
}

impl Grid<GridField> {
    pub fn asteroids(&self) -> impl Iterator<Item = Point2> + '_ {
        self.iter().filter_map(|(pt, t)| match t {
            &GridField::Empty => None,
            &GridField::Asteroid => Some(pt),
        })
    }

    fn best_place(&self) -> Option<(Point2, usize)> {
        self.asteroids()
            .map(|from| {
                let n = self
                    .asteroids()
                    .filter(|&to| self.is_visible(from, to))
                    .count();

                (from, n)
            })
            .max_by_key(|(_, count)| *count)
    }

    fn calc_degree(from: Point2, to: Point2) -> f64 {
        use std::f64::consts::*;

        let slope_x = (to.0 as f64) - (from.0 as f64);
        let slope_y = (to.1 as f64) - (from.1 as f64);

        let rad = slope_y.atan2(slope_x);
        let rad = match rad {
            rad if rad.is_sign_negative() => {
                let rad = PI + rad;

                match rad {
                    rad if rad >= FRAC_PI_2 => rad - FRAC_PI_2,
                    rad if rad == PI => PI + FRAC_PI_2,
                    rad => PI + FRAC_PI_2 + rad,
                }
            }
            rad => rad + FRAC_PI_2,
        };

        rad * 180. / PI
    }

    fn vaporize(&mut self, from: Point2) -> Vec<Point2> {
        let mut vap_seq = Vec::new();
        let mut vaporized = HashSet::new();

        loop {
            let mut ast: Vec<_> = self
                .visible_asteroids(from)
                .filter(|pt| !vaporized.contains(pt))
                .map(|pt| (pt, (Self::calc_degree(from, pt).floor() * 100.) as usize))
                .collect();

            if ast.is_empty() {
                return vap_seq;
            }

            ast.sort_by_key(|(_, deg)| deg.clone());

            for (pt, _) in ast.iter().cloned() {
                vaporized.insert(pt);
                vap_seq.push(pt);
            }

            for vap in vaporized.iter().cloned() {
                *self.get_mut(vap).unwrap() = GridField::Empty;
            }
        }
    }

    fn visible_asteroids(&self, from: Point2) -> impl Iterator<Item = Point2> + '_ {
        self.asteroids()
            .filter(move |&to| self.is_visible(from, to))
    }

    fn is_visible(&self, from: Point2, to: Point2) -> bool {
        if from == to {
            return false;
        }

        let x_delta = (to.0 as isize) - (from.0 as isize);
        let y_delta = (to.1 as isize) - (from.1 as isize);

        let div = match (x_delta, y_delta) {
            (0, y) => y.abs(),
            (x, 0) => x.abs(),
            (x, y) => x.gcd(&y),
        };

        let x_delta = x_delta / div;
        let y_delta = y_delta / div;

        !(1..div)
            .map(|x| from.add(x_delta * x, y_delta * x).unwrap())
            .any(|pt| self.get(pt).unwrap() == &GridField::Asteroid)
    }
}

impl<T> Grid<T>
where
    T: Clone,
{
    pub fn new(width: usize, height: usize, val: T) -> Self {
        let data = vec![val; width * height];
        Grid {
            data,
            width,
            height,
        }
    }

    fn pt_to_ix(&self, pt: &Point2) -> Option<usize> {
        if pt.0 >= self.width || pt.1 >= self.height {
            return None;
        }

        let ix = pt.1 * self.width + pt.0;
        Some(ix)
    }

    fn ix_to_pt(&self, ix: usize) -> Point2 {
        let x = ix % self.width;
        let y = ix / self.width;

        Point2(x, y)
    }

    pub fn get(&self, pt: Point2) -> Option<&T> {
        self.pt_to_ix(&pt).and_then(|ix| self.data.get(ix))
    }

    pub fn get_mut(&mut self, pt: Point2) -> Option<&mut T> {
        self.pt_to_ix(&pt).and_then(move |ix| self.data.get_mut(ix))
    }

    pub fn points(&self) -> impl Iterator<Item = Point2> + 'static {
        let width = self.width;
        let n = self.data.len();

        (0..n).map(move |ix| {
            let x = ix % width;
            let y = ix / width;
            Point2(x, y)
        })
    }

    pub fn iter(&self) -> impl Iterator<Item = (Point2, &T)> + '_ {
        self.points().map(move |pt| (pt, self.get(pt).unwrap()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() -> AocResult<()> {
        let grid: Grid<GridField> = parse_file(FileType::Example, DAY, 1)?;
        assert!(!grid.is_visible(Point2(4, 4), Point2(4, 0)));
        assert_eq!((Point2(3, 4), 8), grid.best_place().unwrap());

        let grid: Grid<GridField> = parse_file(FileType::Example, DAY, 2)?;
        assert_eq!((Point2(5, 8), 33), grid.best_place().unwrap());

        let grid: Grid<GridField> = parse_file(FileType::Example, DAY, 3)?;
        assert_eq!((Point2(1, 2), 35), grid.best_place().unwrap());

        let grid: Grid<GridField> = parse_file(FileType::Example, DAY, 4)?;
        assert_eq!((Point2(6, 3), 41), grid.best_place().unwrap());

        let grid: Grid<GridField> = parse_file(FileType::Example, DAY, 5)?;
        assert_eq!((Point2(11, 13), 210), grid.best_place().unwrap());

        let grid: Grid<GridField> = parse_file(FileType::Input, DAY, 1)?;
        assert_eq!((Point2(23, 29), 263), grid.best_place().unwrap());

        Ok(())
    }

    #[test]
    fn part2() -> AocResult<()> {
        //TODO use nearly equal
        assert_eq!(0., Grid::calc_degree(Point2(1, 1), Point2(1, 0)));
        assert_eq!(90., Grid::calc_degree(Point2(1, 1), Point2(2, 1)));
        assert_eq!(180., Grid::calc_degree(Point2(1, 1), Point2(1, 2)));
        assert_eq!(270., Grid::calc_degree(Point2(1, 1), Point2(0, 1)));

        assert_eq!(90. + 45., Grid::calc_degree(Point2(2, 2), Point2(3, 3)));
        assert_eq!(45., Grid::calc_degree(Point2(2, 2), Point2(3, 1)));
        assert_eq!(180. + 45., Grid::calc_degree(Point2(2, 2), Point2(1, 3)));
        assert_eq!(270. + 45., Grid::calc_degree(Point2(2, 2), Point2(1, 1)));

        let mut grid: Grid<GridField> = parse_file(FileType::Example, DAY, 6)?;
        itertools::assert_equal(
            [Point2(8, 1), Point2(9, 0), Point2(9, 1)].iter(),
            grid.vaporize(Point2(8, 3)).iter().take(3),
        );

        let mut grid: Grid<GridField> = parse_file(FileType::Example, DAY, 5)?;
        let vaps = grid.vaporize(Point2(11, 13));
        assert_eq!(vaps.iter().position(|pt| *pt == Point2(8, 2)).unwrap(), 198);

        let mut grid: Grid<GridField> = parse_file(FileType::Input, DAY, 1)?;
        let vaps = grid.vaporize(Point2(23, 29));
        assert_eq!(vaps[199], Point2(11, 10));
        Ok(())
    }
}

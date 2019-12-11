#![allow(dead_code)]

use crate::*;
use num::Integer;
use std::collections::HashSet;
use std::str::FromStr;
use ndarray::Array2;
use itertools::Itertools;

const DAY: usize = 10;

#[derive(Clone, PartialEq, Debug)]
pub enum GridField {
    Asteroid,
    Empty,
}

impl FromStr for Grid<GridField> {
    type Err = AocErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let width = s.lines().next().map(|l| l.chars().count()).unwrap_or(0);
        let height = s.lines().count();

        let v = s
            .lines()
            .flat_map(|l| l.chars())
            .map(|c| match c {
                '#' => GridField::Asteroid,
                '.' => GridField::Empty,
                _ => unreachable!()
            })
            .collect_vec();

        let mut arr = Array2::from_shape_vec((height, width), v)
            .unwrap();
        arr.swap_axes(1, 0);



        Ok(Grid{
            arr
        })
    }
}


pub struct Grid<T> {
    arr: Array2<T>
}

impl Grid<GridField> {
    pub fn asteroids(&self) -> impl Iterator<Item = ((usize, usize), &GridField)> + '_ {
        self.arr.indexed_iter()
            .filter(|f|  match f.1 {
                &GridField::Empty => false,
                &GridField::Asteroid => true,
            })
    }



    fn visible_asteroids(&self, from: (usize, usize)) -> impl Iterator<Item = ((usize, usize), &GridField)> + '_ {
        self.asteroids()
            .filter(move |(to, _)| self.is_visible(from, *to))
    }

    fn best_place(&self) -> Option<((usize, usize), usize)> {
        self.asteroids()
            .map(|(from, _)| {
                let n = self
                    .asteroids()
                    .filter(|(to, _)| self.is_visible(from, *to))
                    .count();

                (from, n)
            })
            .max_by_key(|(_, count)| *count)
    }

    fn calc_degree(from: (usize, usize), to: (usize, usize)) -> f64 {
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

    fn vaporize(&mut self, from: (usize, usize)) -> Vec<(usize, usize)> {
        let mut vap_seq = Vec::new();
        let mut vaporized = HashSet::new();

        loop {
            let mut ast: Vec<_> = self
                .visible_asteroids(from)
                .filter(|(pt, _)| !vaporized.contains(pt))
                .map(|(pt, _)| (pt, (Self::calc_degree(from, pt).floor() * 100.) as usize))
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
                self.arr[vap] = GridField::Empty;
            }
        }
    }

    fn is_visible(&self, from: (usize, usize), to: (usize, usize)) -> bool {
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

        (1..div)
            .map(|k| {
                let x = (from.0 as isize) + x_delta * k;
                let y = (from.1 as isize) + y_delta * k;
                (x as usize, y as usize)
            })
            .all(|pt| self.arr[pt] != GridField::Asteroid)
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() -> AocResult<()> {
        let grid: Grid<GridField> = parse_file(FileType::Example, DAY, 1)?;
        assert!(!grid.is_visible((4, 4), (4, 0)));
        assert_eq!(((3, 4), 8), grid.best_place().unwrap());

        let grid: Grid<GridField> = parse_file(FileType::Example, DAY, 2)?;
        assert_eq!(((5, 8), 33), grid.best_place().unwrap());

        let grid: Grid<GridField> = parse_file(FileType::Example, DAY, 3)?;
        assert_eq!(((1, 2), 35), grid.best_place().unwrap());

        let grid: Grid<GridField> = parse_file(FileType::Example, DAY, 4)?;
        assert_eq!(((6, 3), 41), grid.best_place().unwrap());

        let grid: Grid<GridField> = parse_file(FileType::Example, DAY, 5)?;
        assert_eq!(((11, 13), 210), grid.best_place().unwrap());

        let grid: Grid<GridField> = parse_file(FileType::Input, DAY, 1)?;
        assert_eq!(((23, 29), 263), grid.best_place().unwrap());

        Ok(())
    }

    #[test]
    fn part2() -> AocResult<()> {
        //TODO use nearly equal
        assert_eq!(0., Grid::calc_degree((1, 1), (1, 0)));
        assert_eq!(90., Grid::calc_degree((1, 1), (2, 1)));
        assert_eq!(180., Grid::calc_degree((1, 1), (1, 2)));
        assert_eq!(270., Grid::calc_degree((1, 1), (0, 1)));

        assert_eq!(90. + 45., Grid::calc_degree((2, 2), (3, 3)));
        assert_eq!(45., Grid::calc_degree((2, 2), (3, 1)));
        assert_eq!(180. + 45., Grid::calc_degree((2, 2), (1, 3)));
        assert_eq!(270. + 45., Grid::calc_degree((2, 2), (1, 1)));

        let mut grid: Grid<GridField> = parse_file(FileType::Example, DAY, 6)?;
        itertools::assert_equal(
            [(8, 1), (9, 0), (9, 1)].iter(),
            grid.vaporize((8, 3)).iter().take(3),
        );

        let mut grid: Grid<GridField> = parse_file(FileType::Example, DAY, 5)?;
        let vaps = grid.vaporize((11, 13));
        assert_eq!(vaps.iter().position(|pt| *pt == (8, 2)).unwrap(), 198);

        let mut grid: Grid<GridField> = parse_file(FileType::Input, DAY, 1)?;
        let vaps = grid.vaporize((23, 29));
        assert_eq!(vaps[199], (11, 10));
        Ok(())
    }
}

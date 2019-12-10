#![allow(dead_code)]
use crate::*;
use std::collections::HashMap;
use std::str::FromStr;

#[derive(Debug, Clone)]
pub enum Dir {
    Up(usize),
    Down(usize),
    Right(usize),
    Left(usize),
}

#[derive(Debug, Clone)]
pub struct Wire(pub Vec<Dir>);

impl FromStr for Wire {
    type Err = AocErr;
    fn from_str(s: &str) -> AocResult<Self> {
        let v: Result<Vec<_>, _> = s.split(',').map(|s| s.parse()).collect();

        Ok(Wire(v?))
    }
}

impl FromStr for Dir {
    type Err = AocErr;

    fn from_str(s: &str) -> AocResult<Self> {
        if s.is_empty() {
            return Err(AocErr::Custom("Empty dir".to_string()));
        }

        let (dir, num) = s.split_at(1);
        let dir = dir.chars().next().unwrap();
        let num = num.parse()?;

        Ok(match dir {
            'U' => Dir::Up(num),
            'D' => Dir::Down(num),
            'R' => Dir::Right(num),
            'L' => Dir::Left(num),
            _ => return Err(AocErr::Custom("Invalid dir".to_string())),
        })
    }
}

#[derive(Debug)]
pub struct Data {
    pub wire1: Wire,
    pub wire2: Wire,
}

impl FromStr for Data {
    type Err = AocErr;

    fn from_str(s: &str) -> AocResult<Self> {
        let v: ParseLineVec<Wire> = s.parse()?;
        let v = v.0;
        if v.len() != 2 {
            return Err(custom_err("Not 2 wires"));
        }

        Ok(Data {
            wire1: v[0].clone(),
            wire2: v[1].clone(),
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Point2(pub isize, pub isize);

impl Point2 {
    fn dist(&self, other: &Point2) -> usize {
        ((self.0 - other.0).abs() + (self.1 - other.1).abs()) as usize
    }

    fn add(&self, x: isize, y: isize) -> Point2 {
        Point2(self.0 + x, self.1 + y)
    }
}

#[derive(Debug)]
struct Circuit {
    board: HashMap<Point2, u8>,
    wire1: Wire,
    wire2: Wire,
}

impl Circuit {
    fn from_data(data: &Data) -> Self {
        let mut circ = Circuit {
            board: HashMap::new(),
            wire1: data.wire1.clone(),
            wire2: data.wire2.clone(),
        };

        circ.add_wire(&data.wire1, 1);
        circ.add_wire(&data.wire2, 2);

        circ
    }

    fn center(&self) -> Point2 {
        Point2(0, 0)
    }

    fn get_mut(&mut self, p: Point2) -> &mut u8 {
        self.board.entry(p).or_insert(0)
    }

    fn get(&self, p: &Point2) -> &u8 {
        self.board.get(&p).unwrap()
    }

    fn set_point(&mut self, pt: Point2, wire_id: u8) {
        let p = self.get_mut(pt);

        let new = match *p {
            0 => wire_id,
            _ if *p == wire_id => wire_id,
            _ => 255,
        };

        *p = new;
    }

    fn add_wire(&mut self, wire: &Wire, wire_id: u8) {
        let mut p = self.center();
        for dir in &wire.0 {
            let (x, y, steps) = match dir {
                Dir::Down(n) => (0, 1, *n),
                Dir::Up(n) => (0, -1, *n),
                Dir::Left(n) => (-1, 0, *n),
                Dir::Right(n) => (1, 0, *n),
            };

            for _i in 0..steps {
                p = p.add(x, y);
                self.set_point(p, wire_id);
            }
        }
    }

    fn intersections(&self) -> impl Iterator<Item = Point2> + '_ {
        self.board
            .iter()
            .filter_map(|(&k, &v)| if v == 255 { Some(k) } else { None })
    }

    fn step_ints(&self, wire: &Wire) -> HashMap<Point2, usize> {
        let mut shortest = HashMap::new();
        let mut step = 0;

        let mut p = self.center();
        for dir in &wire.0 {
            let (x, y, steps) = match dir {
                Dir::Down(n) => (0, 1, *n),
                Dir::Up(n) => (0, -1, *n),
                Dir::Left(n) => (-1, 0, *n),
                Dir::Right(n) => (1, 0, *n),
            };

            for _i in 0..steps {
                step += 1;
                p = p.add(x, y);

                if *self.get(&p) == 255 {
                    shortest.entry(p).or_insert(step);
                }
            }
        }

        shortest
    }

    fn find_shortest_int_steps(&self) -> usize {
        let shortest1 = self.step_ints(&self.wire1);
        let shortest2 = self.step_ints(&self.wire2);

        shortest1
            .iter()
            .map(|(pt, steps)| steps + shortest2[pt])
            .min()
            .unwrap()
    }

    fn shortest_dist_intersection(&self) -> usize {
        let center = self.center();
        self.intersections()
            .map(|int| int.dist(&center))
            .min()
            .unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part12() -> AocResult<()> {
        let data: Data = parse_file(FileType::Input, 3, 1)?;
        let circuit = Circuit::from_data(&data);

        assert_eq!(circuit.shortest_dist_intersection(), 1626);
        assert_eq!(circuit.find_shortest_int_steps(), 27330);

        Ok(())
    }

    #[test]
    fn test1() -> AocResult<()> {
        let data: Data = parse_file(FileType::Example, 3, 1)?;
        let circuit = Circuit::from_data(&data);

        assert_eq!(circuit.shortest_dist_intersection(), 135);
        assert_eq!(circuit.find_shortest_int_steps(), 410);

        Ok(())
    }

    #[test]
    fn test2() -> AocResult<()> {
        let data: Data = parse_file(FileType::Example, 3, 2)?;
        let circuit = Circuit::from_data(&data);

        assert_eq!(circuit.shortest_dist_intersection(), 159);
        assert_eq!(circuit.find_shortest_int_steps(), 610);

        Ok(())
    }
}

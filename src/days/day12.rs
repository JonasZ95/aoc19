#![allow(dead_code)]

use crate::*;
use itertools::Itertools;
use num::integer::Integer;
use std::cmp::Ordering;

const DAY: usize = 12;

pub struct Data([(isize, isize, isize); 4]);

#[derive(Debug)]
pub struct Moon {
    pos: (isize, isize, isize),
    vel: (isize, isize, isize),
}

impl FromStr for Data {
    type Err = AocErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let v = s
            .lines()
            .map(|l| {
                let l = l.trim_start_matches('<');
                let l = l.trim_end_matches('>');

                let (mut x, mut y, mut z) = (0, 0, 0);
                for (i, n) in l.split(',').enumerate() {
                    let pos = n.find('=').ok_or(custom_err("No = divider"))?;

                    let (name, val) = n.split_at(pos + 1);

                    match (name.trim(), val.trim()) {
                        ("x=", val) if i == 0 => x = val.parse()?,
                        ("y=", val) if i == 1 => y = val.parse()?,
                        ("z=", val) if i == 2 => z = val.parse()?,
                        _ => unreachable!(),
                    }
                }

                Ok((x, y, z))
            })
            .collect::<Result<Vec<(isize, isize, isize)>, AocErr>>()?;

        let mut data = [(0, 0, 0); 4];
        data.copy_from_slice(&v);
        Ok(Data(data))
    }
}

fn abs_sum(p: (isize, isize, isize)) -> isize {
    p.0.abs() + p.1.abs() + p.2.abs()
}

fn add(p: (isize, isize, isize), off: (isize, isize, isize)) -> (isize, isize, isize) {
    (p.0 + off.0, p.1 + off.1, p.2 + off.2)
}

fn calc_single_vel_offset(moon_pos: isize, other_pos: isize) -> isize {
    match moon_pos.cmp(&other_pos) {
        Ordering::Equal => 0,
        Ordering::Greater => -1,
        Ordering::Less => 1,
    }
}

fn calc_vel_offset(vel: (isize, isize, isize), moon: &Moon, other: &Moon) -> (isize, isize, isize) {
    (
        vel.0 + calc_single_vel_offset(moon.pos.0, other.pos.0),
        vel.1 + calc_single_vel_offset(moon.pos.1, other.pos.1),
        vel.2 + calc_single_vel_offset(moon.pos.2, other.pos.2),
    )
}

fn update_vel(moons: &mut [Moon; 4]) {
    for moon in 0..moons.len() {
        moons[moon].vel = (0..moons.len())
            .filter(|j| *j != moon)
            .fold(moons[moon].vel, |vel, other| {
                calc_vel_offset(vel, &moons[moon], &moons[other])
            });
    }
}

fn update_pos(moons: &mut [Moon; 4]) {
    for moon in moons.iter_mut() {
        moon.pos = add(moon.pos, moon.vel);
    }
}

fn create_moons(data: Data) -> [Moon; 4] {
    let d = data.0;
    let zero = (0, 0, 0);

    [
        Moon {
            pos: d[0],
            vel: zero,
        },
        Moon {
            pos: d[1],
            vel: zero,
        },
        Moon {
            pos: d[2],
            vel: zero,
        },
        Moon {
            pos: d[3],
            vel: zero,
        },
    ]
}

fn check_axis_repeat(
    moons: &[Moon; 4],
    start: [(isize, isize, isize); 4],
    axis_fn: impl Fn(&(isize, isize, isize)) -> isize,
) -> bool {
    moons
        .iter()
        .zip(start.iter())
        .all(|(moon, start)| axis_fn(&moon.pos) == axis_fn(start) && axis_fn(&moon.vel) == 0)
}

fn find_prev(data: Data) -> usize {
    let points = data.0.clone();
    let mut moons = create_moons(data);
    let mut intervals = [None; 3];

    for i in 0_usize.. {
        update_vel(&mut moons);
        update_pos(&mut moons);

        let i = i + 1;

        //x
        if intervals[0].is_none() && check_axis_repeat(&moons, points, |&(x, _, _)| x) {
            intervals[0] = Some(i);
        }

        //y
        if intervals[1].is_none() && check_axis_repeat(&moons, points, |&(_, y, _)| y) {
            intervals[1] = Some(i);
        }

        //z
        if intervals[2].is_none() && check_axis_repeat(&moons, points, |&(_, _, z)| z) {
            intervals[2] = Some(i);
        }

        if intervals.iter().all(|iv| iv.is_some()) {
            let (x, y, z) = intervals
                .iter()
                .map(|i| i.unwrap())
                .collect_tuple()
                .unwrap();

            let yz = y.lcm(&z);
            return x.lcm(&yz);
        }
    }

    unreachable!()
}

fn calc_energy(data: Data, steps: usize) -> isize {
    let mut moons = create_moons(data);

    for _ in 0..steps {
        //Calc vel
        update_vel(&mut moons);

        //Apply vel
        update_pos(&mut moons);
    }

    moons
        .iter()
        .map(|m| {
            let pot = abs_sum(m.pos);
            let kin = abs_sum(m.vel);
            pot * kin
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() -> AocResult<()> {
        let data: Data = parse_file(FileType::Input, DAY, 01)?;

        assert_eq!((17, -9, 4), data.0[0]);
        assert_eq!((2, 2, -13), data.0[1]);
        assert_eq!((-1, 5, -1), data.0[2]);
        assert_eq!((4, 7, -7), data.0[3]);
        Ok(())
    }

    #[test]
    fn part1() -> AocResult<()> {
        let data: Data = parse_file(FileType::Example, DAY, 01)?;
        assert_eq!(179, calc_energy(data, 10));

        let data: Data = parse_file(FileType::Example, DAY, 02)?;
        assert_eq!(1940, calc_energy(data, 100));

        let data: Data = parse_file(FileType::Input, DAY, 01)?;
        assert_eq!(7202, calc_energy(data, 1000));
        Ok(())
    }

    #[test]
    fn part2() -> AocResult<()> {
        let data: Data = parse_file(FileType::Example, DAY, 01)?;
        assert_eq!(2772, find_prev(data));

        let data: Data = parse_file(FileType::Example, DAY, 02)?;
        assert_eq!(4_686_774_924, find_prev(data));

        let data: Data = parse_file(FileType::Input, DAY, 01)?;
        assert_eq!(537_881_600_740_876, find_prev(data));

        Ok(())
    }
}

#![allow(dead_code)]

use crate::*;
use std::collections::{HashMap, VecDeque};
use std::cmp::Ordering;

const DAY: usize = 14;

pub struct Chemical {
    name: String,
    num: usize
}

impl FromStr for Chemical {
    type Err = AocErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let p = s.find(' ')
            .ok_or_else(|| custom_err("No chemical split"))?;
        let s = s.split_at(p);

        Ok(Chemical {
            name: s.1.trim().to_string(),
            num: s.0.parse()?
        })
    }
}

pub struct Data {
   rules: HashMap<String, (usize, Vec<Chemical>)>
}

impl FromStr for Data {
    type Err = AocErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let m= s.lines()
            .map(|l| {
                let p = l.find("=>")
                    .ok_or_else(|| custom_err("Invalid rule"))?;

                let (left, right) = l.split_at(p);
                let right = &right[2..];

                let right: Chemical = right.trim().parse()?;
                let left = left.split(',')
                    .map(|c| c.trim().parse())
                    .collect::<Result<Vec<Chemical>, AocErr>>()?;

                Ok((right.name, (right.num, left)))
            })
            .collect::<Result<HashMap<String, (usize, Vec<Chemical>)>, AocErr>>()?;


        Ok(Data{rules: m})
    }
}

fn ore_for_fuel(data: &Data) -> usize {
    ore_for_fuel2(data, 1)
}

fn ore_for_fuel2(data: &Data, fuel: usize) -> usize {
    let mut ore = 0;
    let mut excess: HashMap<String, usize> = HashMap::new();
    let mut q = VecDeque::new();
    q.push_back(Chemical{
        name: "FUEL".to_string(),
        num: fuel
    });


    while let Some(chem) = q.pop_front() {
        if &chem.name == "ORE" {
            ore += chem.num;
            continue;
        }

        let num = if let Some(excess) = excess.get_mut(&chem.name) {
            if *excess >= chem.num {
                *excess -= chem.num;
                0
            } else {
                let num = chem.num - *excess;
                *excess = 0;
                num
            }
        } else {
            chem.num
        };


        //println!("chem: {}, num: {}", chem.name, num);
        let rule = &data.rules[&chem.name];
        let n = ((num as f64) / (rule.0 as f64)).ceil() as usize;

        let e = rule.0*n -  num;
        if e > 0 {
            excess.entry(chem.name)
                .and_modify(|excess| *excess += e)
                .or_insert(e);
        }

        for prod in rule.1.iter() {
            q.push_back(Chemical{
                num: prod.num*n,
                name:  prod.name.clone()
            });
        }
    }

    ore
}

fn calc_max_fuel(data: &Data, ores: usize) -> usize {
    let ore_per_fuel = ore_for_fuel(&data);
    let cur = ores/ore_per_fuel;

    let (mut l, mut r) = (cur, cur*4);

    while l <= r {
        let m = l + (r-l) / 2;
        let ore = ore_for_fuel2(&data, m);

        match ore.cmp(&ores) {
            Ordering::Equal => return m,
            Ordering::Less if ore_for_fuel2(&data, m+1) > ores => return m,
            Ordering::Greater => r = m-1,
            Ordering::Less => l = m+1
        }


    }

    unreachable!()

}

/*10 ORE => 10 A
1 ORE => 1 B
7 A, 1 B => 1 C
7 A, 1 C => 1 D
7 A, 1 D => 1 E
7 A, 1 E => 1 FUEL
*/

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() -> AocResult<()> {
        let data: Data = parse_file(FileType::Example, DAY, 01)?;
        let fuel = &data.rules["FUEL"];
        assert_eq!(fuel.0, 1);
        assert_eq!(ore_for_fuel(&data), 31);

        let data: Data = parse_file(FileType::Example, DAY, 02)?;
        assert_eq!(ore_for_fuel(&data), 165);

        let data: Data = parse_file(FileType::Example, DAY, 03)?;
        assert_eq!(ore_for_fuel(&data), 13312);

        let data: Data = parse_file(FileType::Input, DAY, 01)?;
        assert_eq!(ore_for_fuel(&data), 2556890);

        Ok(())
    }

    #[test]
    fn part2() -> AocResult<()> {
        const ORE: usize = 1_000_000_000_000;

        let data: Data = parse_file(FileType::Example, DAY, 03)?;
        assert_eq!(calc_max_fuel(&data, ORE), 82892753);

        let data: Data = parse_file(FileType::Input, DAY, 01)?;
        assert_eq!(calc_max_fuel(&data, ORE), 1120408);
        Ok(())
    }
}
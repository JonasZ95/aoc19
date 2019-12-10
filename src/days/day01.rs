#![allow(dead_code)]

use crate::*;

fn load_masses() -> AocResult<Vec<u64>> {
    let v: ParseLineVec<u64> = parse_file(FileType::Input, 1, 1)?;
    Ok(v.0)
}

fn calc_fuel(mass: u64) -> i64 {
    let f = ((mass as f64) / 3.).floor() as i64;
    f - 2
}

fn calc_total_fuel(mass: u64) -> u64 {
    let mut m = mass;
    let mut total_fuel = 0;

    loop {
        let fuel = calc_fuel(m);
        if fuel < 1 {
            return total_fuel;
        }

        m = fuel as u64;
        total_fuel += m;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_total() {
        assert_eq!(calc_total_fuel(14), 2);
        assert_eq!(calc_total_fuel(1969), 966);
    }

    #[test]
    fn part1() -> AocResult<()> {
        let m = load_masses()?;

        let s: i64 = m.iter().map(|&m| calc_fuel(m)).sum();

        assert_eq!(s, 3_297_896);
        Ok(())
    }

    #[test]
    fn part2() -> AocResult<()> {
        let m = load_masses()?;

        let s: u64 = m.iter().map(|&m| calc_total_fuel(m)).sum();

        assert_eq!(s, 4_943_969);
        Ok(())
    }
}

#![allow(dead_code)]

use crate::*;
use super::day05::*;
use fallible_iterator::{convert, FallibleIterator};
use itertools::Itertools;


pub fn settings_perm(from: u8) -> impl Iterator<Item=[u8; 5]> + 'static {
    (from..from + 5)
        .permutations(5)
        .map(|s| {
            let mut out = [0; 5];
            out.copy_from_slice(&s);
            out
        })
}

pub fn exec_amps(data: &Data, settings: [u8; 5], input: isize) -> AocResult<isize> {
    settings.iter()
        .try_fold(input, |signal, &s| {
            Context::from_data(data.clone(), &[s as isize, signal])
                .exec()
        })
}

fn find_max_signal2(data: Data, input: isize) -> AocResult<isize> {
    Ok(convert(settings_perm(5)
        .map(|s| exec_amps2(&data, s, input)))
        .max()?
        .unwrap())
}

pub fn exec_amps2(data: &Data, settings: [u8; 5], input: isize) -> AocResult<isize> {
    let mut ctxs: Vec<Context> = settings.iter()
        .map(|&s| {
            Context::from_data(data.clone(), &[s as isize])
        })
        .collect();

    let mut signal = input;

    for ix in (0..5).cycle() {
        let ctx = &mut ctxs[ix];
        if ctx.halted() {
            break;
        }

        ctx.push_input(signal);
        ctx.resume()?;
        signal = ctx.output().expect("Output is expected for amp");
    }

    Ok(signal)
}

fn find_max_signal(data: Data, input: isize) -> AocResult<isize> {
    Ok(convert(settings_perm(0)
        .map(|s| exec_amps(&data, s, input)))
        .max()?
        .unwrap())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_test() -> AocResult<()> {
        let data: Data = "3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0".parse()?;
        assert_eq!(exec_amps(&data, [4, 3, 2, 1, 0], 0)?, 43210);

        let data: Data = "3,23,3,24,1002,24,10,24,1002,23,-1,23,101,5,23,23,1,24,23,23,4,23,99,0,0".parse()?;
        assert_eq!(exec_amps(&data, [0, 1, 2, 3, 4], 0)?, 54321);

        let data: Data = "3,31,3,32,1002,32,10,32,1001,31,-2,31,1007,31,0,33,1002,33,7,33,1,33,31,31,1,32,31,31,4,31,99,0,0,0".parse()?;
        assert_eq!(exec_amps(&data, [1, 0, 4, 3, 2], 0)?, 65210);

        Ok(())
    }


    #[test]
    fn part1() -> AocResult<()> {
        let data: Data = "3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0".parse()?;
        assert_eq!(find_max_signal(data, 0)?, 43210);

        let data: Data = parse_file(FileType::Input, 7, 1)?;
        assert_eq!(find_max_signal(data, 0)?, 338_603);

        Ok(())
    }

    #[test]
    fn part2() -> AocResult<()> {
        let data: Data = "3,26,1001,26,-4,26,3,27,1002,27,2,27,1,27,26,27,4,27,1001,28,-1,28,1005,28,6,99,0,0,5".parse()?;
        assert_eq!(find_max_signal2(data, 0)?, 139_629_729);

        let data: Data = parse_file(FileType::Input, 7, 1)?;
        assert_eq!(find_max_signal2(data, 0)?, 63_103_596);

        Ok(())
    }
}
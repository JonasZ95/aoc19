#![allow(dead_code)]

#[cfg(test)]
mod tests {
    use crate::*;
    use crate::days::day05::*;

    #[test]
    fn test1() -> AocResult<()> {
        let data: Data = parse_file(FileType::Example, 2, 1)?;
        let mut ctx = Context::from_data(data, &[]);
        ctx.resume()?;

        assert_eq!(30, ctx.read(0));

        Ok(())
    }

    #[test]
    fn part1() -> AocResult<()> {
        let mut data: Data = parse_file(FileType::Input, 2, 1)?;

        //1202 program alarm
        data.0[1] = 12;
        data.0[2] = 2;

        let mut ctx = Context::from_data(data, &[]);
        ctx.resume()?;

        assert_eq!(3_306_701, ctx.read(0));

        Ok(())
    }

    #[test]
    fn part2() -> AocResult<()> {
        const OUTPUT: isize = 19_690_720;
        let data: Data = parse_file(FileType::Input, 2, 1)?;

        for noun in 0..100 {
            for verb in 0..100 {
                let mut data = data.clone();
                data.0[1] = noun;
                data.0[2] = verb;
                let mut ctx = Context::from_data(data, &[]);

                if ctx.resume().is_ok() && ctx.read(0) == OUTPUT {
                    assert_eq!(7621, 100 * noun + verb);
                    return Ok(());
                }
            }
        }

        unreachable!("All combs used");
    }
}

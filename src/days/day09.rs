#![allow(dead_code)]

use crate::*;

const DAY: usize = 09;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::days::day05::*;

    #[test]
    fn part1() -> AocResult<()> {
        let data: Data = "109,1,204,-1,1001,100,1,100,1008,100,16,101,1006,101,0,99".parse()?;
        let mut ctx = Context::from_data_fill_up(data.clone(), &[]);
        ctx.exec()?;
        assert_eq!(data.0.as_slice(), &ctx.data()[..data.0.len()]);
        assert!(ctx.halted());

        let data: Data = "1102,34915192,34915192,7,4,7,99,0".parse()?;
        let mut ctx = Context::from_data_fill_up(data, &[]);
        ctx.exec()?;
        assert!(1_000_000_000_000_000 <= ctx.output().unwrap());
        assert!(ctx.halted());

        let data: Data = "104,1125899906842624,99".parse()?;
        let mut ctx = Context::from_data_fill_up(data, &[]);
        ctx.exec()?;
        assert_eq!(1_125_899_906_842_624, ctx.output().unwrap());
        assert!(ctx.halted());

        let data = parse_file(FileType::Input, DAY, 01)?;
        let mut ctx = Context::from_data_fill_up(data, &[1]);

        ctx.exec()?;
        assert_eq!(3_235_019_597, ctx.output().unwrap());

        Ok(())
    }

    #[test]
    fn part2() -> AocResult<()> {
        let data = parse_file(FileType::Input, DAY, 01)?;
        let mut ctx = Context::from_data_fill_up(data, &[2]);

        ctx.exec()?;
        assert_eq!(80274, ctx.output().unwrap());

        Ok(())
    }
}

#![allow(dead_code)]

use crate::*;
use days::day05::{Data, Context};
use std::convert::{TryFrom, TryInto};
use image::ImageBuffer;

const DAY: usize = 11;

pub enum Dir {
    North,
    South,
    West,
    East
}

#[derive(Clone, Copy)]
pub enum Color {
    Black,
    White
}

impl TryFrom<u8> for Color {
    type Error = AocErr;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Ok(match value {
            0 => Color::Black,
            1 => Color::White,
            i => return Err(custom_err(format!("Invalid color: {}", i)))
        })
    }
}

impl Into<u8> for Color {
    fn into(self) -> u8 {
        match self {
            Color::Black => 0,
            Color::White => 1
        }
    }
}

impl Dir {
    fn next_right(&self) -> Dir {
        use Dir::*;
        match *self {
            North => East,
            East => South,
            South => West,
            West => North
        }
    }

    fn next_left(&self) -> Dir {
        use Dir::*;
        match *self {
            South => East,
            West => South,
            North => West,
            East => North
        }
    }

    fn next_pos(&self, pos: (usize, usize)) -> (usize, usize) {
        match self {
            Dir::North => (pos.0, pos.1-1),
            Dir::East => (pos.0+1, pos.1),
            Dir::West => (pos.0-1, pos.1),
            Dir::South => (pos.0, pos.1+1)
        }
    }
}

fn run(data: Data, start: Color, save: bool) -> AocResult<usize> {
    const N: usize = 128;
    const CENTER: usize = N / 2;

    let mut ctx = Context::from_data_fill_up(data, &[]);
    let mut grid = [[(start, 0); N]; N];
    let mut pos = (CENTER, CENTER);
    let mut dir = Dir::North;


    loop {
        let color: u8 = grid[pos.1][pos.0].0.into();
        ctx.push_input(color as isize);

        ctx.resume()?;
        ctx.resume()?;

        if ctx.halted() {
            break;
        }

        let turn = ctx.pop_output().unwrap();
        let color = ctx.pop_output().unwrap();

        let counter = grid[pos.1][pos.0].1;
        let color = Color::try_from(color as u8)?;
        grid[pos.1][pos.0] = (color, counter+1);

        dir = match turn {
            0 => dir.next_left(),
            1 => dir.next_right(),
            _ => unreachable!()
        };

        pos = dir.next_pos(pos);
    }

    if save {
        let img = ImageBuffer::from_fn(N as u32, N as u32, |x, y| {
           match grid[y as usize][x as usize].0 {
               Color::Black => image::Luma([0u8]),
               Color::White => image::Luma([255u8])
           }
        });

        img.save("out11.png").unwrap();
    }

    Ok(grid.iter().flat_map(|v| v.iter())
        .filter(|(_, count)| *count > 0)
        .count())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() -> AocResult<()> {
        let data: Data = parse_file(FileType::Input, DAY, 01)?;
        assert_eq!(run(data, Color::Black, false)?, 1909);
        Ok(())
    }

    #[test]
    fn part2() -> AocResult<()> {
        let data: Data = parse_file(FileType::Input, DAY, 01)?;
        assert_eq!(run(data, Color::White, true)?, 249);
        Ok(())
    }
}
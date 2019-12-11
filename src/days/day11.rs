#![allow(dead_code)]

use crate::*;
use days::day05::{Data, Context};
use std::convert::{TryFrom};
use image::ImageBuffer;
use ndarray::Array2;
use crate::helper::point2d::{Dir, Point2D};

const DAY: usize = 11;

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

fn run(data: Data, start: Color, save: bool) -> AocResult<usize> {
    const N: usize = 128;
    const CENTER: isize = (N / 2) as isize;

    let mut grid = Array2::from_elem((N, N), (start,0));
    let mut ctx = Context::from_data_fill_up(data, &[]);
    let mut pos = Point2D(CENTER, CENTER);
    let mut dir = Dir::North;


    loop {
        let ix = pos.into_index().unwrap();
        let color: u8 = grid[ix].0.into();
        ctx.push_input(color as isize);

        ctx.resume()?;
        ctx.resume()?;

        if ctx.halted() {
            break;
        }

        let turn = ctx.pop_output().unwrap();
        let color = ctx.pop_output().unwrap();

        let counter = grid[ix].1;
        let color = Color::try_from(color as u8)?;
        grid[ix] = (color, counter+1);

        dir = match turn {
            0 => dir.left(),
            1 => dir.right(),
            _ => unreachable!()
        };

        pos = dir.next_pos(&pos);
    }

    if save {
        let img = ImageBuffer::from_fn(N as u32, N as u32, |x, y| {
           match grid[(x as usize,y as usize)].0 {
               Color::Black => image::Luma([0u8]),
               Color::White => image::Luma([255u8])
           }
        });

        img.save("out11.png").unwrap();
    }

    Ok(grid.iter()
        .filter(|g| g.1 > 0)
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
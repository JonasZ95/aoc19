#![allow(dead_code)]

use crate::*;
use super::day05::*;
use ndarray::{Array2, Axis};
use std::convert::{TryFrom, TryInto};
use geo::Point;
use std::cmp::Ordering;
use crate::days::day10::GridField;

const DAY: usize = 13;

#[derive(Clone, Debug, PartialEq)]
pub enum Tile {
    Empty,
    Wall,
    Block,
    HorizontalPaddle,
    Ball
}

#[derive(Clone, Copy, Debug)]
pub enum Input {
    Neutral,
    Left,
    Right
}

impl Into<isize> for Input {
    fn into(self) -> isize {
        match self {
            Input::Neutral => 0,
            Input::Left => -1,
            Input::Right => 1
        }
    }
}

impl TryFrom<u8> for Tile {
    type Error = AocErr;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Ok(match value {
            0 => Tile::Empty,
            1 => Tile::Wall,
            2 => Tile::Block,
            3 => Tile::HorizontalPaddle,
            4 => Tile::Ball,
            invalid => return Err(custom_err(format!("Invalid tile: {}", invalid)))
        })
    }
}

pub struct Game  {
    ctx: Context,
    tile_grid: Array2<Tile>,
    score: usize,
    ball_pos: Point<usize>,
    paddle_pos: Point<usize>,
    ball_vel: (isize, isize)
}

impl Game {
    pub fn create(data: Data, play: bool) -> AocResult<Game> {
        let mut ctx = Context::from_data_fill_up(data, &[]);

        let mut grid = Array2::from_elem((30, 50), Tile::Empty);
        grid.swap_axes(1, 0);

        if play {
            ctx.data_mut()[0] = 2;
        }

        loop {
            ctx.resume()?;
            ctx.resume()?;
            ctx.resume()?;

            if ctx.halted() {
                break;
            }

            let tile_id = ctx.pop_output().unwrap();
            let y = ctx.pop_output().unwrap();
            let x = ctx.pop_output().unwrap();

            //Detect game start
            if x == -1 && y == 0 {
                break;
            }

            Self::draw(&mut grid, tile_id, x, y)?;
        }


        let find_pos = |tile_kind: Tile| grid.indexed_iter()
            .filter(|(_, tile)| tile == &&tile_kind)
            .map(|(p, _)| Point::new(p.0, p.1))
            .next()
            .unwrap();

        let ball_pos = find_pos(Tile::Ball);
        let paddle_pos = find_pos(Tile::HorizontalPaddle);


        Ok(Game{
            ctx,
            tile_grid: grid,
            score: 0,
            ball_pos,
            paddle_pos,
            ball_vel: (1, 1)
        })
    }

    pub fn shape(&self) -> (usize, usize) {
        let shape = self.tile_grid.shape();
        (shape[0], shape[1])
    }

    pub fn ball(&self) -> Point<usize> {
        self.ball_pos
    }

    pub fn paddle(&self) -> Point<usize> {
        self.paddle_pos
    }

    pub fn walls(&self) -> impl Iterator<Item=Point<usize>> + '_ {
        self.tile_grid.indexed_iter()
            .filter(|(_, tile)| tile == &&Tile::Wall)
            .map(|(p, _)| Point::new(p.0, p.1))
    }

    pub fn blocks(&self) -> impl Iterator<Item=Point<usize>> + '_ {
        self.tile_grid.indexed_iter()
            .filter(|(_, tile)| tile == &&Tile::Block)
            .map(|(p, _)| Point::new(p.0, p.1))
    }

    fn draw(grid: &mut Array2<Tile>, tile_id: isize, x: isize, y: isize) -> AocResult<()>
    {
        let pt = Point::new(x as usize, y as usize);
        let tile = Tile::try_from(tile_id as u8)?;

        grid[pt.x_y()] = tile;
        Ok(())
    }

    fn game_draw(&mut self, tile_id: isize, x: isize, y: isize) -> AocResult<()>
    {
        let tile: Tile = (tile_id as u8).try_into()?;
        let pt = Point::new(x as usize, y as usize);
        match tile {
            Tile::Ball => {
                let xy = pt.x_y();
                let x_vel = (xy.0 as isize) - (self.ball_pos.x() as isize);
                let y_vel = (xy.1 as isize) - (self.ball_pos.y() as isize);
                self.ball_vel = (x_vel, y_vel);
                self.ball_pos = pt
            },
            Tile::HorizontalPaddle => self.paddle_pos = pt,
            _ => {}
        };

        self.tile_grid[pt.x_y()] = tile;
        Ok(())
    }

    pub fn update(&mut self) -> AocResult<bool> {
        let ctx = &mut self.ctx;
        ctx.resume()?;
        ctx.resume()?;
        ctx.resume()?;

        if ctx.halted() {
            return Ok(false)
        }

        let val = ctx.pop_output().unwrap();
        let y = ctx.pop_output().unwrap();
        let x = ctx.pop_output().unwrap();

        //Detect game start
        if x == -1 && y == 0 {
            self.score = val as usize;
            Ok(false)
        } else {
            self.game_draw(val, x, y)?;

            Ok(val == 4)
        }
    }

    pub fn auto_play(&mut self) -> AocResult<()> {
        if !self.ctx.halted() {
            if self.update()? {
                let input = match self.ball_pos.x().cmp(&self.paddle_pos.x()) {
                    Ordering::Equal => Input::Neutral,
                    Ordering::Greater => Input::Right,
                    Ordering::Less => Input::Left
                };
                self.ctx.push_input(input.into());
            }
        }

        Ok(())
    }

    pub fn score(&self) -> usize {
        self.score
    }

    fn play(mut self) -> AocResult<usize> {
        self.ctx.push_input(Input::Neutral.into());
        while !self.ctx.halted() {
            if self.count_blocks() == 0 {
                self.update()?;
                return Ok(self.score);
            }

            if self.update()? {
                let input = match self.ball_pos.x().cmp(&self.paddle_pos.x()) {
                    Ordering::Equal => Input::Neutral,
                    Ordering::Greater => Input::Right,
                    Ordering::Less => Input::Left
                };
                self.ctx.push_input(input.into());
            }
        }

        Ok(self.score)
    }

    pub fn set_input(&mut self, input: Input) {
        self.ctx.push_input(input.into());
    }

    fn count_blocks(&self) -> usize {
        self.tile_grid.iter()
            .filter(|tile| match tile {
                Tile::Block => true,
                _ => false
            })
            .count()
    }

    pub fn print(&self) {
        for row in self.tile_grid.lanes(Axis(0)) {
            for tile in row.into_iter() {
                let c = match tile {
                    Tile::Empty => ' ',
                    Tile::Ball => 'O',
                    Tile::HorizontalPaddle => '_',
                    Tile::Block => '*',
                    Tile::Wall => '#'
                };

                print!("{}", c);
            }
            println!()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() -> AocResult<()> {
        let data: Data = parse_file(FileType::Input, DAY, 01)?;
        let game = Game::create(data, false)?;
        assert_eq!(game.count_blocks(), 268);

        Ok(())
    }

    #[test]
    fn part2() -> AocResult<()> {
        let data: Data = parse_file(FileType::Input, DAY, 01)?;
        let game = Game::create(data, true)?;
        let score = game.play()?;
        assert_eq!(score, 13989);

        Ok(())
    }
}
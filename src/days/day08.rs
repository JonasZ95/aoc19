#![allow(dead_code)]

use crate::*;
use fallible_iterator::{convert, FallibleIterator};
use std::str::FromStr;

struct Data(Vec<u8>);

impl FromStr for Data {
    type Err = AocErr;

    fn from_str(s: &str) -> AocResult<Self> {
        let v: Vec<u8> = convert(s.chars().map(|c| {
            c.to_digit(10)
                .map(|n| n as u8)
                .ok_or_else(|| custom_err("No digit"))
        }))
        .collect()?;

        Ok(Data(v))
    }
}

struct Image {
    data: Vec<u8>,
    width: usize,
    height: usize,
}

struct Layer<'a> {
    img: &'a Image,
    ix: usize,
}

impl<'a> Layer<'a> {
    fn rows_iter(&self) -> impl Iterator<Item = &'a [u8]> + '_ {
        (0..self.img.height).map(move |i| {
            let ix = self.ix + self.img.width * i;
            &self.img.data[ix..ix + self.img.width]
        })
    }

    fn digits_iter(&self) -> impl Iterator<Item = u8> + '_ {
        (0..self.img.layer_size()).map(move |i| self.img.data[self.ix + i])
    }

    fn print(&self) {
        for row in self.rows_iter() {
            for &digit in row {
                print!("{} ", digit);
            }
            println!();
        }

        println!();
    }
}

impl Image {
    fn from_data(data: Data, height: usize, width: usize) -> Image {
        Image {
            data: data.0,
            height,
            width,
        }
    }

    fn layer_size(&self) -> usize {
        self.width * self.height
    }

    fn layers_len(&self) -> usize {
        (self.data.len() / self.layer_size())
    }

    fn layer(&self, ix: usize) -> Option<Layer<'_>> {
        let ix = ix * self.width * self.height;
        if self.layers_len() == 0 {
            None
        } else {
            Some(Layer { img: self, ix })
        }
    }

    fn layers_iter(&self) -> impl DoubleEndedIterator<Item = Layer<'_>> + '_ {
        (0..self.layers_len()).map(move |ix| self.layer(ix).unwrap())
    }

    fn lowest_layer(&self) -> usize {
        let min_layer = self
            .layers_iter()
            .min_by_key(|l| l.digits_iter().filter(|&d| d == 0).count())
            .unwrap();

        let ones = min_layer.digits_iter().filter(|&d| d == 1).count();

        let twos = min_layer.digits_iter().filter(|&d| d == 2).count();

        ones * twos
    }

    fn draw_image(&self) -> Vec<Vec<u8>> {
        self.layers_iter()
            .rev()
            .fold(vec![vec![0; self.width]; self.height], |mut img, layer| {
                for (x, y, pixel) in layer.rows_iter().enumerate().flat_map(|(y, row)| {
                    row.iter().enumerate().map(move |(x, pixel)| (x, y, pixel))
                }) {
                    match pixel {
                        0 => img[y][x] = 0,
                        1 => img[y][x] = 1,
                        _ => {}
                    }
                }

                img
            })
    }
}

fn print_image(img: &Image) {
    let img = img.draw_image();
    let pallet = [' ', '#', 'O'];

    for r in img.iter() {
        for &pixel in r.iter() {
            print!("{}", pallet[pixel as usize]);
        }
        println!();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() -> AocResult<()> {
        let data: Data = "123456789012".parse()?;
        let img = Image::from_data(data, 2, 3);
        assert_eq!(img.lowest_layer(), 1);

        let data: Data = parse_file(FileType::Input, 8, 1)?;
        let img = Image::from_data(data, 6, 25);
        assert_eq!(img.lowest_layer(), 2806);

        Ok(())
    }

    #[test]
    fn part2() -> AocResult<()> {
        let data: Data = "0222112222120000".parse()?;
        let img = Image::from_data(data, 2, 2);
        let img = img.draw_image();
        assert_eq!(&[0, 1], img[0].as_slice());
        assert_eq!(&[1, 0], img[1].as_slice());

        let data: Data = parse_file(FileType::Input, 8, 1)?;
        let img = Image::from_data(data, 6, 25);
        print_image(&img);

        Ok(())
    }
}

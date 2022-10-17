// https://www.hackerrank.com/challenges/3d-surface-area/problem
// the 3d surface area is the price

use std::convert::TryInto;
use std::error;
use std::io::{stdin, BufRead};
use std::ops::{Index, IndexMut};
use std::str::FromStr;

fn main() {
    let stdin = stdin();
    let mut stdin = stdin.lock();

    let input = Input::read(&mut stdin).unwrap();
    let solution = Solver::solve(input);

    println!("{}", solution);
}

#[derive(Debug)]
struct Input {
    width: usize,
    height: usize,
    matrix: Matrix<usize>,
}

impl Input {
    fn read(reader: &mut impl BufRead) -> Result<Self, Box<dyn error::Error>> {
        let [height, width]: [usize; 2] = Input::read_vec(reader)?.as_slice().try_into()?;
        let mut matrix = Vec::with_capacity(height * width);

        for _ in 0..height {
            let mut row = Input::read_vec(reader)?;
            matrix.append(&mut row);
        }

        Ok(Self {
            width,
            height,
            matrix: Matrix::new(matrix, width, height),
        })
    }

    fn read_vec<T>(reader: &mut impl BufRead) -> Result<Vec<T>, Box<dyn error::Error>>
    where
        T: FromStr,
        <T as FromStr>::Err: error::Error + 'static,
    {
        let mut buf = String::new();
        reader.read_line(&mut buf)?;

        buf.split(' ')
            .filter(|s| !s.is_empty())
            .map(|s| s.trim().parse())
            .collect::<Result<Vec<_>, _>>()
            .map_err(Into::into)
    }
}

#[derive(Debug)]
struct Matrix<T> {
    content: Vec<T>,
    width: usize,
    height: usize,
}

impl<T> Matrix<T> {
    fn new(content: Vec<T>, width: usize, height: usize) -> Self {
        assert_eq!(content.len(), width * height);

        Self {
            content,
            width,
            height,
        }
    }
}

// get the row
impl<T> Index<usize> for Matrix<T> {
    type Output = [T];

    fn index(&self, index: usize) -> &Self::Output {
        &self.content[index * self.width..index * self.width + self.width]
    }
}

// get the row
impl<T> IndexMut<usize> for Matrix<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.content[index * self.height..index * self.height + self.width]
    }
}

struct Solver;

impl Solver {
    fn solve(input: Input) -> usize {
        let Input {
            width,
            height,
            matrix,
        } = input;


        let around_sum = {
            let x_sum = (0..width).map(|x| matrix[0][x]).sum::<usize>()
                + (0..width).map(|x| matrix[height - 1][x]).sum::<usize>();

            let y_sum = (0..height).map(|y| matrix[y][0]).sum::<usize>()
                + (0..height).map(|y| matrix[y][width - 1]).sum::<usize>();

            x_sum + y_sum
        };

        let inner_sum: usize = {
            let x_axis = (1..width)
                .map(|x| {
                    (0..height)
                        .map(|y| (matrix[y][x] as i64 - matrix[y][x - 1] as i64).unsigned_abs())
                        .sum::<u64>() as usize
                })
                .sum::<usize>();

            let y_axis = (1..height)
                .map(|y| {
                    (0..width)
                        .map(|x| (matrix[y][x] as i64 - matrix[y - 1][x] as i64).unsigned_abs())
                        .sum::<u64>() as usize
                })
                .sum::<usize>();

            x_axis + y_axis
        };

        let z_axis_sum = matrix.content.into_iter().filter(|&x| x > 0).count() * 2;

        around_sum + inner_sum + z_axis_sum
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample0() {
        let stdin = "
        1 1
        1
        ";

        let mut stdin = stdin.trim().as_bytes();

        let input = Input::read(&mut stdin).unwrap();
        let solution = Solver::solve(input);

        assert_eq!(solution, 6);
    }

    #[test]
    fn sample1() {
        let stdin = "
        3 3
        1 3 4
        2 2 3
        1 2 4
        ";

        let mut stdin = stdin.trim().as_bytes();

        let input = Input::read(&mut stdin).unwrap();
        let solution = Solver::solve(input);

        assert_eq!(solution, 60);
    }

    #[test]
    fn test1() {
        let stdin = "
        10 1
        51
        32
        28
        49
        28
        21
        98
        56
        99
        77
        ";

        let mut stdin = stdin.trim().as_bytes();

        let input = Input::read(&mut stdin).unwrap();
        let solution = Solver::solve(input);

        assert_eq!(solution, 1482);
    }
}

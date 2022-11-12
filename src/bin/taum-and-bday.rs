// https://www.hackerrank.com/challenges/taum-and-bday/problem

use std::convert::TryInto;
use std::error;
use std::io::{stdin, BufRead};

fn main() {
    let stdin = stdin();
    let mut stdin = stdin.lock();
    let t: usize = Input::get_line(&mut stdin)
        .expect("Unable to read from stdin")
        .trim()
        .parse()
        .expect("Invalid amount of iterations");

    for _ in 0..t {
        let input = Input::read_from(&mut stdin).expect("Invalid input");
        let solution = Solver::solve(input);
        println!("{}", solution)
    }
}

#[derive(Debug)]
struct Input {
    b: usize,  // amount of required black gifts
    w: usize,  // amount of required white gifts
    bc: usize, // black gift cost
    wc: usize, // white gift cost
    z: usize,  // conversion cost, b -> w || w -> b
}

impl Input {
    fn read_from(reader: &mut impl BufRead) -> Result<Self, Box<dyn error::Error>> {
        let line = Input::get_line(reader)?;
        let [b, w]: [usize; 2] = line
            .trim()
            .split(' ')
            .map(str::parse)
            .collect::<Result<Vec<_>, _>>()?
            .as_slice()
            .try_into()?;

        let line = Input::get_line(reader)?;
        let [bc, wc, z]: [usize; 3] = line
            .trim()
            .split(' ')
            .map(str::parse)
            .collect::<Result<Vec<_>, _>>()?
            .as_slice()
            .try_into()?;

        Ok(Input { b, w, bc, wc, z })
    }

    fn get_line(reader: &mut impl BufRead) -> Result<String, Box<dyn error::Error>> {
        let mut buf = String::new();
        reader.read_line(&mut buf)?;
        Ok(buf)
    }
}

struct Solver;

impl Solver {
    // returns the minimum possible cost
    fn solve(input: Input) -> usize {
        let white_min_cost = input.wc.min(input.bc + input.z);
        let black_min_cost = input.bc.min(input.wc + input.z);

        white_min_cost * input.w + black_min_cost * input.b
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let stdin = "
        3 5
        3 4 1
        ";

        let mut stdin = stdin.trim().as_bytes();

        let input = Input::read_from(&mut stdin).unwrap();
        let solution = Solver::solve(input);
        assert_eq!(solution, 29);
    }

    #[test]
    fn sample() {
        let stdin = "
        5
        10 10
        1 1 1
        5 9
        2 3 4
        3 6
        9 1 1
        7 7
        4 2 1
        3 3
        1 9 2
        ";

        let mut stdin = stdin.trim().as_bytes();

        let t: usize = Input::get_line(&mut stdin).unwrap().trim().parse().unwrap();
        let solutions: Vec<usize> = (0..t)
            .map(|_| {
                let input = Input::read_from(&mut stdin).unwrap();
                #[allow(clippy::let_and_return)]
                let solution = Solver::solve(input);
                solution
            })
            .collect();

        assert_eq!(solutions, [20, 37, 12, 35, 12]);
    }
}

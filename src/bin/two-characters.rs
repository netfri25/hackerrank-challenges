// https://www.hackerrank.com/challenges/two-characters/problem

use std::error;
use std::io::{stdin, BufRead};

fn main() {
    let stdin = stdin();
    let mut stdin = stdin.lock();

    let input = Input::read_from(&mut stdin).expect("Invalid input");
    let solution = Solver::solve(input);
    println!("{}", solution);
}

struct Input(String);

impl Input {
    fn read_from(reader: &mut impl BufRead) -> Result<Self, Box<dyn error::Error>> {
        let mut buf = String::new();
        reader.read_line(&mut buf)?;
        buf.clear(); // ignore the first line, because I don't care about the length

        reader.read_line(&mut buf)?;
        Ok(Input(buf))
    }
}

struct Solver;

impl Solver {
    // returns the longest possible length of the output string
    fn solve(input: Input) -> usize {
        let text = input.0.into_bytes();
        let chars = (b'a'..=b'z').filter(|c| text.contains(c));

        chars
            .clone()
            .map(|a| chars.clone().map(move |b| (a, b)))
            .flatten()
            .filter(|(a, b)| a != b)
            .filter(|(a, b)| text.contains(a) || text.contains(b))
            .map(|(a, b)| {
                text.iter()
                    .filter(|c| **c == a || **c == b)
                    .collect::<Vec<_>>()
            })
            .filter(Solver::is_valid)
            .map(|s| s.len())
            .max()
            .unwrap_or_default()
    }

    fn is_valid(text: &Vec<&u8>) -> bool {
        text.iter().skip(1).zip(text.iter()).all(|(a, b)| a != b)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let stdin = "
        9
        abaacdabd
        ";

        let mut stdin = stdin.trim().as_bytes();

        let input = Input::read_from(&mut stdin).unwrap();
        let solution = Solver::solve(input);
        assert_eq!(solution, 4);
    }

    #[test]
    fn sample0() {
        let stdin = "
        10
        beabeefeab
        ";

        let mut stdin = stdin.trim().as_bytes();

        let input = Input::read_from(&mut stdin).unwrap();
        let solution = Solver::solve(input);
        assert_eq!(solution, 5);
    }

    #[test]
    fn sample1() {
        let stdin = "
        28
        asdcbsdcagfsdbgdfanfghbsfdab
        ";

        let mut stdin = stdin.trim().as_bytes();

        let input = Input::read_from(&mut stdin).unwrap();
        let solution = Solver::solve(input);
        assert_eq!(solution, 8);
    }

    #[test]
    fn sample2() {
        let stdin = "
        28
        asvkugfiugsalddlasguifgukvsa
        ";

        let mut stdin = stdin.trim().as_bytes();

        let input = Input::read_from(&mut stdin).unwrap();
        let solution = Solver::solve(input);
        assert_eq!(solution, 0);
    }

    #[test]
    fn my_test() {
        let stdin = "
        4
        aabb
        ";

        let mut stdin = stdin.trim().as_bytes();

        let input = Input::read_from(&mut stdin).unwrap();
        let solution = Solver::solve(input);
        assert_eq!(solution, 0);
    }
}

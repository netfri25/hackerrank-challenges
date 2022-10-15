// https://www.hackerrank.com/challenges/two-characters/problem

use std::collections::BTreeMap;
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
        let text = input.0;

        let mut letters: Vec<Option<char>> = text.chars().map(Some).collect();

        loop {
            let mut adj_char = true;
            while adj_char {
                adj_char = false;

                let text = letters.iter().filter(|c| c.is_some()).map(|c| c.unwrap());
                for (c1, c2) in text.clone().zip(text.clone().skip(1)) {
                    if c1 != c2 {
                        continue;
                    }

                    letters
                        .iter_mut()
                        .for_each(|c| *c = c.and_then(|c| if c == c1 { None } else { Some(c) }));

                    adj_char = true;
                    break;
                }
            }

            let text = letters.iter().filter(|c| c.is_some()).map(|c| c.unwrap());
            let mut letters_map: BTreeMap<char, usize> = BTreeMap::new();

            for c in text {
                *letters_map.entry(c).or_default() += 1;
            }

            if letters_map.len() == 2 {
                break;
            } else if letters_map.len() < 2 {
                return 0;
            }

            let min_c = letters_map
                .iter()
                .min_by(|kv1, kv2| kv1.1.cmp(kv2.1))
                .unwrap()
                .0;

            letters
                .iter_mut()
                .for_each(|c| *c = c.and_then(|c| if c == *min_c { None } else { Some(c) }));
        }

        letters.into_iter().filter(Option::is_some).count()
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
    fn sample() {
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

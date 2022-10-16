// https://www.hackerrank.com/challenges/happy-ladybugs/problem

use std::error;
use std::io::{stdin, BufRead};
use std::str::FromStr;

fn main() {
    let stdin = stdin();
    let mut stdin = stdin.lock();

    let t: usize = Input::read_line(&mut stdin).unwrap();

    for _ in 0..t {
        let input = Input::read(&mut stdin).unwrap();
        let solution = Solver::solve(input);

        println!("{}", if solution { "YES" } else { "NO" })
    }
}

struct Input(String);

impl Input {
    fn read(reader: &mut impl BufRead) -> Result<Self, Box<dyn error::Error>> {
        // ignore the first line, because I don't care about the length
        reader.read_line(&mut String::new())?;

        let line = Input::read_line(reader)?;
        Ok(Input(line))
    }

    fn read_line<T>(reader: &mut impl BufRead) -> Result<T, Box<dyn error::Error>>
    where
        T: FromStr,
        <T as FromStr>::Err: error::Error + 'static,
    {
        let mut buf = String::new();
        reader.read_line(&mut buf)?;
        Ok(buf.trim().parse()?)
    }
}

struct Solver;

impl Solver {
    fn solve(input: Input) -> bool {
        let text = input.0;
        let cells = text.as_bytes();

        let mut any_empty_cell = false;
        let mut any_not_adj = false;
        let mut colors_count = [0; 'Z' as usize - 'A' as usize + 1];

        for i in 0..cells.len() {
            if (b'A'..=b'Z').contains(&cells[i]) {
                colors_count[cells[i] as usize - 'A' as usize] += 1;

                let cond1 = i > 0 && cells[i] != cells[i - 1];
                let cond2 = i < cells.len() - 1 && cells[i] != cells[i + 1];
                any_not_adj |= cond1 && cond2;
            } else {
                any_empty_cell = true;
            }
        }

        let any_lonenly_color = colors_count.into_iter().any(|count| count == 1);

        (!any_not_adj || any_empty_cell) && !any_lonenly_color
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let stdin = "
        8
        YYR_B_BR
        ";

        let mut stdin = stdin.trim().as_bytes();

        let input = Input::read(&mut stdin).unwrap();
        let solution = Solver::solve(input);
        assert!(solution);
    }

    #[test]
    fn sample0() {
        let stdin = "
        4
        7
        RBY_YBR
        6
        X_Y__X
        2
        __
        6
        B_RRBR
        ";

        let mut stdin = stdin.trim().as_bytes();

        let t: usize = Input::read_line(&mut stdin).unwrap();

        let mut iter = (0..t).map(|_| {
            let input = Input::read(&mut stdin).unwrap();
            Solver::solve(input)
        });

        assert!(iter.next().unwrap());
        assert!(!iter.next().unwrap());
        assert!(iter.next().unwrap());
        assert!(iter.next().unwrap());

        assert!(iter.next().is_none());
    }

    #[test]
    fn sample1() {
        let stdin = "
        5
        5
        AABBC
        7
        AABBC_C
        1
        _
        10
        DD__FQ_QQF
        6
        AABCBC
        ";

        let mut stdin = stdin.trim().as_bytes();

        let t: usize = Input::read_line(&mut stdin).unwrap();

        let mut iter = (0..t).map(|_| {
            let input = Input::read(&mut stdin).unwrap();
            Solver::solve(input)
        });

        assert!(!iter.next().unwrap());
        assert!(iter.next().unwrap());
        assert!(iter.next().unwrap());
        assert!(iter.next().unwrap());
        assert!(!iter.next().unwrap());

        assert!(iter.next().is_none());
    }

    #[test]
    fn test1() {
        let stdin = "
        7
        1
        G
        2
        GR
        4
        _GR_
        5
        _R_G_
        5
        R_R_R
        8
        RRGGBBXX
        8
        RRGGBBXY
        ";

        let mut stdin = stdin.trim().as_bytes();

        let t: usize = Input::read_line(&mut stdin).unwrap();

        let mut iter = (0..t).map(|_| {
            let input = Input::read(&mut stdin).unwrap();
            Solver::solve(input)
        });

        assert!(!iter.next().unwrap());
        assert!(!iter.next().unwrap());
        assert!(!iter.next().unwrap());
        assert!(!iter.next().unwrap());
        assert!(iter.next().unwrap());
        assert!(iter.next().unwrap());
        assert!(!iter.next().unwrap());

        assert!(iter.next().is_none());
    }
}

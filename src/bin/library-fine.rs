use std::cmp::{Ordering, PartialOrd};
use std::error;
use std::io::{stdin, BufRead};
use std::convert::TryInto;

const DAY_FINE: usize = 15;
const MONTH_FINE: usize = 500;

fn main() {
    let stdin = stdin();
    let mut stdin = stdin.lock();

    let input = Input::read(&mut stdin).unwrap();
    let solution = Solver::solve(input);
    println!("{}", solution);
}

struct Input {
    return_date: Date,
    due_date: Date,
}

impl Input {
    fn read(reader: &mut impl BufRead) -> Result<Self, Box<dyn error::Error>> {
        let return_date = Input::read_date(reader)?;
        let due_date = Input::read_date(reader)?;

        Ok(Input {
            return_date,
            due_date,
        })
    }

    fn read_date(reader: &mut impl BufRead) -> Result<Date, Box<dyn error::Error>> {
        let mut buf = String::new();
        reader.read_line(&mut buf)?;

        let [d, m, y]: [usize; 3] = buf
            .trim()
            .split(' ')
            .map(|s| s.trim().parse())
            .collect::<Result<Vec<_>, _>>()?
            .as_slice()
            .try_into()?;

        Ok(Date::new(d, m, y))
    }
}

struct Solver;

impl Solver {
    fn solve(input: Input) -> usize {
        Solver::calculate_fine(input.return_date, input.due_date).unwrap_or(0)
    }

    fn calculate_fine(return_date: Date, due_date: Date) -> Option<usize> {
        if return_date < due_date {
            return None;
        }

        let Date {
            day: rd,
            month: rm,
            year: ry,
        } = return_date;

        let Date {
            day: dd,
            month: dm,
            year: dy,
        } = due_date;

        // when the book is returned after the due year, then there is a fixed fine of 10000
        let fine = if ry > dy {
            10000
        } else if rm > dm {
            (rm as usize - dm as usize) * MONTH_FINE
        } else {
            (rd - dd) * DAY_FINE
        };

        Some(fine)
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Date {
    pub day: usize,
    pub month: usize,
    pub year: usize,
}

impl Date {
    pub const fn new(day: usize, month: usize, year: usize) -> Self {
        Self { day, month, year }
    }
}

impl PartialOrd<Date> for Date {
    fn partial_cmp(&self, other: &Date) -> Option<Ordering> {
        let cmp = self
            .year
            .cmp(&other.year)
            .then_with(|| self.month.cmp(&other.month))
            .then_with(|| self.day.cmp(&other.day));

        Some(cmp)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample() {
        let stdin = "
        9 6 2015
        6 6 2015
        ";

        let mut stdin = stdin.trim().as_bytes();

        let input = Input::read(&mut stdin).unwrap();
        let solution = Solver::solve(input);
        assert_eq!(solution, 45);
    }
}

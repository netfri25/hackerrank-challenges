use std::error;
use std::io::{self, BufRead};
use std::str::FromStr;

fn main() {
    let stdin = io::stdin();
    let mut stdin = stdin.lock();
    let t: usize = Input::read_line(&mut stdin).unwrap();

    for _ in 0..t {
        let input = Input::read(&mut stdin).unwrap();
        let solution = Solver::solve(input);
        solution.for_each(|n| print!("{} ", n));
        println!();
    }
}

struct Input {
    n: usize, // the number of rocks
    a: usize, // one possible difference between the rocks
    b: usize, // another possible difference between the rocks
}

impl Input {
    fn read(reader: &mut impl BufRead) -> Result<Self, Box<dyn error::Error>> {
        Ok(Self {
            n: Input::read_line(reader)?,
            a: Input::read_line(reader)?,
            b: Input::read_line(reader)?,
        })
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
    fn solve(input: Input) -> impl Iterator<Item = usize> {
        let min = input.a.min(input.b);
        let max = input.a.max(input.b);
        let start = min * (input.n - 1);
        let end = max * (input.n - 1);
        let diff = max - min;
        (start..=end).step_by(diff.max(1)).take(input.n)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample0() {
        let stdin = "
        2
        3
        1
        2
        4
        10
        100
        ";

        let mut stdin = stdin.trim().as_bytes();

        let t: usize = Input::read_line(&mut stdin).unwrap();

        let mut iter = (0..t).map(|_| {
            let input = Input::read(&mut stdin).unwrap();
            let solution = Solver::solve(input);
            solution.collect::<Vec<_>>()
        });

        assert_eq!(iter.next(), Some(vec![2, 3, 4]));
        assert_eq!(iter.next(), Some(vec![30, 120, 210, 300]));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn sample1() {
        let stdin = "
        2
        7
        9
        11
        4
        8
        16
        ";

        let mut stdin = stdin.trim().as_bytes();

        let t: usize = Input::read_line(&mut stdin).unwrap();

        let mut iter = (0..t).map(|_| {
            let input = Input::read(&mut stdin).unwrap();
            let solution = Solver::solve(input);
            solution.collect::<Vec<_>>()
        });

        assert_eq!(iter.next(), Some(vec![54, 56, 58, 60, 62, 64, 66]));
        assert_eq!(iter.next(), Some(vec![24, 32, 40, 48]));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn test3() {
        let stdin = "
        5
        58
        69
        24
        83
        86
        81
        73
        25
        25
        12
        73
        82
        5
        3
        23
        ";

        let mut stdin = stdin.trim().as_bytes();

        let t: usize = Input::read_line(&mut stdin).unwrap();

        let mut iter = (0..t).map(|_| {
            let input = Input::read(&mut stdin).unwrap();
            let solution = Solver::solve(input);
            solution.collect::<Vec<_>>()
        });

        assert_eq!(
            iter.next(),
            Some(vec![
                1368, 1413, 1458, 1503, 1548, 1593, 1638, 1683, 1728, 1773, 1818, 1863, 1908, 1953,
                1998, 2043, 2088, 2133, 2178, 2223, 2268, 2313, 2358, 2403, 2448, 2493, 2538, 2583,
                2628, 2673, 2718, 2763, 2808, 2853, 2898, 2943, 2988, 3033, 3078, 3123, 3168, 3213,
                3258, 3303, 3348, 3393, 3438, 3483, 3528, 3573, 3618, 3663, 3708, 3753, 3798, 3843,
                3888, 3933
            ])
        );

        assert_eq!(
            iter.next(),
            Some(vec![
                6642, 6647, 6652, 6657, 6662, 6667, 6672, 6677, 6682, 6687, 6692, 6697, 6702, 6707,
                6712, 6717, 6722, 6727, 6732, 6737, 6742, 6747, 6752, 6757, 6762, 6767, 6772, 6777,
                6782, 6787, 6792, 6797, 6802, 6807, 6812, 6817, 6822, 6827, 6832, 6837, 6842, 6847,
                6852, 6857, 6862, 6867, 6872, 6877, 6882, 6887, 6892, 6897, 6902, 6907, 6912, 6917,
                6922, 6927, 6932, 6937, 6942, 6947, 6952, 6957, 6962, 6967, 6972, 6977, 6982, 6987,
                6992, 6997, 7002, 7007, 7012, 7017, 7022, 7027, 7032, 7037, 7042, 7047, 7052
            ])
        );

        assert_eq!(iter.next(), Some(vec![1800]));

        assert_eq!(iter.next(), Some(vec![803, 812, 821, 830, 839, 848, 857, 866, 875, 884, 893, 902]));

        assert_eq!(iter.next(), Some(vec![12, 32, 52, 72, 92]));

        assert_eq!(iter.next(), None);
    }
}

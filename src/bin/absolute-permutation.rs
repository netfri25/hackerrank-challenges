// https://www.hackerrank.com/challenges/absolute-permutation/problem

use std::convert::TryInto;
use std::error;
use std::io::{stdin, BufRead};
use std::str::FromStr;

fn main() {
	let stdin = stdin();
	let mut stdin = stdin.lock();

	let t: i32 = Input::read_line(&mut stdin).unwrap();

	for _ in 0..t {
		let input = Input::read(&mut stdin).unwrap();
		let solution = Solver::solve(input);

		solution.into_iter().for_each(|x| print!("{} ", x));
		println!()
	}
}

struct Input {
	n: usize, // 1..=n
	k: usize, // the absolute difference between the element and its index
}

impl Input {
	fn read(reader: &mut impl BufRead) -> Result<Self, Box<dyn error::Error>> {
		let mut buf = String::new();
		reader.read_line(&mut buf)?;

		let [n, k]: [usize; 2] = buf
			.trim()
			.split(' ')
			.map(|s| s.trim().parse())
			.collect::<Result<Vec<_>, _>>()?
			.as_slice()
			.try_into()?;

		Ok(Self { n, k })
	}

	fn read_line<T>(reader: &mut impl BufRead) -> Result<T, Box<dyn error::Error>>
	where
		T: FromStr,
		<T as FromStr>::Err: error::Error + 'static,
	{
		let mut buf = String::new();
		reader.read_line(&mut buf)?;
		buf.trim().parse().map_err(Into::into)
	}
}

struct Solver;

impl Solver {
	fn solve(input: Input) -> Vec<i64> {
		let Input { k, n } = input;

		if k != 0 && n % (2 * k) != 0 {
			return vec![-1];
		}

		let mut ans = vec![0; n];

		for i in 0..n {
			if ans[i] == 0 {
				let val = i+k;
				ans[i] = (val+1) as i64;
				ans[val] = (i+1) as i64;
			}
		}

		ans
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn sample() {
		let stdin = "
		3
		2 1
		3 0
		3 2
		";

		let mut stdin = stdin.trim().as_bytes();

		let t: i32 = Input::read_line(&mut stdin).unwrap();

		let mut iter = (0..t).map(|_| {
			let input = Input::read(&mut stdin).unwrap();
			Solver::solve(input)
		});

		assert_eq!(iter.next().unwrap(), vec![2, 1]);
		assert_eq!(iter.next().unwrap(), vec![1, 2, 3]);
		assert_eq!(iter.next().unwrap(), vec![-1]);
		assert!(iter.next().is_none());
	}
}

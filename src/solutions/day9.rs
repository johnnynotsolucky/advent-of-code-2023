use crate::Solution;
use atoi::atoi;

#[cfg(test)]
const SIZE: usize = 6;

#[cfg(not(test))]
const SIZE: usize = 21;

pub struct Day9;

impl Solution for Day9 {
	fn part1(input: &str) -> String {
		let mut sequence = [0; SIZE];

		let mut sum = 0;
		for line in input.lines() {
			sum += next_value(line, &mut sequence, false);
		}

		sum.to_string()
	}

	fn part2(input: &str) -> String {
		let mut sequence = [0; SIZE];

		let mut sum = 0;
		for line in input.lines() {
			sum += next_value(line, &mut sequence, true);
		}

		sum.to_string()
	}
}

fn next_value(line: &str, sequence: &mut [i32; SIZE], reversed: bool) -> i32 {
	let items = line
		.as_bytes()
		.split(|x| *x == b' ')
		.map(|item| atoi(item).unwrap())
		.enumerate();

	for (index, item) in items {
		sequence[index] = item;
	}

	let mut next = 0i32;
	let mut len = SIZE;
	loop {
		let mut all_zero = true;

		for index in 0..len - 1 {
			let (current, previous) = if reversed {
				(SIZE - index - 2, SIZE - index - 1)
			} else {
				(index + 1, index)
			};

			let left = sequence[previous];
			let right = sequence[current];
			let diff = right - left;

			sequence[previous] = diff;

			if diff != 0 {
				all_zero = false;
			}

			if index == len - 2 {
				next += right;
			}
		}

		len -= 1;

		if all_zero {
			break;
		}
	}
	next
}

#[cfg(test)]
mod test {
	use crate::{Day9, Solution};

	const INPUT: &str = r#"0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45"#;

	#[test]
	fn test_part1() {
		assert_eq!(Day9::part1(INPUT), 114.to_string());
	}

	#[test]
	fn test_part2() {
		assert_eq!(Day9::part2(INPUT), 2.to_string());
	}
}

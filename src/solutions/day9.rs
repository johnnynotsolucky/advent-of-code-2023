use crate::Solution;
use atoi::atoi;

pub struct Day9;

impl Solution for Day9 {
	fn part1(input: &str) -> String {
		input
			.lines()
			.map(into_sequence(false))
			.map(next_value)
			.sum::<i32>()
			.to_string()
	}

	fn part2(input: &str) -> String {
		input
			.lines()
			.map(into_sequence(true))
			.map(next_value)
			.sum::<i32>()
			.to_string()
	}
}

fn into_sequence(reverse: bool) -> impl Fn(&str) -> Vec<i32> {
	move |line: &str| {
		let iterator = line
			.split_ascii_whitespace()
			.map(|item| atoi(item.as_bytes()).unwrap());

		if reverse {
			iterator.rev().collect()
		} else {
			iterator.collect()
		}
	}
}

fn next_value(mut sequence: Vec<i32>) -> i32 {
	let mut next = 0i32;
	loop {
		let mut all_zero = true;
		for index in 1..sequence.len() {
			let left = sequence[index - 1];
			let right = sequence[index];
			let diff = right - left;

			sequence[index - 1] = diff;
			if diff != 0 {
				all_zero = false;
			}

			if index == sequence.len() - 1 {
				next += right;
			}
		}

		sequence.pop();
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

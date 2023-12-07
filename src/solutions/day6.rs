use crate::Solution;
use atoi::atoi;
use itertools::Itertools;

pub struct Day6;

impl Solution for Day6 {
	fn part1(input: &str) -> String {
		fn into_usize_iter(line: &str) -> impl Iterator<Item = usize> + '_ {
			line.split_once(':')
				.unwrap()
				.1
				.split_ascii_whitespace()
				.map(|x| atoi::<usize>(x.as_bytes()).unwrap())
		}

		fn into_pairs(input: &str) -> impl Iterator<Item = (usize, usize)> + '_ {
			let (time, distance) = input.split_once('\n').unwrap();
			into_usize_iter(time).zip(into_usize_iter(distance))
		}

		let pairs = into_pairs(input);

		let res = pairs.map(calculate_duration).product::<usize>();

		format!("{res}")
	}

	fn part2(input: &str) -> String {
		fn as_number(line: &str) -> usize {
			atoi::<usize>(
				line.split_once(':')
					.unwrap()
					.1
					.split_ascii_whitespace()
					.join("")
					.as_bytes(),
			)
			.unwrap()
		}
		let (time, distance) = input.split_once('\n').unwrap();
		let time = as_number(time);
		let distance = as_number(distance);

		let res = calculate_duration((time, distance));

		format!("{res}")
	}
}

#[inline]
fn calculate_duration((time, distance): (usize, usize)) -> usize {
	let mut min_duration = 0;
	for duration in 1..time / 2 {
		let remaining = time - duration;
		let final_distance = remaining * duration;
		if final_distance > distance {
			min_duration = duration;
			break;
		}
	}

	let max_duration = time - min_duration;

	max_duration - min_duration + 1
}

#[cfg(test)]
mod test {
	use crate::{Day6, Solution};

	const INPUT: &str = r#"Time:      7  15   30
Distance:  9  40  200"#;

	#[test]
	fn test_part1() {
		assert_eq!(Day6::part1(INPUT), 288.to_string());
	}

	#[test]
	fn test_part2() {
		assert_eq!(Day6::part2(INPUT), 71503.to_string());
	}
}

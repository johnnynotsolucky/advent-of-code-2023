// Brute force hacky hacky

use crate::Solution;
use itertools::Itertools;

pub struct Day11;

impl Solution for Day11 {
	fn part1(input: &str) -> String {
		solve(input, 2)
	}

	fn part2(input: &str) -> String {
		solve(input, 1_000_000)
	}
}

fn solve(input: &str, expansion: usize) -> String {
	let mut galaxies = Vec::new();
	let mut empty_rows = Vec::new();
	let mut cols = Vec::new();

	for (y, line) in input.as_bytes().split(|b| *b == b'\n').enumerate() {
		let mut found_row_galaxy = false;
		for (x, c) in line.iter().enumerate() {
			if *c == b'#' {
				galaxies.push((x, y));
				cols.push(x);
				found_row_galaxy = true;
			}
		}

		if !found_row_galaxy {
			empty_rows.push(y);
		}
	}

	let cols = cols.into_iter().unique().collect::<Vec<_>>();

	let mut sum = 0;
	for pair in galaxies.into_iter().combinations(2) {
		let a = pair.first().unwrap();
		let b = pair.get(1).unwrap();

		let start_x = a.0.min(b.0);
		let end_x = a.0.max(b.0);
		let mut diff_x = end_x - start_x;

		for x in start_x..end_x {
			if !cols.contains(&x) {
				diff_x += expansion - 1;
			}
		}

		let start_y = a.1.min(b.1);
		let end_y = a.1.max(b.1);
		let mut diff_y = end_y - start_y;

		for y in start_y + 1..end_y {
			if empty_rows.contains(&y) {
				diff_y += expansion - 1;
			}
		}

		let path = diff_x + diff_y;

		sum += path;
	}

	sum.to_string()
}

#[cfg(test)]
mod test {
	use crate::{solutions::day11::solve, Day11, Solution};

	const INPUT: &str = r#"...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#....."#;

	#[test]
	fn test_part1() {
		assert_eq!(Day11::part1(INPUT), 374.to_string());
	}

	#[test]
	fn test_expansion_10() {
		assert_eq!(solve(INPUT, 10), 1030.to_string());
	}

	#[test]
	fn test_expansion_100() {
		assert_eq!(solve(INPUT, 100), 8410.to_string());
	}
}

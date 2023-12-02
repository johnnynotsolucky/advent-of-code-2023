use atoi::atoi;
use std::{cmp::max, time::Instant};

fn main() {
	let input = std::fs::read_to_string("inputs/day2.txt").expect("Input file should exist");

	let start = Instant::now();
	let part1 = part1(&input);
	let part1_elapsed = start.elapsed().as_micros() as f32 / 1000f32;

	let start = Instant::now();
	let part2 = part2(&input);
	let part2_elapsed = start.elapsed().as_micros() as f32 / 1000f32;

	println!("Part 1: {part1} ({part1_elapsed})");
	println!("Part 2: {part2} ({part2_elapsed})");
}

fn part1(input: &str) -> usize {
	let available_red_cubes = 12;
	let available_green_cubes = 13;
	let available_blue_cubes = 14;

	input
		.lines()
		.enumerate()
		.filter_map(|(index, line)| {
			let (_, game) = line.split_once(": ").unwrap();
			for set in game.split("; ") {
				let mut red_cubes = 0usize;
				let mut green_cubes = 0usize;
				let mut blue_cubes = 0usize;

				for toss in set.split(", ") {
					let (count, colour) = toss.split_once(" ").unwrap();
					let count = atoi::<usize>(count.as_bytes()).unwrap();
					match colour {
						"red" => red_cubes += count,
						"green" => green_cubes += count,
						"blue" => blue_cubes += count,
						_ => panic!("oops"),
					}

					if red_cubes > available_red_cubes {
						return None;
					}

					if green_cubes > available_green_cubes {
						return None;
					}

					if blue_cubes > available_blue_cubes {
						return None;
					}
				}
			}

			Some(index + 1)
		})
		.sum::<usize>()
}

fn part2(input: &str) -> usize {
	input
		.lines()
		.map(|line| {
			let (_, game) = line.split_once(": ").unwrap();

			let mut red_cubes = 0usize;
			let mut green_cubes = 0usize;
			let mut blue_cubes = 0usize;

			for set in game.split("; ") {
				for toss in set.split(", ") {
					let (count, colour) = toss.split_once(' ').unwrap();
					let count = atoi::<usize>(count.as_bytes()).unwrap();
					match colour {
						"red" => red_cubes = max(red_cubes, count),
						"green" => green_cubes = max(green_cubes, count),
						"blue" => blue_cubes = max(blue_cubes, count),
						_ => panic!("oops"),
					}
				}
			}

			red_cubes * green_cubes * blue_cubes
		})
		.sum::<usize>()
}

#[cfg(test)]
mod test {
	use crate::{part1, part2};

	const INPUT: &str = r#"Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"#;

	#[test]
	fn test_part1() {
		assert_eq!(part1(INPUT), 8usize);
	}

	#[test]
	fn test_part2() {
		assert_eq!(part2(INPUT), 2286usize);
	}
}

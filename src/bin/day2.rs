use atoi::atoi;
use nom::{
	branch::alt,
	bytes::complete::tag,
	character::complete::digit1,
	combinator::map,
	multi::separated_list1,
	sequence::{preceded, separated_pair},
	IResult,
};
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

type Game<'line> = (usize, Vec<Vec<(usize, &'line str)>>);
type GameResult<'line> = IResult<&'line str, Game<'line>>;

fn parse_game(line: &str) -> Game {
	let res: GameResult = separated_pair(
		preceded(
			tag("Game "),
			map(digit1, |s: &str| atoi::<usize>(s.as_bytes()).unwrap()),
		),
		tag(": "),
		separated_list1(
			tag("; "),
			separated_list1(
				tag(", "),
				separated_pair(
					map(digit1, |s: &str| atoi::<usize>(s.as_bytes()).unwrap()),
					tag(" "),
					alt((tag("red"), tag("green"), tag("blue"))),
				),
			),
		),
	)(line);

	let (_rest, game) = res.unwrap();
	game
}

fn part1(input: &str) -> usize {
	let available_red_cubes = 12;
	let available_green_cubes = 13;
	let available_blue_cubes = 14;

	input
		.lines()
		.filter_map(|line| {
			let (id, sets) = parse_game(line);

			for set in sets {
				let mut red_cubes = 0usize;
				let mut green_cubes = 0usize;
				let mut blue_cubes = 0usize;

				for (count, colour) in set {
					match colour {
						"red" => red_cubes += count,
						"green" => green_cubes += count,
						"blue" => blue_cubes += count,
						_ => panic!("oops"),
					}
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

			Some(id)
		})
		.sum::<usize>()
}

fn part2(input: &str) -> usize {
	input
		.lines()
		.map(|line| {
			let (_, sets) = parse_game(line);

			let mut red_cubes = 0usize;
			let mut green_cubes = 0usize;
			let mut blue_cubes = 0usize;

			for set in sets {
				for (count, colour) in set {
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

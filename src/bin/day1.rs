use atoi::atoi;
use nom::{
	branch::alt, bytes::complete::tag, character::complete::one_of, combinator::recognize, IResult,
};
use std::time::Instant;

fn main() {
	let input = std::fs::read_to_string("inputs/day1.txt").expect("Input file should exist");

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
	input
		.lines()
		.map(|line| {
			let first = line.chars().find_map(|c| atoi::<u8>(&[c as u8])).unwrap();

			let last = line
				.chars()
				.rev()
				.find_map(|c| atoi::<u8>(&[c as u8]))
				.unwrap();

			atoi::<usize>(format!("{first}{last}").as_bytes()).unwrap()
		})
		.sum()
}

fn map_result(res: &str) -> u8 {
	let found = match res {
		"one" => 1,
		"two" => 2,
		"three" => 3,
		"four" => 4,
		"five" => 5,
		"six" => 6,
		"seven" => 7,
		"eight" => 8,
		"nine" => 9,
		_ => atoi::<u8>(res.as_bytes()).unwrap(),
	};

	found
}

fn parser(line: &str) -> usize {
	let mut parse = alt((
		tag("one"),
		tag("two"),
		tag("three"),
		tag("four"),
		tag("five"),
		tag("six"),
		tag("seven"),
		tag("eight"),
		tag("nine"),
		recognize(one_of("1234566789")),
	));

	let mut numbers = vec![];

	for offset in 0..line.len() {
		let result: IResult<&str, &str> = parse(&line[offset..]);
		if let Ok((_rest, found)) = result {
			numbers.push(map_result(found));
		}
	}

	let first = numbers.first().unwrap();
	let last = numbers.last().unwrap_or_else(|| numbers.first().unwrap());

	atoi::<usize>(format!("{first}{last}").as_bytes()).unwrap()
}

fn part2(input: &str) -> usize {
	input.lines().map(|line| parser(line)).sum()
}

#[cfg(test)]
mod test {
	use crate::{part1, part2};

	const PART_1_INPUT: &str = r#"1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet"#;

	#[test]
	fn test_part1() {
		assert_eq!(part1(PART_1_INPUT), 142usize);
	}

	const PART_2_INPUT: &str = r#"two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen"#;

	#[test]
	fn test_part2() {
		assert_eq!(part2(PART_2_INPUT), 281usize);
	}
}

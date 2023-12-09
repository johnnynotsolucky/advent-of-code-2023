use crate::Solution;
use nom::{
	character::complete::{alphanumeric1, char, multispace0},
	sequence::{preceded, terminated, tuple},
	IResult,
};
use std::{collections::HashMap, str::Chars};

pub struct Day8;

impl Solution for Day8 {
	fn part1(input: &str) -> String {
		let (instructions, elements) = parse(input);

		let mut current_id = "AAA";
		let mut count = 0;
		for instruction in instructions.cycle() {
			if current_id == "ZZZ" {
				break;
			}

			let next = elements.get(current_id).unwrap();

			current_id = match instruction {
				'L' => next.0,
				'R' => next.1,
				_ => unreachable!(),
			};
			count += 1;
		}

		count.to_string()
	}

	fn part2(input: &str) -> String {
		let (instructions, elements) = parse(input);

		let mut current_ids = elements
			.keys()
			.filter(|key| key.ends_with('A'))
			.collect::<Vec<_>>();
		let mut counts = HashMap::new();

		for (iteration, instruction) in instructions.cycle().enumerate() {
			for id in current_ids.iter_mut() {
				let (next_left, next_right) = elements.get(&**id).unwrap();
				let next_id = match instruction {
					'L' => next_left,
					'R' => next_right,
					_ => unreachable!(),
				};

				*id = next_id;

				if id.ends_with('Z') {
					counts.entry(next_id).or_insert(iteration + 1);
				}
			}

			if current_ids.len() == counts.len() {
				break;
			}
		}

		let count = counts
			.values()
			.fold(1, |acc, count| num::integer::lcm(acc, *count));

		count.to_string()
	}
}

fn parse(input: &str) -> (Chars, HashMap<&str, (&str, &str)>) {
	let mut lines = input.lines();
	let instructions = lines.next().unwrap().chars();

	let elements = lines.skip(1).map(parse_element).collect::<HashMap<_, _>>();

	(instructions, elements)
}

fn parse_element(line: &str) -> (&str, (&str, &str)) {
	let result: IResult<&str, (&str, &str, &str)> = tuple((
		terminated(alphanumeric1, preceded(multispace0, char('='))),
		preceded(preceded(multispace0, char('(')), alphanumeric1),
		preceded(
			preceded(char(','), multispace0),
			terminated(alphanumeric1, char(')')),
		),
	))(line);

	let (_, (id, left, right)) = result.unwrap();

	(id, (left, right))
}

#[cfg(test)]
mod test {
	use crate::{Day8, Solution};

	const PART1_INPUT_A: &str = r#"RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)"#;

	const PART1_INPUT_B: &str = r#"LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)"#;

	#[test]
	fn test_part1_a() {
		assert_eq!(Day8::part1(PART1_INPUT_A), 2.to_string());
	}

	#[test]
	fn test_part1_b() {
		assert_eq!(Day8::part1(PART1_INPUT_B), 6.to_string());
	}

	const PART2_INPUT: &str = r#"LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)"#;

	#[test]
	fn test_part2() {
		assert_eq!(Day8::part2(PART2_INPUT), 6.to_string());
	}
}

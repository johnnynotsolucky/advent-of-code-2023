mod solutions;

pub use solutions::*;

pub trait Solution {
	fn run(part: u8, input: &str) -> String {
		match part {
			1 => Self::part1(input),
			2 => Self::part2(input),
			_ => panic!("Oops"),
		}
	}
	fn part1(input: &str) -> String;

	fn part2(input: &str) -> String;
}

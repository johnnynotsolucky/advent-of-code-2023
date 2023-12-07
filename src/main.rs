use aoc2023::*;

fn main() {
	let args = std::env::args().collect::<Vec<_>>();

	let day = args[1].parse::<u8>().unwrap();
	let part = args[2].parse().unwrap();

	let result = match day {
		1 => Day1::run(part, include_str!("../inputs/day1.txt")),
		2 => Day2::run(part, include_str!("../inputs/day2.txt")),
		3 => Day3::run(part, include_str!("../inputs/day3.txt")),
		4 => Day4::run(part, include_str!("../inputs/day4.txt")),
		5 => Day5::run(part, include_str!("../inputs/day5.txt")),
		6 => Day6::run(part, include_str!("../inputs/day6.txt")),
		7 => Day7::run(part, include_str!("../inputs/day7.txt")),
		8 => todo!(),
		9 => todo!(),
		10 => todo!(),
		11 => todo!(),
		12 => todo!(),
		13 => todo!(),
		14 => todo!(),
		15 => todo!(),
		16 => todo!(),
		17 => todo!(),
		18 => todo!(),
		19 => todo!(),
		20 => todo!(),
		21 => todo!(),
		22 => todo!(),
		23 => todo!(),
		24 => todo!(),
		25 => todo!(),
		_ => panic!("oops"),
	};

	println!("{result}");
}

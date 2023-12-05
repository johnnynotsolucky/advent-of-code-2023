use crate::Solution;
use atoi::atoi;
use std::ops::RangeInclusive;

pub struct Day3;

impl Solution for Day3 {
	fn part1(input: &str) -> String {
		let (numbers, symbols) = input.lines().enumerate().fold(
			(Vec::<Number>::new(), Vec::<Symbol>::new()),
			|acc, (y, line)| {
				let (mut numbers, mut symbols) = acc;

				let mut buff = NumberBuffer::default();

				for (x, c) in line.chars().enumerate() {
					if !c.is_numeric() && c != '.' {
						buff.close(&y, &mut numbers);

						symbols.push(Symbol {
							x: x as isize,
							y: y as isize,
						});
						continue;
					}

					if c.is_numeric() {
						if buff.start_x == -1 {
							buff.start_x = x as isize;
							buff.end_x = x as isize; // for single digit numbers
						} else {
							buff.end_x = x as isize; // for single digit numbers
						}
						buff.value.push(c);

						continue;
					}

					buff.close(&y, &mut numbers);
				}

				buff.close(&y, &mut numbers);

				(numbers, symbols)
			},
		);

		format!(
			"{}",
			numbers
				.iter()
				.filter_map(|number| {
					for symbol in symbols.iter() {
						if symbol.adjacent_to(number) {
							return Some(number.value);
						}
					}

					None
				})
				.sum::<usize>()
		)
	}

	fn part2(input: &str) -> String {
		let (numbers, symbols) = input.lines().enumerate().fold(
			(Vec::<Number>::new(), Vec::<Symbol>::new()),
			|acc, (y, line)| {
				let (mut numbers, mut symbols) = acc;

				let mut buff = NumberBuffer::default();

				for (x, c) in line.chars().enumerate() {
					if c == '*' {
						buff.close(&y, &mut numbers);

						symbols.push(Symbol {
							x: x as isize,
							y: y as isize,
						});
						continue;
					}

					if c.is_numeric() {
						if buff.start_x == -1 {
							buff.start_x = x as isize;
							buff.end_x = x as isize; // for single digit numbers
						} else {
							buff.end_x = x as isize; // for single digit numbers
						}
						buff.value.push(c);

						continue;
					}

					buff.close(&y, &mut numbers);
				}

				buff.close(&y, &mut numbers);

				(numbers, symbols)
			},
		);

		format!(
			"{}",
			symbols
				.iter()
				.filter_map(|symbol| {
					let mut first: Option<usize> = None;
					let mut second: Option<usize> = None;

					for number in numbers.iter() {
						if symbol.adjacent_to(number) {
							if first.is_some() && second.is_some() {
								return None; // More than 2
							}

							if first.is_none() {
								first = Some(number.value);
								continue;
							} else if second.is_none() {
								second = Some(number.value);
								continue;
							}
						}
					}

					match (first, second) {
						(Some(first), Some(second)) => Some(first * second),
						_ => None,
					}
				})
				.sum::<usize>()
		)
	}
}

#[derive(Debug)]
struct Number {
	value: usize,
	x: RangeInclusive<isize>,
	y: isize,
}

#[derive(Debug)]
struct Symbol {
	x: isize,
	y: isize,
}

impl Symbol {
	#[inline]
	fn adjacent_to(&self, number: &Number) -> bool {
		let diff_y = (self.y - number.y).abs();

		for x in number.x.clone() {
			let diff_x = (self.x - x).abs();
			if diff_y == 0 && diff_x == 1 {
				// same row, symbol on left/right of number
				return true;
			}

			if diff_y == 1 && diff_x <= 1 {
				return true;
			}
		}

		false
	}
}

struct NumberBuffer {
	value: String,
	start_x: isize,
	end_x: isize,
}

impl Default for NumberBuffer {
	fn default() -> Self {
		Self {
			value: String::new(),
			start_x: -1,
			end_x: -1,
		}
	}
}

impl NumberBuffer {
	#[inline]
	fn is_empty(&self) -> bool {
		self.value.is_empty()
	}

	#[inline]
	fn reset(&mut self) {
		self.value.clear();
		self.start_x = -1;
		self.end_x = -1;
	}

	#[inline]
	fn close(&mut self, y: &usize, numbers: &mut Vec<Number>) {
		if !self.is_empty() {
			let number = Number {
				value: atoi::<usize>(self.value.as_bytes()).unwrap(),
				x: self.start_x..=self.end_x,
				y: *y as isize,
			};
			numbers.push(number);
			self.reset();
		}
	}
}

#[cfg(test)]
mod test {
	use crate::{Day3, Solution};

	const INPUT: &str = r#"
467..114..
...*......
..35..633.
......#...
617*......
.....+..58
..592.....
......755.
...$.*....
.664.598..
"#;

	#[test]
	fn test_part1() {
		assert_eq!(Day3::part1(INPUT), 4361.to_string());
	}

	#[test]
	fn test_part2() {
		assert_eq!(Day3::part2(INPUT), 467835.to_string());
	}
}

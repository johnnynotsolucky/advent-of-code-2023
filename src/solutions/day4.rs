use crate::Solution;

pub struct Day4;

impl Solution for Day4 {
	fn part1(input: &str) -> String {
		format!(
			"{}",
			input
				.lines()
				.enumerate()
				.map(Card::from)
				.map(|card| {
					if card.winning_count > 1 {
						2usize.pow(card.winning_count as u32 - 1)
					} else {
						card.winning_count
					}
				})
				.sum::<usize>()
		)
	}

	fn part2(input: &str) -> String {
		let mut copies = vec![1; input.lines().count()];
		input.lines().enumerate().map(Card::from).for_each(|card| {
			let current_card_copies = copies[card.index];

			for add in 1..=card.winning_count {
				copies[card.index + add] += current_card_copies;
			}
		});

		format!("{}", copies.iter().sum::<usize>())
	}
}

struct Card {
	index: usize,
	winning_count: usize,
}

impl From<(usize, &str)> for Card {
	fn from((index, line): (usize, &str)) -> Self {
		let winning_count = line
			.split(": ")
			.skip(1)
			.take(1)
			.map(|line| {
				let (left, right) = line.split_once('|').unwrap();
				let right = right.split_ascii_whitespace().collect::<Vec<_>>();

				left.split_ascii_whitespace()
					.filter_map(|left| right.iter().find(|right| left == **right))
					.count()
			})
			.last()
			.unwrap();

		Self {
			index,
			winning_count,
		}
	}
}

#[cfg(test)]
mod test {
	use crate::{Day4, Solution};

	const INPUT: &str = r#"Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11"#;

	#[test]
	fn test_part1() {
		assert_eq!(Day4::part1(INPUT), 13.to_string());
	}

	#[test]
	fn test_part2() {
		assert_eq!(Day4::part2(INPUT), 30.to_string());
	}
}

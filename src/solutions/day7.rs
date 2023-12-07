use crate::Solution;
use atoi::atoi;
use itertools::Itertools;
use std::{cmp::Ordering, collections::HashMap};

pub struct Day7;

impl Solution for Day7 {
	fn part1(input: &str) -> String {
		let mut hands = input
			.lines()
			.map(|line| Hand::from((line, false)))
			.collect::<Vec<_>>();

		hands.sort_unstable();

		let len = hands.len();
		let res = hands
			.into_iter()
			.enumerate()
			.map(|(index, hand)| (len - index) * hand.bid)
			.sum::<usize>();

		format!("{res}")
	}

	fn part2(input: &str) -> String {
		let mut hands = input
			.lines()
			.map(|line| Hand::from((line, true)))
			.collect::<Vec<_>>();

		hands.sort_unstable();

		let len = hands.len();
		let res = hands
			.into_iter()
			.enumerate()
			.map(|(index, hand)| (len - index) * hand.bid)
			.sum::<usize>();

		format!("{res}")
	}
}

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq)]
enum HandType {
	HighCard,
	OnePair,
	TwoPair,
	ThreeOfAKind,
	FullHouse,
	FourOfAKind,
	FiveOfAKind,
}

impl From<(&Vec<Card>, bool)> for HandType {
	fn from((value, joker_rule): (&Vec<Card>, bool)) -> Self {
		let mut grouped = HashMap::new();
		for card in value {
			grouped
				.entry(card)
				.and_modify(|count| *count += 1)
				.or_insert(1);
		}

		if joker_rule {
			if let Some(jokers) = grouped.remove(&Card::Joker) {
				if let Some((_, count)) = grouped
					.iter_mut()
					.sorted_by_key(|(_, count)| **count)
					.last()
				{
					*count += jokers;
				} else {
					grouped.insert(&Card::Joker, jokers);
				}
			}
		}

		match grouped.len() {
			1 => HandType::FiveOfAKind,
			2 => {
				if grouped.iter().any(|(_, count)| *count == 4) {
					Self::FourOfAKind
				} else {
					Self::FullHouse
				}
			}
			3 => {
				if grouped.iter().any(|(_, count)| *count == 3) {
					Self::ThreeOfAKind
				} else {
					Self::TwoPair
				}
			}
			4 => Self::OnePair,
			5 => Self::HighCard,
			_ => panic!("oops"),
		}
	}
}

#[derive(Copy, Clone, Hash, Debug, Ord, PartialOrd, Eq, PartialEq)]
enum Card {
	Joker,
	N2,
	N3,
	N4,
	N5,
	N6,
	N7,
	N8,
	N9,
	Ten,
	Jack,
	Queen,
	King,
	Ace,
}

impl From<(char, bool)> for Card {
	fn from((value, joker_rule): (char, bool)) -> Self {
		match value {
			'2' => Self::N2,
			'3' => Self::N3,
			'4' => Self::N4,
			'5' => Self::N5,
			'6' => Self::N6,
			'7' => Self::N7,
			'8' => Self::N8,
			'9' => Self::N9,
			'T' => Self::Ten,
			'J' => {
				if joker_rule {
					Self::Joker
				} else {
					Self::Jack
				}
			}
			'Q' => Self::Queen,
			'K' => Self::King,
			'A' => Self::Ace,
			_ => panic!("oops"),
		}
	}
}

#[derive(Debug, Eq)]
struct Hand {
	cards: Vec<Card>,
	hand_type: HandType,
	bid: usize,
}

impl PartialEq<Self> for Hand {
	fn eq(&self, _other: &Self) -> bool {
		panic!()
	}
}

impl Ord for Hand {
	fn cmp(&self, other: &Self) -> Ordering {
		let ordering = other.hand_type.cmp(&self.hand_type);
		if ordering == Ordering::Equal {
			other
				.cards
				.iter()
				.zip(self.cards.iter())
				.find_map(|(left, right)| {
					let ordering = left.cmp(right);

					if ordering != Ordering::Equal {
						Some(ordering)
					} else {
						None
					}
				})
				.unwrap()
		} else {
			ordering
		}
	}
}

impl PartialOrd for Hand {
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		Some(self.cmp(other))
	}
}

impl From<(&str, bool)> for Hand {
	fn from((line, joker_rule): (&str, bool)) -> Self {
		let (hand, bid) = line.split_once(' ').unwrap();

		let cards = hand.chars().map(|c| Card::from((c, joker_rule))).collect();
		let hand_type = HandType::from((&cards, joker_rule));

		Self {
			cards,
			hand_type,
			bid: atoi(bid.as_bytes()).unwrap(),
		}
	}
}

#[cfg(test)]
mod test {
	use crate::{Day7, Solution};

	const INPUT: &str = r#"32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483"#;

	#[test]
	fn test_part1() {
		assert_eq!(Day7::part1(INPUT), 6440.to_string());
	}

	#[test]
	fn test_part2() {
		assert_eq!(Day7::part2(INPUT), 5905.to_string());
	}
}

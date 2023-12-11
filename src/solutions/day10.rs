use crate::Solution;
use std::collections::HashMap;

pub struct Day10;

impl Solution for Day10 {
	fn part1(input: &str) -> String {
		let (start_pos, grid) = into_grid(input);

		let [(mut left_pos, mut left_piece), (mut right_pos, mut right_piece)] =
			start_targets(start_pos, grid.get(&start_pos).unwrap(), &grid);

		// println!("START L {start_pos:?}: {left_pos:?}");
		// println!("START R {start_pos:?}: {right_pos:?}");

		let (mut prev_left_pos, mut prev_right_pos) = (start_pos, start_pos);

		let mut steps = 1;

		loop {
			// left first
			let next_left = find_next_piece((&left_pos, left_piece), &prev_left_pos, &grid);
			// println!("NEXT L {left_pos:?}: {next_left:?}");
			let next_right = find_next_piece((&right_pos, right_piece), &prev_right_pos, &grid);
			// println!("NEXT R {right_pos:?}: {next_right:?}");
			// println!("");

			if steps == 4 {
				// panic!("");
			}

			prev_left_pos = left_pos;
			prev_right_pos = right_pos;

			left_pos = next_left.0;
			left_piece = next_left.1;

			right_pos = next_right.0;
			right_piece = next_right.1;

			steps += 1;

			if left_pos == right_pos {
				// todo increment thing
				break;
			}
		}

		steps.to_string()
	}

	fn part2(input: &str) -> String {
		todo!()
	}
}

fn start_targets<'grid>(
	pos: (i32, i32),
	piece: &'grid Piece,
	grid: &'grid HashMap<(i32, i32), Piece>,
) -> [((i32, i32), &'grid Piece); 2] {
	let targets = targets((&pos, piece));

	let mut index = 0;
	let mut start_targets = [((0, 0), &Piece::Start); 2];

	for target in targets {
		if target.is_none() {
			break;
		}

		let target = target.unwrap();
		let candidate = grid.get(&target);
		if let Some(candidate) = candidate {
			if connected((&pos, piece), (&target, candidate)) {
				start_targets[index] = (target, candidate);
				index += 1;
				if index == 2 {
					break;
				}
			}
		}
	}

	start_targets
}

fn find_next_piece<'grid>(
	current: (&(i32, i32), &Piece),
	prev: &(i32, i32),
	grid: &'grid HashMap<(i32, i32), Piece>,
) -> ((i32, i32), &'grid Piece) {
	let targets = targets(current);

	for target in targets {
		if target.is_none() {
			break;
		}
		let target = target.unwrap();

		if target == *prev {
			continue;
		}

		if let Some(candidate) = grid.get(&target) {
			if connected(current, (&target, candidate)) {
				return (target, candidate);
			}
		}
	}

	unreachable!()
}

fn into_grid(input: &str) -> ((i32, i32), HashMap<(i32, i32), Piece>) {
	let mut start_pos = None;

	let mut grid = HashMap::new();

	for (y, line) in input.lines().enumerate() {
		for (x, c) in line.chars().enumerate() {
			if c == '.' {
				continue;
			}

			let position = (x as i32, y as i32);
			let piece = Piece::from(c);

			if let Piece::Start = &piece {
				start_pos = Some(position);
			}

			grid.insert(position, piece);
		}
	}

	(start_pos.unwrap(), grid)
}

#[derive(Debug)]
enum Piece {
	Start,
	NorthSouth, // Vertical
	EastWest,   // Horizontal
	NorthEast,  // L
	NorthWest,  // J
	SouthWest,  // 7
	SouthEast,  // F
}

impl From<char> for Piece {
	fn from(value: char) -> Self {
		match value {
			'S' => Self::Start,
			'|' => Self::NorthSouth,
			'-' => Self::EastWest,
			'L' => Self::NorthEast,
			'J' => Self::NorthWest,
			'7' => Self::SouthWest,
			'F' => Self::SouthEast,
			_ => unreachable!(),
		}
	}
}

fn targets(current: (&(i32, i32), &Piece)) -> [Option<(i32, i32)>; 4] {
	let pos = current.0;
	match current.1 {
		Piece::Start => [
			Some((pos.0, pos.1 - 1)), // North
			Some((pos.0 + 1, pos.1)), // East
			Some((pos.0, pos.1 + 1)), // South
			Some((pos.0 - 1, pos.1)), // West
		],
		Piece::NorthSouth => [
			Some((pos.0, pos.1 - 1)), // North
			Some((pos.0, pos.1 + 1)), // South
			None,
			None,
		],
		Piece::EastWest => [
			Some((pos.0 + 1, pos.1)), // East
			Some((pos.0 - 1, pos.1)), // West
			None,
			None,
		],
		Piece::NorthEast => [
			Some((pos.0, pos.1 - 1)), // North
			Some((pos.0 + 1, pos.1)), // East
			None,
			None,
		],
		Piece::NorthWest => [
			Some((pos.0, pos.1 - 1)), // North
			Some((pos.0 - 1, pos.1)), // West
			None,
			None,
		],
		Piece::SouthWest => [
			Some((pos.0, pos.1 + 1)), // South
			Some((pos.0 - 1, pos.1)), // West
			None,
			None,
		],
		Piece::SouthEast => [
			Some((pos.0, pos.1 + 1)), // South
			Some((pos.0 + 1, pos.1)), // East
			None,
			None,
		],
	}
}

fn connected(current: (&(i32, i32), &Piece), candidate: (&(i32, i32), &Piece)) -> bool {
	let orientation = Orientation::new(current.0, candidate.0);

	let res = match orientation {
		Orientation::Horizontal => {
			if current.0 .0 > candidate.0 .0 {
				// Going West
				matches!(
					current.1,
					Piece::Start | Piece::EastWest | Piece::NorthWest | Piece::SouthWest
				) && matches!(
					candidate.1,
					Piece::EastWest | Piece::NorthEast | Piece::SouthEast
				)
			} else {
				// Going East
				matches!(
					current.1,
					Piece::Start | Piece::EastWest | Piece::NorthEast | Piece::SouthEast
				) && matches!(
					candidate.1,
					Piece::EastWest | Piece::NorthWest | Piece::SouthWest
				)
			}
		}
		Orientation::Vertical => {
			if current.0 .1 > candidate.0 .1 {
				// Going North
				matches!(
					current.1,
					Piece::Start | Piece::NorthSouth | Piece::NorthEast | Piece::NorthWest
				) && matches!(
					candidate.1,
					Piece::NorthSouth | Piece::SouthEast | Piece::SouthWest
				)
			} else {
				// Going South
				matches!(
					current.1,
					Piece::Start | Piece::NorthSouth | Piece::SouthEast | Piece::SouthWest
				) && matches!(
					candidate.1,
					Piece::NorthSouth | Piece::NorthEast | Piece::NorthWest
				)
			}
		}
	};

	res
}

#[derive(Debug)]
enum Orientation {
	Horizontal,
	Vertical,
}

impl Orientation {
	fn new(from: &(i32, i32), to: &(i32, i32)) -> Self {
		if from.0 != to.0 {
			Self::Horizontal
		} else {
			Self::Vertical
		}
	}
}

#[cfg(test)]
mod test {
	use crate::{Day10, Solution};

	const INPUT_SIMPLE: &str = r#"-L|F7
7S-7|
L|7||
-L-J|
L|-JF"#;

	const INPUT_COMPLEX: &str = r#"7-F7-
.FJ|7
SJLL7
|F--J
LJ.LJ"#;

	#[test]
	fn test_part1_simple() {
		assert_eq!(Day10::part1(INPUT_SIMPLE), 4.to_string());
	}

	#[test]
	fn test_part1_complex() {
		assert_eq!(Day10::part1(INPUT_COMPLEX), 8.to_string());
	}
	//
	// #[test]
	// fn test_part2() {
	// 	assert_eq!(Day10::part2(INPUT_SIMPLE), 0.to_string());
	// }
}

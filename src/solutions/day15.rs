use crate::Solution;
use arrayvec::ArrayVec;
use atoi::atoi;

pub struct Day15;

impl Solution for Day15 {
	fn part1(input: &str) -> String {
		let steps = input.as_bytes().split(|b| *b == b',');
		let mut sum = 0;

		for step in steps {
			sum += hash(step);
		}

		sum.to_string()
	}

	fn part2(input: &str) -> String {
		let mut boxes = ArrayVec::<ArrayVec<Lens, 10>, 256>::new();
		for _ in 0..256 {
			boxes.push(ArrayVec::new());
		}

		let steps = input.as_bytes().split(|b| *b == b',');

		for step in steps {
			if step.ends_with(&[b'-']) {
				let label = &step[..step.len() - 1];
				let hashed = hash(label) as usize;
				let lens_box = &mut boxes[hashed];
				if let Some(index) = lens_box.iter().position(|lens| lens.label == label) {
					lens_box.remove(index);
				}
			} else {
				let label = &step[..step.len() - 2];
				let hashed = hash(label) as usize;
				let lens_box = &mut boxes[hashed];
				if let Some(index) = lens_box.iter().position(|lens| lens.label == label) {
					let lens = &mut lens_box[index];
					let start = step.len() - 1;
					lens.focal_length = atoi(&step[start..start + 1]).unwrap();
				} else {
					let start = step.len() - 1;
					lens_box.push(Lens::new(label, atoi(&step[start..start + 1]).unwrap()));
				}
			}
		}

		let mut focusing_power = 0;
		for (box_index, lens_box) in boxes.into_iter().enumerate() {
			for (pos, lens) in lens_box.iter().enumerate() {
				focusing_power += (box_index + 1) * (pos + 1) * lens.focal_length as usize;
			}
		}

		focusing_power.to_string()
	}
}

#[inline]
fn hash(bytes: &[u8]) -> u16 {
	let mut current: u16 = 0;
	for b in bytes {
		current += *b as u16;
		current *= 17;
		current %= 256;
	}

	current
}

#[derive(Debug)]
struct Lens<'a> {
	label: &'a [u8],
	focal_length: u8,
}

impl<'a> Lens<'a> {
	#[inline]
	fn new(label: &'a [u8], focal_length: u8) -> Self {
		Self {
			label,
			focal_length,
		}
	}
}

#[cfg(test)]
mod test {
	use crate::{Day15, Solution};

	const INPUT: &str = r#"rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7"#;

	#[test]
	fn test_part1() {
		assert_eq!(Day15::part1(INPUT), 1320.to_string());
	}

	#[test]
	fn test_part2() {
		assert_eq!(Day15::part2(INPUT), 145.to_string());
	}
}

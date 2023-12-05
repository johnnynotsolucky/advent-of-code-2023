use crate::Solution;
use atoi::atoi;
use itertools::Itertools;
use std::{collections::VecDeque, ops::Range};

pub struct Day5;

impl Solution for Day5 {
	fn part1(input: &str) -> String {
		let mut groups = input.split("\n\n").collect::<VecDeque<_>>();

		let seeds = groups
			.pop_front()
			.unwrap()
			.split_ascii_whitespace()
			.skip(1)
			.map(|seed| {
				let seed = atoi::<usize>(seed.as_bytes()).unwrap();
				seed..seed+1
			})
			.collect::<Vec<_>>();

		find_closest(seeds, groups)
	}

	fn part2(input: &str) -> String {
		let mut groups = input.split("\n\n").collect::<VecDeque<_>>();

		let seeds = groups
			.pop_front()
			.unwrap()
			.split_ascii_whitespace()
			.skip(1)
			.map(|seed| atoi::<usize>(seed.as_bytes()).unwrap())
			.tuples::<(usize, usize)>()
			.map(|(start, len)| start..start + len)
			.collect::<Vec<_>>();

		find_closest(seeds, groups)
	}
}

#[derive(Clone)]
struct Map {
	dest: usize,
	source: usize,
	len: usize,
}

impl From<&str> for Map {
	fn from(value: &str) -> Self {
		let parts = value
			.split_ascii_whitespace()
			.map(|v| atoi::<usize>(v.as_bytes()).unwrap())
			.collect::<Vec<_>>();

		Self {
			dest: parts[0],
			source: parts[1],
			len: parts[2],
		}
	}
}

fn find_closest(seeds: Vec<Range<usize>>, groups: VecDeque<&str>) -> String {
	let groups = groups
		.into_iter()
		.map(|group| {
			let mut group = group.lines().skip(1).map(Map::from).collect::<Vec<_>>();
			group.sort_unstable_by(|g1, g2| g1.source.partial_cmp(&g2.source).unwrap());
			group
		})
		.collect::<Vec<_>>();

	let mut out = groups.into_iter().fold(seeds, |current, group| {
		let mut next = vec![];
		for seed_range in current {
			next.extend(ranges_from_source(seed_range, group.clone()));
		}

		next
	});

	out.sort_unstable_by(|a, b| a.start.partial_cmp(&b.start).unwrap());
	format!("{}", out.first().unwrap().start)
}

fn ranges_from_source(source: Range<usize>, group: Vec<Map>) -> Vec<Range<usize>> {
	let mut out = vec![];
	for map in group.clone().into_iter() {
		let mapped_range = map.source..map.source + map.len;

		let intersect_start = source.start.max(mapped_range.start);
		let intersect_end = source.end.min(mapped_range.end);

		if intersect_start >= intersect_end {
			continue;
		}

		let dest = map.dest..map.dest + map.len;
		let target_start = dest.start + (intersect_start - mapped_range.start);
		let target_end = target_start + (intersect_end - intersect_start);

		out.push(target_start..target_end);

		if source.start < intersect_start {
			out.extend(ranges_from_source(
				source.start..intersect_start,
				group.clone(),
			));
		}
		if source.end > intersect_end {
			out.extend(ranges_from_source(intersect_end..source.end, group.clone()));
		}
	}

	if out.is_empty() {
		out.push(source);
	}

	out
}

#[cfg(test)]
mod test {
	use crate::{Day5, Solution};

	const INPUT: &str = r#"seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4"#;

	#[test]
	fn test_part1() {
		assert_eq!(Day5::part1(INPUT), 35.to_string());
	}

	#[test]
	fn test_part2() {
		assert_eq!(Day5::part2(INPUT), 46.to_string());
	}
}

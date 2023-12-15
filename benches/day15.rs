use aoc2023::Solution;
use criterion::{black_box, criterion_group, criterion_main, Criterion};

const INPUT: &str = include_str!("../inputs/day15.txt");

fn part1(c: &mut Criterion) {
	c.bench_function("day15 part 1", |b| {
		b.iter(|| aoc2023::Day15::part1(black_box(INPUT)))
	});
}

fn part2(c: &mut Criterion) {
	c.bench_function("day15 part 2", |b| {
		b.iter(|| aoc2023::Day15::part2(black_box(INPUT)))
	});
}

criterion_group!(day15_benches, part1, part2);
criterion_main!(day15_benches);

use aoc2023::Solution;
use criterion::{black_box, criterion_group, criterion_main, Criterion};

const INPUT: &str = include_str!("../inputs/day10.txt");

fn part1(c: &mut Criterion) {
	c.bench_function("day10 part 1", |b| {
		b.iter(|| aoc2023::Day10::part1(black_box(INPUT)))
	});
}

fn part2(c: &mut Criterion) {
	c.bench_function("day10 part 2", |b| {
		b.iter(|| aoc2023::Day10::part2(black_box(INPUT)))
	});
}

criterion_group!(day10_benches, part1, part2);
criterion_main!(day10_benches);

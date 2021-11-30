use aoc19::*;
use std::fs;
use std::time::Duration;

use criterion::{criterion_group, criterion_main, Criterion};

fn target_01(c: &mut Criterion) {
    let input = fs::read_to_string("inputs/day01.txt").unwrap();
    c.bench_function("day01", |b| b.iter(|| day01::solve(String::from(&input))));
}

fn target_02(c: &mut Criterion) {
    let input = fs::read_to_string("inputs/day02.txt").unwrap();
    c.bench_function("day02", |b| b.iter(|| day02::solve(String::from(&input))));
}

fn target_03(c: &mut Criterion) {
    let input = fs::read_to_string("inputs/day03.txt").unwrap();
    c.bench_function("day03", |b| b.iter(|| day03::solve(String::from(&input))));
}

fn target_04(c: &mut Criterion) {
    let input = fs::read_to_string("inputs/day04.txt").unwrap();
    c.bench_function("day04", |b| b.iter(|| day04::solve(String::from(&input))));
}

fn target_05(c: &mut Criterion) {
    let input = fs::read_to_string("inputs/day05.txt").unwrap();
    c.bench_function("day05", |b| b.iter(|| day05::solve(String::from(&input))));
}

fn target_06(c: &mut Criterion) {
    let input = fs::read_to_string("inputs/day06.txt").unwrap();
    c.bench_function("day06", |b| b.iter(|| day06::solve(String::from(&input))));
}

fn target_07(c: &mut Criterion) {
    let input = fs::read_to_string("inputs/day07.txt").unwrap();
    c.bench_function("day07", |b| b.iter(|| day07::solve(String::from(&input))));
}

fn target_08(c: &mut Criterion) {
    let input = fs::read_to_string("inputs/day08.txt").unwrap();
    c.bench_function("day08", |b| b.iter(|| day08::solve(String::from(&input))));
}

fn target_09(c: &mut Criterion) {
    let input = fs::read_to_string("inputs/day09.txt").unwrap();
    c.bench_function("day09", |b| b.iter(|| day09::solve(String::from(&input))));
}

fn target_10(c: &mut Criterion) {
    let input = fs::read_to_string("inputs/day10.txt").unwrap();
    c.bench_function("day10", |b| b.iter(|| day10::solve(String::from(&input))));
}

fn target_11(c: &mut Criterion) {
    let input = fs::read_to_string("inputs/day11.txt").unwrap();
    c.bench_function("day11", |b| b.iter(|| day11::solve(String::from(&input))));
}

fn target_12(c: &mut Criterion) {
    let input = fs::read_to_string("inputs/day12.txt").unwrap();
    c.bench_function("day12", |b| b.iter(|| day12::solve(String::from(&input))));
}

fn target_13(c: &mut Criterion) {
    let input = fs::read_to_string("inputs/day13.txt").unwrap();
    c.bench_function("day13", |b| b.iter(|| day13::solve(String::from(&input))));
}

fn target_14(c: &mut Criterion) {
    let input = fs::read_to_string("inputs/day14.txt").unwrap();
    c.bench_function("day14", |b| b.iter(|| day14::solve(String::from(&input))));
}

fn target_15(c: &mut Criterion) {
    let input = fs::read_to_string("inputs/day15.txt").unwrap();
    c.bench_function("day15", |b| b.iter(|| day15::solve(String::from(&input))));
}

fn target_16(c: &mut Criterion) {
    let input = fs::read_to_string("inputs/day16.txt").unwrap();
    c.bench_function("day16", |b| b.iter(|| day16::solve(String::from(&input))));
}

fn target_17(c: &mut Criterion) {
    let input = fs::read_to_string("inputs/day17.txt").unwrap();
    c.bench_function("day17", |b| b.iter(|| day17::solve(String::from(&input))));
}

fn target_18(c: &mut Criterion) {
    let input = fs::read_to_string("inputs/day18.txt").unwrap();
    c.bench_function("day18", |b| b.iter(|| day18::solve(String::from(&input))));
}

fn target_19(c: &mut Criterion) {
    let input = fs::read_to_string("inputs/day19.txt").unwrap();
    c.bench_function("day19", |b| b.iter(|| day19::solve(String::from(&input))));
}

fn target_20(c: &mut Criterion) {
    let input = fs::read_to_string("inputs/day20.txt").unwrap();
    c.bench_function("day20", |b| b.iter(|| day20::solve(String::from(&input))));
}

fn target_21(c: &mut Criterion) {
    let input = fs::read_to_string("inputs/day21.txt").unwrap();
    c.bench_function("day21", |b| b.iter(|| day21::solve(String::from(&input))));
}

fn target_22(c: &mut Criterion) {
    let input = fs::read_to_string("inputs/day22.txt").unwrap();
    c.bench_function("day22", |b| b.iter(|| day22::solve(String::from(&input))));
}

fn target_23(c: &mut Criterion) {
    let input = fs::read_to_string("inputs/day23.txt").unwrap();
    c.bench_function("day23", |b| b.iter(|| day23::solve(String::from(&input))));
}

fn target_24(c: &mut Criterion) {
    let input = fs::read_to_string("inputs/day24.txt").unwrap();
    c.bench_function("day24", |b| b.iter(|| day24::solve(String::from(&input))));
}

criterion_group! {
    name = group;
    config = Criterion::default().warm_up_time(Duration::from_secs(5));
    targets =   target_01, target_02, target_03, target_04, target_05,
                target_06, target_07, target_08, target_09, target_10,
                target_11, target_12, target_13, target_14, target_15,
                target_16, target_17, target_18, target_19, target_20,
                target_21, target_22, target_23, target_24,
}

criterion_main!(group);

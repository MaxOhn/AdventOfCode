fn main() {
    divan::main();
}

const INPUT: &str = include_str!("../inputs/day23.txt").trim_ascii();

#[divan::bench(sample_count = 10, sample_size = 1)]
fn part2_naive() {
    aoc24::day23::part2_naive(INPUT);
}

#[divan::bench(sample_count = 100, sample_size = 1)]
fn part2_bk() {
    aoc24::day23::part2_bk(INPUT);
}

#[divan::bench(sample_count = 100, sample_size = 10)]
fn part2_bk_pivot() {
    aoc24::day23::part2_bk_pivot(INPUT);
}

#[divan::bench(sample_count = 100, sample_size = 10)]
fn part2_bk_pivot_arena() {
    aoc24::day23::part2_bk_pivot_arena(INPUT);
}

#[divan::bench(sample_count = 100, sample_size = 1)]
fn part2_bk_degeneracy() {
    aoc24::day23::part2_bk_degeneracy(INPUT);
}

#[divan::bench(sample_count = 100, sample_size = 1)]
fn part2_bk_degeneracy_rc() {
    aoc24::day23::part2_bk_degeneracy_rc(INPUT);
}

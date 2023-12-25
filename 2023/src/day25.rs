#![allow(non_snake_case)]

use std::cmp;

use aoc_rust::Solution;
use eyre::Result;
use fxhash::FxHashMap as HashMap;

pub fn run(input: &str) -> Result<Solution> {
    let input = input.trim();

    let p1 = part1(input);

    Ok(Solution::new().part1(p1).part2("x".to_owned()))
}

fn part1(input: &str) -> usize {
    let mut connections = HashMap::<_, Vec<_>>::default();
    let mut ids = HashMap::<_, usize>::default();
    let mut next_id = 0;

    for line in input.lines() {
        let (from, back) = line.split_once(": ").unwrap();

        let back: Vec<_> = back
            .split(' ')
            .map(|name| {
                *ids.entry(name).or_insert_with(|| {
                    let id = next_id;
                    next_id += 1;

                    id
                })
            })
            .collect();

        let from = *ids.entry(from).or_insert_with(|| {
            let id = next_id;
            next_id += 1;

            id
        });

        connections.entry(from).or_default().extend(back);
    }

    let mut mat = vec![vec![0; ids.len()].into_boxed_slice(); ids.len()];

    for (from, edges) in connections.iter() {
        for to in edges {
            mat[*from][*to] = 1;
            mat[*to][*from] = 1;
        }
    }

    let (cuts, partition) = global_min_cut(&mut mat);

    assert_eq!(cuts, 3);

    partition.len() * (ids.len() - partition.len())
}

type Graph = Vec<Box<[i32]>>;

fn global_min_cut(mat: &mut Graph) -> (i32, Vec<i32>) {
    let mut best = (i32::MAX, Vec::new());
    let n = mat.len();
    let mut co: Vec<_> = (0..n as i32).map(|i| vec![i]).collect();

    for ph in 1..n {
        let mut w = mat[0].clone();
        let mut s = 0;
        let mut t = 0;

        for _ in 0..n - ph {
            w[t] = i32::MIN;
            s = t;

            t = w
                .iter()
                .enumerate()
                .max_by_key(|(_, n)| *n)
                .map(|(i, _)| i)
                .unwrap();

            for i in 0..n {
                w[i] += mat[t][i];
            }
        }

        best = cmp::min(best, (w[t] - mat[t][t], co[t].clone()));
        let mut co_t = co[t].clone();
        co[s].append(&mut co_t);

        for i in 0..n {
            mat[s][i] += mat[t][i];
            mat[i][s] = mat[s][i];
        }

        mat[0][t] = i32::MIN;
    }

    best
}

#![allow(non_snake_case)]

use std::cmp;

use aoc_rust::Solution;
use eyre::Result;
use fxhash::FxHashMap as HashMap;

pub fn run(input: &str) -> Result<Solution> {
    let p1 = part1(input.trim())?;

    Ok(Solution::new().part1(p1).part2("x".to_owned()))
}

fn part1(input: &str) -> Result<usize> {
    let mut connections = HashMap::<_, Vec<_>>::default();
    let mut ids = HashMap::<_, usize>::default();
    let mut next_id = 0;

    for line in input.lines() {
        let (from, back) = line.split_once(": ").unwrap();

        let from = *ids.entry(from).or_insert_with(|| {
            let id = next_id;
            next_id += 1;

            id
        });

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

        connections.insert(from, back);
    }

    let mut mat = vec![vec![0; ids.len()].into_boxed_slice(); ids.len()];

    for (from, edges) in connections.iter() {
        for to in edges {
            mat[*from][*to] = 1;
            mat[*to][*from] = 1;
        }
    }

    let (cuts, partition_len) = global_min_cut(&mut mat);
    eyre::ensure!(cuts == 3, "no 3-cut found");

    Ok(partition_len * (ids.len() - partition_len))
}

// https://en.wikipedia.org/wiki/Stoer%E2%80%93Wagner_algorithm#Example_code
fn global_min_cut(mat: &mut [Box<[i32]>]) -> (i32, usize) {
    let mut best = (i32::MAX, 0);
    let n = mat.len();
    let mut co: Vec<_> = vec![1; n];
    let mut w = vec![0; n];

    for ph in 1..n {
        w.copy_from_slice(&mat[0]);
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

        best = cmp::min(best, (w[t] - mat[t][t], co[t]));
        co[s] += co[t];

        for i in 0..n {
            mat[s][i] += mat[t][i];
            mat[i][s] = mat[s][i];
        }

        mat[0][t] = i32::MIN;
    }

    best
}

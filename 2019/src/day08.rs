use crate::{Error, Solution};

use itertools::Itertools;
use std::collections::HashMap;
use std::fmt;

pub fn solve(input: String) -> Result<Solution<usize, String>, Error> {
    solve_with_dimensions(input, 25, 6)
} // 14.96ms

pub fn solve_with_dimensions(
    input: String,
    width: usize,
    height: usize,
) -> Result<Solution<usize, String>, Error> {
    let layers: Vec<Layer> = input
        .chars()
        .map(|digit| digit as usize - '0' as usize)
        .chunks(width * height)
        .into_iter()
        .map(|layer| Layer::new(layer.collect::<Vec<usize>>(), width))
        .collect();
    let p1 = layers
        .iter()
        .map(|layer| layer.digit_count())
        .min_by_key(|count| *count.get(&0).unwrap_or(&0))
        .unwrap()
        .iter()
        .filter(|(&digit, _)| digit != 0)
        .map(|(_, &count)| count)
        .product();
    let p2 = layers
        .iter()
        .skip(1)
        .fold(layers[0].clone(), |stacked, layer| stacked.stack(layer))
        .to_string();
    Ok(Solution::new(p1, p2))
}

#[derive(Clone)]
struct Layer {
    data: Vec<usize>,
    width: usize,
}

impl Layer {
    fn new(data: Vec<usize>, width: usize) -> Self {
        Layer { data, width }
    }

    fn stack(&self, other: &Layer) -> Self {
        let data: Vec<usize> = self
            .data
            .iter()
            .zip(other.data.iter())
            .map(|(&a, &b)| if a == 2 { b } else { a })
            .collect();
        Layer::new(data, self.width)
    }

    fn digit_count(&self) -> HashMap<usize, usize> {
        let mut count: HashMap<usize, usize> = HashMap::new();
        for &d in &self.data {
            *count.entry(d).or_insert(0) += 1;
        }
        count
    }
}

impl fmt::Display for Layer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            self.data
                .chunks(self.width)
                .map(|row| row
                    .iter()
                    .map(|digit| match digit {
                        0 => ' ',
                        1 => '█',
                        2 => ' ',
                        _ => panic!("Found wrong digit: {}", digit), // TODO: Handle?
                    })
                    .collect::<String>())
                .join("\n")
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test08() {
        let input = "0222112222120000".to_owned();
        assert_eq!(
            solve_with_dimensions(input, 2, 2).unwrap(),
            Solution::new(4, " █\n█ ".to_owned())
        );
        crate::util::tests::test_full_problem(8, solve, 2480, "████ █   ████  █    █  █ \n   █ █   ██  █ █    █  █ \n  █   █ █ ███  █    ████ \n █     █  █  █ █    █  █ \n█      █  █  █ █    █  █ \n████   █  ███  ████ █  █ ".to_owned());
    }
}

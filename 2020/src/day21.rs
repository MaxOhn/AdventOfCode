use std::collections::{BTreeMap, HashMap, HashSet};
use std::hint::unreachable_unchecked;

use aoc_rust::Solution;
use eyre::Result;

pub fn run(input: &str) -> Result<Solution> {
    let mut foods = HashMap::with_capacity(16);
    let mut allergens: HashMap<_, HashSet<_>> = HashMap::with_capacity(8);

    for line in input.lines() {
        let split = line
            .find(" (contains ")
            .unwrap_or_else(|| unsafe { unreachable_unchecked() });

        let food: HashSet<_> = line[..split].split(' ').map(str::to_owned).collect();
        let allergen_list =
            unsafe { line.get_unchecked((split + 11)..(line.trim_end().len() - 1)) }.split(", ");

        for allergen in allergen_list {
            if let Some(entry) = allergens.get_mut(allergen) {
                entry.retain(|f| food.contains(f));
            } else {
                allergens.insert(allergen.to_owned(), food.clone());
            }
        }

        for ingredient in food {
            *foods.entry(ingredient).or_insert(0) += 1;
        }
    }

    for food in allergens.values() {
        for ingredient in food.iter() {
            foods.remove(ingredient);
        }
    }

    let p1: u16 = foods.values().copied().sum();

    let mut mappings = BTreeMap::new();

    while !allergens.is_empty() {
        for (allergen, food) in allergens.iter() {
            if food.len() == 1 {
                let ingredient = food.iter().next().unwrap();
                mappings.insert(allergen.to_owned(), ingredient.to_owned());
            }
        }

        for allergen in mappings.keys() {
            allergens.remove(allergen);
        }

        for food in allergens.values_mut() {
            for ingredient in mappings.values_mut() {
                food.remove(ingredient);
            }
        }
    }

    let len: usize = mappings.values().map(String::len).sum();
    let mut p2 = String::with_capacity(len + mappings.len());

    for ingredient in mappings.values() {
        p2 += ingredient;
        p2.push(',');
    }

    p2.pop();

    Ok(Solution::new().part1(p1).part2(p2))
}

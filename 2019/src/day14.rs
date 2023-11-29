use crate::{Error, Solution};

use std::collections::{HashMap, HashSet};

pub fn run(input: &str) -> eyre::Result<aoc_rust::Solution> {
    let solution = solve(input)?;

    Ok(aoc_rust::Solution::new()
        .part1(solution.part1)
        .part2(solution.part2))
}

pub fn solve(input: &str) -> Result<Solution<u64, u64>, Error> {
    let reactions: HashMap<String, Reaction> = input
        .lines()
        .map(|line| {
            let r = Reaction::new(line)?;
            Ok((r.output.name.clone(), r))
        })
        .collect::<Result<HashMap<String, Reaction>, Error>>()?;
    let p1 = produce_fuel(&reactions, 1)?;
    let mut bot = 0;
    let mut top = 1_000_000_000_000u64;
    let mut best = 0;
    loop {
        let fuel = bot + (top - bot) / 2;
        match produce_fuel(&reactions, fuel)? {
            ore if ore <= 1_000_000_000_000 => {
                if ore == best {
                    return Ok(Solution::new(p1, fuel));
                }
                best = best.max(ore);
                bot += (top - bot) / 2;
            }
            _ => top -= (top - bot) / 2,
        }
    }
} // 169.17ms

fn produce_fuel(reactions: &HashMap<String, Reaction>, fuel: u64) -> Result<u64, Error> {
    let mut ore = 0;
    let mut extra = HashMap::new();
    let mut queue = Vec::new();
    queue.push(Material::new("FUEL".to_owned(), fuel));
    while let Some(curr) = queue.pop() {
        if curr.name == "ORE" {
            ore += curr.amount;
        } else {
            let reaction = reactions
                .get(&curr.name)
                .ok_or_else(|| error!("No key '{}' found in reactions", { &curr.name }))?;
            let used = curr
                .amount
                .min(*extra.entry(curr.name.clone()).or_insert(0));
            let _ = extra
                .get_mut(&curr.name)
                .map_or((), |e| *e = e.saturating_sub(used));
            let amount = curr.amount - used;
            if amount > 0 {
                let mult = (amount as f64 / reaction.output.amount as f64).ceil() as u64;
                let preparing = reaction.output.amount * mult;
                if preparing > amount {
                    let _ = extra
                        .get_mut(&curr.name)
                        .map_or((), |e| *e += preparing - amount);
                }
                for input in &reaction.inputs {
                    queue.push(Material::new(input.name.clone(), input.amount * mult));
                }
            }
        }
    }
    Ok(ore)
}

#[derive(Hash, Eq, PartialEq)]
struct Material {
    name: String,
    amount: u64,
}

impl Material {
    fn new(name: String, amount: u64) -> Self {
        Material { name, amount }
    }
}

struct Reaction {
    inputs: HashSet<Material>,
    output: Material,
}

impl Reaction {
    fn new(line: &str) -> Result<Self, Error> {
        let mut inputs = HashSet::new();
        let mut split_arrow = line.split(" => ");
        let split_comma = split_arrow
            .next()
            .ok_or_else(|| error!("Found no elements after splitting ' => '"))?
            .split(", ");
        for split in split_comma {
            let mut elem = split.split_whitespace();
            let amount = elem
                .next()
                .ok_or_else(|| error!("Missing material amount"))?
                .trim()
                .parse()?;
            let name = elem
                .next()
                .ok_or_else(|| error!("Missing material name"))?
                .to_owned();
            inputs.insert(Material::new(name, amount));
        }
        let mut elem = split_arrow
            .next()
            .ok_or_else(|| error!("Missing second element after ' => ' split"))?
            .split_whitespace();
        let amount = elem
            .next()
            .ok_or_else(|| error!("Missing material amount"))?
            .trim()
            .parse()?;
        let name = elem
            .next()
            .ok_or_else(|| error!("Missing material name"))?
            .to_owned();
        let output = Material::new(name, amount);
        Ok(Reaction { inputs, output })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test14() {
        let input = 
            "157 ORE => 5 NZVS\n165 ORE => 6 DCFZ\n44 XJWVT, 5 KHKGT, 1 QDVJ, 29 NZVS, 9 GPVTF, 48 HKGWZ => 1 FUEL\n12 HKGWZ, 1 GPVTF, 8 PSHF => 9 QDVJ\n179 ORE => 7 PSHF\n177 ORE => 5 HKGWZ\n7 DCFZ, 7 PSHF => 2 XJWVT\n165 ORE => 2 GPVTF\n3 DCFZ, 7 NZVS, 5 HKGWZ, 10 PSHF => 8 KHKGT";
        assert_eq!(solve(input).unwrap(), Solution::new(13_312, 82_892_753));
        let input = 
            "2 VPVL, 7 FWMGM, 2 CXFTF, 11 MNCFX => 1 STKFG\n17 NVRVD, 3 JNWZP => 8 VPVL\n53 STKFG, 6 MNCFX, 46 VJHF, 81 HVMC, 68 CXFTF, 25 GNMV => 1 FUEL\n22 VJHF, 37 MNCFX => 5 FWMGM\n139 ORE => 4 NVRVD\n144 ORE => 7 JNWZP\n5 MNCFX, 7 RFSQX, 2 FWMGM, 2 VPVL, 19 CXFTF => 3 HVMC\n5 VJHF, 7 MNCFX, 9 VPVL, 37 CXFTF => 6 GNMV\n145 ORE => 6 MNCFX\n1 NVRVD => 8 CXFTF\n1 VJHF, 6 MNCFX => 4 RFSQX\n176 ORE => 6 VJHF";
        assert_eq!(solve(input).unwrap(), Solution::new(180_697, 5_586_022));
        let input = 
            "171 ORE => 8 CNZTR\n7 ZLQW, 3 BMBT, 9 XCVML, 26 XMNCP, 1 WPTQ, 2 MZWV, 1 RJRHP => 4 PLWSL\n114 ORE => 4 BHXH\n14 VRPVC => 6 BMBT\n6 BHXH, 18 KTJDG, 12 WPTQ, 7 PLWSL, 31 FHTLT, 37 ZDVW => 1 FUEL\n6 WPTQ, 2 BMBT, 8 ZLQW, 18 KTJDG, 1 XMNCP, 6 MZWV, 1 RJRHP => 6 FHTLT\n15 XDBXC, 2 LTCX, 1 VRPVC => 6 ZLQW\n13 WPTQ, 10 LTCX, 3 RJRHP, 14 XMNCP, 2 MZWV, 1 ZLQW => 1 ZDVW\n5 BMBT => 4 WPTQ\n189 ORE => 9 KTJDG\n1 MZWV, 17 XDBXC, 3 XCVML => 2 XMNCP\n12 VRPVC, 27 CNZTR => 2 XDBXC\n15 KTJDG, 12 BHXH => 5 XCVML\n3 BHXH, 2 VRPVC => 7 MZWV\n121 ORE => 7 VRPVC\n7 XCVML => 6 RJRHP\n5 BHXH, 4 VRPVC => 5 LTCX";
        assert_eq!(solve(input).unwrap(), Solution::new(2_210_736, 460_664));
        crate::util::tests::test_full_problem(14, solve, 598_038, 2_269_325);
    }
}

use aoc_rust::Solution;
use eyre::{ContextCompat, Report, Result};

pub fn run(input: &str) -> Result<Solution> {
    let input = input.trim();

    let p1 = part1(input)?;
    let p2 = part2(input)?;

    Ok(Solution::new().part1(p1).part2(p2))
}

fn part1(input: &str) -> Result<u64> {
    solve(input, |_| (), |_| ())
}

fn part2(input: &str) -> Result<u64> {
    let modify_springs = |springs: &mut Vec<_>| {
        let len = springs.len();
        springs.push(Status::Unknown);
        springs.extend_from_within(0..len);
        springs.push(Status::Unknown);
        springs.extend_from_within(0..len + 1 + len);
        springs.push(Status::Unknown);
        springs.extend_from_within(0..len);
    };

    let modify_nums = |nums: &mut Vec<_>| {
        let len = nums.len();
        nums.extend_from_within(0..len);
        nums.extend_from_within(0..2 * len);
        nums.extend_from_within(0..len);
    };

    solve(input, modify_springs, modify_nums)
}

fn solve<S, N>(input: &str, modify_springs: S, modify_nums: N) -> Result<u64>
where
    S: Fn(&mut Vec<Status>),
    N: Fn(&mut Vec<usize>),
{
    let mut sum = 0;
    let mut cache = Cache::default();

    for line in input.lines() {
        let (springs, nums) = line.split_once(' ').wrap_err("missing whitespace")?;

        let mut springs = springs
            .bytes()
            .map(Status::try_from)
            .collect::<Result<Vec<_>>>()?;

        let mut nums = nums
            .split(',')
            .map(str::parse)
            .collect::<Result<Vec<usize>, _>>()
            .map_err(|_| eyre::eyre!("invalid number"))?;

        modify_springs(&mut springs);
        modify_nums(&mut nums);

        cache.clear();
        let remaining = nums.iter().sum::<usize>() + nums.len() - 1;
        sum += count_recursive(&mut springs, 0, &nums, remaining, &mut cache);
    }

    Ok(sum)
}

type Cache = fxhash::FxHashMap<(usize, usize, usize), u64>;

fn count_recursive(
    springs: &mut [Status],
    prev_damaged: usize,
    nums: &[usize],
    mut remaining: usize,
    cache: &mut Cache,
) -> u64 {
    if springs.is_empty() {
        let is_valid = if nums.is_empty() {
            prev_damaged == 0
        } else {
            nums.len() == 1 && prev_damaged == nums[0]
        };

        return is_valid as u64;
    } else if prev_damaged + springs.len() < remaining {
        return 0;
    }

    let key = (springs.len(), prev_damaged, nums.len());

    if let Some(cached) = cache.get(&key) {
        return *cached;
    }

    let res = match springs[0] {
        Status::Damaged if nums.is_empty() || prev_damaged == nums[0] => 0,
        Status::Damaged => {
            count_recursive(&mut springs[1..], prev_damaged + 1, nums, remaining, cache)
        }
        Status::Operational if prev_damaged == 0 => {
            let next = springs
                .iter()
                .position(|&status| status != Status::Operational)
                .unwrap_or(1);

            count_recursive(&mut springs[next..], 0, nums, remaining, cache)
        }
        Status::Operational if !nums.is_empty() && prev_damaged == nums[0] => {
            let next = springs
                .iter()
                .position(|&status| status != Status::Operational)
                .unwrap_or(1);

            remaining -= prev_damaged + (nums.len() != 1) as usize;

            count_recursive(&mut springs[next..], 0, &nums[1..], remaining, cache)
        }
        Status::Operational => 0,
        Status::Unknown => {
            let mut count = 0;

            springs[0] = Status::Operational;
            count += count_recursive(springs, prev_damaged, nums, remaining, cache);

            springs[0] = Status::Damaged;
            count += count_recursive(&mut springs[1..], prev_damaged + 1, nums, remaining, cache);

            springs[0] = Status::Unknown;

            count
        }
    };

    cache.insert(key, res);

    res
}

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
enum Status {
    Operational,
    Damaged,
    Unknown,
}

impl TryFrom<u8> for Status {
    type Error = Report;

    fn try_from(byte: u8) -> Result<Self, Self::Error> {
        match byte {
            b'.' => Ok(Self::Operational),
            b'#' => Ok(Self::Damaged),
            b'?' => Ok(Self::Unknown),
            _ => eyre::bail!("invalid status byte `{byte}`"),
        }
    }
}

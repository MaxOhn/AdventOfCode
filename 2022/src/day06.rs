use crate::prelude::*;

pub fn run(input: &str) -> Result<Solution> {
    solve_with_dynamic(input)
}

#[allow(unused)]
pub fn solve_with_array(input: &str) -> Result<Solution> {
    let bytes = input.as_bytes();

    let p1 = unique_idx_array(bytes, 4)?;
    let p2 = unique_idx_array(&bytes[p1..], 14)?;

    Ok(Solution::new().part1(p1 + 4).part2(p1 + p2 + 14))
}

#[allow(unused)]
pub fn solve_with_dynamic(input: &str) -> Result<Solution> {
    let bytes = input.as_bytes();

    let p1 = unique_idx_dynamic(bytes, 4)?;
    let p2 = unique_idx_dynamic(&bytes[p1..], 14)?;

    Ok(Solution::new().part1(p1 + 4).part2(p1 + p2 + 14))
}

#[allow(unused)]
pub fn solve_with_bitflags(input: &str) -> Result<Solution> {
    let bytes = input.as_bytes();

    let p1 = unique_idx_bitflags::<4>(bytes)?;
    let p2 = unique_idx_bitflags::<14>(&bytes[p1..])?;

    Ok(Solution::new().part1(p1 + 4).part2(p1 + p2 + 14))
}

fn unique_idx_array(stream: &[u8], target_len: u8) -> Result<usize> {
    ensure!(stream.len() >= target_len as usize, "stream too short");

    let mut counts = [0_u16; 256];
    let mut available = 0;

    let (prefix, rest) = stream.split_at(target_len as usize);

    for &byte in prefix {
        let new = &mut counts[byte as usize];
        *new += 1;
        available += (*new == 1) as u8;
    }

    stream
        .iter()
        .zip(rest)
        .position(|(&out, &new)| {
            let out = get_mut!(counts, out as usize);
            *out -= 1;
            available -= (*out == 0) as u8;

            let new = get_mut!(counts, new as usize);
            *new += 1;
            available += (*new == 1) as u8;

            available == target_len
        })
        .map(|pos| pos + 1)
        .wrap_err_with(|| format!("missing {target_len} successive unique characters"))
}

fn unique_idx_dynamic(stream: &[u8], target_len: usize) -> Result<usize> {
    let mut last_seen = [0_usize; 256];
    let mut min_i = 0;

    for (i, &byte) in stream.iter().enumerate() {
        if last_seen[byte as usize] > min_i {
            min_i = get!(last_seen, byte as usize);
        } else if i - min_i >= target_len {
            return Ok(min_i + 1);
        }

        last_seen[byte as usize] = i;
    }

    bail!("missing {target_len} successive unique characters")
}

fn unique_idx_bitflags<const N: u32>(stream: &[u8]) -> Result<usize> {
    for (i, slice) in stream.windows(N as usize).enumerate() {
        let mut bits: u32 = 0;

        for x in slice {
            bits |= 1 << (x - b'a');
        }

        if bits.count_ones() == N {
            return Ok(i);
        }
    }

    bail!("missing {N} successive unique characters")
}

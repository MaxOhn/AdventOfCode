use crate::prelude::*;

pub fn run(input: &str) -> Result<Solution> {
    let bytes = input.as_bytes();

    let p1 = unique_idx(bytes, 4)?;
    let p2 = unique_idx(&bytes[p1..], 14)?;

    Ok(Solution::new().part1(p1 + 4).part2(p1 + p2 + 14))
}

fn unique_idx(stream: &[u8], target_len: u8) -> Result<usize> {
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
            let out = get_mut!(counts[out as usize]);
            *out -= 1;
            available -= (*out == 0) as u8;

            let new = get_mut!(counts[new as usize]);
            *new += 1;
            available += (*new == 1) as u8;

            available == target_len
        })
        .map(|pos| pos + 1)
        .wrap_err_with(|| format!("missing {target_len} successive unique characters"))
}

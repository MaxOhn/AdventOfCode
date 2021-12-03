pub fn run(input: &[u8]) -> i64 {
    let mut ones = Vec::with_capacity(600);
    let mut zeros = Vec::with_capacity(600);
    let mut n = 0;

    for &byte in input {
        if byte == b'\n' {
            if n <= 0b1000_0000_0000 {
                zeros.push(n);
            } else {
                ones.push(n);
            }

            n = 0;
        } else {
            n = n * 2 + (byte == b'1') as i64;
        }
    }

    if n <= 0b1000_0000_0000 {
        zeros.push(n);
    } else {
        ones.push(n);
    }

    let (mut oxys, mut co2s) = if zeros.len() > ones.len() {
        (zeros, ones)
    } else {
        (ones, zeros)
    };

    for i in 2..=12 {
        let shift = 12 - i;
        let mask = 1 << shift;

        if oxys.len() > 1 {
            let mut ones = 0;

            for num in &oxys {
                ones += (num & mask >= 1) as usize;
            }

            let most_common_bit = ((ones >= (oxys.len() + oxys.len() % 2) / 2) as i64) << shift;
            oxys.retain(|line| line & mask == most_common_bit);
        }

        if co2s.len() > 1 {
            let mut ones = 0;

            for num in &co2s {
                ones += (num & mask > 1) as usize;
            }

            let least_common_bit = ((ones < (co2s.len() + co2s.len() % 2) / 2) as i64) << shift;
            co2s.retain(|line| line & mask == least_common_bit);
        }
    }

    oxys[0] * co2s[0]
}

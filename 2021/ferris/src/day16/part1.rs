pub fn run(input: &[u8]) -> i64 {
    with_vec::run(input)
}

pub mod with_vec {
    use super::hex_map;

    pub fn run(input: &[u8]) -> i64 {
        let trim = input[input.len() - 1] == b'\n';
        let input = &input[..input.len() - trim as usize];

        let mut binary = Vec::with_capacity(input.len() * 4);
        let iter = input.iter().flat_map(hex_map);
        binary.extend(iter);

        let mut sum = 0;
        let mut len = 0;

        process_packet(&binary, &mut sum, &mut len);

        sum
    }

    macro_rules! parse {
        ($bytes:ident[*$len:ident..$end:literal] => $ty:ty) => {
            $bytes[*$len..]
                .iter()
                .copied()
                .inspect(|_| *$len += 1)
                .take($end)
                .fold(0, |v, byte| v * 2 + byte as $ty)
        };
    }

    fn process_packet(bytes: &[u8], versions: &mut i64, len: &mut usize) {
        *versions += parse!(bytes[*len..3] => i64);
        let type_id = parse!(bytes[*len..3] => u8);

        let process_subpackets = if bytes[*len] == 0 {
            process_subpackets_by_len
        } else {
            process_subpackets_by_count
        };

        match type_id {
            0 => process_subpackets(bytes, versions, len),
            1 => process_subpackets(bytes, versions, len),
            2 => process_subpackets(bytes, versions, len),
            3 => process_subpackets(bytes, versions, len),
            4 => process_literal(bytes, len),
            5 => process_two_subpackets(bytes, versions, len),
            6 => process_two_subpackets(bytes, versions, len),
            _ => process_two_subpackets(bytes, versions, len),
        }
    }

    fn process_literal(bytes: &[u8], len: &mut usize) {
        for chunk in bytes[*len..].chunks_exact(5) {
            *len += 5;

            if chunk[0] == 0 {
                return;
            }
        }
    }

    fn process_subpackets_by_len(bytes: &[u8], versions: &mut i64, len: &mut usize) {
        *len += 1;
        let number = parse!(bytes[*len..15] => usize);
        let goal = *len + number;

        while *len < goal {
            process_packet(bytes, versions, len);
        }
    }

    fn process_subpackets_by_count(bytes: &[u8], versions: &mut i64, len: &mut usize) {
        *len += 1;
        let count = parse!(bytes[*len..11] => usize);

        (0..count).for_each(|_| process_packet(bytes, versions, len))
    }

    fn process_two_subpackets(bytes: &[u8], versions: &mut i64, len: &mut usize) {
        let len_type_id = bytes[*len];
        *len += 12 + 4 * (len_type_id == 0) as usize;

        process_packet(bytes, versions, len);
        process_packet(bytes, versions, len);
    }
}

pub mod with_iter {
    use std::{
        iter::{Flatten, Map},
        slice::Iter,
    };

    use super::hex_map;

    pub fn run(input: &[u8]) -> i64 {
        let trim = input[input.len() - 1] == b'\n';
        let input = &input[..input.len() - trim as usize];

        let mut bits = input
            .iter()
            .map(hex_map as for<'r> fn(&'r u8) -> [u8; 4])
            .flatten();

        let mut sum = 0;

        process_packet(&mut bits, &mut sum);

        sum
    }

    type BitIter<'i> = Flatten<Map<Iter<'i, u8>, for<'r> fn(&'r u8) -> [u8; 4]>>;

    macro_rules! parse {
        ($bits:ident[..$end:literal] => $ty:ty) => {
            $bits.take($end).fold(0, |v, bit| v * 2 + bit as $ty)
        };
    }

    fn process_packet(bits: &mut BitIter, versions: &mut i64) -> usize {
        *versions += parse!(bits[..3] => i64);
        let type_id = parse!(bits[..3] => u8);

        6 + match type_id {
            0 => process_subpackets(bits, versions),
            1 => process_subpackets(bits, versions),
            2 => process_subpackets(bits, versions),
            3 => process_subpackets(bits, versions),
            4 => process_literal(bits),
            5 => process_two_subpackets(bits, versions),
            6 => process_two_subpackets(bits, versions),
            _ => process_two_subpackets(bits, versions),
        }
    }

    fn process_literal(bits: &mut BitIter) -> usize {
        let mut len = 0;

        loop {
            let lead = parse!(bits[..1] => u8);
            let _ = bits.nth(3);
            len += 5;

            if lead == 0 {
                return len;
            }
        }
    }

    fn process_subpackets(bits: &mut BitIter, versions: &mut i64) -> usize {
        let len_type_id = parse!(bits[..1] => u8);

        let process_subpackets = if len_type_id == 0 {
            process_subpackets_by_len
        } else {
            process_subpackets_by_count
        };

        process_subpackets(bits, versions)
    }

    fn process_subpackets_by_len(bits: &mut BitIter, versions: &mut i64) -> usize {
        let number = parse!(bits[..15] => usize);
        let mut len = 0;

        while len < number {
            len += process_packet(bits, versions);
        }

        len + 16
    }

    fn process_subpackets_by_count(bits: &mut BitIter, versions: &mut i64) -> usize {
        let count = parse!(bits[..11] => usize);

        1 + (0..count)
            .map(|_| process_packet(bits, versions))
            .sum::<usize>()
    }

    fn process_two_subpackets(bits: &mut BitIter, versions: &mut i64) -> usize {
        let len_type_id = parse!(bits[..1] => u8);
        let skip = 10 + 4 * (len_type_id == 0) as usize;
        let _ = bits.nth(skip);

        2 + skip + process_packet(bits, versions) + process_packet(bits, versions)
    }
}

fn hex_map(byte: &u8) -> [u8; 4] {
    match byte {
        b'0' => [0, 0, 0, 0],
        b'1' => [0, 0, 0, 1],
        b'2' => [0, 0, 1, 0],
        b'3' => [0, 0, 1, 1],
        b'4' => [0, 1, 0, 0],
        b'5' => [0, 1, 0, 1],
        b'6' => [0, 1, 1, 0],
        b'7' => [0, 1, 1, 1],
        b'8' => [1, 0, 0, 0],
        b'9' => [1, 0, 0, 1],
        b'A' => [1, 0, 1, 0],
        b'B' => [1, 0, 1, 1],
        b'C' => [1, 1, 0, 0],
        b'D' => [1, 1, 0, 1],
        b'E' => [1, 1, 1, 0],
        _ => [1, 1, 1, 1],
    }
}

pub fn run(input: &[u8]) -> i64 {
    let trim = input[input.len() - 1] == b'\n';
    let input = &input[..input.len() - trim as usize];
    let mut binary = Vec::with_capacity(input.len() * 4);

    let iter = input
        .iter()
        .map(|byte| match byte {
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
        })
        .flatten();

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
        7 => process_two_subpackets(bytes, versions, len),
        _ => unreachable!(),
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

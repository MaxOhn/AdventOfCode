pub fn run(input: &[u8]) -> i64 {
    median(input)
}

pub fn median(input: &[u8]) -> i64 {
    let len = input.len();
    let input = &input[..len - (input[len - 1] == b'\n') as usize];
    let mut i = 0;
    let mut numbers: Vec<_> = (0..1000).map(|_| parse_number(input, &mut i)).collect();
    numbers.sort_unstable();
    let median = numbers[500];

    numbers.iter().map(|&n| (n - median).abs()).sum()
}

pub fn regular(input: &[u8]) -> i64 {
    let len = input.len();
    let input = &input[..len - (input[len - 1] == b'\n') as usize];
    let mut i = 0;
    let numbers: Vec<_> = (0..1000).map(|_| parse_number(input, &mut i)).collect();

    numbers
        .iter()
        .map(|&pos| numbers.iter().map(|&n| (n - pos).abs()).sum())
        .min()
        .unwrap_or(0)
}

#[inline(always)]
fn parse_number(bytes: &[u8], start: &mut usize) -> i64 {
    bytes[*start..]
        .iter()
        .copied()
        .inspect(|_| *start += 1)
        .take_while(|&byte| byte != b',')
        .fold(0, |n, byte| n * 10 + (byte & 0x0F) as i64)
}

pub fn run(input: &[u8]) -> i64 {
    let mut horizontal = 0;
    let mut depth = 0;
    let mut aim = 0;
    let mut i = 0;
    let len = input.len();

    while i < len {
        match unsafe { input.get_unchecked(i) } {
            b'f' => {
                let val = (unsafe { *input.get_unchecked(i + 8) } & 0x0F) as i64;
                horizontal += val;
                depth += aim * val;
                i += 10;
            }
            b'd' => {
                aim += (unsafe { *input.get_unchecked(i + 5) } & 0x0F) as i64;
                i += 7;
            }
            _ => {
                aim -= (unsafe { *input.get_unchecked(i + 3) } & 0x0F) as i64;
                i += 5;
            }
        }
    }

    horizontal * depth
}

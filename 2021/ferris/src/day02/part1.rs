pub fn run(input: &[u8]) -> i64 {
    let mut horizontal = 0;
    let mut depth = 0;
    let mut i = 0;
    let len = input.len();

    while i < len {
        match unsafe { input.get_unchecked(i) } {
            b'f' => {
                horizontal += (unsafe { *input.get_unchecked(i + 8) } & 0x0F) as i64;
                i += 10;
            }
            b'd' => {
                depth += (unsafe { *input.get_unchecked(i + 5) } & 0x0F) as i64;
                i += 7;
            }
            _ => {
                depth -= (unsafe { *input.get_unchecked(i + 3) } & 0x0F) as i64;
                i += 5;
            }
        }
    }

    horizontal * depth
}

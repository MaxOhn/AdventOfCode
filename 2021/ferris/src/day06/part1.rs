pub fn run(input: &[u8]) -> i64 {
    let mut count = [0; 9];
    let mut n = 0;

    for &byte in input {
        if byte == b',' {
            unsafe { *count.get_unchecked_mut(n) += 1 };
            n = 0;
        } else {
            n = n * 10 + (byte & 0x0F) as usize;
        }
    }

    unsafe { *count.get_unchecked_mut(n) += 1 };

    for _ in 0..80 {
        count.rotate_left(1);
        unsafe { *count.get_unchecked_mut(6) += *count.get_unchecked(8) };
    }

    count.into_iter().sum()
}

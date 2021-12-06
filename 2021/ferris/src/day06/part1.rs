pub fn run(input: &[u8]) -> i64 {
    let mut count = [0; 9];
    let mut n = 0;

    input.into_iter().for_each(|&byte| {
        if byte == b',' {
            unsafe { *count.get_unchecked_mut(n) += 1 };
            n = 0;
        } else {
            n = n * 10 + (byte & 0x0F) as usize;
        }
    });

    unsafe {
        *count.get_unchecked_mut(n) += 1;

        for _ in 0..80 {
            let zero = *count.get_unchecked(0);

            for n in 0..8 {
                *count.get_unchecked_mut(n) = *count.get_unchecked(n + 1);
            }

            *count.get_unchecked_mut(6) += zero;
            *count.get_unchecked_mut(8) = zero;
        }
    }

    count.into_iter().sum()
}

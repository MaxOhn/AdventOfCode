pub fn run(input: &[u8]) -> i64 {
    let mut count = [0; 9];

    for i in (0..600).step_by(2) {
        unsafe {
            *count.get_unchecked_mut((*input.get_unchecked(i) - b'0') as usize) += 1;
        }
    }

    unsafe {
        for _ in 0..256 {
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

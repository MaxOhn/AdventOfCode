use memchr::memchr;

pub fn run(input: &[u8]) -> i64 {
    let trim = input[input.len() - 1] == b'\n';
    let input = &input[..input.len() - trim as usize];

    let i = memchr(b'y', input).unwrap() + 3;
    let mut bytes = input[i..].iter();

    let y_min = (&mut bytes)
        .take_while(|&byte| *byte != b'.')
        .fold(0, |n, &byte| n * 10 + (byte & 0x0F));

    let y_max = (&mut bytes)
        .skip(2)
        .take_while(|&byte| *byte != b',')
        .fold(0, |n, &byte| n * 10 + (byte & 0x0F));

    let y_min = -(y_min as i64);
    let y_max = -(y_max as i64);

    let mut max = 0;

    for vy_init in y_min..=-y_min {
        let mut y = 0;
        let mut vy = vy_init;

        let mut valid = false;
        let mut max_y = 0;

        while y + vy >= y_min {
            y += vy;
            vy -= 1;
            valid |= (y_min..=y_max).contains(&y);
            max_y = max_y.max(y);
        }

        if valid {
            max = max.max(max_y);
        }
    }

    max
}

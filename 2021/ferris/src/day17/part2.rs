pub fn run(input: &[u8]) -> i64 {
    let trim = input[input.len() - 1] == b'\n';
    let input = &input[..input.len() - trim as usize];

    let mut bytes = input[15..].iter();

    let x_min = (&mut bytes)
        .take_while(|&byte| *byte != b'.')
        .fold(0, |n, &byte| n * 10 + (byte & 0x0F));

    let x_max = (&mut bytes)
        .skip(1)
        .take_while(|&byte| *byte != b',')
        .fold(0, |n, &byte| n * 10 + (byte & 0x0F));

    let y_min = (&mut bytes)
        .skip(4)
        .take_while(|&byte| *byte != b'.')
        .fold(0, |n, &byte| n * 10 + (byte & 0x0F));

    let y_max = (&mut bytes)
        .skip(2)
        .take_while(|&byte| *byte != b',')
        .fold(0, |n, &byte| n * 10 + (byte & 0x0F));

    let x_min = x_min as i64;
    let x_max = x_max as i64;
    let y_min = -(y_min as i64);
    let y_max = -(y_max as i64);

    let lower_x = ((1.0 + 8.0 * x_min as f64).sqrt() / 2.0 - 0.5).ceil() as i64;
    let mut count = 0;

    for vx_init in lower_x..=x_max {
        for vy_init in y_min..=-y_min {
            let mut x = 0;
            let mut y = 0;

            let mut vx = vx_init;
            let mut vy = vy_init;

            let mut valid = false;

            while !(y + vy < y_min || (x < x_min && vx <= 0) || (x > x_max && vx >= 0)) {
                x += vx;
                y += vy;

                vx -= (vx > 0) as i64;
                vy -= 1;
                valid |= (x_min..=x_max).contains(&x) && (y_min..=y_max).contains(&y);
            }

            if valid {
                count += 1;
            }
        }
    }

    count
}

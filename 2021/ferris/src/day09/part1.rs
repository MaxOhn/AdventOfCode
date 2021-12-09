use memchr::memchr;

pub fn run(input: &[u8]) -> i64 {
    let len = input.len();
    let mut input = &input[..len - (input[len - 1] == b'\n') as usize];
    let mut grid = Vec::with_capacity(100 * 100);

    while let Some(end) = memchr(b'\n', input) {
        grid.extend(input[..end].iter().map(|&b| b & 0x0F));
        input = &input[end + 1..];
    }

    let w = input.len();
    grid.extend(input.iter().map(|&b| b & 0x0F));
    let h = grid.len() / w;
    let mut sum = 0;

    // First row - first column
    let curr = grid[0];
    if !((grid[1] <= curr) || grid[w] <= curr) {
        sum += curr as i64 + 1;
    }

    // First row - last column
    let curr = grid[w - 1];
    if !((grid[w - 2] <= curr) || grid[2 * w - 1] <= curr) {
        sum += curr as i64 + 1;
    }

    // First row
    for x in 1..w - 1 {
        let curr = grid[x];

        if !((grid[x + 1] <= curr) || (grid[x - 1] <= curr) || grid[w + x] <= curr) {
            sum += curr as i64 + 1;
        }
    }

    // Last row - first column
    let idx = (h - 1) * w;
    let curr = grid[idx];
    if !((grid[idx + 1] <= curr) || grid[idx - w] <= curr) {
        sum += curr as i64 + 1;
    }

    // Last row - last column
    let curr = *grid.last().unwrap();
    if !((grid[w * h - 2] <= curr) || grid[idx - 1] <= curr) {
        sum += curr as i64 + 1;
    }

    // Last row
    for x in 1..w - 1 {
        let curr = grid[idx + x];

        if !((grid[idx + x + 1] <= curr)
            || (grid[idx + x - 1] <= curr)
            || grid[(w * (h - 2)) + x] <= curr)
        {
            sum += curr as i64 + 1;
        }
    }

    // First column
    for y in (w..w * (h - 1)).step_by(w) {
        let curr = grid[y];

        if !((grid[y - w] <= curr) || (grid[y + w] <= curr) || (grid[y + 1] <= curr)) {
            sum += curr as i64 + 1;
        }
    }

    // Last column
    for y in (2 * w - 1..w * (h - 1) - 1).step_by(w) {
        let curr = grid[y];

        if !((grid[y - w] <= curr) || (grid[y + w] <= curr) || (grid[y - 1] <= curr)) {
            sum += curr as i64 + 1;
        }
    }

    for y in 1..h - 1 {
        for x in 1..w - 1 {
            let curr = grid[(y * w + x)];

            if !(grid[(y * w + (x - 1))] <= curr
                || grid[(y * w + (x + 1))] <= curr
                || grid[((y - 1) * w + x)] <= curr
                || grid[((y + 1) * w + x)] <= curr)
            {
                sum += curr as i64 + 1;
            }
        }
    }

    sum
}

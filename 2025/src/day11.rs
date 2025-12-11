use aoc_rust::Solution;
use eyre::Result;
use fxhash::FxHashMap;

pub fn run(input: &str) -> Result<Solution> {
    let input = input.trim();

    let p1 = part1(input);
    let p2 = part2(input);

    Ok(Solution::new().part1(p1).part2(p2))
}

fn part1(input: &str) -> usize {
    let devices = parse_devices(input);
    let mut cache = Cache::default();

    bfs("you", "out", &devices, &mut cache)
}

fn part2(input: &str) -> usize {
    let devices = parse_devices(input);
    let mut cache = Cache::default();

    let fft_to_dac = bfs("fft", "dac", &devices, &mut cache);
    let dac_to_fft = bfs("dac", "fft", &devices, &mut cache);

    if dac_to_fft > 0 {
        let svr_to_dac = bfs("svr", "dac", &devices, &mut cache);
        let fft_to_out = bfs("fft", "out", &devices, &mut cache);

        svr_to_dac * dac_to_fft * fft_to_out
    } else if fft_to_dac > 0 {
        let svr_to_fft = bfs("svr", "fft", &devices, &mut cache);
        let dac_to_out = bfs("dac", "out", &devices, &mut cache);

        svr_to_fft * fft_to_dac * dac_to_out
    } else {
        unreachable!()
    }
}

type Devices<'a> = FxHashMap<&'a str, Vec<&'a str>>;

fn parse_devices(input: &str) -> Devices<'_> {
    let mut devices = Devices::default();

    for device in input.lines() {
        let (name, outputs) = device.split_once(':').unwrap();
        let outputs: Vec<_> = outputs.split_ascii_whitespace().collect();
        devices.insert(name, outputs);
    }

    devices
}

type Cache<'a> = FxHashMap<(&'a str, &'a str), usize>;

fn bfs<'a>(start: &'a str, end: &'a str, devices: &Devices<'a>, cache: &mut Cache<'a>) -> usize {
    if start == end {
        return 1;
    }

    let key = (start, end);

    if let Some(cached) = cache.get(&key) {
        return *cached;
    }

    let Some(outputs) = devices.get(start) else {
        return 0;
    };

    let mut count = 0;

    for &output in outputs.iter() {
        count += bfs(output, end, devices, cache);
    }

    cache.insert(key, count);

    count
}

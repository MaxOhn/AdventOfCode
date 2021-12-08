use std::{
    collections::HashSet,
    error::Error,
    fs::File,
    io::{BufRead, BufReader},
    time::Instant,
};

fn main() {
    if let Err(err) = run() {
        eprintln!("Error: {}", err);
        let mut e: &dyn Error = &*err;

        while let Some(src) = e.source() {
            eprintln!("  - caused by: {}", src);
            e = src;
        }
    }
}

fn run() -> Result<(), Box<dyn Error>> {
    let start = Instant::now();
    let file = File::open("./input")?;
    let mut input = BufReader::new(file);

    let mut line = String::new();

    let mut p1 = 0;
    let mut p2 = 0;

    while input.read_line(&mut line)? != 0 {
        let mut wiring = [0; 7].map(|_| HashSet::new());

        let mut split = line.trim_end().split(" | ");
        let left = split.next().unwrap();
        let right = split.next().unwrap();

        for word in right.split(' ') {
            p1 += ([2, 3, 4, 7].contains(&word.len())) as usize;
        }

        let mut words: Vec<_> = left.split(' ').map(str::as_bytes).collect();

        let eight = words.iter().position(|word| word.len() == 7);

        if let Some(eight) = eight.map(|n| words.remove(n)) {
            for &byte in eight {
                for option in wiring.iter_mut() {
                    option.insert(byte);
                }
            }
        }

        let four = words.iter().position(|word| word.len() == 4);

        if let Some(four) = four.map(|n| words.remove(n)) {
            for i in [1, 2, 3, 5] {
                wiring[i].clear();

                for &byte in four {
                    wiring[i].insert(byte);
                }
            }

            wiring[0].retain(|byte| !four.contains(byte));
            wiring[4].retain(|byte| !four.contains(byte));
            wiring[6].retain(|byte| !four.contains(byte));
        }

        let seven = words.iter().position(|word| word.len() == 3);

        if let Some(seven) = seven.map(|n| words.remove(n)) {
            for i in [0, 2, 5] {
                wiring[i].clear();

                for &byte in seven {
                    wiring[i].insert(byte);
                }
            }

            wiring[1].retain(|byte| !seven.contains(byte));
            wiring[3].retain(|byte| !seven.contains(byte));
            wiring[4].retain(|byte| !seven.contains(byte));
            wiring[6].retain(|byte| !seven.contains(byte));
        }

        let one = words.iter().position(|word| word.len() == 2);

        if let Some(one) = one.map(|n| words.remove(n)) {
            for i in [2, 5] {
                wiring[i].clear();

                for &byte in one {
                    wiring[i].insert(byte);
                }
            }

            wiring[0].retain(|byte| !one.contains(byte));
            wiring[1].retain(|byte| !one.contains(byte));
            wiring[3].retain(|byte| !one.contains(byte));
            wiring[4].retain(|byte| !one.contains(byte));
            wiring[6].retain(|byte| !one.contains(byte));
        }

        // zero, six, nine
        for &word_ in words.iter().filter(|w| w.len() == 6) {
            let mut word: HashSet<_> = word_.into_iter().copied().collect();

            for n in [0, 4, 5, 6] {
                for byte in &wiring[n] {
                    word.remove(byte);
                }
            }

            match word.len() {
                // zero
                1 => {
                    word.retain(|byte| !wiring[2].contains(byte));
                    word.retain(|byte| !wiring[5].contains(byte));
                    let remaining = word.into_iter().next().unwrap();
                    wiring[1].retain(|&byte| byte == remaining);
                    wiring[3].retain(|&byte| byte != remaining);
                }
                // six or nine
                2 => {
                    let mut set: HashSet<_> = word_.iter().copied().collect();

                    for n in [0, 1, 2, 3, 5] {
                        for byte in &wiring[n] {
                            set.remove(byte);
                        }
                    }

                    if set.len() == 1 {
                        // nine
                        let remaining = set.into_iter().next().unwrap();

                        wiring[4].retain(|&byte| byte != remaining);
                        wiring[6].retain(|&byte| byte == remaining);
                    } else {
                        // six
                        for &c in word_ {
                            wiring[2].retain(|&byte| byte != c);
                        }
                    }
                }
                _ => unreachable!(),
            }
        }

        // two, three, five
        for &word in words.iter().filter(|w| w.len() == 5) {
            let mut word: HashSet<_> = word.into_iter().copied().collect();

            for n in [0, 1, 3, 4, 6] {
                for byte in &wiring[n] {
                    word.remove(byte);
                }
            }

            if !word.len() == 1 {
                continue;
            }

            let remaining = word.into_iter().next().unwrap();

            if wiring[2].contains(&remaining) {
                wiring[2].retain(|&byte| byte == remaining);
                wiring[5].retain(|&byte| byte != remaining);
            }
        }

        let wiring = wiring.map(|set| set.into_iter().next().unwrap());
        let [_, b, c, _, e, _, _] = wiring;

        let mut n = 0;

        for word in right.split(' ').map(str::as_bytes) {
            n *= 10;

            match word.len() {
                2 => n += 1,
                3 => n += 7,
                4 => n += 4,
                7 => n += 8,
                5 => {
                    let b_opt = word.iter().position(|&byte| byte == b);
                    let e_opt = word.iter().position(|&byte| byte == e);

                    match (b_opt, e_opt) {
                        (None, None) => n += 3,
                        (None, Some(_)) => n += 2,
                        (Some(_), None) => n += 5,
                        (Some(_), Some(_)) => unreachable!(),
                    }
                }
                6 => {
                    let c_opt = word.iter().position(|&byte| byte == c);
                    let e_opt = word.iter().position(|&byte| byte == e);

                    match (c_opt, e_opt) {
                        (None, None) => unreachable!(),
                        (None, Some(_)) => n += 6,
                        (Some(_), None) => n += 9,
                        (Some(_), Some(_)) => n += 0,
                    }
                }
                _ => unreachable!(),
            }
        }

        p2 += n;

        line.clear();
    }

    println!("Part 1: {}, {:?}", p1, start.elapsed()); //
    println!("Part 2: {}, {:?}", p2, start.elapsed()); //

    Ok(())
}

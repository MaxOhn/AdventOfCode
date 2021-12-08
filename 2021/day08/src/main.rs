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
        let [mut a, mut b, mut c, mut d, mut e, mut f, mut g] =
            [0; 7].map(|_| HashSet::with_capacity(10));
        let (left, right) = line.trim_end().split_once(" | ").unwrap();

        for word in right.split(' ') {
            p1 += ([2, 3, 4, 7].contains(&word.len())) as usize;
        }

        let mut words: Vec<_> = left.split(' ').map(str::as_bytes).collect();
        words.sort_unstable_by_key(|bytes| bytes.len());

        let mut words = words.into_iter();

        let [one, seven, four, _, _, _, six_1, six_2, six_3, eight] = [
            words.next().unwrap(),
            words.next().unwrap(),
            words.next().unwrap(),
            words.next().unwrap(),
            words.next().unwrap(),
            words.next().unwrap(),
            words.next().unwrap(),
            words.next().unwrap(),
            words.next().unwrap(),
            words.next().unwrap(),
        ];

        // Eight
        for &byte in eight {
            e.insert(byte);
            g.insert(byte);
        }

        // Four
        for &byte in four {
            b.insert(byte);
            d.insert(byte);
        }

        e.retain(|byte| !four.contains(byte));
        g.retain(|byte| !four.contains(byte));

        // Seven
        for &byte in seven {
            a.insert(byte);
        }

        b.retain(|byte| !seven.contains(byte));
        d.retain(|byte| !seven.contains(byte));
        e.retain(|byte| !seven.contains(byte));
        g.retain(|byte| !seven.contains(byte));

        // One
        for &byte in one {
            c.insert(byte);
            f.insert(byte);
        }

        a.retain(|byte| !one.contains(byte));
        b.retain(|byte| !one.contains(byte));
        d.retain(|byte| !one.contains(byte));
        e.retain(|byte| !one.contains(byte));
        g.retain(|byte| !one.contains(byte));

        // zero, six, nine
        for word_ in [six_1, six_2, six_3] {
            let mut word: HashSet<_> = word_.iter().copied().collect();

            for options in [&a, &e, &f, &g] {
                for byte in options {
                    word.remove(byte);
                }
            }

            match word.len() {
                // zero
                1 => {
                    word.retain(|byte| !c.contains(byte));
                    word.retain(|byte| !f.contains(byte));
                    let remaining = word.into_iter().next().unwrap();
                    b.retain(|&byte| byte == remaining);
                    // d.retain(|&byte| byte != remaining);
                }
                // six or nine
                2 => {
                    let mut set: HashSet<_> = word_.iter().copied().collect();

                    for options in [&a, &b, &c, &d, &f] {
                        for byte in options {
                            set.remove(byte);
                        }
                    }

                    if set.len() == 1 {
                        // nine
                        let remaining = set.into_iter().next().unwrap();

                        e.retain(|&byte| byte != remaining);
                        // g.retain(|&byte| byte == remaining);
                    } else {
                        // six
                        for byte_ in word_ {
                            c.retain(|byte| byte != byte_);
                        }
                    }
                }
                _ => unreachable!(),
            }
        }

        // two, three, five can be ignored
        // for word in [five_1, five_2, five_3] {
        //     let mut word: HashSet<_> = word.into_iter().copied().collect();

        //     for options in [&a, &b, &d, &e, &g] {
        //         for byte in options {
        //             word.remove(byte);
        //         }
        //     }

        //     if !word.len() == 1 {
        //         continue;
        //     }

        //     let remaining = word.into_iter().next().unwrap();

        //     if c.contains(&remaining) {
        //         c.retain(|&byte| byte == remaining);
        //         f.retain(|&byte| byte != remaining);
        //     }
        // }

        let b = b.into_iter().next().unwrap();
        let c = c.into_iter().next().unwrap();
        let e = e.into_iter().next().unwrap();

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

    println!("Part 1: {}, {:?}", p1, start.elapsed()); // 1.5ms
    println!("Part 2: {}, {:?}", p2, start.elapsed()); // 1.94ms

    assert_eq!(p1, 409);
    assert_eq!(p2, 1_024_649);

    Ok(())
}

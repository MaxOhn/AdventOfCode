use aoc_rust::Solution;
use eyre::Result;

pub fn run(input: &str) -> Result<Solution> {
    let p1 = part1(input);
    let p2 = part2(input);

    // let p1 = _part1_old(input);
    // let p2 = _part2_old(input);

    Ok(Solution::new().part1(p1).part2(p2))
}

fn part1(input: &str) -> usize {
    let mut n = parse_input(input);
    let mut circle = vec![0; 10];

    let last = n % 10;
    let mut curr = last;
    n /= 10;

    while n > 0 {
        let prev = n % 10;
        n /= 10;
        set!(circle, prev, curr);
        curr = prev;
    }

    set!(circle, last, curr);

    execute(&mut circle, curr, 100);

    let p1 = std::iter::successors(Some(1), |&n| Some(get!(circle, n)))
        .skip(1)
        .take(8)
        .fold(0, |res, n| res * 10 + n);

    p1
}

fn part2(input: &str) -> usize {
    let mut n = parse_input(input);
    let mut circle: Vec<_> = (1..=1_000_001).collect();

    let last = n % 10;
    let mut curr = last;
    n /= 10;

    while n > 0 {
        let prev = n % 10;
        n /= 10;
        set!(circle, prev, curr);
        curr = prev;
    }

    set!(circle, last, 10);
    set!(circle, 1_000_000, curr);

    execute(&mut circle, curr, 10_000_000);

    let cup1 = get!(circle, 1);
    let p2 = cup1 * get!(circle, cup1);

    p2
}

fn execute(circle: &mut [usize], mut curr: usize, moves: usize) {
    let len = circle.len() - 1;

    for _ in 0..moves {
        let a = get!(circle, curr);
        let b = get!(circle, a);
        let c = get!(circle, b);

        let mut target = if curr == 1 { len } else { curr - 1 };

        while a == target || b == target || c == target {
            target = if target == 1 { len } else { target - 1 };
        }

        circle.swap(curr, target);
        circle.swap(curr, c);
        curr = get!(circle, curr);
    }
}

fn parse_input(input: &str) -> usize {
    input.lines().next().unwrap().parse().unwrap()
}

fn _part1_old(input: &str) -> u32 {
    let mut n: u32 = input.lines().next().unwrap().parse().unwrap();
    let mut circle = Vec::with_capacity(9);

    while n > 0 {
        circle.push(((n % 10) - 1) as u8);
        n /= 10;
    }

    circle.reverse();

    const LEN: usize = 9;

    let mut i = 0;

    for _m in 0..100 {
        // println!("-- move {} --", _m + 1);
        // println!("{:?}", circle);

        let target = (circle[i] + 8) % 9;
        let destination = circle.iter().position(|&n| n == target).unwrap();
        let distance = (9 + destination - i) % 9;

        if distance <= 3 {
            let target = (circle[i] + 7) % 9;
            let destination = circle.iter().position(|&n| n == target).unwrap();
            let distance = (9 + destination - i) % 9;

            if distance <= 3 {
                let target = (circle[i] + 6) % 9;
                let destination = circle.iter().position(|&n| n == target).unwrap();
                let distance = (9 + destination - i) % 9;

                if distance <= 3 {
                    let target = (circle[i] + 5) % 9;
                    let destination = circle.iter().position(|&n| n == target).unwrap();
                    let distance = (9 + destination - i) % 9;

                    if destination > i {
                        circle[i + 1..destination + 1].rotate_left(3);
                    } else {
                        let mut j = 0;

                        while j + 6 < distance {
                            circle.swap((i + j + 1) % LEN, (i + j + 4) % LEN);
                            circle.swap((i + j + 2) % LEN, (i + j + 5) % LEN);
                            circle.swap((i + j + 3) % LEN, (i + j + 6) % LEN);

                            j += 3;
                        }

                        for l in 0..distance - j - 3 {
                            circle.swap((i + j + l + 3) % LEN, (i + j + l + 4) % LEN);
                            circle.swap((i + j + l + 2) % LEN, (i + j + l + 3) % LEN);
                            circle.swap((i + j + l + 1) % LEN, (i + j + l + 2) % LEN);
                        }
                    }
                } else if destination > i {
                    circle[i + 1..destination + 1].rotate_left(3);
                } else {
                    let mut j = 0;

                    while j + 6 < distance {
                        circle.swap((i + j + 1) % LEN, (i + j + 4) % LEN);
                        circle.swap((i + j + 2) % LEN, (i + j + 5) % LEN);
                        circle.swap((i + j + 3) % LEN, (i + j + 6) % LEN);

                        j += 3;
                    }

                    for l in 0..distance - j - 3 {
                        circle.swap((i + j + l + 3) % LEN, (i + j + l + 4) % LEN);
                        circle.swap((i + j + l + 2) % LEN, (i + j + l + 3) % LEN);
                        circle.swap((i + j + l + 1) % LEN, (i + j + l + 2) % LEN);
                    }
                }
            } else if destination > i {
                circle[i + 1..destination + 1].rotate_left(3);
            } else {
                let mut j = 0;

                while j + 6 < distance {
                    circle.swap((i + j + 1) % LEN, (i + j + 4) % LEN);
                    circle.swap((i + j + 2) % LEN, (i + j + 5) % LEN);
                    circle.swap((i + j + 3) % LEN, (i + j + 6) % LEN);

                    j += 3;
                }

                for l in 0..distance - j - 3 {
                    circle.swap((i + j + l + 3) % LEN, (i + j + l + 4) % LEN);
                    circle.swap((i + j + l + 2) % LEN, (i + j + l + 3) % LEN);
                    circle.swap((i + j + l + 1) % LEN, (i + j + l + 2) % LEN);
                }
            }
        } else if destination > i {
            circle[i + 1..destination + 1].rotate_left(3);
        } else {
            let mut j = 0;

            while j + 6 < distance {
                circle.swap((i + j + 1) % LEN, (i + j + 4) % LEN);
                circle.swap((i + j + 2) % LEN, (i + j + 5) % LEN);
                circle.swap((i + j + 3) % LEN, (i + j + 6) % LEN);

                j += 3;
            }

            for l in 0..distance - j - 3 {
                circle.swap((i + j + l + 3) % LEN, (i + j + l + 4) % LEN);
                circle.swap((i + j + l + 2) % LEN, (i + j + l + 3) % LEN);
                circle.swap((i + j + l + 1) % LEN, (i + j + l + 2) % LEN);
            }
        }

        i = (i + 1) % 9;
    }

    let p1 = circle
        .into_iter()
        .cycle()
        .skip_while(|&n| n != 0)
        .skip(1)
        .map(|n| n as u32 + 1)
        .take(8)
        .fold(0, |num, n| num * 10 + n);

    p1
}

fn _part2_old(input: &str) -> u64 {
    let mut n: u32 = input.lines().next().unwrap().parse().unwrap();
    let mut circle = Vec::with_capacity(1_000_000);

    while n > 0 {
        circle.push((n % 10) - 1);
        n /= 10;
    }

    circle.reverse();

    circle.extend(9..1_000_000);

    const LEN: usize = 1_000_000;
    const LEN_U: u32 = LEN as u32;

    let mut i = 0;

    for _ in 0..10_000_000 {
        let target = (circle[i] + LEN_U - 1) % LEN_U;
        let destination = circle.iter().position(|&n| n == target).unwrap();
        let distance = (LEN + destination - i) % LEN;

        if distance <= 3 {
            let target = (circle[i] + LEN_U - 2) % LEN_U;
            let destination = circle.iter().position(|&n| n == target).unwrap();
            let distance = (LEN + destination - i) % LEN;

            if distance <= 3 {
                let target = (circle[i] + LEN_U - 3) % LEN_U;
                let destination = circle.iter().position(|&n| n == target).unwrap();
                let distance = (LEN + destination - i) % LEN;

                if distance <= 3 {
                    let target = (circle[i] + LEN_U - 4) % LEN_U;
                    let destination = circle.iter().position(|&n| n == target).unwrap();
                    let distance = (LEN + destination - i) % LEN;

                    if destination > i {
                        circle[i + 1..destination + 1].rotate_left(3);
                    } else {
                        let mut j = 0;

                        while j + 6 < distance {
                            circle.swap((i + j + 1) % LEN, (i + j + 4) % LEN);
                            circle.swap((i + j + 2) % LEN, (i + j + 5) % LEN);
                            circle.swap((i + j + 3) % LEN, (i + j + 6) % LEN);

                            j += 3;
                        }

                        let k = i + j;

                        for l in 0..distance - j - 3 {
                            circle.swap((k + l + 3) % LEN, (k + l + 4) % LEN);
                            circle.swap((k + l + 2) % LEN, (k + l + 3) % LEN);
                            circle.swap((k + l + 1) % LEN, (k + l + 2) % LEN);
                        }
                    }
                } else if destination > i {
                    circle[i + 1..destination + 1].rotate_left(3);
                } else {
                    let mut j = 0;

                    while j + 6 < distance {
                        circle.swap((i + j + 1) % LEN, (i + j + 4) % LEN);
                        circle.swap((i + j + 2) % LEN, (i + j + 5) % LEN);
                        circle.swap((i + j + 3) % LEN, (i + j + 6) % LEN);

                        j += 3;
                    }

                    let k = i + j;

                    for l in 0..distance - j - 3 {
                        circle.swap((k + l + 3) % LEN, (k + l + 4) % LEN);
                        circle.swap((k + l + 2) % LEN, (k + l + 3) % LEN);
                        circle.swap((k + l + 1) % LEN, (k + l + 2) % LEN);
                    }
                }
            } else if destination > i {
                circle[i + 1..destination + 1].rotate_left(3);
            } else {
                let mut j = 0;

                while j + 6 < distance {
                    circle.swap((i + j + 1) % LEN, (i + j + 4) % LEN);
                    circle.swap((i + j + 2) % LEN, (i + j + 5) % LEN);
                    circle.swap((i + j + 3) % LEN, (i + j + 6) % LEN);

                    j += 3;
                }

                let k = i + j;

                for l in 0..distance - j - 3 {
                    circle.swap((k + l + 3) % LEN, (k + l + 4) % LEN);
                    circle.swap((k + l + 2) % LEN, (k + l + 3) % LEN);
                    circle.swap((k + l + 1) % LEN, (k + l + 2) % LEN);
                }
            }
        } else if destination > i {
            circle[i + 1..destination + 1].rotate_left(3);
        } else {
            let mut j = 0;

            while j + 6 < distance {
                circle.swap((i + j + 1) % LEN, (i + j + 4) % LEN);
                circle.swap((i + j + 2) % LEN, (i + j + 5) % LEN);
                circle.swap((i + j + 3) % LEN, (i + j + 6) % LEN);

                j += 3;
            }

            let k = i + j;

            for l in 0..distance - j - 3 {
                circle.swap((k + l + 3) % LEN, (k + l + 4) % LEN);
                circle.swap((k + l + 2) % LEN, (k + l + 3) % LEN);
                circle.swap((k + l + 1) % LEN, (k + l + 2) % LEN);
            }
        }

        i = (i + 1) % LEN;
    }

    let p2 = circle
        .into_iter()
        .cycle()
        .skip_while(|&n| n != 0)
        .skip(1)
        .map(|n| n as u32 + 1)
        .take(2)
        .fold(1, |prod, cup| prod * cup as u64);

    p2
}

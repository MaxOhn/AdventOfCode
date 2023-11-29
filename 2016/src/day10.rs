use std::collections::{HashMap, VecDeque};

use aoc_rust::Solution;

macro_rules! parse {
    ($num:expr) => {
        $num.unwrap().parse().unwrap()
    };
}

pub fn run(input: &str) -> eyre::Result<Solution> {
    let mut bots: HashMap<_, Bot> = HashMap::with_capacity(32);
    let mut instructions = VecDeque::with_capacity(256);
    let mut outputs = [0; 32];
    let mut p1 = None;

    for line in input.lines() {
        let mut words = line.split(' ');

        match words.next().unwrap() {
            "value" => {
                let val = parse!(words.next());
                let id = parse!(words.next_back());

                assert_ne!(bots.entry(id).or_default().receive(val), None);
            }
            "bot" => {
                let id = parse!(words.next());
                let low = (words.nth(3).unwrap(), parse!(words.next())).into();
                let high = (words.nth(3).unwrap(), parse!(words.next())).into();
                instructions.push_back(Instruction { id, low, high });
            }
            _ => unsafe { std::hint::unreachable_unchecked() },
        }
    }

    while let Some(instruction) = instructions.pop_front() {
        let Instruction { id, low, high } = instruction;

        if let Some((l, h)) = bots.entry(id).or_default().take() {
            match low {
                Receiver::Output(receiver) => outputs[receiver as usize] = l,
                Receiver::Bot(receiver) => match bots.entry(receiver).or_default().receive(l) {
                    None => {
                        p1.get_or_insert(receiver);
                    }
                    Some(true) => {}
                    Some(false) => {
                        bots.entry(id).and_modify(|bot| {
                            let _ = bot.receive(l);
                            let _ = bot.receive(h);
                        });
                        instructions.push_back(instruction);
                        continue;
                    }
                },
            }

            match high {
                Receiver::Output(receiver) => outputs[receiver as usize] = h,
                Receiver::Bot(receiver) => match bots.entry(receiver).or_default().receive(h) {
                    None => {
                        p1.get_or_insert(receiver);
                    }
                    Some(true) => {}
                    Some(false) => {
                        bots.entry(id).and_modify(|bot| {
                            let _ = bot.receive(l);
                            let _ = bot.receive(h);
                        });
                        instructions.push_back(instruction);
                        continue;
                    }
                },
            }
        } else {
            instructions.push_back(instruction);
        }
    }

    let p1 = p1.unwrap();
    let p2 = outputs[0] as usize * outputs[1] as usize * outputs[2] as usize;

    Ok(Solution::new().part1(p1).part2(p2))
}

const LOW: u8 = 17;
const HIGH: u8 = 61;

#[derive(Copy, Clone, Debug)]
enum Receiver {
    Bot(u8),
    Output(u8),
}

impl From<(&str, u8)> for Receiver {
    fn from((ty, id): (&str, u8)) -> Self {
        match ty {
            "output" => Self::Output(id),
            "bot" => Self::Bot(id),
            _ => unsafe { std::hint::unreachable_unchecked() },
        }
    }
}

#[derive(Copy, Clone)]
struct Instruction {
    id: u8,
    low: Receiver,
    high: Receiver,
}

#[derive(Copy, Clone, Default, Debug)]
struct Bot {
    low: Option<u8>,
    high: Option<u8>,
}

impl Bot {
    fn take(&self) -> Option<(u8, u8)> {
        if let (Some(low), Some(high)) = (self.low, self.high) {
            Some((low, high))
        } else {
            None
        }
    }

    fn receive(&mut self, val: u8) -> Option<bool> {
        match (self.low, self.high) {
            (None, _) => {
                self.low.replace(val);
            }
            (Some(low), None) => {
                if low <= val {
                    self.high.replace(val);

                    if low == LOW && val == HIGH {
                        return None;
                    }
                } else {
                    self.low.replace(val);
                    self.high.replace(low);

                    if val == LOW && low == HIGH {
                        return None;
                    }
                }
            }
            _ => return Some(false),
        };

        Some(true)
    }
}

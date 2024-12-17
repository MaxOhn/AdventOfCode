use aoc_rust::Solution;
use eyre::Result;
use itoa::Buffer;
use nom::{
    bytes::complete as by,
    character::complete as ch,
    combinator::all_consuming,
    error::Error as NomError,
    multi::separated_list1,
    sequence::{delimited, preceded},
    Err as NomErr,
};

pub fn run(input: &str) -> Result<Solution> {
    let input = input.trim();

    let p1 = part1(input);
    let p2 = part2(input);

    Ok(Solution::new().part1(p1).part2(p2))
}

fn part1(input: &str) -> String {
    let (mut state, program) = State::parse(input).unwrap();

    let mut buf = Buffer::new();
    let mut output = String::new();

    let Some(val) = state.run_until_print(&program) else {
        return output;
    };

    output.push_str(buf.format(val));

    while let Some(val) = state.run_until_print(&program) {
        output.push(',');
        output.push_str(buf.format(val));
    }

    output
}

fn part2(input: &str) -> i64 {
    let (mut state, program) = State::parse(input).unwrap();

    let mut a = 0;
    let b = state.b;
    let c = state.c;

    let mut stack = program.clone();

    'stack: while let Some(next) = stack.pop() {
        a *= 8;

        'sim: loop {
            a += 1;
            state = State::new(a, b, c);

            if state.run_until_print(&program) == Some(next) {
                continue 'stack;
            } else {
                continue 'sim;
            }
        }
    }

    a
}

struct State {
    ip: usize,
    a: i64,
    b: i64,
    c: i64,
}

impl State {
    fn new(a: i64, b: i64, c: i64) -> Self {
        State { ip: 0, a, b, c }
    }

    fn parse(input: &str) -> Result<(Self, Vec<u8>), NomErr<NomError<&str>>> {
        let name = |name| delimited(by::tag("Register "), by::tag(name), by::tag(": "));
        let register = |n| delimited(name(n), ch::i64, ch::newline);

        let (rest, a) = register("A")(input)?;
        let (rest, b) = register("B")(rest)?;
        let (rest, c) = register("C")(rest)?;

        let program = preceded(ch::newline, by::tag("Program: "));
        let list = separated_list1(by::tag(","), ch::u8);
        let (_, program) = all_consuming(preceded(program, list))(rest)?;

        Ok((Self::new(a, b, c), program))
    }

    fn run_until_print(&mut self, program: &[u8]) -> Option<u8> {
        while self.ip < program.len() {
            let op = program[self.ip];
            let operand = program[self.ip + 1];

            let combo = || match operand {
                0..=3 => operand as i64,
                4 => self.a,
                5 => self.b,
                6 => self.c,
                _ => panic!("bad combo operand"),
            };

            match op {
                0 => self.a = self.a >> (combo() as u32),
                1 => self.b = self.b ^ operand as i64,
                2 => self.b = combo() % 8,
                3 => {
                    if self.a != 0 && self.ip != operand as usize {
                        self.ip = operand as usize;

                        continue;
                    }
                }
                4 => self.b = self.b ^ self.c,
                5 => {
                    self.ip += 2;

                    return Some((combo() % 8) as u8);
                }
                6 => self.b = self.a >> (combo() as u32),
                7 => self.c = self.a >> (combo() as u32),
                _ => panic!("bad op code"),
            }

            self.ip += 2;
        }

        None
    }
}

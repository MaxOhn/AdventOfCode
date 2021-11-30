use crate::Error;
use std::collections::VecDeque;

pub struct Computer {
    memory: Vec<i64>,
    pc: usize,
    rb: i32,
    input: VecDeque<i64>,
    output: VecDeque<i64>,
    state: State,
}

impl Computer {
    pub fn new(input: String) -> Result<Self, Error> {
        let memory = input
            .split(',')
            .map(|n| n.parse().map_err(Error::from))
            .collect::<Result<Vec<_>, Error>>()?;
        Ok(Computer {
            memory,
            pc: 0,
            rb: 0,
            input: VecDeque::new(),
            output: VecDeque::new(),
            state: State::Ready,
        })
    }

    pub fn run(&mut self) -> Result<&mut Self, Error> {
        if self.state == State::Done {
            return Ok(self);
        } else if self.state == State::Wait {
            if self.input.is_empty() {
                bail!("Cannot run while waiting for input");
            }
            self.state = State::Ready;
        }
        if self.memory[self.pc] == 99 {
            self.state = State::Done;
            return Ok(self);
        }
        while self.step()? == State::Ready {}
        Ok(self)
    }

    pub fn step(&mut self) -> Result<State, Error> {
        if let Some(mut op) = Operation::new(&mut self.memory, self.pc, self.rb)? {
            match op.opcode {
                1 => self.memory[op.w] = op.v1 + op.v2,
                2 => self.memory[op.w] = op.v1 * op.v2,
                3 => match self.input.pop_front() {
                    Some(input) => self.memory[op.w] = input,
                    None => {
                        self.state = State::Wait;
                        return Ok(State::Wait);
                    }
                },
                4 => self.output.push_back(op.v1),
                5 => {
                    op.pc = if op.v1 != 0 {
                        op.v2 as usize
                    } else {
                        op.pc + 3
                    }
                }
                6 => {
                    op.pc = if op.v1 == 0 {
                        op.v2 as usize
                    } else {
                        op.pc + 3
                    }
                }
                7 => self.memory[op.w] = if op.v1 < op.v2 { 1 } else { 0 },
                8 => self.memory[op.w] = if op.v1 == op.v2 { 1 } else { 0 },
                9 => self.rb = op.rb,
                _ => bail!("Can't process opcode {}", op.opcode),
            }
            self.pc = op.pc;
            Ok(State::Ready)
        } else {
            Ok(State::Done)
        }
    }

    pub fn pop(&mut self) -> Option<i64> {
        self.output.pop_front()
    }

    pub fn insert(&mut self, input: i64) -> &mut Self {
        self.input.push_back(input);
        self
    }

    pub fn output_iter<'a>(&'a mut self) -> impl Iterator<Item = &i64> + 'a {
        self.output.iter()
    }
}

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum State {
    Ready,
    Done,
    Wait,
}

struct Operation {
    opcode: i64,
    v1: i64,
    v2: i64,
    w: usize,
    pc: usize,
    rb: i32,
}

impl Operation {
    fn new(mem: &mut Vec<i64>, pc: usize, rb: i32) -> Result<Option<Self>, Error> {
        while mem.len() <= pc + 3 {
            mem.push(0);
        }
        let opcode = mem[pc] % 100;
        if opcode == 99 {
            return Ok(None);
        }
        if opcode == 3 {
            let w = match (mem[pc] / 100) % 10 {
                0 => mem[pc + 1] as usize,
                2 => (rb + mem[pc + 1] as i32) as usize,
                other => bail!("Can't process mode {} for writing", other),
            };
            if mem.len() <= w {
                mem.resize(w + 1, 0);
            }
            return Ok(Some(Operation {
                opcode,
                v1: 0,
                v2: 0,
                w,
                pc: pc + 2,
                rb,
            }));
        }

        let v1 = match (mem[pc] / 100) % 10 {
            0 => {
                if mem.len() as i64 <= mem[pc + 1] {
                    mem.resize(mem[pc + 1] as usize + 1, 0);
                }
                mem[mem[pc + 1] as usize]
            }
            1 => mem[pc + 1],
            2 => {
                if mem.len() as i32 <= rb + mem[pc + 1] as i32 {
                    mem.resize((rb as i64 + mem[pc + 1]) as usize + 1, 0);
                }
                mem[(rb + mem[pc + 1] as i32) as usize]
            }
            other => bail!("Can't process mode {}", other),
        };
        if opcode == 4 {
            return Ok(Some(Operation {
                opcode,
                v1,
                v2: 0,
                w: 0,
                pc: pc + 2,
                rb,
            }));
        }
        if opcode == 9 {
            return Ok(Some(Operation {
                opcode,
                v1: 0,
                v2: 0,
                w: 0,
                pc: pc + 2,
                rb: rb + v1 as i32,
            }));
        }
        let v2 = match (mem[pc] / 1000) % 10 {
            0 => {
                if mem.len() as i64 <= mem[pc + 2] {
                    mem.resize(mem[pc + 2] as usize + 1, 0);
                }
                mem[mem[pc + 2] as usize]
            }
            1 => mem[pc + 2],
            2 => {
                if mem.len() as i32 <= rb + mem[pc + 2] as i32 {
                    mem.resize((rb + mem[pc + 2] as i32) as usize + 1, 0);
                }
                mem[(rb + mem[pc + 2] as i32) as usize]
            }
            other => bail!("Can't mode opcode {}", other),
        };
        match opcode {
            1 | 2 | 7 | 8 => {
                let w = match (mem[pc] / 10000) % 10 {
                    0 => mem[pc + 3] as usize,
                    2 => (rb + mem[pc + 3] as i32) as usize,
                    _ => unreachable!(),
                };
                if mem.len() <= w {
                    mem.resize(w + 1, 0);
                }
                Ok(Some(Operation {
                    opcode,
                    v1,
                    v2,
                    w,
                    pc: pc + 4,
                    rb,
                }))
            }
            5 | 6 => Ok(Some(Operation {
                opcode,
                v1,
                v2,
                w: 0,
                pc,
                rb,
            })),
            other => bail!("Can't process opcode {}", other),
        }
    }
}

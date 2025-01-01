use std::collections::VecDeque;

use crate::Error;

type Result<T> = std::result::Result<T, Error>;

pub struct Computer {
    memory: Vec<i64>,
    pc: usize,
    rb: i64,
    input: VecDeque<i64>,
    output: VecDeque<i64>,
    state: State,
}

impl Computer {
    pub fn new(input: &str) -> Result<Self> {
        let memory = input
            .split(',')
            .map(|n| n.parse().map_err(Error::from))
            .collect::<Result<Vec<_>>>()?;

        Ok(Computer {
            memory,
            pc: 0,
            rb: 0,
            input: VecDeque::new(),
            output: VecDeque::new(),
            state: State::Ready,
        })
    }

    pub fn run(&mut self) -> Result<&mut Self> {
        if self.state == State::Done {
            return Ok(self);
        }

        if self.state == State::Wait {
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

    pub fn step(&mut self) -> Result<State> {
        let Some(op) = Operation::new(&mut self.memory, self.pc, self.rb)? else {
            return Ok(State::Done);
        };

        match op {
            Operation::Add { w, v1, v2 } => self.memory[w] = v1 + v2,
            Operation::Mul { w, v1, v2 } => self.memory[w] = v1 * v2,
            Operation::Input { w } => match self.input.pop_front() {
                Some(input) => self.memory[w] = input,
                None => {
                    self.state = State::Wait;

                    return Ok(State::Wait);
                }
            },
            Operation::Output(val) => self.output.push_back(val),
            Operation::JumpIfTrue { v1, v2 } => {
                if v1 != 0 {
                    self.pc = v2 as usize;
                } else {
                    self.pc += 3;
                }
            }
            Operation::JumpIfFalse { v1, v2 } => {
                if v1 == 0 {
                    self.pc = v2 as usize;
                } else {
                    self.pc += 3;
                }
            }
            Operation::LessThan { w, v1, v2 } => self.memory[w] = if v1 < v2 { 1 } else { 0 },
            Operation::Equals { w, v1, v2 } => self.memory[w] = if v1 == v2 { 1 } else { 0 },
            Operation::RelativeBase(rb) => self.rb += rb,
        }

        self.pc += op.pc();

        Ok(State::Ready)
    }

    pub fn pop(&mut self) -> Option<i64> {
        self.output.pop_front()
    }

    pub fn insert(&mut self, input: i64) -> &mut Self {
        self.input.push_back(input);

        self
    }

    pub fn output_iter(&self) -> impl Iterator<Item = &'_ i64> {
        self.output.iter()
    }
}

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum State {
    Ready,
    Done,
    Wait,
}

enum Operation {
    Add { w: usize, v1: i64, v2: i64 },
    Mul { w: usize, v1: i64, v2: i64 },
    Input { w: usize },
    Output(i64),
    JumpIfTrue { v1: i64, v2: i64 },
    JumpIfFalse { v1: i64, v2: i64 },
    LessThan { w: usize, v1: i64, v2: i64 },
    Equals { w: usize, v1: i64, v2: i64 },
    RelativeBase(i64),
}

impl Operation {
    fn pc(&self) -> usize {
        match self {
            Operation::Add { .. }
            | Operation::Mul { .. }
            | Operation::LessThan { .. }
            | Operation::Equals { .. } => 4,
            Operation::JumpIfTrue { .. } | Operation::JumpIfFalse { .. } => 0,
            Operation::Input { .. } | Operation::Output(_) | Operation::RelativeBase(_) => 2,
        }
    }

    fn new(mem: &mut Vec<i64>, pc: usize, rb: i64) -> Result<Option<Self>> {
        if mem.len() < pc + 4 {
            mem.resize(pc + 4, 0);
        }

        let opcode = mem[pc] % 100;

        match opcode {
            3 => {
                let w = Self::write_idx::<1>(mem, pc, rb)?;

                return Ok(Some(Self::Input { w }));
            }
            99 => return Ok(None),
            _ => {}
        }

        let v1 = Self::value::<1>(mem, pc, rb)?;

        match opcode {
            4 => return Ok(Some(Self::Output(v1))),
            9 => return Ok(Some(Self::RelativeBase(v1))),
            _ => {}
        }

        let v2 = Self::value::<2>(mem, pc, rb)?;

        match opcode {
            5 => return Ok(Some(Self::JumpIfTrue { v1, v2 })),
            6 => return Ok(Some(Self::JumpIfFalse { v1, v2 })),
            _ => {}
        }

        let w = Self::write_idx::<3>(mem, pc, rb)?;

        match opcode {
            1 => Ok(Some(Self::Add { w, v1, v2 })),
            2 => Ok(Some(Self::Mul { w, v1, v2 })),
            7 => Ok(Some(Self::LessThan { w, v1, v2 })),
            8 => Ok(Some(Self::Equals { w, v1, v2 })),
            opcode => bail!("Can't process opcode {opcode}"),
        }
    }

    fn value<const IDX: usize>(mem: &mut Vec<i64>, pc: usize, rb: i64) -> Result<i64> {
        let idx = match (mem[pc] / 10_i64.pow(IDX as u32 + 1)) % 10 {
            0 => mem[pc + IDX] as usize,
            1 => pc + IDX,
            2 => (rb + mem[pc + IDX]) as usize,
            mode => bail!("Unknown value mode {mode}"),
        };

        if mem.len() <= idx {
            mem.resize(idx + 1, 0);
        }

        Ok(mem[idx])
    }

    fn write_idx<const IDX: usize>(mem: &mut Vec<i64>, pc: usize, rb: i64) -> Result<usize> {
        let idx = match (mem[pc] / 10_i64.pow(IDX as u32 + 1)) % 10 {
            0 => mem[pc + IDX] as usize,
            2 => (rb + mem[pc + IDX]) as usize,
            mode => bail!("Unknown write mode {mode}"),
        };

        if mem.len() <= idx {
            mem.resize(idx + 1, 0);
        }

        Ok(idx)
    }
}

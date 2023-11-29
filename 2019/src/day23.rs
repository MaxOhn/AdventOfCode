use crate::{
    computer::{Computer, State},
    Error, Solution,
};

use std::collections::{HashSet, VecDeque};

pub fn run(input: &str) -> eyre::Result<aoc_rust::Solution> {
    let solution = solve(input)?;

    Ok(aoc_rust::Solution::new()
        .part1(solution.part1)
        .part2(solution.part2))
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
struct Packet {
    dst: usize,
    x: i64,
    y: Option<i64>,
}

impl Packet {
    fn new(dst: usize, x: i64, y: Option<i64>) -> Self {
        Self { dst, x, y }
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
struct PacketBuffer {
    buf: [i64; 2],
    len: usize,
}

impl PacketBuffer {
    fn new() -> Self {
        PacketBuffer {
            buf: [0; 2],
            len: 0,
        }
    }

    fn push(&mut self, val: i64) -> Option<Packet> {
        match self.len {
            0 | 1 => {
                if self.len == 1 && val == -1 {
                    self.len = 0;
                    return Some(Packet::new(self.buf[0] as usize, -1, None));
                }
                self.buf[self.len] = val;
                self.len += 1;
                None
            }
            2 => {
                self.len = 0;
                Some(Packet::new(self.buf[0] as usize, self.buf[1], Some(val)))
            }
            _ => unreachable!(),
        }
    }
}

pub fn solve(input: &str) -> Result<Solution<i64, i64>, Error> {
    let mut computers = Vec::with_capacity(50);
    for network_address in 0..50 {
        let mut computer = Computer::new(input.to_owned())?;
        computer.insert(network_address).insert(-1);
        computers.push(computer);
    }
    let mut buffers = [PacketBuffer::new(); 50];
    let mut idle = HashSet::new();
    #[allow(non_snake_case)]
    let mut NAT: Option<Packet> = None;
    #[allow(non_snake_case)]
    let mut prev_NATs = HashSet::with_capacity(50);
    let mut input_queues: Vec<VecDeque<Packet>> = vec![VecDeque::new(); 50];
    let mut p1 = None;
    loop {
        for i in 0..50 {
            match computers[i].step()? {
                State::Wait => {
                    match input_queues[i].pop_front() {
                        Some(packet) => {
                            idle.remove(&i);
                            computers[i].insert(packet.x);
                            if let Some(y) = packet.y {
                                computers[i].insert(y);
                            }
                        }
                        None => {
                            idle.insert(i);
                            computers[i].insert(-1);
                        }
                    };
                    computers[i].step()?;
                }
                State::Ready => {
                    if let Some(output) = computers[i].pop() {
                        if let Some(packet) = buffers[i].push(output) {
                            if packet.dst < 50 {
                                input_queues[packet.dst].push_back(packet);
                            } else {
                                assert_eq!(packet.dst, 255);
                                assert!(packet.y.is_some());
                                if p1.is_none() {
                                    p1 = packet.y;
                                }
                                NAT = Some(packet);
                            }
                        }
                    }
                }
                State::Done => bail!(
                    "Computer {} finished, state should not have been reachable",
                    i
                ),
            }
        }
        if idle.len() == 50 {
            match NAT {
                Some(packet) => {
                    input_queues[0].push_back(packet);
                    idle.remove(&0);
                    if !prev_NATs.insert(packet) {
                        break;
                    }
                }
                None => bail!("Tried sending NAT packet but NAT is None"),
            }
        }
    }
    Ok(Solution::new(p1.unwrap(), NAT.unwrap().y.unwrap()))
} // 2.99s

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test23() {
        crate::util::tests::test_full_problem(23, solve, 17283, 11319);
    }
}

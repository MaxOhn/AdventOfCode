use hashbrown::HashMap;
use std::cell::RefCell;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::rc::Rc;
use std::time::Instant;

type Signal = u16;
type RcWire = Rc<RefCell<Wire>>;

trait StrExt {
    fn hash(&self) -> u32;
}

impl StrExt for &str {
    #[inline]
    fn hash(&self) -> u32 {
        self.chars().fold(0, |hash, c| (hash << 8) + c as u32)
    }
}

trait HashMapExt {
    fn get_or_insert_source(&mut self, name: &str) -> Source;
    fn get_or_insert_target(&mut self, name: &str) -> RcWire;
}

impl HashMapExt for HashMap<u32, Rc<RefCell<Wire>>> {
    #[inline]
    fn get_or_insert_source(&mut self, name: &str) -> Source {
        if name.chars().all(|c| c.is_numeric()) {
            let n: Signal = name.parse().unwrap();

            return Source::Value(n);
        }

        Source::Wire(self.get_or_insert_target(name))
    }

    #[inline]
    fn get_or_insert_target(&mut self, name: &str) -> RcWire {
        let value = self
            .entry(name.hash())
            .or_insert_with(|| Rc::new(RefCell::new(Wire::new())));

        Rc::clone(value)
    }
}

fn main() {
    let start = Instant::now();

    let file = File::open("./input").unwrap();
    let mut reader = BufReader::new(file);
    let mut buf = String::new();

    let mut part1_wires = HashMap::new();
    let mut part2_wires = HashMap::new();

    while reader.read_line(&mut buf).unwrap() > 0 {
        let mut words = buf.trim_end().split(' ');

        let first = words.next().unwrap();
        let second = words.next().unwrap();

        if first == "NOT" {
            let third = words.nth(1).unwrap();

            let src = part1_wires.get_or_insert_source(second);
            let gate = Gate::Not { src };
            let target = part1_wires.get_or_insert_target(third);
            target.borrow_mut().src.replace(gate.into());

            let src = part2_wires.get_or_insert_source(second);
            let gate = Gate::Not { src };
            let target = part2_wires.get_or_insert_target(third);
            target.borrow_mut().src.replace(gate.into());
        } else if second == "->" {
            let third = words.next().unwrap();

            let src = part1_wires.get_or_insert_source(first);
            let target = part1_wires.get_or_insert_target(third);
            target.borrow_mut().src.replace(src);

            let src = part2_wires.get_or_insert_source(first);
            let target = part2_wires.get_or_insert_target(third);
            target.borrow_mut().src.replace(src);
        } else {
            let third = words.next().unwrap();
            let fourth = words.nth(1).unwrap();

            let lhs = part1_wires.get_or_insert_source(first);
            let rhs = part1_wires.get_or_insert_source(third);

            #[rustfmt::skip]
            let gate = match second {
                "AND" => Gate::And { lhs, rhs },
                "OR" => Gate::Or{ lhs, rhs },
                "RSHIFT" => Gate::Rshift { src: lhs, shift: rhs },
                "LSHIFT" => Gate::Lshift { src: lhs, shift: rhs },
                _ => unreachable!(),
            };

            let target = part1_wires.get_or_insert_target(fourth);
            target.borrow_mut().src.replace(gate.into());

            let lhs = part2_wires.get_or_insert_source(first);
            let rhs = part2_wires.get_or_insert_source(third);

            #[rustfmt::skip]
            let gate = match second {
                "AND" => Gate::And { lhs, rhs },
                "OR" => Gate::Or{ lhs, rhs },
                "RSHIFT" => Gate::Rshift { src: lhs, shift: rhs },
                "LSHIFT" => Gate::Lshift { src: lhs, shift: rhs },
                _ => unreachable!(),
            };

            let target = part2_wires.get_or_insert_target(fourth);
            target.borrow_mut().src.replace(gate.into());
        }

        buf.clear();
    }

    println!("Parsed input [{:?}]", start.elapsed());

    let part1 = part1_wires[&"a".hash()].borrow_mut().value();
    println!("Part 1: {:?} [{:?}]", part1, start.elapsed());

    part2_wires[&"b".hash()]
        .borrow_mut()
        .src
        .replace(part1.into());

    let part2 = part2_wires[&"a".hash()].borrow_mut().value();
    println!("Part 2: {:?} [{:?}]", part2, start.elapsed());
}

#[derive(Clone, Debug)]
enum Source {
    Wire(Rc<RefCell<Wire>>),
    Value(Signal),
    Gate(Box<Gate>),
}

impl Source {
    #[inline]
    fn value(&self) -> Signal {
        match self {
            Source::Wire(wire) => wire.borrow_mut().value(),
            Source::Value(value) => *value,
            Source::Gate(gate) => gate.value(),
        }
    }
}

impl From<Signal> for Source {
    #[inline]
    fn from(value: Signal) -> Self {
        Self::Value(value)
    }
}

impl From<Gate> for Source {
    #[inline]
    fn from(gate: Gate) -> Self {
        Self::Gate(Box::new(gate))
    }
}

#[derive(Clone, Debug)]
enum Gate {
    And { lhs: Source, rhs: Source },
    Or { lhs: Source, rhs: Source },
    Not { src: Source },
    Lshift { src: Source, shift: Source },
    Rshift { src: Source, shift: Source },
}

impl Gate {
    #[inline]
    fn value(&self) -> Signal {
        match self {
            Gate::And { lhs, rhs } => lhs.value() & rhs.value(),
            Gate::Or { lhs, rhs } => lhs.value() | rhs.value(),
            Gate::Not { src } => !src.value(),
            Gate::Lshift { src, shift } => src.value() << shift.value(),
            Gate::Rshift { src, shift } => src.value() >> shift.value(),
        }
    }
}

#[derive(Clone, Debug)]
struct Wire {
    src: Option<Source>,
}

impl Wire {
    #[inline]
    fn new() -> Self {
        Self { src: None }
    }

    #[inline]
    fn value(&mut self) -> Signal {
        let value = self.src.as_ref().map(Source::value).unwrap();
        self.src.replace(value.into());

        value
    }
}

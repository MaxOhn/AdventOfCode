use std::{
    error::Error,
    fmt,
    fs::File,
    intrinsics::transmute,
    io::{BufRead, BufReader},
    ops::Add,
    str::Chars,
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

    let mut elems: Vec<Elem> = Vec::new();
    let mut sum = Elem::Number(0);
    let mut p2 = 0;

    while input.read_line(&mut line)? != 0 {
        let elem = Elem::from_str(line.trim_end());
        sum = sum + elem.clone();

        for prev in &elems {
            p2 = p2
                .max((elem.clone() + prev.to_owned()).magnitude())
                .max((prev.to_owned() + elem.clone()).magnitude());
        }

        elems.push(elem);
        line.clear();
    }

    let p1 = sum.magnitude();
    let elapsed = start.elapsed();
    println!("Part 1: {}", p1);
    println!("Part 2: {}", p2);
    println!("Elapsed: {:?}", elapsed); // 39.8ms

    assert_eq!(p1, 4480);
    assert_eq!(p2, 4676);

    Ok(())
}

#[derive(Clone)]
enum Elem {
    Pair(Box<(Elem, Elem)>),
    Number(u32),
}

enum ElemInner<'e> {
    Pair(&'e Elem, &'e Elem),
    Number(&'e u32),
}

impl fmt::Debug for Elem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.inner() {
            ElemInner::Pair(lhs, rhs) => write!(f, "[{:?},{:?}]", lhs, rhs),
            ElemInner::Number(n) => write!(f, "{}", n),
        }
    }
}

struct ToExplode<'e> {
    pair: &'e Elem,
    next_lhs: Option<&'e u32>,
    next_rhs: Option<&'e u32>,
}

impl fmt::Debug for ToExplode<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "..,{:?}],{:?},[{:?},..",
            self.next_lhs, self.pair, self.next_rhs
        )
    }
}

impl Elem {
    fn from_str(s: &str) -> Self {
        let mut chars = s.chars();

        Self::from_chars(&mut chars)
    }

    fn from_chars(chars: &mut Chars<'_>) -> Self {
        match chars.next() {
            Some('[') => {
                let lhs = Self::from_chars(chars);
                assert_eq!(chars.next(), Some(','));
                let rhs = Self::from_chars(chars);
                assert_eq!(chars.next(), Some(']'));

                Elem::Pair(Box::new((lhs, rhs)))
            }
            Some(n @ '0'..='9') => Elem::Number(n as u32 & 0x0F),
            c => unreachable!("expected [ or num, got {:?}", c),
        }
    }

    fn inner(&self) -> ElemInner<'_> {
        match self {
            Self::Pair(pair) => ElemInner::Pair(&pair.0, &pair.1),
            Self::Number(n) => ElemInner::Number(n),
        }
    }

    fn find_nested_four(&self) -> Option<ToExplode<'_>> {
        match self.inner() {
            ElemInner::Pair(lhs, rhs) => lhs
                .find_nested_n(None, Some(rhs), 4)
                .or_else(|| rhs.find_nested_n(Some(lhs), None, 4)),
            ElemInner::Number(_) => None,
        }
    }

    fn find_nested_n<'e>(
        &'e self,
        next_lhs: Option<&'e Elem>,
        next_rhs: Option<&'e Elem>,
        n: u8,
    ) -> Option<ToExplode<'e>> {
        match self.inner() {
            ElemInner::Pair(lhs, rhs) if n > 1 => lhs
                .find_nested_n(next_lhs, Some(rhs), n - 1)
                .or_else(|| rhs.find_nested_n(Some(lhs), next_rhs, n - 1)),
            ElemInner::Pair(lhs, rhs) => Self::is_number_pair(lhs, rhs).then(|| ToExplode {
                pair: self,
                next_lhs: next_lhs.map(Elem::far_right),
                next_rhs: next_rhs.map(Elem::far_left),
            }),
            ElemInner::Number(_) => None,
        }
    }

    fn is_number_pair(lhs: &Elem, rhs: &Elem) -> bool {
        matches!((lhs, rhs), (Self::Number(_), Self::Number(_)))
    }

    fn far_left(&self) -> &u32 {
        match self {
            Self::Pair(pair) => pair.0.far_left(),
            Self::Number(n) => n,
        }
    }

    fn far_right(&self) -> &u32 {
        match self {
            Self::Pair(pair) => pair.1.far_right(),
            Self::Number(n) => n,
        }
    }

    fn magnitude(&self) -> u64 {
        match self {
            Self::Pair(pair) => 3 * pair.0.magnitude() + 2 * pair.1.magnitude(),
            Self::Number(n) => *n as u64,
        }
    }

    #[allow(mutable_transmutes)]
    fn explode(&self) -> Option<bool> {
        self.find_nested_four().map(|to_explode| {
            let ToExplode {
                pair,
                next_lhs,
                next_rhs,
            } = to_explode;

            let mut ten = false;

            if let Some(next_lhs) = next_lhs.map(|lhs| unsafe { transmute::<_, &mut u32>(lhs) }) {
                *next_lhs += pair.far_left();
                ten |= *next_lhs >= 10;
            }

            if let Some(next_rhs) = next_rhs.map(|rhs| unsafe { transmute::<_, &mut u32>(rhs) }) {
                *next_rhs += pair.far_right();
                ten |= *next_rhs >= 10;
            }

            let pair: &mut Elem = unsafe { transmute(pair) };
            *pair = Elem::Number(0);

            ten
        })
    }

    fn split(&mut self) -> bool {
        match self {
            Self::Pair(pair) => pair.0.split() || pair.1.split(),
            Self::Number(n) if *n >= 10 => {
                let lhs = Elem::Number(*n / 2);
                let rhs = Elem::Number(*n / 2 + ((*n % 2) == 1) as u32);
                *self = Elem::Pair(Box::new((lhs, rhs)));

                true
            }
            Self::Number(_) => false,
        }
    }
}

impl Add for Elem {
    type Output = Elem;

    fn add(self, rhs: Self) -> Self::Output {
        let mut sum = Self::Pair(Box::new((self, rhs)));
        let mut ten = false;

        loop {
            if let Some(ten_) = sum.explode() {
                ten |= ten_;
                continue;
            }

            if ten && sum.split() {
                continue;
            }

            return sum;
        }
    }
}

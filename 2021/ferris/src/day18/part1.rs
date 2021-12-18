pub fn run(input: &[u8]) -> i64 {
    single_box::run(input)
}

pub mod double_box {
    use std::{intrinsics::transmute, ops::Add, str::Chars};

    pub fn run(input: &[u8]) -> i64 {
        let input = unsafe { std::str::from_utf8_unchecked(input) };

        input
            .lines()
            .map(Elem::from_str)
            .fold(Elem::Number(0), |sum, elem| sum + elem)
            .magnitude()
    }

    #[derive(Clone)]
    enum Elem {
        Pair(Box<Elem>, Box<Elem>),
        Number(i64),
    }

    struct NestedResult<'e> {
        pair: &'e Elem,
        next_lhs: Option<&'e i64>,
        next_rhs: Option<&'e i64>,
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

                    Elem::Pair(Box::new(lhs), Box::new(rhs))
                }
                Some(n @ '0'..='9') => Elem::Number(n as i64 & 0x0F),
                c => unreachable!("expected [ or num, got {:?}", c),
            }
        }

        fn find_nested_four(&self) -> Option<NestedResult<'_>> {
            match self {
                Self::Pair(lhs, rhs) => lhs
                    .find_nested_n(None, Some(rhs), 4)
                    .or_else(|| rhs.find_nested_n(Some(lhs), None, 4)),
                Self::Number(_) => None,
            }
        }

        fn find_nested_n<'e>(
            &'e self,
            next_lhs: Option<&'e Elem>,
            next_rhs: Option<&'e Elem>,
            n: u8,
        ) -> Option<NestedResult<'e>> {
            match self {
                Self::Pair(lhs, rhs) => {
                    if n == 1 {
                        matches!((&**lhs, &**rhs), (Self::Number(_), Self::Number(_))).then(|| {
                            NestedResult {
                                pair: self,
                                next_lhs: next_lhs.map(Elem::far_right),
                                next_rhs: next_rhs.map(Elem::far_left),
                            }
                        })
                    } else {
                        lhs.find_nested_n(next_lhs, Some(rhs), n - 1)
                            .or_else(|| rhs.find_nested_n(Some(lhs), next_rhs, n - 1))
                    }
                }
                Self::Number(_) => None,
            }
        }

        fn far_left(&self) -> &i64 {
            match self {
                Self::Pair(a, _) => a.far_left(),
                Self::Number(n) => n,
            }
        }

        fn far_right(&self) -> &i64 {
            match self {
                Self::Pair(_, b) => b.far_right(),
                Self::Number(n) => n,
            }
        }

        fn magnitude(&self) -> i64 {
            match self {
                Self::Pair(lhs, rhs) => 3 * lhs.magnitude() + 2 * rhs.magnitude(),
                Self::Number(n) => *n,
            }
        }

        #[allow(mutable_transmutes)]
        fn explode(&self) -> Option<bool> {
            self.find_nested_four().map(|nested| {
                let NestedResult {
                    pair,
                    next_lhs,
                    next_rhs,
                } = nested;

                let mut ten = false;

                if let Some(next_lhs) = next_lhs.map(|lhs| unsafe { transmute::<_, &mut i64>(lhs) })
                {
                    *next_lhs += pair.far_left();
                    ten |= *next_lhs >= 10;
                }

                if let Some(next_rhs) = next_rhs.map(|rhs| unsafe { transmute::<_, &mut i64>(rhs) })
                {
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
                Self::Pair(a, b) => a.split() || b.split(),
                Self::Number(n) if *n >= 10 => {
                    let lhs = Elem::Number(*n / 2);
                    let rhs = Elem::Number(*n / 2 + ((*n % 2) == 1) as i64);
                    *self = Elem::Pair(Box::new(lhs), Box::new(rhs));

                    true
                }
                Self::Number(_) => false,
            }
        }
    }

    impl Add for Elem {
        type Output = Elem;

        fn add(self, rhs: Self) -> Self::Output {
            let mut sum = Self::Pair(Box::new(self), Box::new(rhs));
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
}

pub mod single_box {
    use std::{intrinsics::transmute, ops::Add, str::Chars};

    pub fn run(input: &[u8]) -> i64 {
        let input = unsafe { std::str::from_utf8_unchecked(input) };

        input
            .lines()
            .map(Elem::from_str)
            .fold(Elem::Number(0), |sum, elem| sum + elem)
            .magnitude()
    }

    #[derive(Clone)]
    enum Elem {
        Pair(Box<(Elem, Elem)>),
        Number(i64),
    }

    enum ElemInner<'e> {
        Pair(&'e Elem, &'e Elem),
        Number(&'e i64),
    }

    struct NestedResult<'e> {
        pair: &'e Elem,
        next_lhs: Option<&'e i64>,
        next_rhs: Option<&'e i64>,
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
                    chars.next();
                    let rhs = Self::from_chars(chars);
                    chars.next();

                    Elem::Pair(Box::new((lhs, rhs)))
                }
                Some(n @ '0'..='9') => Elem::Number(n as i64 & 0x0F),
                _ => unreachable!(),
            }
        }

        fn inner(&self) -> ElemInner<'_> {
            match self {
                Self::Pair(pair) => ElemInner::Pair(&pair.0, &pair.1),
                Self::Number(n) => ElemInner::Number(n),
            }
        }

        fn find_nested_four(&self) -> Option<NestedResult<'_>> {
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
        ) -> Option<NestedResult<'e>> {
            match self.inner() {
                ElemInner::Pair(lhs, rhs) => {
                    if n == 1 {
                        matches!((lhs, rhs), (Self::Number(_), Self::Number(_))).then(|| {
                            NestedResult {
                                pair: self,
                                next_lhs: next_lhs.map(Elem::far_right),
                                next_rhs: next_rhs.map(Elem::far_left),
                            }
                        })
                    } else {
                        lhs.find_nested_n(next_lhs, Some(rhs), n - 1)
                            .or_else(|| rhs.find_nested_n(Some(lhs), next_rhs, n - 1))
                    }
                }
                ElemInner::Number(_) => None,
            }
        }

        fn far_left(&self) -> &i64 {
            match self {
                Self::Pair(pair) => pair.0.far_left(),
                Self::Number(n) => n,
            }
        }

        fn far_right(&self) -> &i64 {
            match self {
                Self::Pair(pair) => pair.1.far_right(),
                Self::Number(n) => n,
            }
        }

        fn magnitude(&self) -> i64 {
            match self {
                Self::Pair(pair) => 3 * pair.0.magnitude() + 2 * pair.1.magnitude(),
                Self::Number(n) => *n,
            }
        }

        #[allow(mutable_transmutes)]
        fn explode(&self) -> Option<bool> {
            self.find_nested_four().map(|nested| {
                let NestedResult {
                    pair,
                    next_lhs,
                    next_rhs,
                } = nested;

                let mut ten = false;

                if let Some(next_lhs) = next_lhs.map(|lhs| unsafe { transmute::<_, &mut i64>(lhs) })
                {
                    *next_lhs += pair.far_left();
                    ten |= *next_lhs >= 10;
                }

                if let Some(next_rhs) = next_rhs.map(|rhs| unsafe { transmute::<_, &mut i64>(rhs) })
                {
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
                    let rhs = Elem::Number(*n / 2 + ((*n % 2) == 1) as i64);
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
}

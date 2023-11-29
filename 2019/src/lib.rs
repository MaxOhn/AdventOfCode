#[macro_use]
mod macros;

mod computer;
pub mod day01;
pub mod day01_2018;
pub mod day02;
pub mod day02_2018;
pub mod day03;
pub mod day04;
pub mod day05;
pub mod day06;
pub mod day07;
pub mod day08;
pub mod day09;
pub mod day10;
pub mod day11;
pub mod day12;
pub mod day13;
pub mod day14;
pub mod day15;
pub mod day16;
pub mod day17;
pub mod day18;
pub mod day19;
pub mod day20;
pub mod day21;
pub mod day22;
pub mod day23;
pub mod day24;
pub mod day25;

pub use self::error::Error;
pub use self::solution::Solution;

mod error {
    use std::fmt;
    use std::io;

    #[derive(Debug)]
    pub enum Error {
        Custom(String),
        ParseInt(std::num::ParseIntError),
        Io(io::Error),
    }

    impl From<std::num::ParseIntError> for Error {
        fn from(e: std::num::ParseIntError) -> Self {
            Self::ParseInt(e)
        }
    }

    impl From<io::Error> for Error {
        fn from(e: io::Error) -> Self {
            Self::Io(e)
        }
    }

    impl fmt::Display for Error {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            match self {
                Self::Custom(e) => write!(f, "{}", e),
                Self::ParseInt(e) => write!(f, "{}", e),
                Self::Io(e) => write!(f, "{}", e),
            }
        }
    }

    impl std::error::Error for Error {}
}

mod solution {
    use std::fmt;

    #[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
    pub struct Solution<U, V> {
        pub part1: U,
        pub part2: V,
    }

    impl<U, V> Solution<U, V> {
        pub fn new(part1: U, part2: V) -> Self {
            Solution { part1, part2 }
        }
    }

    impl<U, V> fmt::Display for Solution<U, V>
    where
        U: fmt::Display,
        V: fmt::Display,
    {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "Part 1:\n{}\nPart 2:\n{}", self.part1, self.part2)
        }
    }
}

pub mod util {
    #![allow(unused)]

    use crate::error::Error;
    use itertools::Itertools;
    use num::traits::{
        identities::{One, Zero},
        sign::{Signed, Unsigned},
        Num,
    };
    use std::{
        cmp::Ordering,
        collections::{BTreeMap, HashMap, HashSet},
        convert::TryFrom,
        fmt::{self, Display},
        hash::Hash,
        iter::FromIterator,
        ops::{
            Add, AddAssign, Deref, DerefMut, Div, DivAssign, Mul, MulAssign, Neg, Rem, RemAssign,
            Sub, SubAssign,
        },
        str::FromStr,
    };

    #[cfg(test)]
    pub mod tests {
        use crate::Error;
        use crate::Solution;
        use std::fmt::Debug;
        use std::fs;

        /// Check whether the solution of a day still gives the correct answer
        pub fn test_full_problem<F, U, V>(day: usize, solve_function: F, part1: U, part2: V)
        where
            F: Fn(&str) -> Result<Solution<U, V>, Error>,
            U: Eq + Debug,
            V: Eq + Debug,
        {
            let input = fs::read_to_string(format!("inputs/day{:02}.txt", day)).unwrap();
            let solution = solve_function(&input).unwrap();
            assert_eq!(solution.part1, part1);
            assert_eq!(solution.part2, part2);
        }
    }

    /// Calculates the greatest common denominator of two numbers
    /// # Examples
    /// ```
    /// use aoc19::util::gcd;
    ///
    /// let n = gcd(6, 15);
    /// assert_eq!(n, 3);
    /// ```
    pub fn gcd<T>(a: T, b: T) -> T
    where
        T: Num + Copy,
    {
        if b == T::zero() {
            a
        } else {
            gcd(b, a % b)
        }
    }

    /// Calculates the least common multiple of two numbers
    /// # Examples
    /// ```
    /// use aoc19::util::lcm;
    ///
    /// let n = lcm(3, 4);
    /// assert_eq!(n, 12);
    /// ```
    pub fn lcm<T>(a: T, b: T) -> T
    where
        T: Num + Copy,
    {
        a * b / gcd(a, b)
    }

    /// Calculate (b^p)%m
    /// # Examples
    /// ```
    /// use aoc19::util::mod_pow;
    ///
    /// let n = mod_pow(2, 3, 5);
    /// assert_eq!(n, 3);
    /// ```
    pub fn mod_pow(b: i64, p: i64, m: i64) -> i64 {
        match p {
            0 => 1,
            e if e % 2 == 0 => mod_pow((b * b) % m, e / 2, m),
            _ => (b * mod_pow(b, p - 1, m)) % m,
        }
    }

    /// Store a grid in a `HashMap<Point2i, T>`.
    /// This way negative coordinates are more easily handled than with a grid of `Vec<Vec<T>>`
    #[derive(Clone, Eq, PartialEq, Debug, Default)]
    pub struct GridMap<T> {
        grid: BTreeMap<Point2i, T>,
        default: T,
    }

    impl<T> GridMap<T> {
        /// Create a new GridMap with a specified default element.
        /// The default element serves as fallback symbol for all
        /// coordinates inbetween contained positions when displaying the grid.
        /// # Examples
        /// ```
        /// use aoc19::util::{GridMap, Point2i};
        ///
        /// let mut grid = GridMap::with_default(5);
        /// grid.insert(Point2i::new(0, 0), 1);
        /// grid.insert(Point2i::new(2, 0), 1);
        /// assert_eq!(grid.to_string(), "151")
        /// ```
        pub fn with_default(default: T) -> Self {
            Self {
                grid: BTreeMap::new(),
                default,
            }
        }

        /// Creates a new GridMap
        pub fn new() -> Self
        where
            T: Default,
        {
            Self::with_default(T::default())
        }

        /// Calculate the current width of the grid
        /// # Examples
        /// ```
        /// use aoc19::util::{GridMap, Point2i};
        ///
        /// let mut grid = GridMap::new();
        /// grid.insert(Point2i::new(-2, 3), 0);
        /// grid.insert(Point2i::new(3, -1), 0);
        /// assert_eq!(grid.get_width(), 6);
        /// ```
        pub fn get_width(&self) -> usize {
            if self.grid.is_empty() {
                0
            } else {
                let mut min = i32::max_value();
                let mut max = i32::min_value();
                for Point2i { x, .. } in self.grid.keys() {
                    if *x < min {
                        min = *x;
                    }
                    if *x > max {
                        max = *x;
                    }
                }
                (max - min + 1) as usize
            }
        }

        /// Calculate the current height of the grid
        /// # Examples
        /// ```
        /// use aoc19::util::{GridMap, Point2i};
        ///
        /// let mut grid = GridMap::new();
        /// grid.insert(Point2i::new(-2, 3), 0);
        /// grid.insert(Point2i::new(3, -1), 0);
        /// assert_eq!(grid.get_height(), 5);
        /// ```
        pub fn get_height(&self) -> usize {
            if self.grid.is_empty() {
                0
            } else {
                let min = self.grid.keys().nth(0).unwrap().y;
                let max = self.grid.keys().last().unwrap().y;
                (max - min + 1) as usize
            }
        }

        /// Get the smallest x coordinate of the grid that has a value.
        /// ```
        /// use aoc19::util::{GridMap, Point2i};
        ///
        /// let mut grid = GridMap::new();
        /// grid.insert(Point2i::new(2, 3), 0);
        /// grid.insert(Point2i::new(-1, 2), 0);
        /// assert_eq!(grid.get_min_x().unwrap(), -1);
        /// ```
        pub fn get_min_x(&self) -> Option<i32> {
            if self.grid.is_empty() {
                None
            } else {
                Some(
                    self.grid
                        .keys()
                        .map(|p| p.x)
                        .fold(i32::max_value(), |min, next| min.min(next)),
                )
            }
        }

        /// Get the largest x coordinate of the grid that has a value.
        /// ```
        /// use aoc19::util::{GridMap, Point2i};
        ///
        /// let mut grid = GridMap::new();
        /// grid.insert(Point2i::new(2, 3), 0);
        /// grid.insert(Point2i::new(-1, 2), 0);
        /// assert_eq!(grid.get_max_x().unwrap(), 2);
        /// ```
        pub fn get_max_x(&self) -> Option<i32> {
            if self.grid.is_empty() {
                None
            } else {
                Some(
                    self.grid
                        .keys()
                        .map(|p| p.x)
                        .fold(i32::min_value(), |max, next| max.max(next)),
                )
            }
        }

        /// Get the smallest y coordinate of the grid that has a value.
        /// ```
        /// use aoc19::util::{GridMap, Point2i};
        ///
        /// let mut grid = GridMap::new();
        /// grid.insert(Point2i::new(2, 3), 0);
        /// grid.insert(Point2i::new(-1, 2), 0);
        /// assert_eq!(grid.get_min_y().unwrap(), 2);
        /// ```
        pub fn get_min_y(&self) -> Option<i32> {
            if self.grid.is_empty() {
                None
            } else {
                Some(self.grid.iter().next().unwrap().0.y)
            }
        }

        /// Get the largest y coordinate of the grid that has a value.
        /// ```
        /// use aoc19::util::{GridMap, Point2i};
        ///
        /// let mut grid = GridMap::new();
        /// grid.insert(Point2i::new(2, 3), 0);
        /// grid.insert(Point2i::new(-1, 2), 0);
        /// assert_eq!(grid.get_max_y().unwrap(), 3);
        /// ```
        pub fn get_max_y(&self) -> Option<i32> {
            if self.grid.is_empty() {
                None
            } else {
                Some(self.grid.iter().last().unwrap().0.y)
            }
        }

        /// Set the default element for the grid.
        /// The default element serves as fallback symbol for all
        /// coordinates inbetween contained positions when displaying the grid.
        /// The suggested alternative to `let g = GridMap::new(); g.set_default(x);`
        /// is `let g = GridMap::with_default(5);`
        /// # Examples
        /// ```
        /// use aoc19::util::{GridMap, Point2i};
        ///
        /// let mut grid = GridMap::new();
        /// grid.set_default(5);
        /// grid.insert(Point2i::new(0, 0), 1);
        /// grid.insert(Point2i::new(2, 0), 1);
        /// assert_eq!(grid.to_string(), "151")
        /// ```
        pub fn set_default(&mut self, default: T) {
            self.default = default;
        }

        /// This method maps entries of the grid according to the mapping of a HashMap.
        /// The default argument serves as fallback symbol for all
        /// coordinates in the new GridMap inbetween inserted positions.
        /// # Examples
        /// ```
        /// # use aoc19::Error;
        /// use std::collections::HashMap;
        /// use aoc19::util::{Point2i, GridMap};
        ///
        /// # fn main() -> Result<(), Error> {
        /// let mut mapping = HashMap::new();
        /// mapping.insert(0, 'x');
        /// mapping.insert(1, '█');
        /// let mut grid = GridMap::new();
        /// grid.insert(Point2i::new(0, 0), 1); // becomes '█'
        /// grid.insert(Point2i::new(1, 0), 0); // becomes 'x'
        /// let mapped_grid = grid.map_values(&mapping, None)?;
        /// assert_eq!(mapped_grid.to_string(), "█x".to_string());
        /// # Ok(())
        /// # }
        /// ```
        pub fn map_values<U>(
            &self,
            mapping: &HashMap<T, U>,
            default: Option<U>,
        ) -> Result<GridMap<U>, Error>
        where
            T: Eq + Hash + Display,
            U: Clone + Default,
        {
            let grid = BTreeMap::from_iter(self.grid.iter().map(|(p, v)| {
                (
                    *p,
                    mapping
                        .get(&v)
                        .or_else(|| panic!("Could not find mapping for {}", v)) // TODO: Handle?
                        .unwrap()
                        .clone(),
                )
            }));
            Ok(GridMap {
                grid,
                default: default.unwrap_or_else(U::default),
            })
        }
    }

    impl<T> Deref for GridMap<T> {
        type Target = BTreeMap<Point2i, T>;
        fn deref(&self) -> &Self::Target {
            &self.grid
        }
    }

    impl<T> DerefMut for GridMap<T> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.grid
        }
    }

    impl<T> FromIterator<(Point2i, T)> for GridMap<T>
    where
        T: Default,
    {
        fn from_iter<I>(iter: I) -> Self
        where
            I: IntoIterator<Item = (Point2i, T)>,
        {
            GridMap {
                grid: BTreeMap::from_iter(iter),
                default: T::default(),
            }
        }
    }

    impl<T> Display for GridMap<T>
    where
        T: Display + Copy + Clone,
    {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let mut matrix = vec![vec![self.default; self.get_width()]; self.get_height()];
            let min_x = self.get_min_x();
            if min_x.is_none() {
                error!("Could not find min x to display GridMap");
            }
            let min_y = self.get_min_y();
            if min_y.is_none() {
                error!("Could not find min y to display GridMap");
            }
            for (p, v) in self.iter() {
                matrix[(p.y - min_y.unwrap()) as usize][(p.x - min_x.unwrap()) as usize] = *v;
            }
            write!(
                f,
                "{}",
                matrix
                    .iter()
                    .map(|row| row.iter().map(T::to_string).collect::<String>())
                    .join("\n")
            )
        }
    }

    /// Simple struct containing two i32 values, generally used as coordinate.
    pub type Point2i = Point2<i32>;
    /// Simple struct containing two usize values, generally used as coordinate.
    pub type Point2us = Point2<usize>;

    /// Simple struct containing two values of the same type, generally used as coordinate.
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
    pub struct Point2<T> {
        pub x: T,
        pub y: T,
    }

    impl<T> Point2<T>
    where
        T: Add<Output = T> + Ord + PartialOrd + Copy,
    {
        pub fn new(x: T, y: T) -> Self {
            Point2 { x, y }
        }

        /// Sum the x and y fields
        /// # Examples
        /// ```
        /// use aoc19::util::Point2;
        ///
        /// let p = Point2::new(-3, 4);
        /// assert_eq!(p.sum(), 1);
        /// ```
        pub fn sum(&self) -> T {
            self.x + self.y
        }

        /// Check whether the point is whithin the given bounds
        /// # Examples
        /// ```
        /// use aoc19::util::Point2;
        ///
        /// let p = Point2::new(4, 5);
        /// assert!(p.in_bounds(0, 0, 5, 6));
        /// assert!(!p.in_bounds(0, 0, 5, 5));
        /// ```
        pub fn in_bounds(&self, low_x: T, low_y: T, high_x: T, high_y: T) -> bool {
            low_x <= self.x && self.x < high_x && low_y <= self.y && self.y < high_y
        }

        /// Clamp x and y value inbetween a min and max
        /// # Examples
        /// ```
        /// use aoc19::util::Point2;
        ///
        /// let p = Point2::new(-2, 7).restrict(-4, 4);
        /// assert_eq!(p.x, -2);
        /// assert_eq!(p.y, 4);
        /// ```
        pub fn restrict(mut self, min: T, max: T) -> Self {
            self.x = self.x.min(max).max(min);
            self.y = self.y.min(max).max(min);
            self
        }
    }

    impl<T> Ord for Point2<T>
    where
        T: Ord + PartialOrd,
    {
        fn cmp(&self, other: &Self) -> Ordering {
            if self.y != other.y {
                self.y.cmp(&other.y)
            } else {
                self.x.cmp(&other.x)
            }
        }
    }

    impl<T> PartialOrd for Point2<T>
    where
        T: Ord + PartialOrd,
    {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            Some(self.cmp(other))
        }
    }

    impl<T> Display for Point2<T>
    where
        T: Display,
    {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "{},{}", self.x, self.y)
        }
    }

    macro_rules! impl_op {
        ($struct:ident, $trait:ident, $fn:ident, $assign_trait:ident, $assign_fn:ident) => {
            impl<T> $trait for $struct<T>
            where
                T: $trait<Output = T>,
            {
                type Output = Self;
                fn $fn(self, other: Self) -> Self {
                    $struct {
                        x: T::$fn(self.x, other.x),
                        y: T::$fn(self.y, other.y),
                    }
                }
            }

            impl<T> $assign_trait for $struct<T>
            where
                T: $assign_trait,
            {
                fn $assign_fn(&mut self, other: Self) {
                    T::$assign_fn(&mut self.x, other.x);
                    T::$assign_fn(&mut self.y, other.y);
                }
            }
        };
    }

    macro_rules! impl_scalar {
        ($struct: ident, $trait:ident, $fn:ident, $assign_trait:ident, $assign_fn:ident) => {
            impl<T> $trait<T> for $struct<T>
            where
                T: $trait<Output = T> + Clone,
            {
                type Output = Self;
                fn $fn(self, other: T) -> Self {
                    $struct {
                        x: T::$fn(self.x, other.clone()),
                        y: T::$fn(self.y, other),
                    }
                }
            }

            impl<T> $assign_trait<T> for $struct<T>
            where
                T: $assign_trait + Clone,
            {
                fn $assign_fn(&mut self, other: T) {
                    T::$assign_fn(&mut self.x, other.clone());
                    T::$assign_fn(&mut self.y, other);
                }
            }
        };
    }

    impl_op!(Point2, Add, add, AddAssign, add_assign);
    impl_op!(Point2, Sub, sub, SubAssign, sub_assign);
    impl_op!(Point2, Mul, mul, MulAssign, mul_assign);
    impl_op!(Point2, Div, div, DivAssign, div_assign);
    impl_op!(Point2, Rem, rem, RemAssign, rem_assign);

    impl_scalar!(Point2, Mul, mul, MulAssign, mul_assign);
    impl_scalar!(Point2, Div, div, DivAssign, div_assign);
    impl_scalar!(Point2, Rem, rem, RemAssign, rem_assign);

    impl<T> Neg for Point2<T>
    where
        T: Neg<Output = T>,
    {
        type Output = Self;
        fn neg(self) -> Self {
            Point2 {
                x: T::neg(self.x),
                y: T::neg(self.y),
            }
        }
    }

    impl<T> Zero for Point2<T>
    where
        T: Zero,
    {
        fn zero() -> Self {
            Point2 {
                x: T::zero(),
                y: T::zero(),
            }
        }

        fn is_zero(&self) -> bool {
            T::is_zero(&self.x) && T::is_zero(&self.y)
        }
    }

    impl<T> One for Point2<T>
    where
        T: One,
    {
        fn one() -> Self {
            Point2 {
                x: T::one(),
                y: T::one(),
            }
        }
    }

    impl<T> Num for Point2<T>
    where
        T: Num,
    {
        type FromStrRadixErr = T::FromStrRadixErr;
        fn from_str_radix(s: &str, radix: u32) -> Result<Self, Self::FromStrRadixErr> {
            let mut iter = s.split(',');
            if let Some(value) = iter.next() {
                let x = T::from_str_radix(value.trim(), radix)?;
                if let Some(value) = iter.next() {
                    let y = T::from_str_radix(value.trim(), radix)?;
                    if iter.next().is_none() {
                        return Ok(Point2 { x, y });
                    }
                }
            }
            if let Err(e) = T::from_str_radix("", radix) {
                return Err(e);
            }
            unreachable!()
        }
    }

    impl<T> Signed for Point2<T>
    where
        T: Signed + Clone,
    {
        fn abs(&self) -> Self {
            Point2 {
                x: T::abs(&self.x),
                y: T::abs(&self.y),
            }
        }

        fn abs_sub(&self, other: &Self) -> Self {
            let delta = self.clone() - other.clone();
            <Self as Signed>::abs(&delta)
        }

        fn signum(&self) -> Self {
            Point2 {
                x: T::signum(&self.x),
                y: T::signum(&self.y),
            }
        }

        fn is_positive(&self) -> bool {
            T::is_positive(&self.x) && T::is_positive(&self.y)
        }

        fn is_negative(&self) -> bool {
            T::is_negative(&self.x) && T::is_negative(&self.y)
        }
    }

    impl<T> Unsigned for Point2<T> where T: Unsigned {}

    impl<T> FromStr for Point2<T>
    where
        T: Num,
    {
        type Err = <Self as Num>::FromStrRadixErr;
        fn from_str(s: &str) -> Result<Self, Self::Err> {
            <Self as Num>::from_str_radix(s, 10)
        }
    }

    /// Simple struct containing three i32 values, generally used as coordinate.
    pub type Point3i = Point3<i32>;
    /// Simple struct containing three usize values, generally used as coordinate.
    pub type Point3us = Point3<usize>;

    /// Simple struct containing three values of the same type, generally used as coordinate.
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
    pub struct Point3<T> {
        pub x: T,
        pub y: T,
        pub z: T,
    }

    impl<T> Point3<T>
    where
        T: Add<Output = T> + Ord + PartialOrd + Copy,
    {
        pub fn new(x: T, y: T, z: T) -> Self {
            Point3 { x, y, z }
        }

        /// Sum the x, y, and z fields
        /// # Examples
        /// ```
        /// use aoc19::util::Point3;
        ///
        /// let p = Point3::new(-3, 4, 1);
        /// assert_eq!(p.sum(), 2);
        /// ```
        pub fn sum(&self) -> T {
            self.x + self.y + self.z
        }

        /// Check whether the point is whithin the given bounds
        /// # Examples
        /// ```
        /// use aoc19::util::Point3;
        ///
        /// let p = Point3::new(4, 5, -2);
        /// assert!(p.in_bounds(0, 0, -2, 5, 6, 3));
        /// assert!(!p.in_bounds(0, 0, -2, 5, 5, 3));
        /// ```
        pub fn in_bounds(
            &self,
            low_x: T,
            low_y: T,
            low_z: T,
            high_x: T,
            high_y: T,
            high_z: T,
        ) -> bool {
            low_x <= self.x
                && self.x < high_x
                && low_y <= self.y
                && self.y < high_y
                && low_z <= self.z
                && self.z < high_z
        }

        /// Clamp x, y, and z values inbetween a min and max
        /// # Examples
        /// ```
        /// use aoc19::util::Point3;
        ///
        /// let p = Point3::new(-2, 7, 3).restrict(-4, 4);
        /// assert_eq!(p.x, -2);
        /// assert_eq!(p.y, 4);
        /// assert_eq!(p.z, 3);
        /// ```
        pub fn restrict(mut self, min: T, max: T) -> Self {
            self.x = self.x.min(max).max(min);
            self.y = self.y.min(max).max(min);
            self.z = self.z.min(max).max(min);
            self
        }
    }

    impl<T> Ord for Point3<T>
    where
        T: Ord + PartialOrd,
    {
        fn cmp(&self, other: &Self) -> Ordering {
            if self.z != other.z {
                self.z.cmp(&other.z)
            } else if self.y != other.y {
                self.y.cmp(&other.y)
            } else {
                self.x.cmp(&other.x)
            }
        }
    }

    impl<T> PartialOrd for Point3<T>
    where
        T: Ord + PartialOrd,
    {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            Some(self.cmp(other))
        }
    }

    impl<T> Display for Point3<T>
    where
        T: Display,
    {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "{},{},{}", self.x, self.y, self.z)
        }
    }

    macro_rules! impl_op {
        ($struct:ident, $trait:ident, $fn:ident, $assign_trait:ident, $assign_fn:ident) => {
            impl<T> $trait for $struct<T>
            where
                T: $trait<Output = T>,
            {
                type Output = Self;
                fn $fn(self, other: Self) -> Self {
                    $struct {
                        x: T::$fn(self.x, other.x),
                        y: T::$fn(self.y, other.y),
                        z: T::$fn(self.z, other.z),
                    }
                }
            }

            impl<T> $assign_trait for $struct<T>
            where
                T: $assign_trait,
            {
                fn $assign_fn(&mut self, other: Self) {
                    T::$assign_fn(&mut self.x, other.x);
                    T::$assign_fn(&mut self.y, other.y);
                    T::$assign_fn(&mut self.z, other.z);
                }
            }
        };
    }

    macro_rules! impl_scalar {
        ($struct: ident, $trait:ident, $fn:ident, $assign_trait:ident, $assign_fn:ident) => {
            impl<T> $trait<T> for $struct<T>
            where
                T: $trait<Output = T> + Clone,
            {
                type Output = Self;
                fn $fn(self, other: T) -> Self {
                    $struct {
                        x: T::$fn(self.x, other.clone()),
                        y: T::$fn(self.y, other.clone()),
                        z: T::$fn(self.z, other),
                    }
                }
            }

            impl<T> $assign_trait<T> for $struct<T>
            where
                T: $assign_trait + Clone,
            {
                fn $assign_fn(&mut self, other: T) {
                    T::$assign_fn(&mut self.x, other.clone());
                    T::$assign_fn(&mut self.y, other.clone());
                    T::$assign_fn(&mut self.z, other);
                }
            }
        };
    }

    impl_op!(Point3, Add, add, AddAssign, add_assign);
    impl_op!(Point3, Sub, sub, SubAssign, sub_assign);
    impl_op!(Point3, Mul, mul, MulAssign, mul_assign);
    impl_op!(Point3, Div, div, DivAssign, div_assign);
    impl_op!(Point3, Rem, rem, RemAssign, rem_assign);

    impl_scalar!(Point3, Mul, mul, MulAssign, mul_assign);
    impl_scalar!(Point3, Div, div, DivAssign, div_assign);
    impl_scalar!(Point3, Rem, rem, RemAssign, rem_assign);

    impl<T> Neg for Point3<T>
    where
        T: Neg<Output = T>,
    {
        type Output = Self;
        fn neg(self) -> Self {
            Point3 {
                x: T::neg(self.x),
                y: T::neg(self.y),
                z: T::neg(self.z),
            }
        }
    }

    impl<T> Zero for Point3<T>
    where
        T: Zero,
    {
        fn zero() -> Self {
            Point3 {
                x: T::zero(),
                y: T::zero(),
                z: T::zero(),
            }
        }

        fn is_zero(&self) -> bool {
            T::is_zero(&self.x) && T::is_zero(&self.y) && T::is_zero(&self.z)
        }
    }

    impl<T> One for Point3<T>
    where
        T: One,
    {
        fn one() -> Self {
            Point3 {
                x: T::one(),
                y: T::one(),
                z: T::one(),
            }
        }
    }

    impl<T> Num for Point3<T>
    where
        T: Num,
    {
        type FromStrRadixErr = T::FromStrRadixErr;
        fn from_str_radix(s: &str, radix: u32) -> Result<Self, Self::FromStrRadixErr> {
            let mut iter = s.split(',');
            if let Some(value) = iter.next() {
                let x = T::from_str_radix(value.trim(), radix)?;
                if let Some(value) = iter.next() {
                    let y = T::from_str_radix(value.trim(), radix)?;
                    if let Some(value) = iter.next() {
                        let z = T::from_str_radix(value.trim(), radix)?;
                        if iter.next().is_none() {
                            return Ok(Point3 { x, y, z });
                        }
                    }
                }
            }

            if let Err(e) = T::from_str_radix("", radix) {
                return Err(e);
            }
            unreachable!()
        }
    }

    impl<T> Signed for Point3<T>
    where
        T: Signed + Clone,
    {
        fn abs(&self) -> Self {
            Point3 {
                x: T::abs(&self.x),
                y: T::abs(&self.y),
                z: T::abs(&self.z),
            }
        }

        fn abs_sub(&self, other: &Self) -> Self {
            let delta = self.clone() - other.clone();
            <Self as Signed>::abs(&delta)
        }

        fn signum(&self) -> Self {
            Point3 {
                x: T::signum(&self.x),
                y: T::signum(&self.y),
                z: T::signum(&self.z),
            }
        }

        fn is_positive(&self) -> bool {
            T::is_positive(&self.x) && T::is_positive(&self.y) && T::is_positive(&self.z)
        }

        fn is_negative(&self) -> bool {
            T::is_negative(&self.x) && T::is_negative(&self.y) && T::is_negative(&self.z)
        }
    }

    impl<T> Unsigned for Point3<T> where T: Unsigned {}

    impl<T> FromStr for Point3<T>
    where
        T: Num,
    {
        type Err = <Self as Num>::FromStrRadixErr;
        fn from_str(s: &str) -> Result<Self, Self::Err> {
            <Self as Num>::from_str_radix(s, 10)
        }
    }

    /// North, South, West, East enumeration
    #[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
    pub enum Direction {
        N,
        S,
        W,
        E,
    }

    impl Direction {
        /// Going north decrements the y value, south increments y value, west decrements x and east increments x of a Point2i
        /// # Examples
        /// ```
        /// use aoc19::util::{Direction, Point2i};
        ///
        /// let d = Direction::W;
        /// let p = Point2i::new(-2, 3) + d.shift();
        /// assert_eq!(p.x, -3);
        /// assert_eq!(p.y, 3);
        /// ```
        pub fn shift(self) -> Point2i {
            match self {
                Direction::N => Point2i::new(0, -1),
                Direction::W => Point2i::new(-1, 0),
                Direction::S => Point2i::new(0, 1),
                Direction::E => Point2i::new(1, 0),
            }
        }

        /// Iterate over all directions
        /// # Examples
        /// ```
        /// use aoc19::util::Direction;
        ///
        /// for dir in Direction::iter() {
        ///     // ...
        /// }
        /// ```
        pub fn iter() -> std::slice::Iter<'static, Self> {
            static VALS: [Direction; 4] = [Direction::N, Direction::W, Direction::S, Direction::E];
            VALS.iter()
        }

        /// Get the direction when going to the left i.e. counter-clockwise
        /// # Examples
        /// use aoc19::util::Direction;
        ///
        /// assert_eq!(Direction::N.to_left(), Direction::W);
        /// assert_eq!(Direction::E.to_left(), Direction::N);
        pub fn to_left(self) -> Self {
            match self {
                Direction::N => Direction::W,
                Direction::W => Direction::S,
                Direction::S => Direction::E,
                Direction::E => Direction::N,
            }
        }

        /// Get the direction when going to the right i.e. clockwise
        /// # Examples
        /// use aoc19::util::Direction;
        ///
        /// assert_eq!(Direction::N.to_right(), Direction::E);
        /// assert_eq!(Direction::W.to_right(), Direction::N);
        pub fn to_right(self) -> Self {
            match self {
                Direction::N => Direction::E,
                Direction::W => Direction::N,
                Direction::S => Direction::W,
                Direction::E => Direction::S,
            }
        }
    }

    impl TryFrom<char> for Direction {
        type Error = Error;
        fn try_from(c: char) -> Result<Self, Self::Error> {
            let direction = match c {
                'N' | 'n' | 'U' | 'u' => Direction::N,
                'W' | 'w' | 'L' | 'l' => Direction::W,
                'S' | 's' | 'D' | 'd' => Direction::S,
                'E' | 'e' | 'R' | 'r' => Direction::E,
                _ => bail!("Cannot parse direction from char {}", c),
            };
            Ok(direction)
        }
    }
}

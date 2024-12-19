use std::{collections::HashMap, convert::identity, mem};

use aoc_rust::Solution;
use eyre::Result;
use nom::{
    bytes::complete as by,
    character::complete as ch,
    combinator::{flat_map, map, opt},
    multi::fold_many1,
    sequence::terminated,
};

pub fn run(input: &str) -> Result<Solution> {
    let input = input.trim();

    let p1 = part1(input);
    let p2 = part2(input);

    Ok(Solution::new().part1(p1).part2(p2))
}

type Cache<'a, P> = HashMap<&'a str, <P as Part>::Cached, ahash::RandomState>;

fn part1(input: &str) -> usize {
    solve::<Part1>(input)
}

fn part2(input: &str) -> i64 {
    solve::<Part2>(input)
}

fn solve<P: Part>(input: &str) -> P::Output {
    let pattern = terminated(ch::alpha1, opt(by::tag(", ")));
    let fold_trie = fold_many1(pattern, Trie::default, |mut trie, next| {
        trie.insert(next);

        trie
    });
    let trie = terminated(fold_trie, ch::multispace1);

    let applied = |trie| {
        let mut cache = Cache::<P>::default();
        let line = terminated(ch::alpha1, opt(ch::newline::<_, ()>));
        let apply = move |line| P::recurse(&trie, line, &mut cache);

        fold_many1(map(line, apply), <P::Output>::default, P::fold_output)
    };

    let (_, res) = flat_map(trie, applied)(input).expect("bad input");

    res
}

trait Part {
    type Cached: Copy + Default;
    type Output: Default;

    const EMPTY: Self::Cached;

    fn fold_cached(iter: impl Iterator<Item = Self::Cached>) -> Self::Cached;

    fn fold_output(acc: Self::Output, next: Self::Cached) -> Self::Output;

    fn recurse<'a>(trie: &Trie<'a>, rest: &'a str, cache: &mut Cache<'a, Self>) -> Self::Cached {
        if let Some(cached) = cache.get(rest) {
            *cached
        } else if rest.is_empty() {
            Self::EMPTY
        } else {
            let iter = trie
                .common_prefix_search(rest)
                .map(|len| Self::recurse(trie, &rest[len..], cache));

            let res = Self::fold_cached(iter);
            cache.insert(rest, res);

            res
        }
    }
}

struct Part1;

impl Part for Part1 {
    type Cached = bool;
    type Output = usize;

    const EMPTY: Self::Cached = true;

    fn fold_cached(mut iter: impl Iterator<Item = Self::Cached>) -> Self::Cached {
        iter.any(identity)
    }

    fn fold_output(acc: Self::Output, next: Self::Cached) -> Self::Output {
        acc + usize::from(next)
    }
}

struct Part2;

impl Part for Part2 {
    type Cached = i64;
    type Output = Self::Cached;

    const EMPTY: Self::Cached = 1;

    fn fold_cached(iter: impl Iterator<Item = Self::Cached>) -> Self::Cached {
        iter.sum()
    }

    fn fold_output(acc: Self::Output, next: Self::Cached) -> Self::Output {
        acc + next
    }
}

#[derive(Default)]
pub struct Trie<'a> {
    root: Node<'a>,
}

impl<'a> Trie<'a> {
    pub fn insert(&mut self, value: &'a str) {
        let mut curr = &mut self.root;
        let mut rest = value;

        'outer: loop {
            for i in (0..curr.children.len()).rev() {
                if rest == curr.children[i].value {
                    curr.children[i].real = true;

                    return;
                } else if let Some(new_value) = curr.children[i].value.strip_prefix(rest) {
                    let new_node = Node::new(rest, true);
                    let mut old_node = mem::replace(&mut curr.children[i], new_node);
                    old_node.value = new_value;
                    curr.children[i].children.push(old_node);

                    return;
                } else if let Some(new_rest) = rest.strip_prefix(curr.children[i].value) {
                    rest = new_rest;
                    curr = &mut curr.children[i];

                    continue 'outer;
                }

                let same_prefix_len = curr.children[i]
                    .value
                    .bytes()
                    .zip(rest.bytes())
                    .take_while(|(a, b)| a == b)
                    .count();

                if same_prefix_len > 0 {
                    let (prefix, new_rest) = rest.split_at(same_prefix_len);
                    let shared = Node::new(prefix, false);
                    let new_node = Node::new(new_rest, true);
                    let mut old_node = mem::replace(&mut curr.children[i], shared);
                    old_node.value = &old_node.value[same_prefix_len..];
                    curr.children[i].children.push(old_node);
                    curr.children[i].children.push(new_node);

                    return;
                }
            }

            curr.children.push(Node::new(rest, true));

            return;
        }
    }

    pub fn common_prefix_search(&'a self, query: &'a str) -> CommonPrefixIter<'a> {
        CommonPrefixIter::new(&self.root, query)
    }
}

#[derive(Default)]
struct Node<'a> {
    value: &'a str,
    real: bool,
    children: Vec<Node<'a>>,
}

impl<'a> Node<'a> {
    fn new(value: &'a str, real: bool) -> Self {
        Node {
            value,
            real,
            children: Vec::new(),
        }
    }
}

pub struct CommonPrefixIter<'a> {
    curr: &'a Node<'a>,
    rest: &'a str,
    len: usize,
}

impl<'a> CommonPrefixIter<'a> {
    fn new(curr: &'a Node<'a>, rest: &'a str) -> Self {
        Self { curr, rest, len: 0 }
    }
}

impl Iterator for CommonPrefixIter<'_> {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        'outer: loop {
            for child in self.curr.children.iter() {
                if let Some(new_rest) = self.rest.strip_prefix(child.value) {
                    self.len += child.value.len();
                    self.curr = child;
                    self.rest = new_rest;

                    if child.real {
                        return Some(self.len);
                    } else {
                        continue 'outer;
                    }
                }
            }

            return None;
        }
    }
}

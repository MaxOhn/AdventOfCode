use std::{
    collections::{btree_map::Entry as BTreeEntry, hash_map::Entry as HashEntry, BTreeMap},
    mem::ManuallyDrop,
    ops::{Deref, DerefMut},
    ptr::NonNull,
    sync::Mutex,
};

use aoc_rust::Solution;
use eyre::Result;
use fxhash::{FxHashMap, FxHashSet};

pub fn run(input: &str) -> Result<Solution> {
    let input = input.trim();

    let p1 = part1(input);
    let p2 = part2_bk_pivot_arena(input);

    // assert_eq!(p2, part2_naive(input));
    // assert_eq!(p2, part2_bk(input));
    // assert_eq!(p2, part2_bk_pivot(input));
    // assert_eq!(p2, part2_bk_pivot_arena(input));
    assert_eq!(p2, part2_bk_degeneracy(input));
    assert_eq!(p2, part2_bk_degeneracy_arena(input));

    Ok(Solution::new().part1(p1).part2(p2))
}

type Computer = [u8; 2];

type List = Vec<Computer>;
type Set<T = Computer> = FxHashSet<T>;
type Map<K = Computer, V = Set> = FxHashMap<K, V>;

struct Connection {
    a: Computer,
    b: Computer,
}

impl Connection {
    fn parse(line: &str) -> Self {
        let [a, b, b'-', c, d] = line.as_bytes() else {
            panic!("bad input")
        };

        Self {
            a: [*a, *b],
            b: [*c, *d],
        }
    }
}

fn parse_input(input: &str) -> Map {
    let mut map = Map::<_, Set>::default();

    for line in input.lines() {
        let Connection { a, b } = Connection::parse(line);
        map.entry(a).or_default().insert(b);
        map.entry(b).or_default().insert(a);
    }

    map
}

pub fn part1(input: &str) -> usize {
    let map = parse_input(input);

    let mut seen = Set::default();
    let mut count = 0;

    for (&a, values_a) in map.iter() {
        if a[0] != b't' {
            continue;
        }

        for &b in values_a {
            let values_b = &map[&b];

            for &c in values_b {
                if values_a.contains(&c) {
                    let mut list = [a, b, c];
                    list.sort_unstable();

                    if seen.insert(list) {
                        count += 1;
                    }
                }
            }
        }
    }

    count
}

// awful solution :D
pub fn part2_naive(input: &str) -> String {
    let map = parse_input(input);

    fn recurse(n: usize, curr: List, map: &Map, cache: &mut Set<List>) -> Option<List> {
        if curr.len() == n {
            return Some(curr);
        }

        for value in &map[&curr[0]] {
            if !curr.contains(value) && curr.iter().all(|curr| map[curr].contains(value)) {
                let mut next = curr.clone();
                next.push(*value);
                next.sort_unstable();

                if cache.insert(next.clone()) {
                    if let res @ Some(_) = recurse(n, next, map, cache) {
                        return res;
                    }
                }
            }
        }

        None
    }

    let mut seen = FxHashSet::default();

    let mut longest = None;

    'outer: for size in 4.. {
        seen.clear();

        for (&a, values_a) in map.iter() {
            for &b in values_a {
                let mut list = vec![a, b];
                list.sort_unstable();

                if let list @ Some(_) = recurse(size, list, &map, &mut seen) {
                    longest = list;

                    continue 'outer;
                }
            }
        }

        break;
    }

    password(&longest.unwrap())
}

fn password(list: &List) -> String {
    let mut iter = list.iter();

    let Some(name) = iter.next() else {
        return String::new();
    };

    let mut bytes = Vec::with_capacity(list.len() * 3 - 1);
    bytes.extend_from_slice(name);

    for name in list {
        bytes.push(b',');
        bytes.extend_from_slice(name);
    }

    String::from_utf8(bytes).unwrap()
}

fn map_to_password(c: &BTreeMap<usize, Set>) -> String {
    let (_, clique) = c.last_key_value().unwrap();
    let mut list: List = clique.iter().copied().collect();
    list.sort_unstable();

    password(&list)
}

// <https://en.wikipedia.org/wiki/Bron%E2%80%93Kerbosch_algorithm>
pub fn part2_bk(input: &str) -> String {
    let map = parse_input(input);

    let r = Set::default();
    let p = map.keys().copied().collect();
    let x = Set::default();
    let mut c = BTreeMap::default();
    bk(r, p, x, &mut c, &map);

    map_to_password(&c)
}

fn bk(r: Set, mut p: Set, mut x: Set, c: &mut BTreeMap<usize, Set>, n: &Map) {
    if p.is_empty() && x.is_empty() {
        if let BTreeEntry::Vacant(e) = c.entry(r.len()) {
            e.insert(r);
        }

        return;
    }

    for v in p.clone() {
        let mut next_r = r.clone();
        next_r.insert(v);

        let neighbors = &n[&v];
        let next_p = p.intersection(neighbors).copied().collect();
        let next_x = x.intersection(neighbors).copied().collect();

        bk(next_r, next_p, next_x, c, n);

        p.remove(&v);
        x.insert(v);
    }
}

// definitely faster than part2_bk
pub fn part2_bk_pivot(input: &str) -> String {
    let map = parse_input(input);

    let r = Set::default();
    let p = map.keys().copied().collect();
    let x = Set::default();
    let mut c = BTreeMap::default();
    bk_pivot(r, p, x, &mut c, &map);

    map_to_password(&c)
}

fn bk_pivot(r: Set, mut p: Set, mut x: Set, c: &mut BTreeMap<usize, Set>, n: &Map) {
    let Some(pivot) = p.union(&x).next() else {
        if let BTreeEntry::Vacant(e) = c.entry(r.len()) {
            e.insert(r);
        }

        return;
    };

    for &v in p.clone().difference(&n[pivot]) {
        let mut next_r = r.clone();
        next_r.insert(v);

        let neighbors = &n[&v];
        let next_p = p.intersection(neighbors).copied().collect();
        let next_x = x.intersection(neighbors).copied().collect();

        bk_pivot(next_r, next_p, next_x, c, n);

        p.remove(&v);
        x.insert(v);
    }
}

// the arena was worth it yay
pub fn part2_bk_pivot_arena(input: &str) -> String {
    let map = parse_input(input);

    let arena = Arena::default();

    let r = arena.get();
    let mut p = arena.get();
    p.extend(map.keys().copied());
    let x = arena.get();
    let mut c = BTreeMap::default();
    bk_pivot_arena(r, p, x, &mut c, &map, &arena);

    map_to_password(&c)
}

/// Accumulator of allocated sets to allow later re-use.
#[derive(Default)]
struct Arena {
    allocated: Mutex<Vec<NonNull<Set>>>,
}

impl Arena {
    /// Acquires a set and returns a handle to it.
    ///
    /// When the handle is dropped, the set will be reclaimed automatically.
    fn get(&self) -> ArenaSet<'_> {
        if let Some(ptr) = self.allocated.lock().unwrap().pop() {
            let mut set = ArenaSet { arena: self, ptr };
            set.clear();

            set
        } else {
            let raw = Box::into_raw(Box::new(Set::default()));
            let ptr = unsafe { NonNull::new_unchecked(raw) };

            ArenaSet { arena: self, ptr }
        }
    }
}

impl Drop for Arena {
    fn drop(&mut self) {
        for ptr in self.allocated.lock().unwrap().iter() {
            unsafe {
                std::ptr::drop_in_place(ptr.as_ptr());
            }
        }
    }
}

/// Handle to sets allocated in [`Arena`].
///
/// When this handle is dropped, the [`Arena`] will automatically reclaim the
/// set.
struct ArenaSet<'a> {
    arena: &'a Arena,
    ptr: NonNull<Set>,
}

impl ArenaSet<'_> {
    /// Claim ownership of the set.
    ///
    /// This prevents the set to be reclaimed by the [`Arena`].
    fn into_owned(self) -> Set {
        unsafe { std::ptr::read(ManuallyDrop::new(self).ptr.as_ptr()) }
    }
}

impl Drop for ArenaSet<'_> {
    fn drop(&mut self) {
        self.arena.allocated.lock().unwrap().push(self.ptr);
    }
}

impl Deref for ArenaSet<'_> {
    type Target = Set;

    fn deref(&self) -> &Self::Target {
        unsafe { self.ptr.as_ref() }
    }
}

impl DerefMut for ArenaSet<'_> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { self.ptr.as_mut() }
    }
}

impl Clone for ArenaSet<'_> {
    fn clone(&self) -> Self {
        let mut cloned = self.arena.get();
        let set = &mut *cloned;
        set.clone_from(self);

        cloned
    }
}

fn bk_pivot_arena(
    r: ArenaSet<'_>,
    mut p: ArenaSet<'_>,
    mut x: ArenaSet<'_>,
    c: &mut BTreeMap<usize, Set>,
    n: &Map,
    arena: &Arena,
) {
    let Some(pivot) = p.union(&x).next() else {
        if let BTreeEntry::Vacant(e) = c.entry(r.len()) {
            e.insert(r.into_owned());
        }

        return;
    };

    for &v in p.clone().difference(&n[pivot]) {
        let mut next_r = r.clone();
        next_r.insert(v);

        let neighbors = &n[&v];

        let mut next_p = arena.get();
        next_p.extend(p.intersection(neighbors).copied());

        let mut next_x = arena.get();
        next_x.extend(x.intersection(neighbors).copied());

        bk_pivot_arena(next_r, next_p, next_x, c, n, arena);

        p.remove(&v);
        x.insert(v);
    }
}

// the more favorable vertex order doesn't seem quite worth its calculation time
pub fn part2_bk_degeneracy(input: &str) -> String {
    let map = parse_input(input);

    let mut c = BTreeMap::default();
    bk_degeneracy(&mut c, &map);

    map_to_password(&c)
}

fn bk_degeneracy(c: &mut BTreeMap<usize, Set>, n: &Map) {
    let mut p: Set = n.keys().copied().collect();
    let mut x = Set::default();

    for v in degeneracy_graph(n) {
        let mut r = Set::default();
        r.insert(v);

        let neighbors = &n[&v];
        let next_p = p.intersection(neighbors).copied().collect();
        let next_x = x.intersection(neighbors).copied().collect();

        bk_pivot(r, next_p, next_x, c, n);

        p.remove(&v);
        x.insert(v);
    }
}

// <https://en.wikipedia.org/wiki/Degeneracy_(graph_theory)>
fn degeneracy_graph(n: &Map) -> List {
    let mut l = List::new();
    let mut l_set = Set::default();

    let cap = 1 + n.values().map(Set::len).max().unwrap_or(0);
    let mut d = vec![List::new(); cap];
    let mut d_indices = Map::default();

    for (v, ns) in n {
        let dv = ns.len();
        d[dv].push(*v);
        d_indices.insert(*v, dv);
    }

    while let Some(item) = d.iter_mut().find_map(List::pop) {
        l.push(item);
        l_set.insert(item);

        for w in n[&item].iter() {
            if l_set.contains(w) {
                continue;
            }

            let HashEntry::Occupied(mut entry) = d_indices.entry(*w) else {
                unreachable!()
            };
            let dw = entry.get_mut();
            d[*dw].retain(|item| item != w);
            *dw -= 1;
            d[*dw].push(*w);
        }
    }

    l.reverse();

    l
}

pub fn part2_bk_degeneracy_arena(input: &str) -> String {
    let map = parse_input(input);

    let arena = Arena::default();
    let mut c = BTreeMap::default();
    bk_degeneracy_arena(&mut c, &map, &arena);

    map_to_password(&c)
}

fn bk_degeneracy_arena(c: &mut BTreeMap<usize, Set>, n: &Map, arena: &Arena) {
    let mut p = arena.get();
    p.extend(n.keys().copied());
    let mut x = arena.get();

    for v in degeneracy_graph_arena(n, arena) {
        let mut r = arena.get();
        r.insert(v);

        let neighbors = &n[&v];

        let mut next_p = arena.get();
        next_p.extend(p.intersection(neighbors).copied());

        let mut next_x = arena.get();
        next_x.extend(x.intersection(neighbors).copied());

        bk_pivot_arena(r, next_p, next_x, c, n, arena);

        p.remove(&v);
        x.insert(v);
    }
}

fn degeneracy_graph_arena(n: &Map, arena: &Arena) -> List {
    let mut l = List::new();
    let mut l_set = arena.get();

    let cap = 1 + n.values().map(Set::len).max().unwrap_or(0);
    let mut d = vec![List::new(); cap];
    let mut d_indices = Map::default();

    for (v, ns) in n {
        let dv = ns.len();
        d[dv].push(*v);
        d_indices.insert(*v, dv);
    }

    while let Some(item) = d.iter_mut().find_map(List::pop) {
        l.push(item);
        l_set.insert(item);

        for w in n[&item].iter() {
            if l_set.contains(w) {
                continue;
            }

            let HashEntry::Occupied(mut entry) = d_indices.entry(*w) else {
                unreachable!()
            };
            let dw = entry.get_mut();
            d[*dw].retain(|item| item != w);
            *dw -= 1;
            d[*dw].push(*w);
        }
    }

    l.reverse();

    l
}

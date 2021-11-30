use hashbrown::HashMap;
use std::fmt;
use std::hint::unreachable_unchecked;
use std::io::{BufRead, BufReader};
use std::ops::{Deref, DerefMut};
use std::time::Instant;

macro_rules! get {
    ($bytes:expr, $i:expr) => {
        unsafe { *$bytes.get_unchecked($i) }
    };
}

trait Grid {
    fn grid(&mut self) -> (&mut [u8], usize);

    fn _flip_hizontal(&mut self) {
        let (grid, w) = self.grid();

        for x in 0..w {
            for y in 0..w / 2 {
                grid.swap(x + w * y, x + w * (w - y - 1));
            }
        }
    }

    fn _flip_vertical(&mut self) {
        let (grid, w) = self.grid();

        for x in 0..w / 2 {
            for y in 0..w {
                grid.swap(x + w * y, w - x - 1 + w * y);
            }
        }
    }

    fn _transpose(&mut self) {
        let (grid, w) = self.grid();

        for x in 0..w {
            for y in x..w {
                grid.swap(x + w * y, y + w * x);
            }
        }
    }

    fn _rotate_cw(&mut self) {
        self._transpose();
        self._flip_vertical();
    }
}

#[derive(Clone)]
struct Tile {
    id: u16,
    w: usize,
    tile: Vec<u8>,
    borders: [u16; 4],
}

impl Grid for Tile {
    fn grid(&mut self) -> (&mut [u8], usize) {
        (&mut self.tile, self.w)
    }
}

impl Tile {
    fn new(id: u16, w: usize) -> Self {
        Self {
            id,
            w,
            tile: Vec::with_capacity(w * w),
            borders: [0; 4],
        }
    }

    fn borders(&mut self) {
        for &c in &self.tile[..self.w] {
            self.borders[0] <<= 1;
            self.borders[0] += (c == b'#') as u16;
        }

        for y in 1..=self.w {
            self.borders[1] <<= 1;
            self.borders[1] += (self[y * self.w - 1] == b'#') as u16;
        }

        for &c in &self.tile[self.w * self.w - self.w..] {
            self.borders[2] <<= 1;
            self.borders[2] += (c == b'#') as u16;
        }

        for y in 0..self.w {
            self.borders[3] <<= 1;
            self.borders[3] += (self[y * self.w] == b'#') as u16;
        }
    }

    fn top(&self) -> u16 {
        get!(self.borders, 0)
    }

    fn right(&self) -> u16 {
        get!(self.borders, 1)
    }

    fn bot(&self) -> u16 {
        get!(self.borders, 2)
    }

    fn left(&self) -> u16 {
        get!(self.borders, 3)
    }

    fn flip_horizontal(&mut self) {
        self._flip_hizontal();

        unsafe {
            *self.borders.get_unchecked_mut(1) =
                self.borders.get_unchecked(1).reverse_bits() >> (16 - self.w);
            *self.borders.get_unchecked_mut(3) =
                self.borders.get_unchecked(3).reverse_bits() >> (16 - self.w);
        }
        self.borders.swap(0, 2);
    }

    fn flip_vertical(&mut self) {
        self._flip_vertical();

        unsafe {
            *self.borders.get_unchecked_mut(0) =
                self.borders.get_unchecked(0).reverse_bits() >> (16 - self.w);
            *self.borders.get_unchecked_mut(2) =
                self.borders.get_unchecked(2).reverse_bits() >> (16 - self.w);
        }
        self.borders.swap(1, 3);
    }

    fn transpose(&mut self) {
        self._transpose();

        self.borders.swap(0, 3);
        self.borders.swap(1, 2);
    }

    fn rotate_cw(&mut self) {
        self.transpose();
        self.flip_vertical();
    }
}

impl fmt::Display for Tile {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "Tile {}:", self.id)?;
        let mut rows = self.chunks(self.w);

        if let Some(first) = rows.next() {
            for &c in first {
                write!(f, "{}", c as char)?;
            }

            for row in rows {
                f.write_str("\n")?;

                for &c in row {
                    write!(f, "{}", c as char)?;
                }
            }
        }

        Ok(())
    }
}

fn main() {
    let p1 = part1();
    let p2 = part2();

    assert_eq!(p1, 29_125_888_761_511);
    assert_eq!(p2, 2219);
}

fn part1() -> u64 {
    let start = Instant::now();
    let file =
        std::fs::File::open("./input").unwrap_or_else(|_| unsafe { unreachable_unchecked() });
    let mut input = BufReader::new(file);

    let mut line = String::new();

    let w;
    let mut tiles = Vec::with_capacity(16);
    let mut edges = HashMap::with_capacity(1024);

    let _ = input.read_line(&mut line);
    let bytes = line.as_bytes();
    let mut id = 0;
    let mut i = 5;

    loop {
        match get!(bytes, i) {
            b':' => break,
            digit => id = id * 10 + (digit & 0x0F) as u16,
        }

        i += 1;
    }

    line.clear();
    let _ = input.read_line(&mut line);

    w = line.len() - 1;

    let mut tile = Tile::new(id, w);
    tile.extend(&line.as_bytes()[..w]);

    loop {
        line.clear();
        let _ = input.read_line(&mut line);

        if line.len() == 1 {
            break;
        }

        tile.extend(&line.as_bytes()[..w]);
    }

    tile.borders();
    count_edges(&tile, &mut edges);
    tiles.push(tile);

    line.clear();

    loop {
        let _ = input.read_line(&mut line);

        if line.is_empty() {
            break;
        }

        let bytes = line.as_bytes();
        let mut id = 0;
        let mut i = 5;

        loop {
            match get!(bytes, i) {
                b':' => break,
                digit => id = id * 10 + (digit & 0x0F) as u16,
            }

            i += 1;
        }

        let mut tile = Tile::new(id, w);

        while {
            line.clear();

            let read = input
                .read_line(&mut line)
                .unwrap_or_else(|_| unsafe { unreachable_unchecked() });

            read > 0 && line.len() > 1
        } {
            tile.extend(&line.as_bytes()[..w]);
        }

        tile.borders();
        count_edges(&tile, &mut edges);
        tiles.push(tile);

        line.clear();
    }

    let mut p1 = 1;

    for mut tile in tiles {
        if valid_corner_tile(&tile, &edges) {
            p1 *= tile.id as u64;
            continue;
        }

        tile.flip_horizontal();
        tile.flip_vertical();

        if valid_corner_tile(&tile, &edges) {
            p1 *= tile.id as u64;
        }
    }

    println!("Part 1: {} [{:?}]", p1, start.elapsed()); // 260µs

    p1
}

fn valid_corner_tile(tile: &Tile, edges: &HashMap<u16, u8>) -> bool {
    (edges[&tile.top()] == 1) as u8
        + (edges[&tile.bot()] == 1) as u8
        + (edges[&tile.left()] == 1) as u8
        + (edges[&tile.right()] == 1) as u8
        == 2
}

fn count_edges(tile: &Tile, edges: &mut HashMap<u16, u8>) {
    let shift = 16 - tile.w;

    for i in 0..4 {
        let edge = get!(tile.borders, i);
        *edges.entry(edge).or_insert(0) += 1;
        *edges.entry(edge.reverse_bits() >> shift).or_insert(0) += 1;
    }
}

fn part2() -> usize {
    let start = Instant::now();
    let file =
        std::fs::File::open("./input").unwrap_or_else(|_| unsafe { unreachable_unchecked() });
    let mut input = BufReader::new(file);

    let mut line = String::new();

    let w;
    let mut tiles = Vec::with_capacity(128);
    let mut edges = HashMap::with_capacity(1024);

    let _ = input.read_line(&mut line);
    let bytes = line.as_bytes();
    let mut id = 0;
    let mut i = 5;

    loop {
        match get!(bytes, i) {
            b':' => break,
            digit => id = id * 10 + (digit & 0x0F) as u16,
        }

        i += 1;
    }

    line.clear();
    let _ = input.read_line(&mut line);

    w = line.len() - 1;

    let mut tile = Tile::new(id, w);
    tile.extend(&line.as_bytes()[..w]);

    loop {
        line.clear();
        let _ = input.read_line(&mut line);

        if line.len() == 1 {
            break;
        }

        tile.extend(&line.as_bytes()[..w]);
    }

    tile.borders();

    count_edges(&tile, &mut edges);
    tiles.push(tile);

    line.clear();

    loop {
        let _ = input.read_line(&mut line);

        if line.is_empty() {
            break;
        }

        let bytes = line.as_bytes();
        let mut id = 0;
        let mut i = 5;

        loop {
            match get!(bytes, i) {
                b':' => break,
                digit => id = id * 10 + (digit & 0x0F) as u16,
            }

            i += 1;
        }

        let mut tile = Tile::new(id, w);

        while {
            line.clear();

            let read = input
                .read_line(&mut line)
                .unwrap_or_else(|_| unsafe { unreachable_unchecked() });

            read > 0 && line.len() > 1
        } {
            tile.extend(&line.as_bytes()[..w]);
        }

        tile.borders();
        count_edges(&tile, &mut edges);
        tiles.push(tile);

        line.clear();
    }

    let w_outer = (tiles.len() as f32).sqrt() as usize;
    let mut i = tiles.len() - 1;

    let mut corner = loop {
        let tile = &mut tiles[i];
        if valid_corner_tile(&tile, &edges) {
            break tiles.swap_remove(i);
        } else {
            tile.flip_horizontal();
            tile.flip_vertical();

            if valid_corner_tile(&tile, &edges) {
                break tiles.swap_remove(i);
            }
        }

        i -= 1;
    };

    while edges[&corner.left()] != 1 || edges[&corner.top()] != 1 {
        corner.rotate_cw();
    }

    let mut outer = vec![Vec::with_capacity(w_outer); w_outer];
    outer[0].push(corner);

    recurse((0, 0), &mut tiles, &mut outer);
    // assert!(check_valid(&outer, w_outer, w), "picture not valid");

    let w_outer = w_outer * (w - 2);
    let mut picture = Picture::new(w_outer);

    for row in outer {
        for y in 1..w - 1 {
            for tile in row.iter() {
                picture
                    .extend_from_slice(unsafe { tile.get_unchecked(1 + y * w..(y + 1) * w - 1) });
            }
        }
    }

    let p2 = bytecount::count(&picture, b'#') - mark_monsters_iter(&mut picture);

    println!("Part 2: {} [{:?}]", p2, start.elapsed()); // 13ms

    p2
}

fn recurse((x, y): (usize, usize), tiles: &mut Vec<Tile>, picture: &mut [Vec<Tile>]) {
    let bot_neighbor = {
        let tile = unsafe { picture.get_unchecked(y).get_unchecked(x) };
        let from_top = tile.bot();

        tiles.iter_mut().position(|tile| {
            if tile.top() == from_top {
                return true;
            }

            for _ in 0..3 {
                tile.rotate_cw();

                if tile.top() == from_top {
                    return true;
                }
            }

            tile.flip_horizontal();

            if tile.top() == from_top {
                return true;
            }

            for _ in 0..4 {
                tile.rotate_cw();

                if tile.top() == from_top {
                    return true;
                }
            }

            false
        })
    };

    if let Some(bot_neighbor) = bot_neighbor {
        let bot_neighbor = tiles.swap_remove(bot_neighbor);
        unsafe { picture.get_unchecked_mut(y + 1).push(bot_neighbor) }

        recurse((x, y + 1), tiles, picture);
    }

    let right_neighbor = {
        let tile = unsafe { picture.get_unchecked(y).get_unchecked(x) };
        let from_left = tile.right();

        let from_bot = if y + 1 < picture.len() {
            unsafe { picture.get_unchecked(y + 1).get(x + 1).map(Tile::top) }
        } else {
            None
        };

        tiles
            .iter_mut()
            .enumerate()
            .filter_map(|(i, tile)| {
                if tile.left() == from_left {
                    return Some((i, tile));
                }

                for _ in 0..3 {
                    tile.rotate_cw();

                    if tile.left() == from_left {
                        return Some((i, tile));
                    }
                }

                tile.flip_horizontal();

                if tile.left() == from_left {
                    return Some((i, tile));
                }

                for _ in 0..4 {
                    tile.rotate_cw();

                    if tile.left() == from_left {
                        return Some((i, tile));
                    }
                }

                None
            })
            .find_map(|(i, tile)| match from_bot {
                Some(from_bot) if tile.bot() == from_bot => Some(i),
                Some(_) => None,
                None => Some(i),
            })
    };

    if let Some(right_neighbor) = right_neighbor {
        let right_neighbor = tiles.swap_remove(right_neighbor);
        unsafe { picture.get_unchecked_mut(y).push(right_neighbor) }

        recurse((x + 1, y), tiles, picture);
    }
}

#[rustfmt::skip]
static MONSTER: [[u8; 20]; 3] = [
    [b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b'#', b' '],
    [b'#', b' ', b' ', b' ', b' ', b'#', b'#', b' ', b' ', b' ', b' ', b'#', b'#', b' ', b' ', b' ', b' ', b'#', b'#', b'#'],
    [b' ', b'#', b' ', b' ', b'#', b' ', b' ', b'#', b' ', b' ', b'#', b' ', b' ', b'#', b' ', b' ', b'#', b' ', b' ', b' '],
];

fn mark_monsters_iter(picture: &mut Picture) -> usize {
    let mut marked = mark_monsters(picture);

    if marked > 0 {
        return marked;
    }

    for _ in 0..3 {
        picture._rotate_cw();
        marked = mark_monsters(picture);

        if marked > 0 {
            return marked;
        }
    }

    picture._flip_hizontal();

    marked = mark_monsters(picture);

    if marked > 0 {
        return marked;
    }

    for _ in 0..3 {
        picture._rotate_cw();
        marked = mark_monsters(picture);

        if marked > 0 {
            return marked;
        }
    }

    unsafe { unreachable_unchecked() }
}

fn mark_monsters(picture: &mut Picture) -> usize {
    let w = picture.w;

    let mut marked = 0;

    for x in 0..w - 19 {
        'row: for y in 0..w - 2 {
            let mut curr = 0;

            for i in 0..3 {
                for j in 0..20 {
                    let pixel = get!(picture, x + j + w * (y + i));

                    if get!(MONSTER.get_unchecked(i), j) == b'#' {
                        if pixel != b'#' {
                            continue 'row;
                        }

                        curr += 1;
                    }
                }
            }

            marked += curr;
        }
    }

    marked
}

struct Picture {
    w: usize,
    grid: Vec<u8>,
}

impl Picture {
    fn new(w: usize) -> Self {
        Self {
            w,
            grid: Vec::with_capacity(w * w),
        }
    }
}

impl fmt::Display for Picture {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut rows = self.grid.chunks_exact(self.w);

        if let Some(first) = rows.next() {
            writeln!(f, "{}", String::from_utf8_lossy(first))?;

            for row in rows {
                writeln!(f, "{}", String::from_utf8_lossy(row))?;
            }
        }

        Ok(())
    }
}

impl Grid for Picture {
    fn grid(&mut self) -> (&mut [u8], usize) {
        (&mut self.grid, self.w)
    }
}

#[allow(dead_code)]
fn check_valid(tiles: &[Vec<Tile>], w_outer: usize, w_inner: usize) -> bool {
    for row in tiles.iter() {
        assert_eq!(row.len(), w_outer, "row is missing tiles");

        for x in 0..w_outer - 1 {
            let left = &row[x];
            let right = &row[x + 1];

            for y_inner in 0..w_inner {
                if left[w_inner - 1 + w_inner * y_inner] != right[w_inner * y_inner] {
                    return false;
                }
            }
        }
    }

    let start = w_inner * w_inner - w_inner;

    for x in 0..w_outer {
        for y in 0..w_outer - 1 {
            let top = &tiles[y][x];
            let bot = &tiles[y + 1][x];

            for x_inner in 0..w_inner {
                if top[start + x_inner] != bot[x_inner] {
                    return false;
                }
            }
        }
    }

    true
}

impl Deref for Tile {
    type Target = Vec<u8>;

    fn deref(&self) -> &Self::Target {
        &self.tile
    }
}

impl DerefMut for Tile {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.tile
    }
}

impl Deref for Picture {
    type Target = Vec<u8>;

    fn deref(&self) -> &Self::Target {
        &self.grid
    }
}

impl DerefMut for Picture {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.grid
    }
}

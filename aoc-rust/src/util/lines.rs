use std::mem;

use memchr::Memchr;

pub struct Lines<'a> {
    rest: &'a str,
    iter: Memchr<'a>,
    start: usize,
}

impl<'a> Lines<'a> {
    pub fn new(s: &'a str) -> Self {
        Self {
            rest: s,
            iter: Memchr::new(b'\n', s.as_bytes()),
            start: 0,
        }
    }

    pub fn rest(&self) -> &'a str {
        self.rest
    }
}

impl<'a> Iterator for Lines<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<Self::Item> {
        match self.iter.next() {
            Some(end) => {
                let len = end - self.start;
                let line = &self.rest[..len];
                self.rest = &self.rest[len + 1..];
                self.start = end + 1;

                Some(line)
            }
            None if self.rest.is_empty() => None,
            None => Some(mem::replace(&mut self.rest, "")),
        }
    }
}

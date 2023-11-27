use std::hash::{BuildHasher, Hasher};

pub struct IntHasher;

impl BuildHasher for IntHasher {
    type Hasher = IntHash;

    fn build_hasher(&self) -> Self::Hasher {
        IntHash(0)
    }
}

pub struct IntHash(u64);

#[rustfmt::skip]
impl Hasher for IntHash {
    #[inline] fn finish(&self) -> u64 { self.0 }

    #[inline] fn write_u8(&mut self, i: u8) { self.0 = u64::from(i) }
    #[inline] fn write_u16(&mut self, i: u16) { self.0 = u64::from(i) }
    #[inline] fn write_u32(&mut self, i: u32) { self.0 = u64::from(i) }
    #[inline] fn write_u64(&mut self, i: u64) { self.0 = i }
    #[inline] fn write_usize(&mut self, i: usize) { self.0 = i as u64 }
    #[inline] fn write_i8(&mut self, i: i8) { self.0 = i as u64 }
    #[inline] fn write_i16(&mut self, i: i16) { self.0 = i as u64 }
    #[inline] fn write_i32(&mut self, i: i32) { self.0 = i as u64 }
    #[inline] fn write_i64(&mut self, i: i64) { self.0 = i as u64 }
    #[inline] fn write_isize(&mut self, i: isize) { self.0 = i as u64 }

    fn write(&mut self, _: &[u8]) { unimplemented!() }
}

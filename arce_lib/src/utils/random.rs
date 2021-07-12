use std::cell::RefCell;

thread_local! {pub static RAND: RefCell<Random> = RefCell::new(Random::new());}

pub struct Random {
    state: u32,
}

impl Random {
    pub const fn new() -> Random {
        Random {
            state: 1_804_289_383,
        }
    }

    /// generate pseudorandom u32 with XOR shift algorithm.
    pub fn rand_u32(&mut self) -> u32 {
        let mut number = self.state;
        number ^= number << 13;
        number ^= number >> 17;
        number ^= number << 5;
        self.state = number;
        number
    }

    /// generate pseudorandom u64
    pub fn rand_u64(&mut self) -> u64 {
        let n1 = (self.rand_u32() & 0xFFFF) as u64;
        let n2 = (self.rand_u32() & 0xFFFF) as u64;
        let n3 = (self.rand_u32() & 0xFFFF) as u64;
        let n4 = (self.rand_u32() & 0xFFFF) as u64;

        n1 | (n2 << 16) | (n3 << 32) | (n4 << 48)
    }

    pub fn gen_magic_number(&mut self) -> u64 {
        self.rand_u64() & self.rand_u64() & self.rand_u64()
    }
}

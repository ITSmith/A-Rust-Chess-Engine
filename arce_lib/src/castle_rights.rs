use std::fmt::Display;

use crate::square::Square;

const SQUARE_CASTLE_RIGHTS: [u8; 64] = [
    0b1101, 0b1111, 0b1111, 0b1111, 0b1100, 0b1111, 0b1111, 0b1110, 0b1111, 0b1111, 0b1111, 0b1111,
    0b1111, 0b1111, 0b1111, 0b1111, 0b1111, 0b1111, 0b1111, 0b1111, 0b1111, 0b1111, 0b1111, 0b1111,
    0b1111, 0b1111, 0b1111, 0b1111, 0b1111, 0b1111, 0b1111, 0b1111, 0b1111, 0b1111, 0b1111, 0b1111,
    0b1111, 0b1111, 0b1111, 0b1111, 0b1111, 0b1111, 0b1111, 0b1111, 0b1111, 0b1111, 0b1111, 0b1111,
    0b1111, 0b1111, 0b1111, 0b1111, 0b1111, 0b1111, 0b1111, 0b1111, 0b0111, 0b1111, 0b1111, 0b1111,
    0b0011, 0b1111, 0b1111, 0b1011,
];
#[derive(Clone, Copy, Debug)]
pub struct CastleRights(u8);

impl CastleRights {
    #[inline]
    pub const fn wk(&self) -> bool {
        self.0 & 0b0001 != 0
    }

    #[inline]
    pub const fn wq(&self) -> bool {
        self.0 & 0b0010 != 0
    }

    #[inline]
    pub const fn bk(&self) -> bool {
        self.0 & 0b0100 != 0
    }

    #[inline]
    pub const fn bq(&self) -> bool {
        self.0 & 0b1000 != 0
    }

    #[inline]
    pub fn update(&mut self, square: Square) {
        self.0 &= SQUARE_CASTLE_RIGHTS[square as usize];
    }
}

impl From<&str> for CastleRights {
    fn from(value: &str) -> Self {
        let mut rights = 0;
        if value.contains('K') {
            rights |= 0b1000;
        }
        if value.contains('Q') {
            rights |= 0b0100;
        }
        if value.contains('k') {
            rights |= 0b0010;
        }
        if value.contains('q') {
            rights |= 0b0001;
        }
        CastleRights(rights)
    }
}

impl Display for CastleRights {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let wk = if self.wk() { 'K' } else { '-' };
        let wq = if self.wq() { 'Q' } else { '-' };
        let bk = if self.bk() { 'k' } else { '-' };
        let bq = if self.bq() { 'q' } else { '-' };
        write!(f, "{}{}{}{}", wk, wq, bk, bq)
    }
}

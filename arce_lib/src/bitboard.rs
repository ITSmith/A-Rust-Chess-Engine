use std::{
    convert::{TryFrom, TryInto},
    fmt::Display,
    ops::{
        BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Mul, MulAssign, Not, Shl,
        ShlAssign, Shr, ShrAssign,
    },
};

use crate::square::Square;

#[derive(Clone, Copy, Debug)]
pub struct BitBoard(pub u64);

impl BitBoard {
    #[inline]
    pub fn set_bit(&mut self, square: Square) {
        *self |= BitBoard::from(square);
    }

    #[inline]
    pub fn get_bit(self, square: Square) -> BitBoard {
        self & square.into()
    }

    #[inline]
    pub fn pop_bit(&mut self, square: Square) {
        *self &= !BitBoard::from(square);
    }

    #[inline]
    pub fn count_bits(mut self) -> u8 {
        let mut count = 0;
        while !self.is_empty() {
            count += 1;
            self &= BitBoard(self.0 - 1);
        }
        count
    }

    #[inline]
    pub fn get_lsb_square(self) -> Option<Square> {
        if self.is_not_empty() {
            Some(
                BitBoard((self.0 & (self.0 as i64).wrapping_neg() as u64) - 1)
                    .count_bits()
                    .try_into()
                    .unwrap(),
            )
        } else {
            None
        }
    }

    /// Return the occupancy of an attack mask
    #[inline]
    pub fn set_occupancy(mut self, index: u32, bits_in_mask: u8) -> BitBoard {
        let mut occupancy = BitBoard::empty();
        let mut count = 0;
        while count < bits_in_mask {
            let square = self.get_lsb_square().unwrap();
            self.pop_bit(square);
            if (index & (1 << count)) != 0 {
                occupancy |= BitBoard::from(square);
            }
            count += 1;
        }
        occupancy
    }

    #[inline]
    pub fn is_empty(self) -> bool {
        self.0 == 0
    }

    #[inline]
    pub fn is_not_empty(self) -> bool {
        self.0 != 0
    }

    #[inline]
    pub const fn empty() -> BitBoard {
        BitBoard(0)
    }
}

impl Mul for BitBoard {
    type Output = BitBoard;

    fn mul(self, rhs: Self) -> Self::Output {
        BitBoard(self.0.mul(rhs.0))
    }
}

impl Mul<u64> for BitBoard {
    type Output = BitBoard;

    fn mul(self, rhs: u64) -> Self::Output {
        BitBoard(self.0.wrapping_mul(rhs))
    }
}

impl MulAssign for BitBoard {
    fn mul_assign(&mut self, rhs: Self) {
        self.0 = self.0.wrapping_mul(rhs.0);
    }
}

impl MulAssign<u64> for BitBoard {
    fn mul_assign(&mut self, rhs: u64) {
        self.0 = self.0.wrapping_mul(rhs);
    }
}

impl From<Square> for BitBoard {
    fn from(value: Square) -> Self {
        BitBoard(1_u64 << value as u8)
    }
}

impl From<BitBoard> for u64 {
    fn from(b: BitBoard) -> Self {
        b.0
    }
}

impl PartialEq for BitBoard {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl Shl<u8> for BitBoard {
    type Output = BitBoard;

    fn shl(self, rhs: u8) -> Self::Output {
        BitBoard(self.0 << rhs)
    }
}

impl Shr<u8> for BitBoard {
    type Output = BitBoard;

    fn shr(self, rhs: u8) -> Self::Output {
        BitBoard(self.0 >> rhs)
    }
}

impl Not for BitBoard {
    type Output = BitBoard;

    fn not(self) -> Self::Output {
        BitBoard(!self.0)
    }
}

impl BitAnd for BitBoard {
    type Output = BitBoard;

    fn bitand(self, rhs: Self) -> Self::Output {
        BitBoard(self.0 & rhs.0)
    }
}

impl BitOr for BitBoard {
    type Output = BitBoard;

    fn bitor(self, rhs: Self) -> Self::Output {
        BitBoard(self.0 | rhs.0)
    }
}

impl BitXor for BitBoard {
    type Output = BitBoard;

    fn bitxor(self, rhs: Self) -> Self::Output {
        BitBoard(self.0 ^ rhs.0)
    }
}

impl ShrAssign<u8> for BitBoard {
    fn shr_assign(&mut self, rhs: u8) {
        self.0 >>= rhs;
    }
}

impl ShlAssign<u8> for BitBoard {
    fn shl_assign(&mut self, rhs: u8) {
        self.0 <<= rhs;
    }
}

impl BitAndAssign for BitBoard {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0 &= rhs.0
    }
}

impl BitAndAssign<&mut BitBoard> for BitBoard {
    fn bitand_assign(&mut self, rhs: &mut BitBoard) {
        self.0 &= rhs.0
    }
}

impl BitOrAssign for BitBoard {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0
    }
}

impl BitOrAssign<&mut BitBoard> for BitBoard {
    fn bitor_assign(&mut self, rhs: &mut BitBoard) {
        self.0 |= rhs.0
    }
}

impl BitXorAssign for BitBoard {
    fn bitxor_assign(&mut self, rhs: Self) {
        self.0 ^= rhs.0
    }
}

impl BitXorAssign<&mut BitBoard> for BitBoard {
    fn bitxor_assign(&mut self, rhs: &mut BitBoard) {
        self.0 ^= rhs.0
    }
}

impl Display for BitBoard {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut bstr = String::with_capacity(179);
        for r in (0..8).rev() {
            bstr.push(' ');
            bstr.push_str((r + 1).to_string().as_str());
            bstr.push(' ');
            for c in 0..8 {
                let b = BitBoard::from(Square::try_from((r, c)).unwrap());
                if (*self & b).is_not_empty() {
                    bstr.push('1');
                } else {
                    bstr.push('0')
                }
                bstr.push(' ');
            }
            bstr.push('\n');
        }
        bstr.push_str("   A B C D E F G H\n");
        write!(f, "{}", bstr)
    }
}

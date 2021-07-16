use std::{fmt::Display, ops::Index, usize};

use crate::{piece::Piece, square::Square};

pub struct MoveList {
    pub moves: Vec<Move>,
}

impl MoveList {
    pub const fn new() -> MoveList {
        MoveList { moves: Vec::new() }
    }

    pub fn with_capacity(capacity: usize) -> MoveList {
        MoveList {
            moves: Vec::with_capacity(capacity),
        }
    }

    pub fn clear(&mut self) {
        self.moves.clear();
    }

    pub fn push(&mut self, mov: Move) {
        self.moves.push(mov);
    }

    pub fn pop(&mut self) -> Option<Move> {
        self.moves.pop()
    }

    pub fn print_move_list(&self) {
        println!(" idx:  move:  piece:  prompted:  capture:  double:  en passant:  castling:");
        self.moves.iter().enumerate().for_each(|(i, m)| {
            println!(
                " {:03}   {:7}{:8}{:11}{:10}{:9}{:13}{}",
                i,
                m.to_string(),
                m.extract_piece().to_unicode(),
                m.extract_promoted_piece().to_unicode(),
                m.extract_capture().to_string(),
                m.extract_double_push().to_string(),
                m.extract_en_passant(),
                m.extract_castling()
            )
        });
        println!(" Number of moves: {}", self.moves.len());
    }
}

impl Index<usize> for MoveList {
    type Output = Move;

    fn index(&self, index: usize) -> &Self::Output {
        &self.moves[index]
    }
}

impl IntoIterator for MoveList {
    type Item = Move;

    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.moves.into_iter()
    }
}

#[derive(Clone, Copy)]
pub struct Move(u32);

impl Move {
    #[inline]
    pub const fn empty() -> Move {
        Move(0)
    }
    #[inline]
    #[allow(clippy::too_many_arguments)]
    pub const fn encode(
        source_square: Square,
        target_square: Square,
        piece: Piece,
        promoted_piece: Piece,
        capture: bool,
        double_push: bool,
        en_passant: bool,
        castling: bool,
    ) -> Move {
        let mut mv = source_square as u32
            | ((target_square as u32) << 6)
            | ((piece as u32) << 12)
            | ((promoted_piece as u32) << 16);
        if capture {
            mv |= 1 << 20;
        }
        if double_push {
            mv |= 1 << 21;
        }
        if en_passant {
            mv |= 1 << 22;
        }
        if castling {
            mv |= 1 << 23;
        }
        Move(mv)
    }

    #[inline]
    pub fn extract_source(self) -> Square {
        Square::from_u8_unchecked((self.0 & 0x3f) as u8)
    }

    #[inline]
    pub const fn extract_target(self) -> Square {
        Square::from_u8_unchecked(((self.0 & 0xfc0) >> 6) as u8)
    }

    #[inline]
    pub const fn extract_piece(self) -> Piece {
        Piece::from_u8(((self.0 & 0xf000) >> 12) as u8)
    }

    #[inline]
    pub const fn extract_promoted_piece(self) -> Piece {
        Piece::from_u8(((self.0 & 0xf_0000) >> 16) as u8)
    }

    #[inline]
    pub const fn extract_capture(self) -> bool {
        (self.0 & 0x10_0000) != 0
    }

    #[inline]
    pub const fn extract_double_push(self) -> bool {
        (self.0 & 0x20_0000) != 0
    }

    #[inline]
    pub const fn extract_en_passant(self) -> bool {
        (self.0 & 0x40_0000) != 0
    }

    #[inline]
    pub const fn extract_castling(self) -> bool {
        (self.0 & 0x80_0000) != 0
    }
}

impl Display for Move {
    #[inline]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let p = match self.extract_promoted_piece() {
            Piece::WKnight | Piece::BKnight => 'n',
            Piece::WBishop | Piece::BBishop => 'b',
            Piece::WRook | Piece::BRook => 'r',
            Piece::WQueen | Piece::BQueen => 'q',
            _ => return write!(f, "{}{}", self.extract_source(), self.extract_target()),
        };
        write!(f, "{}{}{}", self.extract_source(), self.extract_target(), p)
    }
}

#[cfg(test)]
mod test {
    use crate::{piece::Piece, square::Square};

    use super::Move;

    #[test]
    fn test_encode_decode_move() {
        let m = Move::encode(
            Square::E4,
            Square::H8,
            Piece::BBishop,
            Piece::BQueen,
            true,
            true,
            true,
            true,
        );
        assert_eq!(m.extract_source(), Square::E4);
        assert_eq!(m.extract_target(), Square::H8);
        assert_eq!(m.extract_piece(), Piece::BBishop);
        assert_eq!(m.extract_promoted_piece(), Piece::BQueen);
        assert!(m.extract_capture());
        assert!(m.extract_double_push());
        assert!(m.extract_en_passant());
        assert!(m.extract_castling());
    }
}

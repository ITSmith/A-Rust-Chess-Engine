use std::{convert::TryFrom, fmt::Display, mem};

use crate::{
    attacks::Attacks,
    bitboard::BitBoard,
    castle_rights::CastleRights,
    move_list::Move,
    piece::{Piece, BLACK_PIECES, WHITE_PIECES},
    side::Side,
    square::Square,
};

#[derive(Clone)]
pub struct Position {
    pub w_pawns: BitBoard,
    pub w_knights: BitBoard,
    pub w_bishops: BitBoard,
    pub w_rooks: BitBoard,
    pub w_queens: BitBoard,
    pub w_king: BitBoard,
    pub b_pawns: BitBoard,
    pub b_knights: BitBoard,
    pub b_bishops: BitBoard,
    pub b_rooks: BitBoard,
    pub b_queens: BitBoard,
    pub b_king: BitBoard,
    // Occupancies
    pub w_occupancies: BitBoard,
    pub b_occupancies: BitBoard,
    pub all_occupancies: BitBoard,

    pub side: Side,
    pub en_passant: Option<Square>,
    pub castle: CastleRights,
}

impl Position {
    /// Attempts to make a move. Reverts and returns false if the move is illegal. Returns true otherwise.
    #[inline]
    pub fn make_move(&mut self, mov: Move, attacks: &Attacks) -> bool {
        let copy = self.clone();
        // Parse move
        let source_square = mov.extract_source();
        let target_square = mov.extract_target();
        let piece = mov.extract_piece();
        let promoted_piece = mov.extract_promoted_piece();
        let capture = mov.extract_capture();
        let double_push = mov.extract_double_push();
        let en_passant = mov.extract_en_passant();
        let castling = mov.extract_castling();

        // Move piece
        self.get_piece_bitboard_mut(piece).pop_bit(source_square);
        self.get_piece_bitboard_mut(piece).set_bit(target_square);

        // Handle captures
        if capture {
            let opponent_pieces = if self.side == Side::White {
                BLACK_PIECES
            } else {
                WHITE_PIECES
            };
            // Remove captured piece
            for piece in opponent_pieces {
                if self
                    .get_piece_bitboard(piece)
                    .get_bit(target_square)
                    .is_not_empty()
                {
                    self.get_piece_bitboard_mut(piece).pop_bit(target_square);
                    break;
                }
            }
        }

        // Handle promotions
        if promoted_piece != Piece::None {
            // Remove pawn
            self.get_piece_bitboard_mut(piece).pop_bit(target_square);
            // Set promeoted piece
            self.get_piece_bitboard_mut(promoted_piece)
                .set_bit(target_square);
        }

        // Handle en passant
        if en_passant {
            match self.side {
                Side::White => self
                    .b_pawns
                    .pop_bit(Square::from_u8_unchecked(target_square as u8 - 8)),
                Side::Black => self
                    .w_pawns
                    .pop_bit(Square::from_u8_unchecked(target_square as u8 + 8)),
            }
        }

        // Handle double pawn push
        self.en_passant = if double_push {
            Some(match self.side {
                Side::White => Square::from_u8_unchecked(target_square as u8 - 8),
                Side::Black => Square::from_u8_unchecked(target_square as u8 + 8),
            })
        } else {
            None
        };

        // Handle castling
        if castling {
            match target_square {
                Square::C1 => {
                    // White queenside
                    // Move A1 rook
                    self.w_rooks.pop_bit(Square::A1);
                    self.w_rooks.set_bit(Square::D1);
                }
                Square::G1 => {
                    // White kingside
                    // Move H1 rook
                    self.w_rooks.pop_bit(Square::H1);
                    self.w_rooks.set_bit(Square::F1);
                }
                Square::C8 => {
                    // Black queenside
                    // Move A8 rook
                    self.b_rooks.pop_bit(Square::A8);
                    self.b_rooks.set_bit(Square::D8);
                }
                Square::G8 => {
                    // Black kingnside
                    // Move H8 rook
                    self.b_rooks.pop_bit(Square::H8);
                    self.b_rooks.set_bit(Square::F8);
                }
                invalid => panic!("'{}' is not a valid castling square", invalid),
            }
        }
        // Update castle rights
        self.castle.update(source_square);
        self.castle.update(target_square);

        // Update occupancies
        self.w_occupancies = self.w_pawns
            | self.w_knights
            | self.w_bishops
            | self.w_rooks
            | self.w_queens
            | self.w_king;
        self.b_occupancies = self.b_pawns
            | self.b_knights
            | self.b_bishops
            | self.b_rooks
            | self.b_queens
            | self.b_king;
        self.all_occupancies = self.w_occupancies | self.b_occupancies;

        // Switch side
        self.side = if self.side == Side::White {
            Side::Black
        } else {
            Side::White
        };

        // If king attacked revert and return false
        if (self.side == Side::White
            && attacks.is_square_attacked(&self, self.b_king.get_lsb_square().unwrap(), self.side))
            || (self.side == Side::Black
                && attacks.is_square_attacked(
                    &self,
                    self.w_king.get_lsb_square().unwrap(),
                    self.side,
                ))
        {
            let _ = mem::replace(self, copy);
            false
        } else {
            true
        }
    }

    #[inline]
    pub fn make_capture(&mut self, mov: Move, attacks: &Attacks) -> bool {
        if mov.extract_capture() {
            self.make_move(mov, attacks)
        } else {
            false
        }
    }

    #[inline]
    pub fn get_piece_bitboard(&self, piece: Piece) -> BitBoard {
        match piece {
            Piece::WPawn => self.w_pawns,
            Piece::WKnight => self.w_knights,
            Piece::WBishop => self.w_bishops,
            Piece::WRook => self.w_rooks,
            Piece::WQueen => self.w_queens,
            Piece::WKing => self.w_king,
            Piece::BPawn => self.b_pawns,
            Piece::BKnight => self.b_knights,
            Piece::BBishop => self.b_bishops,
            Piece::BRook => self.b_rooks,
            Piece::BQueen => self.b_queens,
            Piece::BKing => self.b_king,
            Piece::None => BitBoard::empty(),
        }
    }

    #[inline]
    pub fn get_piece_bitboard_mut(&mut self, piece: Piece) -> &mut BitBoard {
        assert_ne!(piece, Piece::None);
        match piece {
            Piece::WPawn => &mut self.w_pawns,
            Piece::WKnight => &mut self.w_knights,
            Piece::WBishop => &mut self.w_bishops,
            Piece::WRook => &mut self.w_rooks,
            Piece::WQueen => &mut self.w_queens,
            Piece::WKing => &mut self.w_king,
            Piece::BPawn => &mut self.b_pawns,
            Piece::BKnight => &mut self.b_knights,
            Piece::BBishop => &mut self.b_bishops,
            Piece::BRook => &mut self.b_rooks,
            Piece::BQueen => &mut self.b_queens,
            Piece::BKing => &mut self.b_king,
            Piece::None => unreachable!(),
        }
    }

    #[inline]
    pub fn get_occupancy_bitboard(&self, side: Side) -> BitBoard {
        match side {
            Side::White => self.w_occupancies,
            Side::Black => self.b_occupancies,
        }
    }

    #[inline]
    pub fn get_occupancy_bitboard_mut(&mut self, side: Side) -> &mut BitBoard {
        match side {
            Side::White => &mut self.w_occupancies,
            Side::Black => &mut self.b_occupancies,
        }
    }

    pub fn print_board(&self) {
        for r in (0..8).rev() {
            print!(" {}", r + 1);
            for f in 0..8 {
                let b = BitBoard::from(Square::from_fr_unchecked(f, r));
                let piece = if (self.w_pawns & b).is_not_empty() {
                    Piece::WPawn
                } else if (self.w_knights & b).is_not_empty() {
                    Piece::WKnight
                } else if (self.w_bishops & b).is_not_empty() {
                    Piece::WBishop
                } else if (self.w_rooks & b).is_not_empty() {
                    Piece::WRook
                } else if (self.w_queens & b).is_not_empty() {
                    Piece::WQueen
                } else if (self.w_king & b).is_not_empty() {
                    Piece::WKing
                } else if (self.b_pawns & b).is_not_empty() {
                    Piece::BPawn
                } else if (self.b_knights & b).is_not_empty() {
                    Piece::BKnight
                } else if (self.b_bishops & b).is_not_empty() {
                    Piece::BBishop
                } else if (self.b_rooks & b).is_not_empty() {
                    Piece::BRook
                } else if (self.b_queens & b).is_not_empty() {
                    Piece::BQueen
                } else if (self.b_king & b).is_not_empty() {
                    Piece::BKing
                } else {
                    Piece::None
                };

                print!(" {}", piece.to_unicode());
            }
            println!();
        }
        println!("   A B C D E F G H");
        println!(" Side to move:      {}", self.side);
        let en_passant = match self.en_passant {
            Some(s) => s.to_string(),
            None => "None".to_string(),
        };
        println!(" En passant square: {}", en_passant);
        println!(" Castling rights:   {}", self.castle);
    }
}

impl Display for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut bstr = String::with_capacity(72);
        for r in (0..8).rev() {
            for c in 0..8 {
                let b = BitBoard::from(Square::try_from((r, c)).unwrap());
                if (self.w_pawns & b).is_not_empty() {
                    bstr.push('P');
                } else if (self.w_knights & b).is_not_empty() {
                    bstr.push('N');
                } else if (self.w_bishops & b).is_not_empty() {
                    bstr.push('B');
                } else if (self.w_rooks & b).is_not_empty() {
                    bstr.push('R');
                } else if (self.w_queens & b).is_not_empty() {
                    bstr.push('Q');
                } else if (self.w_king & b).is_not_empty() {
                    bstr.push('K');
                } else if (self.b_pawns & b).is_not_empty() {
                    bstr.push('p');
                } else if (self.b_knights & b).is_not_empty() {
                    bstr.push('n');
                } else if (self.b_bishops & b).is_not_empty() {
                    bstr.push('b');
                } else if (self.b_rooks & b).is_not_empty() {
                    bstr.push('r');
                } else if (self.b_queens & b).is_not_empty() {
                    bstr.push('q');
                } else if (self.b_king & b).is_not_empty() {
                    bstr.push('k');
                } else {
                    bstr.push('_');
                }
                if c == 7 {
                    bstr.push('\n')
                } else {
                    bstr.push('|')
                }
            }
        }
        write!(f, "{}", bstr)
    }
}

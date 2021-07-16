use crate::{piece::PIECES, position::Position, side::Side, square::Square};

const MATERIAL_SCORE: [i32; 12] = [
    100,    // White pawn
    300,    // White knight
    350,    // White bishop
    500,    // White rook
    1000,   // White queen
    10000,  // White king
    -100,   // Black pawn
    -300,   // Black knight
    -350,   // Black bishop
    -500,   // Black rook
    -1000,  // Black queen
    -10000, // Black king
];

#[rustfmt::skip]
const PAWN_SCORE: [i32; 64] = [
      0,   0,   0,   0,   0,   0,   0,   0,
      0,   0,   0, -10, -10,   0,   0,   0,
      0,   0,   0,   5,   5,   0,   0,   0,
      5,   5,  10,  20,  20,   5,   5,   5,
     10,  10,  10,  20,  20,  10,  10,  10,
     20,  20,  20,  30,  30,  30,  20,  20,
     30,  30,  30,  40,  40,  30,  30,  30,
     90,  90,  90,  90,  90,  90,  90,  90,
];

#[rustfmt::skip]
const KNIGHT_SCORE: [i32; 64] = [
     -5, -10,   0,   0,   0,   0, -10, -5,
     -5,   0,   0,   0,   0,   0,   0, -5,
     -5,   5,  20,  10,  10,  20,   5, -5,
     -5,  10,  20,  30,  30,  20,  10, -5,
     -5,  10,  20,  30,  30,  20,  10, -5,
     -5,   5,  20,  20,  20,  20,   5, -5,
     -5,   0,   0,  10,  10,   0,   0, -5,
     -5,   0,   0,   0,   0,   0,   0, -5,
];

#[rustfmt::skip]
const BISHOP_SCORE: [i32; 64] = [
      0,   0, -10,   0,   0, -10,   0,   0,
      0,  30,   0,   0,   0,   0,  30,   0,
      0,  10,   0,   0,   0,   0,  10,   0,
      0,   0,  10,  20,  20,  10,   0,   0,
      0,   0,  10,  20,  20,  10,   0,   0,
      0,   0,   0,  10,  10,   0,   0,   0,
      0,   0,   0,   0,   0,   0,   0,   0,
      0,   0,   0,   0,   0,   0,   0,   0,
];

#[rustfmt::skip]
const ROOK_SCORE: [i32; 64] = [
      0,   0,   0,  20,  20,   0,   0,   0,
      0,   0,  10,  20,  20,  10,   0,   0,
      0,   0,  10,  20,  20,  10,   0,   0,
      0,   0,  10,  20,  20,  10,   0,   0,
      0,   0,  10,  20,  20,  10,   0,   0,
      0,   0,  10,  20,  20,  10,   0,   0,
     50,  50,  50,  50,  50,  50,  50,  50,
     50,  50,  50,  50,  50,  50,  50,  50,
];

#[rustfmt::skip]
const KING_SCORE: [i32; 64] = [
      0,   0,   5,   0, -15,   0,  10,   0,
      0,   5,   5,  -5,  -5,   0,   5,   0,
      0,   0,   5,  10,  10,   5,   0,   0,
      0,   5,  10,  20,  20,  10,   5,   0,
      0,   5,  10,  20,  20,  10,   5,   0,
      0,   5,   5,  10,  10,   5,   5,   0,
      0,   0,   5,   5,   5,   5,   0,   0,
      0,   0,   0,   0,   0,   0,   0,   0,
];

#[rustfmt::skip]
const MIRROR_SCORE: [Square; 64] = [
    Square::A8, Square::B8, Square::C8, Square::D8, Square::E8, Square::F8, Square::G8, Square::H8,
    Square::A7, Square::B7, Square::C7, Square::D7, Square::E7, Square::F7, Square::G7, Square::H7,
    Square::A6, Square::B6, Square::C6, Square::D6, Square::E6, Square::F6, Square::G6, Square::H6,
    Square::A5, Square::B5, Square::C5, Square::D5, Square::E5, Square::F5, Square::G5, Square::H5,
    Square::A4, Square::B4, Square::C4, Square::D4, Square::E4, Square::F4, Square::G4, Square::H4,
    Square::A3, Square::B3, Square::C3, Square::D3, Square::E3, Square::F3, Square::G3, Square::H3,
    Square::A2, Square::B2, Square::C2, Square::D2, Square::E2, Square::F2, Square::G2, Square::H2,
    Square::A1, Square::B1, Square::C1, Square::D1, Square::E1, Square::F1, Square::G1, Square::H1,
];

pub fn evaluate(position: Position) -> i32 {
    // Evaluation score
    let mut score = 0;

    let mut bitboard;
    let mut piece;
    let mut square;

    for p in PIECES {
        bitboard = position.get_piece_bitboard(p);

        while let Some(s) = bitboard.get_lsb_square() {
            piece = p;
            square = s;
            score += MATERIAL_SCORE[piece as usize];

            match p {
                crate::piece::Piece::WPawn => score += PAWN_SCORE[square as usize],
                crate::piece::Piece::WKnight => score += KNIGHT_SCORE[square as usize],
                crate::piece::Piece::WBishop => score += BISHOP_SCORE[square as usize],
                crate::piece::Piece::WRook => score += ROOK_SCORE[square as usize],
                crate::piece::Piece::WKing => score += KING_SCORE[square as usize],
                crate::piece::Piece::BPawn => {
                    score -= PAWN_SCORE[MIRROR_SCORE[square as usize] as usize]
                }
                crate::piece::Piece::BKnight => {
                    score -= KNIGHT_SCORE[MIRROR_SCORE[square as usize] as usize]
                }
                crate::piece::Piece::BBishop => {
                    score -= BISHOP_SCORE[MIRROR_SCORE[square as usize] as usize]
                }
                crate::piece::Piece::BRook => {
                    score -= ROOK_SCORE[MIRROR_SCORE[square as usize] as usize]
                }
                crate::piece::Piece::BKing => {
                    score -= KING_SCORE[MIRROR_SCORE[square as usize] as usize]
                }
                _ => (),
            }
            bitboard.pop_bit(square);
        }
    }
    if position.side == Side::White {
        score
    } else {
        -score
    }
}

use crate::{
    piece::PIECES,
    position::Position,
    side::Side,
    square::{Square, Square::*},
};

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
    A8, B8, C8, D8, E8, F8, G8, H8,
    A7, B7, C7, D7, E7, F7, G7, H7,
    A6, B6, C6, D6, E6, F6, G6, H6,
    A5, B5, C5, D5, E5, F5, G5, H5,
    A4, B4, C4, D4, E4, F4, G4, H4,
    A3, B3, C3, D3, E3, F3, G3, H3,
    A2, B2, C2, D2, E2, F2, G2, H2,
    A1, B1, C1, D1, E1, F1, G1, H1,
];

pub fn evaluate(position: &Position) -> i32 {
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

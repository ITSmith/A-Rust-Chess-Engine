use crate::{bitboard::BitBoard, side::Side, square::Square};

use super::{NON_A_FILE, NON_H_FILE};

pub fn mask_pawn_attacks(square: Square, side: Side) -> BitBoard {
    let mut attacks = BitBoard::empty();
    let sb = BitBoard::from(square);

    match side {
        Side::White => {
            if ((sb << 7) & NON_H_FILE).is_not_empty() {
                attacks |= sb << 7;
            }
            if ((sb << 9) & NON_A_FILE).is_not_empty() {
                attacks |= sb << 9;
            }
        }
        Side::Black => {
            if ((sb >> 7) & NON_A_FILE).is_not_empty() {
                attacks |= sb >> 7;
            }
            if ((sb >> 9) & NON_H_FILE).is_not_empty() {
                attacks |= sb >> 9;
            }
        }
    }
    attacks
}

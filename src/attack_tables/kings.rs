use crate::{bitboard::BitBoard, square::Square};

use super::{NON_A_FILE, NON_H_FILE};

pub fn mask_king_attacks(square: Square) -> BitBoard {
    let mut attacks = BitBoard::empty();
    let sb = BitBoard::from(square);

    if ((sb << 1) & NON_A_FILE).is_not_empty() {
        attacks |= sb << 1;
    }
    if ((sb << 7) & NON_H_FILE).is_not_empty() {
        attacks |= sb << 7;
    }
    attacks |= sb << 8;
    if ((sb << 9) & NON_A_FILE).is_not_empty() {
        attacks |= sb << 9;
    }

    if ((sb >> 1) & NON_H_FILE).is_not_empty() {
        attacks |= sb >> 1;
    }
    attacks |= sb >> 8;
    if ((sb >> 7) & NON_A_FILE).is_not_empty() {
        attacks |= sb >> 7;
    }
    if ((sb >> 9) & NON_H_FILE).is_not_empty() {
        attacks |= sb >> 9;
    }

    attacks
}

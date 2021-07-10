use crate::{bitboard::BitBoard, square::Square};

use super::{NON_AB_FILE, NON_A_FILE, NON_GH_FILE, NON_H_FILE};

pub fn mask_knight_attacks(square: Square) -> BitBoard {
    let mut attacks = BitBoard::empty();
    let sb = BitBoard::from(square);

    if ((sb << 17) & NON_A_FILE).is_not_empty() {
        attacks |= sb << 17;
    }
    if ((sb << 15) & NON_H_FILE).is_not_empty() {
        attacks |= sb << 15;
    }
    if ((sb << 10) & NON_AB_FILE).is_not_empty() {
        attacks |= sb << 10;
    }
    if ((sb << 6) & NON_GH_FILE).is_not_empty() {
        attacks |= sb << 6;
    }

    if ((sb >> 17) & NON_H_FILE).is_not_empty() {
        attacks |= sb >> 17;
    }
    if ((sb >> 15) & NON_A_FILE).is_not_empty() {
        attacks |= sb >> 15;
    }
    if ((sb >> 10) & NON_GH_FILE).is_not_empty() {
        attacks |= sb >> 10;
    }
    if ((sb >> 6) & NON_AB_FILE).is_not_empty() {
        attacks |= sb >> 6;
    }

    attacks
}

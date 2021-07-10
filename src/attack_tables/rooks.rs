use crate::{bitboard::BitBoard, square::Square};

pub fn mask_rook_attacks(square: Square) -> BitBoard {
    let mut attacks = BitBoard::empty();

    let (tf, tr): (i8, i8) = square.into();

    let mut r = tr + 1;
    while r <= 6 {
        attacks |= BitBoard(1) << (r * 8 + tf) as u8;
        r += 1;
    }

    r = tr - 1;
    while r >= 1 {
        attacks |= BitBoard(1) << (r * 8 + tf) as u8;
        r -= 1;
    }

    let mut f = tf + 1;
    while f <= 6 {
        attacks |= BitBoard(1) << (tr * 8 + f) as u8;
        f += 1;
    }

    f = tf - 1;
    while f >= 1 {
        attacks |= BitBoard(1) << (tr * 8 + f) as u8;
        f -= 1;
    }

    attacks
}

pub fn gen_rook_attacks(square: Square, block: BitBoard) -> BitBoard {
    let mut attacks = BitBoard::empty();

    let (tf, tr): (i8, i8) = square.into();

    let mut r = tr + 1;
    while r <= 7 {
        let cb = BitBoard(1) << (r * 8 + tf) as u8;
        attacks |= cb;

        if (cb & block).is_not_empty() {
            break;
        }

        r += 1;
    }

    r = tr - 1;
    while r >= 0 {
        let cb = BitBoard(1) << (r * 8 + tf) as u8;
        attacks |= cb;

        if (cb & block).is_not_empty() {
            break;
        }

        r -= 1;
    }

    let mut f = tf + 1;
    while f <= 7 {
        let cb = BitBoard(1) << (tr * 8 + f) as u8;
        attacks |= cb;

        if (cb & block).is_not_empty() {
            break;
        }

        f += 1;
    }

    f = tf - 1;
    while f >= 0 {
        let cb = BitBoard(1) << (tr * 8 + f) as u8;
        attacks |= cb;

        if (cb & block).is_not_empty() {
            break;
        }

        f -= 1;
    }

    attacks
}

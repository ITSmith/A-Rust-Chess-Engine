use crate::{bitboard::BitBoard, square::Square};

pub fn mask_bishop_attacks(square: Square) -> BitBoard {
    let mut attacks = BitBoard::empty();

    let (tf, tr): (i8, i8) = square.into();

    let mut f = tf + 1;
    let mut r = tr + 1;
    while f <= 6 && r <= 6 {
        attacks |= BitBoard(1) << (r * 8 + f) as u8;
        f += 1;
        r += 1;
    }

    f = tf + 1;
    r = tr - 1;
    while f <= 6 && r >= 1 {
        attacks |= BitBoard(1) << (r * 8 + f) as u8;
        f += 1;
        r -= 1;
    }

    f = tf - 1;
    r = tr - 1;
    while f >= 1 && r >= 1 {
        attacks |= BitBoard(1) << (r * 8 + f) as u8;
        f -= 1;
        r -= 1;
    }

    f = tf - 1;
    r = tr + 1;
    while f >= 1 && r <= 6 {
        attacks |= BitBoard(1) << (r * 8 + f) as u8;
        f -= 1;
        r += 1;
    }

    attacks
}

pub fn gen_bishop_attacks(square: Square, block: BitBoard) -> BitBoard {
    let mut attacks = BitBoard::empty();

    let (tf, tr): (i8, i8) = square.into();

    let mut f = tf + 1;
    let mut r = tr + 1;
    while f <= 7 && r <= 7 {
        let cb = BitBoard(1) << (r * 8 + f) as u8;
        attacks |= cb;

        if (cb & block).is_not_empty() {
            break;
        }

        f += 1;
        r += 1;
    }

    f = tf + 1;
    r = tr - 1;
    while f <= 7 && r >= 0 {
        let cb = BitBoard(1) << (r * 8 + f) as u8;
        attacks |= cb;

        if (cb & block).is_not_empty() {
            break;
        }

        f += 1;
        r -= 1;
    }

    f = tf - 1;
    r = tr - 1;
    while f >= 0 && r >= 0 {
        let cb = BitBoard(1) << (r * 8 + f) as u8;
        attacks |= cb;

        if (cb & block).is_not_empty() {
            break;
        }

        f -= 1;
        r -= 1;
    }

    f = tf - 1;
    r = tr + 1;
    while f >= 0 && r <= 7 {
        let cb = BitBoard(1) << (r * 8 + f) as u8;
        attacks |= cb;

        if (cb & block).is_not_empty() {
            break;
        }

        f -= 1;
        r += 1;
    }

    attacks
}

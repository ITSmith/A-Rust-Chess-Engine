use std::str;

pub const PIECES: [Piece; 12] = [
    Piece::WPawn,
    Piece::WKnight,
    Piece::WBishop,
    Piece::WRook,
    Piece::WQueen,
    Piece::WKing,
    Piece::BPawn,
    Piece::BKnight,
    Piece::BBishop,
    Piece::BRook,
    Piece::BQueen,
    Piece::BKing,
];

pub const WHITE_PIECES: [Piece; 6] = [
    Piece::WPawn,
    Piece::WKnight,
    Piece::WBishop,
    Piece::WRook,
    Piece::WQueen,
    Piece::WKing,
];

pub const BLACK_PIECES: [Piece; 6] = [
    Piece::BPawn,
    Piece::BKnight,
    Piece::BBishop,
    Piece::BRook,
    Piece::BQueen,
    Piece::BKing,
];

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Piece {
    WPawn = 0,
    WKnight,
    WBishop,
    WRook,
    WQueen,
    WKing,
    BPawn,
    BKnight,
    BBishop,
    BRook,
    BQueen,
    BKing,
    None,
}

impl Piece {
    pub fn from_u8(value: u8) -> Piece {
        match value {
            0 => Piece::WPawn,
            1 => Piece::WKnight,
            2 => Piece::WBishop,
            3 => Piece::WRook,
            4 => Piece::WQueen,
            5 => Piece::WKing,
            6 => Piece::BPawn,
            7 => Piece::BKnight,
            8 => Piece::BBishop,
            9 => Piece::BRook,
            10 => Piece::BQueen,
            11 => Piece::BKing,
            _ => Piece::None,
        }
    }

    #[allow(unused)]
    pub fn from_ascii(ch: char) -> Piece {
        match ch {
            'P' => Piece::WPawn,
            'N' => Piece::WKnight,
            'B' => Piece::WBishop,
            'R' => Piece::WRook,
            'Q' => Piece::WQueen,
            'K' => Piece::WKing,
            'p' => Piece::BPawn,
            'n' => Piece::BKnight,
            'b' => Piece::BBishop,
            'r' => Piece::BRook,
            'q' => Piece::BQueen,
            'k' => Piece::BKing,
            _ => Piece::None,
        }
    }

    #[allow(unused)]
    pub fn to_ascii(self) -> char {
        match self {
            Piece::WPawn => 'P',
            Piece::WKnight => 'N',
            Piece::WBishop => 'B',
            Piece::WRook => 'R',
            Piece::WQueen => 'Q',
            Piece::WKing => 'K',
            Piece::BPawn => 'p',
            Piece::BKnight => 'n',
            Piece::BBishop => 'b',
            Piece::BRook => 'r',
            Piece::BQueen => 'q',
            Piece::BKing => 'k',
            Piece::None => '-',
        }
    }

    pub fn to_unicode(self) -> &'static str {
        match self {
            Piece::WPawn => "\u{2659}",
            Piece::WKnight => "\u{2658}",
            Piece::WBishop => "\u{2657}",
            Piece::WRook => "\u{2656}",
            Piece::WQueen => "\u{2655}",
            Piece::WKing => "\u{2654}",
            Piece::BPawn => "\u{265f}",
            Piece::BKnight => "\u{265e}",
            Piece::BBishop => "\u{265d}",
            Piece::BRook => "\u{265c}",
            Piece::BQueen => "\u{265b}",
            Piece::BKing => "\u{265a}",
            Piece::None => "-",
        }
    }
}

use std::convert::TryFrom;

use crate::{
    bitboard::BitBoard, board::Board, castle_rights::CastleRights, side::Side, square::Square,
};

pub const EMPTY_BOARD: &str = "8/8/8/8/8/8/8/8 w - -";
pub const START_POSITION: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
pub const TRICKY_POSITION: &str =
    "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1";
pub const KILLER_POSITION: &str =
    "rnbqkb1r/pp1p1pPp/8/2p1pP2/1P1P4/3P3P/P1P1P3/RNBQKBNR w KQkq e6 0 1";
pub const CMK_POSITION: &str =
    "r2q1rk1/ppp2ppp/2n1bn2/2b1p3/3pP3/3P1NPP/PPP1NPB1/R1BQ1RK1 b - - 0 9";

pub fn parse(fen: &str) -> Option<Board> {
    let mut fen = fen.trim().split_ascii_whitespace();
    let mut fen_board = fen.next()?.split('/');
    let fen_side = fen.next()?;
    let fen_castle = fen.next()?;
    let fen_en_passant = fen.next()?;

    let mut w_pawns = BitBoard::empty();
    let mut w_knights = BitBoard::empty();
    let mut w_bishops = BitBoard::empty();
    let mut w_rooks = BitBoard::empty();
    let mut w_queens = BitBoard::empty();
    let mut w_king = BitBoard::empty();
    let mut b_pawns = BitBoard::empty();
    let mut b_knights = BitBoard::empty();
    let mut b_bishops = BitBoard::empty();
    let mut b_rooks = BitBoard::empty();
    let mut b_queens = BitBoard::empty();
    let mut b_king = BitBoard::empty();

    for rank in (0..8).rev() {
        let mut file = 0;
        for ch in fen_board.next()?.chars() {
            let square = Square::from_fr_unchecked(file, rank);
            match ch {
                '1' => (),
                '2' => file += 1,
                '3' => file += 2,
                '4' => file += 3,
                '5' => file += 4,
                '6' => file += 5,
                '7' => file += 6,
                '8' => file += 7,
                'P' => w_pawns.set_bit(square),
                'N' => w_knights.set_bit(square),
                'B' => w_bishops.set_bit(square),
                'R' => w_rooks.set_bit(square),
                'Q' => w_queens.set_bit(square),
                'K' => w_king.set_bit(square),
                'p' => b_pawns.set_bit(square),
                'n' => b_knights.set_bit(square),
                'b' => b_bishops.set_bit(square),
                'r' => b_rooks.set_bit(square),
                'q' => b_queens.set_bit(square),
                'k' => b_king.set_bit(square),
                _ => return None,
            }
            file += 1;
        }
        if file != 8 {
            return None;
        }
    }

    let w_occupancies = w_pawns | w_knights | w_bishops | w_rooks | w_queens | w_king;
    let b_occupancies = b_pawns | b_knights | b_bishops | b_rooks | b_queens | b_king;
    let a_occupancies = w_occupancies | b_occupancies;

    let side = match fen_side {
        "w" => Side::White,
        "b" => Side::Black,
        _ => return None,
    };

    let en_passant = match Square::try_from(fen_en_passant) {
        Ok(square) => Some(square),
        Err(_) => None,
    };

    let castle = CastleRights::from(fen_castle);

    Some(Board {
        w_pawns,
        w_knights,
        w_bishops,
        w_rooks,
        w_queens,
        w_king,
        b_pawns,
        b_knights,
        b_bishops,
        b_rooks,
        b_queens,
        b_king,
        w_occupancies,
        b_occupancies,
        a_occupancies,
        side,
        en_passant,
        castle,
    })
}

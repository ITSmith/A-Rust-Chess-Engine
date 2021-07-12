use std::convert::TryFrom;

use crate::{
    move_list::{Move, MoveList},
    piece::Piece,
    square::Square,
};

pub fn parse_move(move_str: &str, move_list: &MoveList) -> Result<Move, ()> {
    if move_str.len() >= 4 {
        let source_square = Square::try_from(&move_str[..2])?;
        let target_square = Square::try_from(&move_str[2..4])?;
        let promoted = if move_str.len() == 5 {
            move_str.as_bytes()[4] as char
        } else {
            '-'
        };

        for mov in move_list.moves.iter() {
            if source_square == mov.extract_source()
                && target_square == mov.extract_target()
                && promoted == mov.extract_promoted_piece().to_ascii().to_ascii_lowercase()
            {
                return Ok(*mov);
            }
        }
    }
    Err(())
}

use crate::{evaluation::evaluate, position::Position};

/// Negamax alpha beta search
#[inline]
pub fn negamax(position: Position, alpha: i32, beta: i32, depth: u8) -> i32 {
    if depth == 0 {
        return evaluate(position);
    }

    todo!()
}

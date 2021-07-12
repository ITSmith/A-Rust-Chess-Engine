use crate::{attacks::Attacks, board::Board, move_gen::MoveGen, move_list::MoveList};

pub struct Perft<'a> {
    move_gen: MoveGen<'a>,

    nodes: u32,
}

impl<'a> Perft<'a> {
    pub fn new() {}
}

pub fn perft(board: Board, depth: u32) {
    let at = Attacks::gen();
    let mg = MoveGen::new(&at);
    let mut moves = mg.generate_moves(&board);

    let nodes = 0;
}

#[inline]
fn perft_driver(nodes: &mut u32, depth: u32) {
    if depth == 0 {
        *nodes += 1;
        return;
    } else {
    }
}

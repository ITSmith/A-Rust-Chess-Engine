use std::mem;

use crate::{attacks::Attacks, board::Board, move_gen::MoveGen};

pub struct Perft {
    move_gen: MoveGen,
    board: Board,
    pub nodes: u64,
}

impl Perft {
    pub fn new(board: Board) -> Perft {
        Perft {
            move_gen: MoveGen::new(Attacks::gen()),
            board,
            nodes: 0,
        }
    }

    #[inline]
    pub fn perft_driver(&mut self, depth: u32) {
        if depth == 0 {
            self.nodes += 1;
            return;
        }
        let moves = self.move_gen.generate_moves(&self.board);
        for mov in moves.moves.iter() {
            let copy = self.board.clone();
            if !self.board.make_move(*mov, &self.move_gen.attacks) {
                continue;
            }
            self.perft_driver(depth - 1);
            let _ = mem::replace(&mut self.board, copy);
        }
    }
}

// #[inline]
// pub fn perft_driver(depth: u32, board: Board) {
//     let move_gen = MoveGen::new(Attacks::gen());

//     let perft_rec = |depth: u32| -> u32 {
//         if depth == 0 {
//             return 1;
//         }
//         let moves = move_gen.generate_moves(&board);
//         for mov in moves.moves.iter() {
//             let copy = board.clone();
//             if board.make_move(*mov, &move_gen.attacks) {
//                 continue;
//             }
//             self.perft_driver(depth - 1);
//             let _ = mem::replace(&mut self.board, copy);
//         }
//     }
//     let nodes = perft_rec(depth);
// }

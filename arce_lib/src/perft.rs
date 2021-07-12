use std::{mem, time::Instant};

use crate::{attacks::Attacks, board::Board, move_gen::MoveGen};

pub fn perft_test(board: Board, depth: u32) {
    println!("Performance Test:");
    let mut p = Perft::new(board);
    let moves = p.move_gen.generate_moves(&p.board);
    // Start timer
    let start_time = Instant::now();
    // Find perft of all legal moves
    for mov in moves.moves.iter() {
        let copy = p.board.clone();
        if !p.board.make_move(*mov, &p.move_gen.attacks) {
            continue;
        }
        let prev_nodes = p.nodes;
        p.perft_driver(depth - 1);
        let new_nodes = p.nodes - prev_nodes;
        // Reset board
        let _ = mem::replace(&mut p.board, copy);
        println!(" move: {:5}  nodes: {}", mov, new_nodes);
    }
    let time = start_time.elapsed();
    println!("Depth: {}", depth);
    println!("Nodes: {}", p.nodes);
    println!("Time: {:02?}", time);
}

struct Perft {
    move_gen: MoveGen,
    board: Board,
    pub nodes: u64,
}

impl Perft {
    fn new(board: Board) -> Perft {
        Perft {
            move_gen: MoveGen::new(Attacks::gen()),
            board,
            nodes: 0,
        }
    }

    #[inline]
    fn perft_driver(&mut self, depth: u32) {
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

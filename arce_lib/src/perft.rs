use std::{mem, time::Instant};

use crate::{attacks::Attacks, move_gen::MoveGen, position::Position};

pub fn perft_test(pos: Position, depth: u32) {
    println!("Performance Test:");
    let mut p = Perft::new(pos);
    let moves = p.move_gen.generate_moves(&p.pos);
    // Start timer
    let start_time = Instant::now();
    // Find perft of all legal moves
    for mov in moves.moves.iter() {
        let copy = p.pos.clone();
        if !p.pos.make_move(*mov, &p.move_gen.attacks) {
            continue;
        }
        let prev_nodes = p.nodes;
        p.perft_driver(depth - 1);
        let new_nodes = p.nodes - prev_nodes;
        // Reset pos
        let _ = mem::replace(&mut p.pos, copy);
        println!(" move: {:5}  nodes: {}", mov, new_nodes);
    }
    let time = start_time.elapsed();
    println!("Depth: {}", depth);
    println!("Nodes: {}", p.nodes);
    println!("Time: {:02?}", time);
}

struct Perft {
    move_gen: MoveGen,
    pos: Position,
    pub nodes: u64,
}

impl Perft {
    fn new(pos: Position) -> Perft {
        Perft {
            move_gen: MoveGen::new(Attacks::gen()),
            pos,
            nodes: 0,
        }
    }

    #[inline]
    fn perft_driver(&mut self, depth: u32) {
        if depth == 0 {
            self.nodes += 1;
            return;
        }
        let moves = self.move_gen.generate_moves(&self.pos);
        for mov in moves.moves.iter() {
            let copy = self.pos.clone();
            if !self.pos.make_move(*mov, &self.move_gen.attacks) {
                continue;
            }
            self.perft_driver(depth - 1);
            let _ = mem::replace(&mut self.pos, copy);
        }
    }
}

use std::time::Instant;

use crate::{fen::parse, move_gen::MoveGen, move_list::MoveList};

mod attack_tables;
mod bitboard;
mod board;
mod castle_rights;
mod fen;
mod move_gen;
mod move_list;
mod piece;
mod random;
mod side;
mod square;

fn main() {
    let now = Instant::now();

    let mg = MoveGen::new();

    let mut b =
        parse("r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1").unwrap();
    b.print_board();
    let mut ml = MoveList::with_capacity(40);
    mg.generate_moves(&b, &mut ml);
    let m = ml[7];
    println!("{}", m);
    b.make_move(m);
    b.print_board();
    ml.print_move_list();

    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
}

use std::time::Instant;

use arce_lib::{fen::parse, move_gen::MoveGen, move_list::MoveList};

fn main() {
    let now = Instant::now();

    let mg = MoveGen::new();

    let mut b =
        parse("r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1").unwrap();
    let mut ml = MoveList::with_capacity(40);
    mg.generate_moves(&b, &mut ml);
    ml.print_move_list();
    b.print_board();
    let m = ml[1];
    println!("{}", m);
    b.make_move(m);
    b.print_board();

    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
}

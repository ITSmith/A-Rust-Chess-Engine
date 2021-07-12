use std::time::Instant;

use arce_lib::{attacks::Attacks, fen::parse, move_gen::MoveGen, move_list::MoveList};

fn main() {
    let now = Instant::now();

    let at = Attacks::gen();
    let mg = MoveGen::new(&at);

    let mut b =
        parse("r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R2qK2R w KQkq - 0 1").unwrap();
    let mut ml = mg.generate_moves(&b);
    ml.print_move_list();
    b.print_board();
    let m = ml[32];
    println!("{}", m);
    let ms = b.make_move(m, &at);
    b.print_board();
    println!("Move success: {}", ms);

    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
}

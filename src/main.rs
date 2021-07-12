use std::time::Instant;

use arce_lib::{
    attacks::Attacks,
    move_gen::MoveGen,
    utils::{
        fen::{parse, START_POSITION, TRICKY_POSITION},
        uci::parse_move,
    },
};

fn main() {
    let now = Instant::now();

    let at = Attacks::gen();
    let mg = MoveGen::new(at);

    let mut b =
        parse("r3k2r/pPppqpp1/bn2pnp1/2pPN3/1p2P3/2N2Q1p/PPPBBPpP/R3K2R w KQkq c6 0 1").unwrap();
    let ml = mg.generate_moves(&b);
    ml.print_move_list();
    b.print_board();
    match parse_move("d5c6", &ml) {
        Ok(m) => {
            b.make_move(m, &mg.attacks);
            b.print_board();
        }
        Err(_) => println!("Invalid move!"),
    };

    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
}

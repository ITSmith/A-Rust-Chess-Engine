use std::time::Instant;

use arce_lib::{
    attacks::Attacks,
    move_gen::MoveGen,
    utils::{
        fen::{parse_fen, START_POSITION, TRICKY_POSITION},
        uci::{parse_go, parse_move, parse_position},
    },
};

fn main() {
    let now = Instant::now();

    let at = Attacks::gen();
    let mg = MoveGen::new(at);

    // let mut b = parse_fen("r3k2r/pPppqpp1/bn2pnp1/2pPN3/1p2P3/2N2Q1p/PPPBBPpP/R3K2R w KQkq c6 0 1")
    //     .unwrap();
    // let ml = mg.generate_moves(&b);
    // ml.print_move_list();
    // b.print_board();
    // match parse_move("d5c6", &ml) {
    //     Ok(m) => {
    //         b.make_move(m, &mg.attacks);
    //         b.print_board();
    //     }
    //     Err(_) => println!("Invalid move!"),
    // };

    match parse_position(
        "position startpos moves e2e4 e7e5 f1d3 f8b4 g1e2 g8f6 e1g1 e8g8 f2f4",
        &mg,
    ) {
        Some(p) => p.print_board(),
        None => println!("Invalid position"),
    }

    parse_go("go depth 5");

    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
}

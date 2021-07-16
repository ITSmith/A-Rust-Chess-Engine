use std::time::Instant;

use arce_lib::{
    attacks::Attacks,
    utils::{
        fen::{parse_fen, START_POSITION, TRICKY_POSITION},
        uci::{parse_go, parse_move, parse_position, uci_loop},
    },
};

fn main() {
    // let now = Instant::now();

    // let at = Attacks::gen();
    // let mg = MoveGen::new(at);

    // match parse_position(
    //     "position startpos moves e2e4 e7e5 f1d3 f8b4 g1e2 g8f6 e1g1 e8g8 f2f4",
    //     &mg,
    // ) {
    //     Some(p) => p.print_board(),
    //     None => println!("Invalid position"),
    // }

    // parse_go("go depth 5");

    // let elapsed = now.elapsed();
    // println!("Elapsed: {:.2?}", elapsed);
    let debug = true;
    if debug {
        let b =
            parse_fen("rnbqkbnr/pppp1ppp/8/4p3/4P3/5N2/PPPP1PPP/RNBQKB1R w KQkq - 0 1").unwrap();
        b.print_board();
        print!("Score: {}", arce_lib::evaluation::evaluate(b));
    } else {
        uci_loop();
    }
}

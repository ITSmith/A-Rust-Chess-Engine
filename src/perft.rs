use arce_lib::{perft, utils::fen};

fn main() {
    perft::perft_test(fen::parse(fen::TRICKY_POSITION).unwrap(), 6);
}

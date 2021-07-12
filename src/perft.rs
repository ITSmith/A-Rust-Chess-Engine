use std::time::Instant;

use arce_lib::{
    fen::{parse, START_POSITION, TRICKY_POSITION},
    perft::Perft,
};

fn main() {
    let mut p = Perft::new(parse(START_POSITION).unwrap());
    let start_time = Instant::now();
    p.perft_driver(6);
    let elapsed = start_time.elapsed();
    println!("Nodes: {}", p.nodes);
    println!("Time taken to execute: {:0.2?}", elapsed);
}

use std::{
    convert::TryFrom,
    io::{stdin, BufRead, Read},
    num::NonZeroU8,
};

use crate::{
    attacks::Attacks,
    move_gen,
    move_list::{Move, MoveList},
    position::Position,
    square::Square,
    utils::fen::{parse_fen, START_POSITION},
};

pub fn uci_loop() {
    let attacks = Attacks::gen();
    let mut pos: Position;
    println!("id name ARCE");
    println!("id name Ian Smith");
    println!("uciok");

    let mut input = String::new();
    loop {
        input.clear();
        // Get user/GUI input
        if let Err(_) = stdin().lock().read_line(&mut input) {
            continue;
        }

        if input.starts_with("isready") {
            println!("readyok");
            continue;
        } else if input.starts_with("position") {
            match parse_position(&input, &attacks) {
                Some(p) => pos = p,
                None => (),
            };
        } else if input.starts_with("ucinewgame") {
            pos = parse_position("position startpos", &attacks).unwrap();
        } else if input.starts_with("go") {
            let _ = parse_go(&input);
        } else if input.starts_with("quit") {
            break;
        } else if input.starts_with("uci") {
            println!("id name ARCE");
            println!("id name Ian Smith");
            println!("uciok");
        }
    }
}

pub fn parse_move(move_str: &str, move_list: &MoveList) -> Result<Move, ()> {
    if move_str.len() >= 4 {
        let source_square = Square::try_from(&move_str[..2])?;
        let target_square = Square::try_from(&move_str[2..4])?;
        let promoted = if move_str.len() == 5 {
            move_str.as_bytes()[4] as char
        } else {
            '-'
        };

        for mov in move_list.moves.iter() {
            if source_square == mov.extract_source()
                && target_square == mov.extract_target()
                && promoted == mov.extract_promoted_piece().to_ascii().to_ascii_lowercase()
            {
                return Ok(*mov);
            }
        }
    }
    Err(())
}

pub fn parse_position(uci_str: &str, attacks: &Attacks) -> Option<Position> {
    // Check if string has correct prefix
    let pos_str = uci_str.strip_prefix("position ")?;
    // Check whether position is standard start position or FEN
    if let Some(moves_str) = pos_str.strip_prefix("startpos") {
        // Create start position
        let mut pos = parse_fen(START_POSITION)?;
        // Check for moves
        if let Some(moves_str) = moves_str.strip_prefix(" moves ") {
            // Make moves
            for mov in moves_str.split_ascii_whitespace() {
                if let Ok(mov) = parse_move(mov, &move_gen::generate_moves(attacks, &pos)) {
                    if !pos.make_move(mov, attacks) {
                        return None;
                    }
                } else {
                    return None;
                }
            }
        }

        Some(pos)
    } else if let Some(pos_str) = pos_str.strip_prefix("fen ") {
        // Check for moves
        match pos_str.split_once(" moves ") {
            Some((fen, moves_str)) => {
                // Create position
                let mut pos = parse_fen(fen)?;
                // Make moves
                for mov in moves_str.split_ascii_whitespace() {
                    if let Ok(mov) = parse_move(mov, &move_gen::generate_moves(attacks, &pos)) {
                        if !pos.make_move(mov, attacks) {
                            return None;
                        }
                    } else {
                        return None;
                    }
                }

                Some(pos)
            }
            None => parse_fen(pos_str),
        }
    } else {
        None
    }
}

pub fn parse_go(go_str: &str) -> Option<()> {
    let mut go_args = go_str.split_ascii_whitespace();
    if go_args.next()? != "go" {
        return None;
    }
    let mut depth = 6;
    // Check for fixed depth
    if go_args.next()? == "depth" {
        match go_args.next()?.parse::<u8>() {
            Ok(d) => depth = d,
            Err(_) => (),
        }
    }
    search_pos(depth);
    None
}

pub fn search_pos(depth: u8) {
    // Placeholder
    println!("bestmove d2d4");
}

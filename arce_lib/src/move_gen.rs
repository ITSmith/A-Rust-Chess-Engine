use crate::{
    attacks::Attacks,
    move_list::{Move, MoveList},
    piece::{Piece, BLACK_PIECES, WHITE_PIECES},
    position::Position,
    side::Side,
    square::Square,
};

#[inline]
pub fn generate_moves(attacks: Attacks, pos: &Position) -> MoveList {
    let mut moves = MoveList::with_capacity(256);
    match pos.side {
        Side::White => gen_w_moves(attacks, pos, &mut moves),
        Side::Black => gen_b_moves(attacks, pos, &mut moves),
    }
    moves
}

#[inline]
fn gen_w_moves(attacks: Attacks, pos: &Position, moves: &mut MoveList) {
    for piece in WHITE_PIECES {
        if piece == Piece::WPawn {
            gen_w_pawn_moves(attacks, pos, moves)
        } else if piece == Piece::WKnight {
            gen_w_knight_moves(attacks, pos, moves)
        } else if piece == Piece::WBishop {
            gen_w_bishop_moves(attacks, pos, moves)
        } else if piece == Piece::WRook {
            gen_w_rook_moves(attacks, pos, moves)
        } else if piece == Piece::WQueen {
            gen_w_queen_moves(attacks, pos, moves)
        } else if piece == Piece::WKing {
            gen_w_king_moves(attacks, pos, moves)
        }
    }
}

fn gen_w_pawn_moves(attacks: Attacks, pos: &Position, moves: &mut MoveList) {
    let mut bitboard = pos.get_piece_bitboard(Piece::WPawn);
    while let Some(source_square) = bitboard.get_lsb_square() {
        // Generate quiet pawn moves
        if let Some(target_square) = Square::from_u8(source_square as u8 + 8) {
            if pos.all_occupancies.get_bit(target_square).is_empty() {
                // Pawn promotion
                if source_square >= Square::A7 && source_square <= Square::H7 {
                    moves.push(Move::encode(
                        source_square,
                        target_square,
                        Piece::WPawn,
                        Piece::WQueen,
                        false,
                        false,
                        false,
                        false,
                    ));
                    moves.push(Move::encode(
                        source_square,
                        target_square,
                        Piece::WPawn,
                        Piece::WRook,
                        false,
                        false,
                        false,
                        false,
                    ));
                    moves.push(Move::encode(
                        source_square,
                        target_square,
                        Piece::WPawn,
                        Piece::WBishop,
                        false,
                        false,
                        false,
                        false,
                    ));
                    moves.push(Move::encode(
                        source_square,
                        target_square,
                        Piece::WPawn,
                        Piece::WKnight,
                        false,
                        false,
                        false,
                        false,
                    ));
                } else {
                    // Pawn push
                    moves.push(Move::encode(
                        source_square,
                        target_square,
                        Piece::WPawn,
                        Piece::None,
                        false,
                        false,
                        false,
                        false,
                    ));

                    if (source_square >= Square::A2 && source_square <= Square::H2)
                        && pos
                            .all_occupancies
                            .get_bit(Square::from_u8_unchecked(target_square as u8 + 8))
                            .is_empty()
                    {
                        // Double pawn push
                        moves.push(Move::encode(
                            source_square,
                            Square::from_u8_unchecked(target_square as u8 + 8),
                            Piece::WPawn,
                            Piece::None,
                            false,
                            true,
                            false,
                            false,
                        ));
                    }
                }
            }
        }
        // Init pawn attacks_bb
        let mut attacks_bb = attacks.get_w_pawn_attacks(source_square) & pos.b_occupancies;
        // Generate pawn captures
        while let Some(target_square) = attacks_bb.get_lsb_square() {
            if source_square >= Square::A7 && source_square <= Square::H7 {
                // Pawn capture promotion
                moves.push(Move::encode(
                    source_square,
                    target_square,
                    Piece::WPawn,
                    Piece::WQueen,
                    true,
                    false,
                    false,
                    false,
                ));
                moves.push(Move::encode(
                    source_square,
                    target_square,
                    Piece::WPawn,
                    Piece::WRook,
                    true,
                    false,
                    false,
                    false,
                ));
                moves.push(Move::encode(
                    source_square,
                    target_square,
                    Piece::WPawn,
                    Piece::WBishop,
                    true,
                    false,
                    false,
                    false,
                ));
                moves.push(Move::encode(
                    source_square,
                    target_square,
                    Piece::WPawn,
                    Piece::WKnight,
                    true,
                    false,
                    false,
                    false,
                ));
            } else {
                // Regular capture
                moves.push(Move::encode(
                    source_square,
                    target_square,
                    Piece::WPawn,
                    Piece::None,
                    true,
                    false,
                    false,
                    false,
                ));
            }
            attacks_bb.pop_bit(target_square);
        }
        // Generate en passant captures
        if let Some(en_passant_square) = pos.en_passant {
            let en_passant_attacks =
                attacks.get_w_pawn_attacks(source_square) & en_passant_square.into();
            if let Some(target_square) = en_passant_attacks.get_lsb_square() {
                moves.push(Move::encode(
                    source_square,
                    target_square,
                    Piece::WPawn,
                    Piece::None,
                    true,
                    false,
                    true,
                    false,
                ));
            }
        }

        bitboard.pop_bit(source_square);
    }
}

fn gen_w_knight_moves(attacks: Attacks, pos: &Position, moves: &mut MoveList) {
    let mut bitboard = pos.get_piece_bitboard(Piece::WKnight);
    while let Some(source_square) = bitboard.get_lsb_square() {
        let mut attacks_bb = attacks.get_knight_attacks(source_square) & !pos.w_occupancies;
        while let Some(target_square) = attacks_bb.get_lsb_square() {
            if pos.b_occupancies.get_bit(target_square).is_empty() {
                // Quiet move
                moves.push(Move::encode(
                    source_square,
                    target_square,
                    Piece::WKnight,
                    Piece::None,
                    false,
                    false,
                    false,
                    false,
                ));
            } else {
                // Capture
                moves.push(Move::encode(
                    source_square,
                    target_square,
                    Piece::WKnight,
                    Piece::None,
                    true,
                    false,
                    false,
                    false,
                ));
            }

            attacks_bb.pop_bit(target_square);
        }
        bitboard.pop_bit(source_square);
    }
}

fn gen_w_bishop_moves(attacks: Attacks, pos: &Position, moves: &mut MoveList) {
    let mut bitboard = pos.get_piece_bitboard(Piece::WBishop);
    while let Some(source_square) = bitboard.get_lsb_square() {
        let mut attacks_bb =
            attacks.get_bishop_attacks(source_square, pos.all_occupancies) & !pos.w_occupancies;
        while let Some(target_square) = attacks_bb.get_lsb_square() {
            if pos.b_occupancies.get_bit(target_square).is_empty() {
                // Quiet move
                moves.push(Move::encode(
                    source_square,
                    target_square,
                    Piece::WBishop,
                    Piece::None,
                    false,
                    false,
                    false,
                    false,
                ));
            } else {
                // Capture
                moves.push(Move::encode(
                    source_square,
                    target_square,
                    Piece::WBishop,
                    Piece::None,
                    true,
                    false,
                    false,
                    false,
                ));
            }

            attacks_bb.pop_bit(target_square);
        }
        bitboard.pop_bit(source_square);
    }
}

fn gen_w_rook_moves(attacks: Attacks, pos: &Position, moves: &mut MoveList) {
    let mut bitboard = pos.get_piece_bitboard(Piece::WRook);
    while let Some(source_square) = bitboard.get_lsb_square() {
        let mut attacks_bb =
            attacks.get_rook_attacks(source_square, pos.all_occupancies) & !pos.w_occupancies;
        while let Some(target_square) = attacks_bb.get_lsb_square() {
            if pos.b_occupancies.get_bit(target_square).is_empty() {
                // Quiet move
                moves.push(Move::encode(
                    source_square,
                    target_square,
                    Piece::WRook,
                    Piece::None,
                    false,
                    false,
                    false,
                    false,
                ));
            } else {
                // Capture
                moves.push(Move::encode(
                    source_square,
                    target_square,
                    Piece::WRook,
                    Piece::None,
                    true,
                    false,
                    false,
                    false,
                ));
            }

            attacks_bb.pop_bit(target_square);
        }
        bitboard.pop_bit(source_square);
    }
}

fn gen_w_queen_moves(attacks: Attacks, pos: &Position, moves: &mut MoveList) {
    let mut bitboard = pos.get_piece_bitboard(Piece::WQueen);
    while let Some(source_square) = bitboard.get_lsb_square() {
        let mut attacks_bb =
            attacks.get_queen_attacks(source_square, pos.all_occupancies) & !pos.w_occupancies;
        while let Some(target_square) = attacks_bb.get_lsb_square() {
            if pos.b_occupancies.get_bit(target_square).is_empty() {
                // Quiet move
                moves.push(Move::encode(
                    source_square,
                    target_square,
                    Piece::WQueen,
                    Piece::None,
                    false,
                    false,
                    false,
                    false,
                ));
            } else {
                // Capture
                moves.push(Move::encode(
                    source_square,
                    target_square,
                    Piece::WQueen,
                    Piece::None,
                    true,
                    false,
                    false,
                    false,
                ));
            }

            attacks_bb.pop_bit(target_square);
        }
        bitboard.pop_bit(source_square);
    }
}

fn gen_w_king_moves(attacks: Attacks, pos: &Position, moves: &mut MoveList) {
    let mut bitboard = pos.get_piece_bitboard(Piece::WKing);
    while let Some(source_square) = bitboard.get_lsb_square() {
        let mut attacks_bb = attacks.get_king_attacks(source_square) & !pos.w_occupancies;
        while let Some(target_square) = attacks_bb.get_lsb_square() {
            if pos.b_occupancies.get_bit(target_square).is_empty() {
                // Quiet move
                moves.push(Move::encode(
                    source_square,
                    target_square,
                    Piece::WKing,
                    Piece::None,
                    false,
                    false,
                    false,
                    false,
                ));
            } else {
                // Capture
                moves.push(Move::encode(
                    source_square,
                    target_square,
                    Piece::WKing,
                    Piece::None,
                    true,
                    false,
                    false,
                    false,
                ));
            }

            attacks_bb.pop_bit(target_square);
        }
        bitboard.pop_bit(source_square);
    }
    // Kingside castling possible
    if pos.castle.wk() {
        // Make sure squares between king and king's rook are empty
        if pos.all_occupancies.get_bit(Square::F1).is_empty()
            && pos.all_occupancies.get_bit(Square::G1).is_empty()
        {
            // Make sure king and f1 are not attacked
            if !attacks.is_square_attacked(pos, Square::E1, Side::Black)
                && !attacks.is_square_attacked(pos, Square::F1, Side::Black)
            {
                // Kingside castle
                moves.push(Move::encode(
                    Square::E1,
                    Square::G1,
                    Piece::WKing,
                    Piece::None,
                    false,
                    false,
                    false,
                    true,
                ));
            }
        }
    }
    // Queenside castling possible
    if pos.castle.wq() {
        // Make sure squares between king and queen's rook are empty
        if pos.all_occupancies.get_bit(Square::B1).is_empty()
            && pos.all_occupancies.get_bit(Square::C1).is_empty()
            && pos.all_occupancies.get_bit(Square::D1).is_empty()
        {
            // Make sure king and d1 are not attacked
            if !attacks.is_square_attacked(pos, Square::E1, Side::Black)
                && !attacks.is_square_attacked(pos, Square::D1, Side::Black)
            {
                // Queenside castle
                moves.push(Move::encode(
                    Square::E1,
                    Square::C1,
                    Piece::WKing,
                    Piece::None,
                    false,
                    false,
                    false,
                    true,
                ));
            }
        }
    }
}

fn gen_b_moves(attacks: Attacks, pos: &Position, moves: &mut MoveList) {
    for piece in BLACK_PIECES {
        if piece == Piece::BPawn {
            gen_b_pawn_moves(attacks, pos, moves)
        } else if piece == Piece::BKnight {
            gen_b_knight_moves(attacks, pos, moves)
        } else if piece == Piece::BBishop {
            gen_b_bishop_moves(attacks, pos, moves)
        } else if piece == Piece::BRook {
            gen_b_rook_moves(attacks, pos, moves)
        } else if piece == Piece::BQueen {
            gen_b_queen_moves(attacks, pos, moves)
        } else if piece == Piece::BKing {
            gen_b_king_moves(attacks, pos, moves)
        }
    }
}

fn gen_b_pawn_moves(attacks: Attacks, pos: &Position, moves: &mut MoveList) {
    let mut bitboard = pos.get_piece_bitboard(Piece::BPawn);
    while let Some(source_square) = bitboard.get_lsb_square() {
        // Generate quiet pawn moves
        if let Some(target_square) = Square::from_u8(source_square as u8 - 8) {
            if pos.all_occupancies.get_bit(target_square).is_empty() {
                if source_square >= Square::A2 && source_square <= Square::H2 {
                    // Pawn promotion
                    moves.push(Move::encode(
                        source_square,
                        target_square,
                        Piece::BPawn,
                        Piece::BQueen,
                        false,
                        false,
                        false,
                        false,
                    ));
                    moves.push(Move::encode(
                        source_square,
                        target_square,
                        Piece::BPawn,
                        Piece::BRook,
                        false,
                        false,
                        false,
                        false,
                    ));
                    moves.push(Move::encode(
                        source_square,
                        target_square,
                        Piece::BPawn,
                        Piece::BBishop,
                        false,
                        false,
                        false,
                        false,
                    ));
                    moves.push(Move::encode(
                        source_square,
                        target_square,
                        Piece::BPawn,
                        Piece::BKnight,
                        false,
                        false,
                        false,
                        false,
                    ));
                } else {
                    // Pawn push
                    moves.push(Move::encode(
                        source_square,
                        target_square,
                        Piece::BPawn,
                        Piece::None,
                        false,
                        false,
                        false,
                        false,
                    ));
                    if (source_square >= Square::A7 && source_square <= Square::H7)
                        && pos
                            .all_occupancies
                            .get_bit(Square::from_u8_unchecked(target_square as u8 - 8))
                            .is_empty()
                    {
                        // Double pawn push
                        moves.push(Move::encode(
                            source_square,
                            Square::from_u8_unchecked(target_square as u8 - 8),
                            Piece::BPawn,
                            Piece::None,
                            false,
                            true,
                            false,
                            false,
                        ));
                    }
                }
            }
        }
        // Init pawn attacks_bb
        let mut attacks_bb = attacks.get_b_pawn_attacks(source_square) & pos.w_occupancies;
        // Generate pawn captures
        while let Some(target_square) = attacks_bb.get_lsb_square() {
            // Pawn capture promotion
            if source_square >= Square::A2 && source_square <= Square::H2 {
                moves.push(Move::encode(
                    source_square,
                    target_square,
                    Piece::BPawn,
                    Piece::BQueen,
                    true,
                    false,
                    false,
                    false,
                ));
                moves.push(Move::encode(
                    source_square,
                    target_square,
                    Piece::BPawn,
                    Piece::BRook,
                    true,
                    false,
                    false,
                    false,
                ));
                moves.push(Move::encode(
                    source_square,
                    target_square,
                    Piece::BPawn,
                    Piece::BBishop,
                    true,
                    false,
                    false,
                    false,
                ));
                moves.push(Move::encode(
                    source_square,
                    target_square,
                    Piece::BPawn,
                    Piece::BKnight,
                    true,
                    false,
                    false,
                    false,
                ));
            } else {
                // Regular capture
                moves.push(Move::encode(
                    source_square,
                    target_square,
                    Piece::BPawn,
                    Piece::None,
                    true,
                    false,
                    false,
                    false,
                ));
            }
            attacks_bb.pop_bit(target_square);
        }
        // Generate en passant captures
        if let Some(en_passant_square) = pos.en_passant {
            let en_passant_attacks =
                attacks.get_b_pawn_attacks(source_square) & en_passant_square.into();
            if let Some(target_square) = en_passant_attacks.get_lsb_square() {
                moves.push(Move::encode(
                    source_square,
                    target_square,
                    Piece::BPawn,
                    Piece::None,
                    true,
                    false,
                    true,
                    false,
                ));
            }
        }

        bitboard.pop_bit(source_square);
    }
}

fn gen_b_knight_moves(attacks: Attacks, pos: &Position, moves: &mut MoveList) {
    let mut bitboard = pos.get_piece_bitboard(Piece::BKnight);
    while let Some(source_square) = bitboard.get_lsb_square() {
        let mut attacks_bb = attacks.get_knight_attacks(source_square) & !pos.b_occupancies;
        while let Some(target_square) = attacks_bb.get_lsb_square() {
            if pos.w_occupancies.get_bit(target_square).is_empty() {
                // Quiet move
                moves.push(Move::encode(
                    source_square,
                    target_square,
                    Piece::BKnight,
                    Piece::None,
                    false,
                    false,
                    false,
                    false,
                ));
            } else {
                // Capture
                moves.push(Move::encode(
                    source_square,
                    target_square,
                    Piece::BKnight,
                    Piece::None,
                    true,
                    false,
                    false,
                    false,
                ));
            }

            attacks_bb.pop_bit(target_square);
        }
        bitboard.pop_bit(source_square);
    }
}

fn gen_b_bishop_moves(attacks: Attacks, pos: &Position, moves: &mut MoveList) {
    let mut bitboard = pos.get_piece_bitboard(Piece::BBishop);
    while let Some(source_square) = bitboard.get_lsb_square() {
        let mut attacks_bb =
            attacks.get_bishop_attacks(source_square, pos.all_occupancies) & !pos.b_occupancies;
        while let Some(target_square) = attacks_bb.get_lsb_square() {
            if pos.w_occupancies.get_bit(target_square).is_empty() {
                // Quiet move
                moves.push(Move::encode(
                    source_square,
                    target_square,
                    Piece::BBishop,
                    Piece::None,
                    false,
                    false,
                    false,
                    false,
                ));
            } else {
                // Capture
                moves.push(Move::encode(
                    source_square,
                    target_square,
                    Piece::BBishop,
                    Piece::None,
                    true,
                    false,
                    false,
                    false,
                ));
            }

            attacks_bb.pop_bit(target_square);
        }
        bitboard.pop_bit(source_square);
    }
}

fn gen_b_rook_moves(attacks: Attacks, pos: &Position, moves: &mut MoveList) {
    let mut bitboard = pos.get_piece_bitboard(Piece::BRook);
    while let Some(source_square) = bitboard.get_lsb_square() {
        let mut attacks_bb =
            attacks.get_rook_attacks(source_square, pos.all_occupancies) & !pos.b_occupancies;
        while let Some(target_square) = attacks_bb.get_lsb_square() {
            if pos.w_occupancies.get_bit(target_square).is_empty() {
                // Quiet move
                moves.push(Move::encode(
                    source_square,
                    target_square,
                    Piece::BRook,
                    Piece::None,
                    false,
                    false,
                    false,
                    false,
                ));
            } else {
                // Capture
                moves.push(Move::encode(
                    source_square,
                    target_square,
                    Piece::BRook,
                    Piece::None,
                    true,
                    false,
                    false,
                    false,
                ));
            }

            attacks_bb.pop_bit(target_square);
        }
        bitboard.pop_bit(source_square);
    }
}

fn gen_b_queen_moves(attacks: Attacks, pos: &Position, moves: &mut MoveList) {
    let mut bitboard = pos.get_piece_bitboard(Piece::BQueen);
    while let Some(source_square) = bitboard.get_lsb_square() {
        let mut attacks_bb =
            attacks.get_queen_attacks(source_square, pos.all_occupancies) & !pos.b_occupancies;
        while let Some(target_square) = attacks_bb.get_lsb_square() {
            if pos.w_occupancies.get_bit(target_square).is_empty() {
                // Quiet move
                moves.push(Move::encode(
                    source_square,
                    target_square,
                    Piece::BQueen,
                    Piece::None,
                    false,
                    false,
                    false,
                    false,
                ));
            } else {
                moves.push(Move::encode(
                    source_square,
                    target_square,
                    Piece::BQueen,
                    Piece::None,
                    true,
                    false,
                    false,
                    false,
                ));
            }

            attacks_bb.pop_bit(target_square);
        }
        bitboard.pop_bit(source_square);
    }
}

fn gen_b_king_moves(attacks: Attacks, pos: &Position, moves: &mut MoveList) {
    let mut bitboard = pos.get_piece_bitboard(Piece::BKing);
    while let Some(source_square) = bitboard.get_lsb_square() {
        let mut attacks_bb = attacks.get_king_attacks(source_square) & !pos.b_occupancies;
        while let Some(target_square) = attacks_bb.get_lsb_square() {
            if pos.w_occupancies.get_bit(target_square).is_empty() {
                // Quiet move
                moves.push(Move::encode(
                    source_square,
                    target_square,
                    Piece::BKing,
                    Piece::None,
                    false,
                    false,
                    false,
                    false,
                ));
            } else {
                // Capture
                moves.push(Move::encode(
                    source_square,
                    target_square,
                    Piece::BKing,
                    Piece::None,
                    true,
                    false,
                    false,
                    false,
                ));
            }

            attacks_bb.pop_bit(target_square);
        }
        bitboard.pop_bit(source_square);
    }
    // Kingside castling possible
    if pos.castle.bk() {
        // Make sure squares between king and  king's rook are empty
        if pos.all_occupancies.get_bit(Square::F8).is_empty()
            && pos.all_occupancies.get_bit(Square::G8).is_empty()
        {
            // Make sure king and f8 are not attacked
            if !attacks.is_square_attacked(pos, Square::E8, Side::White)
                && !attacks.is_square_attacked(pos, Square::F8, Side::White)
            {
                // Kingside castle
                moves.push(Move::encode(
                    Square::E8,
                    Square::G8,
                    Piece::BKing,
                    Piece::None,
                    false,
                    false,
                    false,
                    true,
                ));
            }
        }
    }
    // Queenside castling possible
    if pos.castle.bq() {
        // Make sure squares between king and queen's rook are empty
        if pos.all_occupancies.get_bit(Square::B8).is_empty()
            && pos.all_occupancies.get_bit(Square::C8).is_empty()
            && pos.all_occupancies.get_bit(Square::D8).is_empty()
        {
            // Make sure king and d8 are not attacked
            if !attacks.is_square_attacked(pos, Square::E8, Side::White)
                && !attacks.is_square_attacked(pos, Square::D8, Side::White)
            {
                // Queenside castle
                moves.push(Move::encode(
                    Square::E8,
                    Square::C8,
                    Piece::BKing,
                    Piece::None,
                    false,
                    false,
                    false,
                    true,
                ));
            }
        }
    }
}

use crate::{
    attack_tables::AttackTables,
    board::Board,
    move_list::{Move, MoveList},
    piece::{Piece, BLACK_PIECES, WHITE_PIECES},
    side::Side,
    square::Square,
};

pub struct MoveGen {
    pub attacks: AttackTables,
}

impl MoveGen {
    pub fn new() -> MoveGen {
        MoveGen {
            attacks: AttackTables::gen_attacks(),
        }
    }

    #[inline]
    pub fn generate_moves(&self, board: &Board, moves: &mut MoveList) {
        match board.side {
            Side::White => self.gen_w_moves(board, moves),
            Side::Black => self.gen_b_moves(board, moves),
        }
    }

    #[inline]
    fn gen_w_moves(&self, board: &Board, moves: &mut MoveList) {
        for piece in WHITE_PIECES {
            if piece == Piece::WPawn {
                self.gen_w_pawn_moves(board, moves)
            } else if piece == Piece::WKnight {
                self.gen_w_knight_moves(board, moves)
            } else if piece == Piece::WBishop {
                self.gen_w_bishop_moves(board, moves)
            } else if piece == Piece::WRook {
                self.gen_w_rook_moves(board, moves)
            } else if piece == Piece::WQueen {
                self.gen_w_queen_moves(board, moves)
            } else if piece == Piece::WKing {
                self.gen_w_king_moves(board, moves)
            }
        }
    }

    fn gen_w_pawn_moves(&self, board: &Board, moves: &mut MoveList) {
        let mut bitboard = board.get_piece_bitboard(Piece::WPawn);
        while let Some(source_square) = bitboard.get_lsb_square() {
            // Generate quiet pawn moves
            if let Some(target_square) = Square::from_u8(source_square as u8 + 8) {
                if board.both.get_bit(target_square).is_empty() {
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
                            && board
                                .both
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
            // Init pawn attacks
            let mut attacks = self.attacks.get_w_pawn_attacks(source_square) & board.black;
            // Generate pawn captures
            while let Some(target_square) = attacks.get_lsb_square() {
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
                attacks.pop_bit(target_square);
            }
            // Generate en passant captures
            if let Some(en_passant_square) = board.en_passant {
                let en_passant_attacks =
                    self.attacks.get_w_pawn_attacks(source_square) & en_passant_square.into();
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

    fn gen_w_knight_moves(&self, board: &Board, moves: &mut MoveList) {
        let mut bitboard = board.get_piece_bitboard(Piece::WKnight);
        while let Some(source_square) = bitboard.get_lsb_square() {
            let mut attacks = self.attacks.get_knight_attacks(source_square) & !board.white;
            while let Some(target_square) = attacks.get_lsb_square() {
                if board.black.get_bit(target_square).is_empty() {
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

                attacks.pop_bit(target_square);
            }
            bitboard.pop_bit(source_square);
        }
    }

    fn gen_w_bishop_moves(&self, board: &Board, moves: &mut MoveList) {
        let mut bitboard = board.get_piece_bitboard(Piece::WBishop);
        while let Some(source_square) = bitboard.get_lsb_square() {
            let mut attacks =
                self.attacks.get_bishop_attacks(source_square, board.both) & !board.white;
            while let Some(target_square) = attacks.get_lsb_square() {
                if board.black.get_bit(target_square).is_empty() {
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

                attacks.pop_bit(target_square);
            }
            bitboard.pop_bit(source_square);
        }
    }

    fn gen_w_rook_moves(&self, board: &Board, moves: &mut MoveList) {
        let mut bitboard = board.get_piece_bitboard(Piece::WRook);
        while let Some(source_square) = bitboard.get_lsb_square() {
            let mut attacks =
                self.attacks.get_rook_attacks(source_square, board.both) & !board.white;
            while let Some(target_square) = attacks.get_lsb_square() {
                if board.black.get_bit(target_square).is_empty() {
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

                attacks.pop_bit(target_square);
            }
            bitboard.pop_bit(source_square);
        }
    }

    fn gen_w_queen_moves(&self, board: &Board, moves: &mut MoveList) {
        let mut bitboard = board.get_piece_bitboard(Piece::WQueen);
        while let Some(source_square) = bitboard.get_lsb_square() {
            let mut attacks =
                self.attacks.get_queen_attacks(source_square, board.both) & !board.white;
            while let Some(target_square) = attacks.get_lsb_square() {
                if board.black.get_bit(target_square).is_empty() {
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

                attacks.pop_bit(target_square);
            }
            bitboard.pop_bit(source_square);
        }
    }

    fn gen_w_king_moves(&self, board: &Board, moves: &mut MoveList) {
        let mut bitboard = board.get_piece_bitboard(Piece::WKing);
        while let Some(source_square) = bitboard.get_lsb_square() {
            let mut attacks = self.attacks.get_king_attacks(source_square) & !board.white;
            while let Some(target_square) = attacks.get_lsb_square() {
                if board.black.get_bit(target_square).is_empty() {
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

                attacks.pop_bit(target_square);
            }
            bitboard.pop_bit(source_square);
        }
        // Kingside castling possible
        if board.castle.wk {
            // Make sure squares between king and king's rook are empty
            if board.both.get_bit(Square::F1).is_empty()
                && board.both.get_bit(Square::G1).is_empty()
            {
                // Make sure king and f1 are not attacked
                if !self.is_square_attacked(board, Square::E1, Side::Black)
                    && !self.is_square_attacked(board, Square::F1, Side::Black)
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
        if board.castle.wq {
            // Make sure squares between king and queen's rook are empty
            if board.both.get_bit(Square::B1).is_empty()
                && board.both.get_bit(Square::C1).is_empty()
                && board.both.get_bit(Square::D1).is_empty()
            {
                // Make sure king and d1 are not attacked
                if !self.is_square_attacked(board, Square::E1, Side::Black)
                    && !self.is_square_attacked(board, Square::D1, Side::Black)
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

    fn gen_b_moves(&self, board: &Board, moves: &mut MoveList) {
        for piece in BLACK_PIECES {
            if piece == Piece::BPawn {
                self.gen_b_pawn_moves(board, moves)
            } else if piece == Piece::BKnight {
                self.gen_b_knight_moves(board, moves)
            } else if piece == Piece::BBishop {
                self.gen_b_bishop_moves(board, moves)
            } else if piece == Piece::BRook {
                self.gen_b_rook_moves(board, moves)
            } else if piece == Piece::BQueen {
                self.gen_b_queen_moves(board, moves)
            } else if piece == Piece::BKing {
                self.gen_b_king_moves(board, moves)
            }
        }
    }

    fn gen_b_pawn_moves(&self, board: &Board, moves: &mut MoveList) {
        let mut bitboard = board.get_piece_bitboard(Piece::BPawn);
        while let Some(source_square) = bitboard.get_lsb_square() {
            // Generate quiet pawn moves
            if let Some(target_square) = Square::from_u8(source_square as u8 - 8) {
                if board.both.get_bit(target_square).is_empty() {
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
                            && board
                                .both
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
            // Init pawn attacks
            let mut attacks = self.attacks.get_b_pawn_attacks(source_square) & board.white;
            // Generate pawn captures
            while let Some(target_square) = attacks.get_lsb_square() {
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
                attacks.pop_bit(target_square);
            }
            // Generate en passant captures
            if let Some(en_passant_square) = board.en_passant {
                let en_passant_attacks =
                    self.attacks.get_b_pawn_attacks(source_square) & en_passant_square.into();
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

    fn gen_b_knight_moves(&self, board: &Board, moves: &mut MoveList) {
        let mut bitboard = board.get_piece_bitboard(Piece::BKnight);
        while let Some(source_square) = bitboard.get_lsb_square() {
            let mut attacks = self.attacks.get_knight_attacks(source_square) & !board.black;
            while let Some(target_square) = attacks.get_lsb_square() {
                if board.white.get_bit(target_square).is_empty() {
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

                attacks.pop_bit(target_square);
            }
            bitboard.pop_bit(source_square);
        }
    }

    fn gen_b_bishop_moves(&self, board: &Board, moves: &mut MoveList) {
        let mut bitboard = board.get_piece_bitboard(Piece::BBishop);
        while let Some(source_square) = bitboard.get_lsb_square() {
            let mut attacks =
                self.attacks.get_bishop_attacks(source_square, board.both) & !board.black;
            while let Some(target_square) = attacks.get_lsb_square() {
                if board.white.get_bit(target_square).is_empty() {
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

                attacks.pop_bit(target_square);
            }
            bitboard.pop_bit(source_square);
        }
    }

    fn gen_b_rook_moves(&self, board: &Board, moves: &mut MoveList) {
        let mut bitboard = board.get_piece_bitboard(Piece::BRook);
        while let Some(source_square) = bitboard.get_lsb_square() {
            let mut attacks =
                self.attacks.get_rook_attacks(source_square, board.both) & !board.black;
            while let Some(target_square) = attacks.get_lsb_square() {
                if board.white.get_bit(target_square).is_empty() {
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

                attacks.pop_bit(target_square);
            }
            bitboard.pop_bit(source_square);
        }
    }

    fn gen_b_queen_moves(&self, board: &Board, moves: &mut MoveList) {
        let mut bitboard = board.get_piece_bitboard(Piece::BQueen);
        while let Some(source_square) = bitboard.get_lsb_square() {
            let mut attacks =
                self.attacks.get_queen_attacks(source_square, board.both) & !board.black;
            while let Some(target_square) = attacks.get_lsb_square() {
                if board.white.get_bit(target_square).is_empty() {
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

                attacks.pop_bit(target_square);
            }
            bitboard.pop_bit(source_square);
        }
    }

    fn gen_b_king_moves(&self, board: &Board, moves: &mut MoveList) {
        let mut bitboard = board.get_piece_bitboard(Piece::BKing);
        while let Some(source_square) = bitboard.get_lsb_square() {
            let mut attacks = self.attacks.get_king_attacks(source_square) & !board.black;
            while let Some(target_square) = attacks.get_lsb_square() {
                if board.white.get_bit(target_square).is_empty() {
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

                attacks.pop_bit(target_square);
            }
            bitboard.pop_bit(source_square);
        }
        // Kingside castling possible
        if board.castle.bk {
            // Make sure squares between king and  king's rook are empty
            if board.both.get_bit(Square::F8).is_empty()
                && board.both.get_bit(Square::G8).is_empty()
            {
                // Make sure king and f8 are not attacked
                if !self.is_square_attacked(board, Square::E8, Side::White)
                    && !self.is_square_attacked(board, Square::F8, Side::White)
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
        if board.castle.bq {
            // Make sure squares between king and queen's rook are empty
            if board.both.get_bit(Square::B8).is_empty()
                && board.both.get_bit(Square::C8).is_empty()
                && board.both.get_bit(Square::D8).is_empty()
            {
                // Make sure king and d8 are not attacked
                if !self.is_square_attacked(board, Square::E8, Side::White)
                    && !self.is_square_attacked(board, Square::D8, Side::White)
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

    #[inline]
    pub fn is_square_attacked(&self, board: &Board, square: Square, side: Side) -> bool {
        if side == Side::White {
            (self.attacks.get_b_pawn_attacks(square) & board.w_pawns).is_not_empty()
                || (self.attacks.get_knight_attacks(square) & board.w_knights).is_not_empty()
                || (self.attacks.get_bishop_attacks(square, board.both) & board.w_bishops)
                    .is_not_empty()
                || (self.attacks.get_rook_attacks(square, board.both) & board.w_rooks)
                    .is_not_empty()
                || (self.attacks.get_queen_attacks(square, board.both) & board.w_queens)
                    .is_not_empty()
                || (self.attacks.get_king_attacks(square) & board.w_king).is_not_empty()
        } else {
            (self.attacks.get_w_pawn_attacks(square) & board.b_pawns).is_not_empty()
                || (self.attacks.get_knight_attacks(square) & board.b_knights).is_not_empty()
                || (self.attacks.get_bishop_attacks(square, board.both) & board.b_bishops)
                    .is_not_empty()
                || (self.attacks.get_rook_attacks(square, board.both) & board.b_rooks)
                    .is_not_empty()
                || (self.attacks.get_queen_attacks(square, board.both) & board.b_queens)
                    .is_not_empty()
                || (self.attacks.get_king_attacks(square) & board.b_king).is_not_empty()
        }
    }

    pub fn print_attacked_squares(&self, board: &Board, side: Side) {
        for r in (0..8).rev() {
            print!(" {}", r + 1);
            for f in 0..8 {
                let square = Square::from_fr_unchecked(f, r);
                let is_attacked = if self.is_square_attacked(board, square, side) {
                    1
                } else {
                    0
                };

                print!(" {}", is_attacked);
            }
            println!();
        }
        println!("   A B C D E F G H");
    }
}

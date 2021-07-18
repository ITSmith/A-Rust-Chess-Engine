use crate::{
    attacks::Attacks, evaluation::evaluate, move_gen::generate_moves, move_list::Move,
    position::Position, side::Side,
};

pub fn search_pos(position: &mut Position, attacks: &Attacks, depth: u8) {
    let mut search = Search::new();
    let score = search.negamax(position, attacks, -50000, 50000, depth);
    println!("info score cp {} nodes {}", score, search.nodes);
    println!("bestmove {}", search.best_move);
}

pub struct Search {
    nodes: u32,
    ply: i32,
    best_move: Move,
}

impl Search {
    pub fn new() -> Search {
        Search {
            nodes: 0,
            ply: 0,
            best_move: Move::empty(),
        }
    }

    /// Negamax alpha beta search
    #[inline]
    pub fn negamax(
        &mut self,
        position: &mut Position,
        attacks: &Attacks,
        mut alpha: i32,
        beta: i32,
        depth: u8,
    ) -> i32 {
        if depth == 0 {
            return evaluate(position);
        }
        let (king_square, opp_side) = match position.side {
            Side::White => (position.w_king.get_lsb_square().unwrap(), Side::Black),
            Side::Black => (position.b_king.get_lsb_square().unwrap(), Side::White),
        };

        let in_check = attacks.is_square_attacked(&position, king_square, opp_side);
        let mut legal_moves = 0;

        self.nodes += 1;
        let mut best_move = Move::empty();
        let old_alpha = alpha;

        let moves = generate_moves(attacks, position);

        for mov in moves.into_iter() {
            let mut copy = position.clone();
            self.ply += 1;
            // Check move legality
            if !copy.make_move(mov, attacks) {
                self.ply -= 1;
                // Skip move
                continue;
            }
            legal_moves += 1;

            // Score current move
            let score = -self.negamax(&mut copy, attacks, -beta, -alpha, depth - 1);
            self.ply -= 1;

            // Fail-hard cutoff
            if score >= beta {
                // Move fails high
                return beta;
            }
            // Found better move
            if score > alpha {
                // Principal variation move
                alpha = score;
                // If root move
                if self.ply == 0 {
                    best_move = mov;
                }
            }
        }
        // Check if any legal moves
        if legal_moves == 0 {
            if in_check {
                // Return mating score (ply is added so that faster mates are prioritized)
                return -49000 + self.ply;
            } else {
                // Return drawing score
                return 0;
            }
        }
        // Set new best move
        if old_alpha != alpha {
            self.best_move = best_move;
        }
        // Move fails low
        alpha
    }
}

impl Default for Search {
    fn default() -> Self {
        Self::new()
    }
}

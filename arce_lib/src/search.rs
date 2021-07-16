use crate::{
    attacks::Attacks, evaluation::evaluate, move_gen::generate_moves, move_list::Move,
    position::Position, square::Square,
};

pub fn search_pos(position: &mut Position, attacks: &Attacks, depth: u8) {
    let mut search = Search::new();
    let score = search.negamax(position, attacks, -50000, 50000, depth);
    // Placeholder
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
        self.nodes += 1;
        let mut best_move = Move::empty();
        let old_alpha = alpha;

        let moves = generate_moves(attacks, position);

        for mov in moves.into_iter() {
            let mut copy = position.clone();
            self.ply += 1;
            // Check move legality
            if !position.make_move(mov, attacks) {
                self.ply -= 1;
                // Skip move
                continue;
            }

            // Score current move
            let score = -self.negamax(position, attacks, -beta, -alpha, depth - 1);
            self.ply -= 1;

            *position = copy;

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

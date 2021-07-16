use crate::{
    attacks::{
        bishops::gen_bishop_attacks,
        magics::{
            bishop::{BISHOP_MAGIC_NUMBERS, BISHOP_RELEVANT_BITS},
            rook::{ROOK_MAGIC_NUMBERS, ROOK_RELEVANT_BITS},
        },
        rooks::gen_rook_attacks,
    },
    bitboard::BitBoard,
    position::Position,
    side::Side,
    square::{Square, SQUARES},
};

use self::{
    bishops::mask_bishop_attacks, kings::mask_king_attacks, knights::mask_knight_attacks,
    pawns::mask_pawn_attacks, rooks::mask_rook_attacks,
};

mod bishops;
mod kings;
mod knights;
mod magics;
mod pawns;
mod rooks;

#[allow(clippy::unreadable_literal)]
const NON_A_FILE: BitBoard = BitBoard(18374403900871474942);
#[allow(clippy::unreadable_literal)]
const NON_H_FILE: BitBoard = BitBoard(9187201950435737471);
#[allow(clippy::unreadable_literal)]
const NON_AB_FILE: BitBoard = BitBoard(18229723555195321596);
#[allow(clippy::unreadable_literal)]
const NON_GH_FILE: BitBoard = BitBoard(4557430888798830399);

pub struct Attacks {
    w_pawn: [BitBoard; 64],
    b_pawn: [BitBoard; 64],
    knight: [BitBoard; 64],
    king: [BitBoard; 64],
    bishop_masks: [BitBoard; 64],
    rook_masks: [BitBoard; 64],
    bishop_attacks: Box<[[BitBoard; 512]; 64]>,
    rook_attacks: Box<[[BitBoard; 4096]]>,
}

impl Attacks {
    pub fn gen() -> Attacks {
        // Leapers
        let mut w_pawn = [BitBoard::empty(); 64];
        let mut b_pawn = [BitBoard::empty(); 64];
        let mut knight = [BitBoard::empty(); 64];
        let mut king = [BitBoard::empty(); 64];

        // Sliders
        let mut bishop_masks = [BitBoard::empty(); 64];
        let mut rook_masks = [BitBoard::empty(); 64];
        let mut bishop_attacks = Box::new([[BitBoard::empty(); 512]; 64]);
        let mut rook_attacks = vec![[BitBoard::empty(); 4096]; 64].into_boxed_slice();

        SQUARES.iter().for_each(|&square| {
            let i = square as usize;
            w_pawn[i] = mask_pawn_attacks(square, Side::White);
            b_pawn[i] = mask_pawn_attacks(square, Side::Black);
            knight[i] = mask_knight_attacks(square);
            king[i] = mask_king_attacks(square);

            // Bishops
            bishop_masks[i] = mask_bishop_attacks(square);

            let attack_mask = bishop_masks[i];
            let relevant_bits_count = attack_mask.count_bits();
            let occupancy_indices = 1 << relevant_bits_count;
            for index in 0..occupancy_indices {
                let occupancy = attack_mask.set_occupancy(index, relevant_bits_count);
                let magic_index = (occupancy * BISHOP_MAGIC_NUMBERS[square as usize])
                    >> (64 - BISHOP_RELEVANT_BITS[square as usize]);

                bishop_attacks[square as usize][magic_index.0 as usize] =
                    gen_bishop_attacks(square, occupancy);
            }

            // Rooks
            rook_masks[i] = mask_rook_attacks(square);

            let attack_mask = rook_masks[i];
            let relevant_bits_count = attack_mask.count_bits();
            let occupancy_indices = 1 << relevant_bits_count;
            for index in 0..occupancy_indices {
                let occupancy = attack_mask.set_occupancy(index, relevant_bits_count);
                let magic_index = (occupancy * ROOK_MAGIC_NUMBERS[square as usize])
                    >> (64 - ROOK_RELEVANT_BITS[square as usize]);

                rook_attacks[square as usize][magic_index.0 as usize] =
                    gen_rook_attacks(square, occupancy);
            }
        });

        Attacks {
            w_pawn,
            b_pawn,
            knight,
            king,
            bishop_masks,
            rook_masks,
            bishop_attacks,
            rook_attacks,
        }
    }

    #[inline]
    pub fn get_w_pawn_attacks(&self, square: Square) -> BitBoard {
        self.w_pawn[square as usize]
    }

    #[inline]
    pub fn get_b_pawn_attacks(&self, square: Square) -> BitBoard {
        self.b_pawn[square as usize]
    }

    #[inline]
    pub fn get_knight_attacks(&self, square: Square) -> BitBoard {
        self.knight[square as usize]
    }

    #[inline]
    pub fn get_king_attacks(&self, square: Square) -> BitBoard {
        self.king[square as usize]
    }

    #[inline]
    pub fn get_bishop_attacks(&self, square: Square, mut occupancy: BitBoard) -> BitBoard {
        occupancy &= self.bishop_masks[square as usize];
        occupancy *= BISHOP_MAGIC_NUMBERS[square as usize];
        occupancy >>= 64 - BISHOP_RELEVANT_BITS[square as usize];

        self.bishop_attacks[square as usize][occupancy.0 as usize]
    }

    #[inline]
    pub fn get_rook_attacks(&self, square: Square, mut occupancy: BitBoard) -> BitBoard {
        occupancy &= self.rook_masks[square as usize];
        occupancy *= ROOK_MAGIC_NUMBERS[square as usize];
        occupancy >>= 64 - ROOK_RELEVANT_BITS[square as usize];

        self.rook_attacks[square as usize][occupancy.0 as usize]
    }

    #[inline]
    pub fn get_queen_attacks(&self, square: Square, occupancy: BitBoard) -> BitBoard {
        self.get_bishop_attacks(square, occupancy) | self.get_rook_attacks(square, occupancy)
    }

    #[inline]
    pub fn is_square_attacked(&self, pos: &Position, square: Square, by: Side) -> bool {
        if by == Side::White {
            (self.get_b_pawn_attacks(square) & pos.w_pawns).is_not_empty()
                || (self.get_knight_attacks(square) & pos.w_knights).is_not_empty()
                || (self.get_bishop_attacks(square, pos.all_occupancies) & pos.w_bishops)
                    .is_not_empty()
                || (self.get_rook_attacks(square, pos.all_occupancies) & pos.w_rooks).is_not_empty()
                || (self.get_queen_attacks(square, pos.all_occupancies) & pos.w_queens)
                    .is_not_empty()
                || (self.get_king_attacks(square) & pos.w_king).is_not_empty()
        } else {
            (self.get_w_pawn_attacks(square) & pos.b_pawns).is_not_empty()
                || (self.get_knight_attacks(square) & pos.b_knights).is_not_empty()
                || (self.get_bishop_attacks(square, pos.all_occupancies) & pos.b_bishops)
                    .is_not_empty()
                || (self.get_rook_attacks(square, pos.all_occupancies) & pos.b_rooks).is_not_empty()
                || (self.get_queen_attacks(square, pos.all_occupancies) & pos.b_queens)
                    .is_not_empty()
                || (self.get_king_attacks(square) & pos.b_king).is_not_empty()
        }
    }

    pub fn print_attacked_squares(&self, pos: &Position, side: Side) {
        for r in (0..8).rev() {
            print!(" {}", r + 1);
            for f in 0..8 {
                let square = Square::from_fr_unchecked(f, r);
                let is_attacked = if self.is_square_attacked(pos, square, side) {
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

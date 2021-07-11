use crate::{
    attack_tables::{
        bishops::gen_bishop_attacks,
        magics::{
            bishop::{BISHOP_MAGIC_NUMBERS, BISHOP_RELEVANT_BITS},
            rook::{ROOK_MAGIC_NUMBERS, ROOK_RELEVANT_BITS},
        },
        rooks::gen_rook_attacks,
    },
    bitboard::BitBoard,
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

pub struct AttackTables {
    w_pawn: Box<[BitBoard; 64]>,
    b_pawn: Box<[BitBoard; 64]>,
    knight: Box<[BitBoard; 64]>,
    king: Box<[BitBoard; 64]>,
    bishop_masks: Box<[BitBoard; 64]>,
    rook_masks: Box<[BitBoard; 64]>,
    bishop_attacks: Box<[[BitBoard; 512]; 64]>,
    rook_attacks: Box<[[BitBoard; 4096]]>,
}

impl AttackTables {
    pub fn gen_attacks() -> AttackTables {
        // Leapers
        let mut w_pawn = Box::new([BitBoard::empty(); 64]);
        let mut b_pawn = Box::new([BitBoard::empty(); 64]);
        let mut knight = Box::new([BitBoard::empty(); 64]);
        let mut king = Box::new([BitBoard::empty(); 64]);

        // Sliders
        let mut bishop_masks = Box::new([BitBoard::empty(); 64]);
        let mut rook_masks = Box::new([BitBoard::empty(); 64]);
        let mut bishop_attacks = Box::new([[BitBoard::empty(); 512]; 64]);
        let mut rook_attacks = vec![[BitBoard::empty(); 4096]; 64].into_boxed_slice();

        for square in SQUARES {
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
        }

        AttackTables {
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
}

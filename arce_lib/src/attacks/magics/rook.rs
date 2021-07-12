use crate::{
    attacks::rooks::{gen_rook_attacks, mask_rook_attacks},
    bitboard::BitBoard,
    square::Square,
    utils::random::RAND,
};

pub const ROOK_RELEVANT_BITS: [u8; 64] = [
    12, 11, 11, 11, 11, 11, 11, 12, 11, 10, 10, 10, 10, 10, 10, 11, 11, 10, 10, 10, 10, 10, 10, 11,
    11, 10, 10, 10, 10, 10, 10, 11, 11, 10, 10, 10, 10, 10, 10, 11, 11, 10, 10, 10, 10, 10, 10, 11,
    11, 10, 10, 10, 10, 10, 10, 11, 12, 11, 11, 11, 11, 11, 11, 12,
];

// Generated from print_init_magic
#[allow(clippy::unreadable_literal)]
pub const ROOK_MAGIC_NUMBERS: [u64; 64] = [
    0x8a80104000800020,
    0x140002000100040,
    0x2801880a0017001,
    0x100081001000420,
    0x200020010080420,
    0x3001c0002010008,
    0x8480008002000100,
    0x2080088004402900,
    0x800098204000,
    0x2024401000200040,
    0x100802000801000,
    0x120800800801000,
    0x208808088000400,
    0x2802200800400,
    0x2200800100020080,
    0x801000060821100,
    0x80044006422000,
    0x100808020004000,
    0x12108a0010204200,
    0x140848010000802,
    0x481828014002800,
    0x8094004002004100,
    0x4010040010010802,
    0x20008806104,
    0x100400080208000,
    0x2040002120081000,
    0x21200680100081,
    0x20100080080080,
    0x2000a00200410,
    0x20080800400,
    0x80088400100102,
    0x80004600042881,
    0x4040008040800020,
    0x440003000200801,
    0x4200011004500,
    0x188020010100100,
    0x14800401802800,
    0x2080040080800200,
    0x124080204001001,
    0x200046502000484,
    0x480400080088020,
    0x1000422010034000,
    0x30200100110040,
    0x100021010009,
    0x2002080100110004,
    0x202008004008002,
    0x20020004010100,
    0x2048440040820001,
    0x101002200408200,
    0x40802000401080,
    0x4008142004410100,
    0x2060820c0120200,
    0x1001004080100,
    0x20c020080040080,
    0x2935610830022400,
    0x44440041009200,
    0x280001040802101,
    0x2100190040002085,
    0x80c0084100102001,
    0x4024081001000421,
    0x20030a0244872,
    0x12001008414402,
    0x2006104900a0804,
    0x1004081002402,
];

pub fn find_rook_magic_number(square: Square, relevant_bits: u8) -> u64 {
    let mut occupancies = [BitBoard::empty(); 4096];
    let mut attacks = [BitBoard::empty(); 4096];
    let mut used_attacks: [BitBoard; 4096];

    let attack_mask = mask_rook_attacks(square);

    let occupancy_indices = 1 << relevant_bits;

    for index in 0..occupancy_indices {
        occupancies[index] = attack_mask.set_occupancy(index as u32, relevant_bits);
        attacks[index] = gen_rook_attacks(square, occupancies[index]);
    }

    for _ in 0..100_000_000 {
        // Generate candidate
        let magic_number = RAND.with(|r| r.borrow_mut().gen_magic_number());
        // Skip if invalid
        if BitBoard((attack_mask.0.wrapping_mul(magic_number)) & 0xFF00000000000000).count_bits()
            < 6
        {
            continue;
        }

        used_attacks = [BitBoard::empty(); 4096];

        // Test magic index loop
        let mut index = 0;
        let mut fail = false;
        while !fail && index < occupancy_indices {
            let magic_index =
                (occupancies[index].0.wrapping_mul(magic_number)) >> (64 - relevant_bits);

            // If magic index works
            if used_attacks[magic_index as usize] == BitBoard::empty() {
                used_attacks[magic_index as usize] = attacks[index as usize];
            } else if used_attacks[magic_index as usize] != attacks[index as usize] {
                fail = true;
            }
            index += 1;
        }

        if !fail {
            return magic_number;
        }
    }
    println!("find_rook_magic_number failed!");
    0
}

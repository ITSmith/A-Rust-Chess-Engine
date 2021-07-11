use crate::{
    attacks::bishops::{gen_bishop_attacks, mask_bishop_attacks},
    bitboard::BitBoard,
    random::RAND,
    square::Square,
};

pub const BISHOP_RELEVANT_BITS: [u8; 64] = [
    6, 5, 5, 5, 5, 5, 5, 6, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 7, 7, 7, 7, 5, 5, 5, 5, 7, 9, 9, 7, 5, 5,
    5, 5, 7, 9, 9, 7, 5, 5, 5, 5, 7, 7, 7, 7, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 6, 5, 5, 5, 5, 5, 5, 6,
];

// Generated from print_init_magic
#[allow(clippy::unreadable_literal)]
pub const BISHOP_MAGIC_NUMBERS: [u64; 64] = [
    0x40040844404084,
    0x2004208a004208,
    0x10190041080202,
    0x108060845042010,
    0x581104180800210,
    0x2112080446200010,
    0x1080820820060210,
    0x3c0808410220200,
    0x4050404440404,
    0x21001420088,
    0x24d0080801082102,
    0x1020a0a020400,
    0x40308200402,
    0x4011002100800,
    0x401484104104005,
    0x801010402020200,
    0x400210c3880100,
    0x404022024108200,
    0x810018200204102,
    0x4002801a02003,
    0x85040820080400,
    0x810102c808880400,
    0xe900410884800,
    0x8002020480840102,
    0x220200865090201,
    0x2010100a02021202,
    0x152048408022401,
    0x20080002081110,
    0x4001001021004000,
    0x800040400a011002,
    0xe4004081011002,
    0x1c004001012080,
    0x8004200962a00220,
    0x8422100208500202,
    0x2000402200300c08,
    0x8646020080080080,
    0x80020a0200100808,
    0x2010004880111000,
    0x623000a080011400,
    0x42008c0340209202,
    0x209188240001000,
    0x400408a884001800,
    0x110400a6080400,
    0x1840060a44020800,
    0x90080104000041,
    0x201011000808101,
    0x1a2208080504f080,
    0x8012020600211212,
    0x500861011240000,
    0x180806108200800,
    0x4000020e01040044,
    0x300000261044000a,
    0x802241102020002,
    0x20906061210001,
    0x5a84841004010310,
    0x4010801011c04,
    0xa010109502200,
    0x4a02012000,
    0x500201010098b028,
    0x8040002811040900,
    0x28000010020204,
    0x6000020202d0240,
    0x8918844842082200,
    0x4010011029020020,
];

pub fn find_bishop_magic_number(square: Square, relevant_bits: u8) -> u64 {
    let mut occupancies = [BitBoard::empty(); 512];
    let mut attacks = [BitBoard::empty(); 512];
    let mut used_attacks: [BitBoard; 512];

    let attack_mask = mask_bishop_attacks(square);

    let occupancy_indices = 1 << relevant_bits;

    for index in 0..occupancy_indices {
        occupancies[index] = attack_mask.set_occupancy(index as u32, relevant_bits);
        attacks[index] = gen_bishop_attacks(square, occupancies[index]);
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

        used_attacks = [BitBoard::empty(); 512];

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
    println!("find_bishop_magic_number failed!");
    0
}

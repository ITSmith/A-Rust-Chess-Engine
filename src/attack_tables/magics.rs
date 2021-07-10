pub mod bishop;
pub mod rook;

use crate::{
    attack_tables::magics::{
        bishop::{find_bishop_magic_number, BISHOP_RELEVANT_BITS},
        rook::{find_rook_magic_number, ROOK_RELEVANT_BITS},
    },
    square::SQUARES,
};

// Used to pre-generate magic numbers
#[allow(unused)]
pub(crate) fn print_init_magic_numbers() {
    println!("Rook magic numbers:");
    for square in SQUARES {
        println!(
            "{:#x},",
            find_rook_magic_number(square, ROOK_RELEVANT_BITS[square as usize])
        );
    }
    println!("\nBishop Magic numbers:");
    for square in SQUARES {
        println!(
            "{:#x},",
            find_bishop_magic_number(square, BISHOP_RELEVANT_BITS[square as usize])
        );
    }
}

#[cfg(test)]
mod test {
    use std::convert::TryInto;

    use crate::{
        attack_tables::magics::{
            bishop::{find_bishop_magic_number, BISHOP_RELEVANT_BITS},
            rook::{find_rook_magic_number, ROOK_MAGIC_NUMBERS, ROOK_RELEVANT_BITS},
        },
        square::SQUARES,
    };

    use super::bishop::BISHOP_MAGIC_NUMBERS;

    #[test]
    fn test_magics() {
        for square in SQUARES {
            assert_eq!(
                ROOK_MAGIC_NUMBERS[square as usize],
                find_rook_magic_number(
                    (square as u8).try_into().unwrap(),
                    ROOK_RELEVANT_BITS[square as usize]
                )
            );
        }
        println!("\nBishop Magic numbers:");
        for square in SQUARES {
            assert_eq!(
                BISHOP_MAGIC_NUMBERS[square as usize],
                find_bishop_magic_number(
                    (square as u8).try_into().unwrap(),
                    BISHOP_RELEVANT_BITS[square as usize]
                )
            );
        }
    }
}

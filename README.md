# ARCE (A Rust Chess Engine)
This chess engine is based off information gathered from the [Chess Programming Wiki](https://www.chessprogramming.org/Main_Page) and a C chess engine [tutorial](https://www.youtube.com/playlist?list=PLmN0neTso3Jxh8ZIylk74JpwfiWNI76Cs).

## Board Representation
The engine uses a [bitboard](https://www.chessprogramming.org/Bitboards) based board representation. The [Position](arce_lib/src/position.rs) struct contains twelve piece bitboards (one for each type of piece (white king, black pawns, etc)), and three bitboards for white, black, and all pieces. This allows it to efficiently perform certain operations, such as checkingfor attacked pieces, in O(1) time. It also allows the use of [magic bitboards](https://www.chessprogramming.org/Magic_Bitboards) to generate sliding piece attacks (bishop, rook, queen) in O(1) time. These techniques use more memory than some other approaches (due to storing about 2.4MB of attack tables in memory) in return for much faster move generation.

## To Do
This list is non-exhaustive and not necessarilly in order.

 - [ ] Better comments and documentation
 - [ ] Tests and performance benchmarking
 - [ ] Quiescence search
 - [ ] Better evaluation heuristics

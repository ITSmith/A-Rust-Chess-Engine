# ARCE (A Rust Chess Engine)
This currently incomplete chess engine is loosely based off of a C chess engine [tutorial](https://www.youtube.com/playlist?list=PLmN0neTso3Jxh8ZIylk74JpwfiWNI76Cs). The [Chess Programming Wiki](https://www.chessprogramming.org/Main_Page) was invaluable for understanding certain aspects of engine programming.

## Board Representation
The engine uses a [bitboard](https://www.chessprogramming.org/Bitboards) based board representation. The [Board](src/board.rs) struct contains twelve piece bitboards (one for each type of piece (white king, black pawns, etc)), and three bitboards for white, black, and all pieces. This allows it to efficiently perform certain operations, such as checkingfor attacked pieces, in O(1) time. It also allows the use of [magic bitboards](https://www.chessprogramming.org/Magic_Bitboards) to generate sliding piece attacks (bishop, rook, queen) in O(1) time. These techniques use more memory than some other approaches (due to storing about 2.4MB of attack tables in memory) in return for much faster move generation.

## To Do
These lists are non-exhaustive and not necessarilly in order.

### Features necessary for a working engine:
 - [x] Move generation
 - [x] Move making/undoing
 - [ ] Move evaluation
 - [ ] Move searching
 - [ ] UCI interface (I/O)

### "Advanced" Features
 - [ ] Performance benchmarking
 - [ ] Quiescence search
 - [ ] Better evaluation heuristics

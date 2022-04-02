# Inanis
UCI chess engine written in Rust, proud successor of [Proxima b](https://github.com/Tearth/Proxima-b), [Proxima b 2.0](https://github.com/Tearth/Proxima-b-2.0) and [Cosette](https://github.com/Tearth/Cosette). The project is written after hours, ~~with the goal to reach 2600 Elo (or at least to be stronger than the last version of [Cosette](https://github.com/Tearth/Cosette) which was about 2500 Elo)~~ with the goal to reach 3000 Elo. Perfect as a sparring partner for other chess engines, since it was heavily tested using very fast games (5+0.1). Supports pondering and multithreading.

**Current strength**: 2750 Elo (22.03.2022)

**Documentation**: https://tearth.github.io/Inanis

## Releases
| Version                                                                | Release date | Elo    | Description  |
|------------------------------------------------------------------------|--------------|--------|--------------|
| *soon*                                                                 | *soon*       | *soon* | *soon*       |

Each release contains a set of binaries for various platforms: Linux (x86, x86-64 ARM, AArch64) and Windows (x86, x86-64). Both Linux x86-64 and Windows x86-64 were also compiled with two additional instruction set variants: POPCNT and POPCNT + BMI1 + BMI2 - to get the best performance, please try to run the `benchmark` command using different engine's variants and choose the one which didn't return an error and has the most advanced instructions.

## Rating lists
*soon*

## How to play online
Inanis has an official lichess account, where you can try to challenge the engine: https://lichess.org/@/InanisBot. Please note that ratings there are very understated and not comparable to CCRL ones. Accepts standard chess with a bullet, blitz, rapid, and classic time control (up to 3 games at a time).

## UCI options
 - `Hash` - a total size (in megabytes) for transposition table and pawn hashtable
 - `Move Overhead` - amount of time (in milliseconds) that should be reserved during a search for some unexpected delays (like the slowness of GUI or network lags)
 - `Threads` - number of threads to use during search (should be less than a number of processor cores to get the best performance)
 - `Ponder` - allows the engine to think during the opponent's time

## Algorithms
 - **board representation**: bitboards (a hybrid of make/undo scheme and storing data on stacks)
 - **move generator**: staged (hash move, captures, quiet moves), magic bitboards, precalculated arrays for knight and king
 - **move ordering**: hash move, good captures (SEE with support for x-ray attacks), killer/history table (with random noise if Lazy SMP is enabled), bad captures
 - **search**: negamax, alpha-beta pruning, quiescence search, null-move pruning, static null move pruning, razoring, extension pruning, late move reduction, late move pruning, lazy SMP
 - **cache**: transposition table, pawn hashtable
 - **evaluation**: material, piece-square tables, pawn structure, mobility, king safety

## Tuner
Inanis has a built-in tuner, which allows optimizing all evaluation parameters using a well-known [Texel's tuning method](https://www.chessprogramming.org/Texel%27s_Tuning_Method). As an output, there are Rust source files generated in a way that allows them to be directly pasted into the engine's source code. 

Example input file:
```
r2qkr2/p1pp1ppp/1pn1pn2/2P5/3Pb3/2N1P3/PP3PPP/R1B1KB1R b KQq - c9 "0-1";
r4rk1/3bppb1/p3q1p1/1p1p3p/2pPn3/P1P1PN1P/1PB1QPPB/1R3RK1 b - - c9 "1/2-1/2";
4Q3/8/8/8/6k1/4K2p/3N4/5q2 b - - c9 "0-1";
r4rk1/1Qpbq1bp/p1n2np1/3p1p2/3P1P2/P1NBPN1P/1P1B2P1/R4RK1 b - - c9 "0-1";
```

Examples of running the tuner:

 - `tuner ./input/quiet.epd ./output/ false false 1` - run single-threaded tuning (including piece values) for positions stored in `quiet.epd`, starting from the random values, and saving the result in the `output` directory

 - `tuner ./input/quiet.epd ./output/ true true 4` - run tuning with 4 threads (excluding piece values) for positions stored in `quiet.epd`, starting from the values already set in the engine, and saving the result in the `output` directory

## Test suites 
Testing of strategic evaluation performance can be done by using the `test` command, which performs a fixed-depth search for positions stored in the EPD file.

Example test suite file:
```
1k2r2r/1bq2p2/pn4p1/3pP3/pbpN1P1p/4QN1B/1P4PP/2RR3K b - - bm Nd7; c0 "Nd7=10, Bc5=8, Bc6=2, Be7=7"; id "STS: Knight Outposts/Repositioning/Centralization.001";
1q2bn2/6pk/2p1pr1p/2Q2p1P/1PP5/5N2/5PP1/4RBK1 w - - bm Ne5; c0 "Ne5=10, Nd4=8, Ra1=6, b5=9"; id "STS: Knight Outposts/Repositioning/Centralization.002";
1r1q1rk1/1b1n1p1p/p2b1np1/3pN3/3P1P2/P1N5/3BB1PP/1R1Q1RK1 b - - bm Ne4; c0 "Ne4=10, Bxa3=6, Nb6=6"; id "STS: Knight Outposts/Repositioning/Centralization.003";
1k2r2r/1bq2p2/pn4p1/3pP3/pbpN1P1p/4QN1B/1P4PP/2RR3K b - - bm Nd7; c0 "Nd7=10, Bc5=8, Bc6=2, Be7=7"; id "STS: Knight Outposts/Repositioning/Centralization.001";
1q2bn2/6pk/2p1pr1p/2Q2p1P/1PP5/5N2/5PP1/4RBK1 w - - bm Ne5; c0 "Ne5=10, Nd4=8, Ra1=6, b5=9"; id "STS: Knight Outposts/Repositioning/Centralization.002";
1r1q1rk1/1b1n1p1p/p2b1np1/3pN3/3P1P2/P1N5/3BB1PP/1R1Q1RK1 b - - bm Ne4; c0 "Ne4=10, Bxa3=6, Nb6=6"; id "STS: Knight Outposts/Repositioning/Centralization.003";
```

Examples of running the tests:

 - `test ./input/STS1.epd 16 5` - run fixed-depth (16 in this case) search for all positions stored in the `STS1.epd` file. If the best position is correct and repeats five times in a row, or the result of the last iteration was correct, stop the test and mark it as success

## Dependencies
 - [arr_macro](https://github.com/JoshMcguigan/arr_macro) - macro for easier array initialization
 - [fastrand](https://github.com/smol-rs/fastrand) - a simple and fast random number generator
 - [chrono](https://github.com/chronotope/chrono) - feature-complete superset of the time library
 - [bitflags](https://github.com/bitflags/bitflags) - macro to generate structures which behave like a set of bitflags
 - [prettytable-rs](https://github.com/phsym/prettytable-rs) - a formatted and aligned table printer library
 - [nameof](https://github.com/SilentByte/nameof) - macro which takes a binding, type, const, or function as an argument and returns its unqualified string representation
 - [crossbeam](https://github.com/crossbeam-rs/crossbeam) - a set of tools for concurrent programming
 - [criterion](https://github.com/bheisler/criterion.rs) - benchmark framework

## Contributing
Because Inanis is my pet project, I don't currently accept pull requests - this may or may not change in the future, depending on the way the project will go. However, feel free to make issues or suggestions, they are greatly appreciated. 

## Commands
```
=== General ===
 benchmark - run test for a set of positions
 evaluate [fen] - show score for the position
 magic - generate magic numbers
 test [epd] [depth] [tries_to_confirm] - run test of positions
 tuner [epd] [output] [lock_material] [randomize] [threads_count] - run tuning
 uci - run Universal Chess Interface
 quit - close the application

=== Perft ===
 perft [depth]
 perft [depth] fen [fen]
 perft [depth] moves [moves]

=== Divided Perft ===
 dperft [depth]
 dperft [depth] fen [fen]
 dperft [depth] moves [moves]

=== Quick Perft ===
 qperft [depth] [threads_count] [hashtable_size_mb]
 qperft [depth] [threads_count] [hashtable_size_mb] fen [fen]
 qperft [depth] [threads_count] [hashtable_size_mb] moves [moves]
```
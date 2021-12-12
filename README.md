# Inanis
Experimental UCI chess engine written in Rust, proud successor of [Proxima b](https://github.com/Tearth/Proxima-b), [Proxima b 2.0](https://github.com/Tearth/Proxima-b-2.0) and [Cosette](https://github.com/Tearth/Cosette). The project is written after hours for educational purposes, with the goal to reach 2600 Elo (or at least to be stronger than the last version of [Cosette](https://github.com/Tearth/Cosette) which was about 2500 Elo).

## How to play
At the current stage, it's too early to make any official releases, thus you have to compile the binary yourself using Rust toolkit. After this, use your favorite GUI client compatible with UCI protocol (the engine still doesn't support some commands, but should be playable in typical games). Also, I expect in the future to make the Lichess account to make it more available.

## Algorithms
 - **board representation**: bitboards (hybrid of make/undo scheme and storing data on stacks)
 - **move generator**: magic bitboards, precalculated arrays for knight and king
 - **move ordering**: hash move, good captures (SEE with support for x-ray attacks), killer/history table
 - **search**: negamax, alpha-beta pruning, quiescence search, null-move pruning, static null move pruning, razoring, late move reduction
 - **cache**: transposition table, pawn hash table
 - **evaluation**: material, piece-square table, pawn structure, mobility, king safety

## Commands
```
=== General ===
 benchmark - run test for a set of positions
 evaluate [fen] - show score for the position
 magic - generate magic numbers
 test [epd] [depth] [tries_to_confirm] - run test of positions
 tuner [epd] [output] [lock_material] [randomize] - run tuning
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

## Inspirational music
[![Violet](https://img.youtube.com/vi/8ZdLXELdF9Q/0.jpg)](https://www.youtube.com/watch?v=8ZdLXELdF9Q)
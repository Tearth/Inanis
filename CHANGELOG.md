# Version 1.5.0 (01-11-2024)
 - Added aspiration windows
 - Added support for tuning based on evaluation
 - Added "k" and "wdl_ratio" parameters to "tuner" command
 - Added prefetching of transposition table entries
 - Added packed evaluation
 - Added pawn structure to fast evaluation
 - Removed "lock material" parameter from "tuner" command
 - Removed "Crash Files" UCI option from the release version
 - Removed Lazy SMP artificial noise
 - Removed allocator, pawn hashtable has fixed size now
 - Improved tuner performance and excessive memory usage
 - Improved tuner output by setting unused parameters to zero
 - Improved indexing of transposition, pawn and perft tables
 - Improved rand algorithm (change from xorshift to xorshift*)
 - Improved engine strength when using multiple threads
 - Improved evaluation parameters
 - Improved overall performance
 - Incorporated piece values in PST
 - Fixed endless search in fixed-nodes mode
 - Fixed tuner output filename
 - Fixed crash when parsing certain FENs
 - Fixed incorrect workload between threads in tuner
 - Fixed incorrect evaluation of black pawn chains

**Strength**: 3000 Elo

# Version 1.4.0 (03-08-2024)
 - Added relative PST
 - Added check extensions
 - Added countermove heuristic
 - Added simplified benchmark when "dev" feature is not enabled
 - Added history table penalties and reduced aging divisor
 - Added non-standard "fen" command to UCI
 - Added crash when the best move is invalid (only in dev version)
 - Added pawn attacks cache
 - Added support for LZCNT instruction
 - Improved evaluation parameters by using a new dataset for tuning
 - Improved search parameters
 - Improved header, now it also includes LLVM version, target, profile and enabled features
 - Renamed "tunerset" command to "dataset"
 - Merged "bindgen" and "syzygy" features
 - Fixed rare bug with invalid moves when a search was aborted
 - Fixed crash when ply is larger than the killer table size
 - Fixed performance overhead of setting a new position

**Strength**: 2950 Elo

# Version 1.3.0 (14-06-2024)
 - Added search parameters as UCI options (only if the "dev" feature is enabled)
 - Added gradient descent tuner in place of local search
 - Added internal iterative reduction
 - Added bishop pair evaluation
 - Removed "avg_game_phase" in "tunerset" command
 - Removed "magic", "testset", "tuner" and "tunerset" commands from the release builds
 - Improved king safety evaluation
 - Improved quality of tunerset output
 - Improved search parameters
 - Improved pawn structure evaluation
 - Improved mobility evaluation by excluding squares attacked by enemy pawns
 - Fixed invalid position score when both kings are checked
 - Fixed incorrect SEE results for sliding pieces 

**Strength**: 2900 Elo

# Version 1.2.1 (04-09-2023)
 - Added executing commands directly from a command line
 - Added perft in UCI mode (go perft)

# Version 1.2.0 (15-01-2023)
 - Added integration with Fathom library to better support Syzygy tablebases
 - Added "tbhits" to the search output
 - Added "avg_game_phase" parameter to "tunerset" command
 - Added "syzygy" and "bindgen" as switchable Cargo features
 - Added information about captures, en passants, castles, promotions and checks in perft's output
 - Added attackers/defenders cache
 - Added killer moves as separate move generator phase
 - Removed unnecessary check detection in null move pruning
 - Removed redundant abort flag check
 - Removed underpromotions in qsearch
 - Reduced binary size by removing dependencies and replacing them with custom implementation
 - Renamed "test" command to "testset"
 - Simplified evaluation by reducing the number of score taperings 
 - Improved build process
 - Improved benchmark output
 - Improved allocation of all hashtables, now their size will always be a power of 2 for better performance
 - Improved king safety evaluation by taking a number of attacked adjacent fields more serious
 - Improved overall performance by a lot of minor refactors and adjustments
 - Improved game phase evaluation
 - Improved killer heuristic
 - Improved history table aging
 - Improved reduction formula in null move pruning
 - Fixed a few "tunerset" command bugs related to the game phase
 - Fixed PGN parser when there were no spaces between dots and moves
 - Fixed invalid evaluation of doubled passing pawns
 - Fixed invalid cut-offs statistics
 - Fixed qsearch failing hard instead of failing soft

**Strength**: 2850 Elo

# Version 1.1.1 (14-08-2022)
 - Added support for FEN property in PGN parser and "tunerset" command
 - Replaced crossbeam package with native scoped threads
 - Fixed invalid handling of "isready" UCI command during a search
 - Fixed engine crash when trying to search invalid position
 - Fixed incorrect version of toolchain used in GitHub Actions

**No change in Elo strength** 

# Version 1.1.0 (31-07-2022)
 - Added support for Syzygy tablebases
 - Added support for "MultiPV" UCI option
 - Added support for "searchmoves" in "go" UCI command
 - Added "hashfull" in the UCI search output
 - Added "tunerset" command
 - Added "transposition_table_size" and "threads_count" parameters to "test" command
 - Added instant move when there is only one possible in the position
 - Added new benchmarks
 - Added tuner dataset generator
 - Added information about the compiler and a list of target features at the startup
 - Added diagnostic mode in search functions to gather statistics only if necessary
 - Added a simple PGN parser
 - Removed "tries_to_confirm" parameter from "test" command
 - Removed arr_macro crate from dependencies
 - Improved mobility evaluation, now the parameters are defined per piece instead of one value for all
 - Improved null move reduction formula, now should be more aggressive
 - Improved null move pruning, now it shouldn't be tried for hopeless positions
 - Improved make-undo scheme performance
 - Improved release script, now it's shorter and more flexible
 - Improved error messages and made them more detailed
 - Improved repetition draw detection
 - Increased late move pruning max depth
 - Increased amount of memory allocated for pawn hashtable
 - Adjusted evaluation parameters
 - Made LMR less aggressive in PV nodes
 - Made aging in the transposition table faster and more reliable
 - Merged reduction pruning with late move pruning
 - Decreased memory usage during tuner work
 - Deferred evaluation of evasion mask
 - Reduced amount of lazy evaluations
 - Reduced amount of locks in the UCI interface
 - Removed duplicated search calls in the PVS framework
 - Fixed crash when "tuner" command had not enough parameters
 - Fixed crash when FEN didn't have information about halfmove clock and move number
 - Fixed crash when search in ponder mode was trying to be started in already checkmated position
 - Fixed tuner and tester not being able to examine all positions when multithreading is enabled
 - Fixed draw detection issue caused by transposition table
 - Fixed undefined behaviors and reduced the amount of unsafe code
 - Fixed incorrect benchmark statistics
 - Fixed a few edge cases in the short algebraic notation parser

**Strength**: 2800 Elo

# Version 1.0.1 (05-04-2022)
 - Added a new UCI option "Crash Files" (disabled by default)
 - Fixed move legality check which in rare cases was leading to engine crashes
 - Fixed PV lines being too long due to endless repetitions

**No change in Elo strength** 

# Version 1.0.0 (02-04-2022)
 - Initial release
 
**Strength**: 2750 Elo
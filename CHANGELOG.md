# Version 1.1.0 (--.--.----)
 - added support for Syzygy tablebases
 - added support for MultiPV UCI option
 - added support for multithreading in "test" command
 - added support for "searchmoves" in "go" UCI command
 - added "tunerset" command
 - added transposition_table_size parameter to "test" command
 - added instant move when there is only one possible
 - added new benchmarks
 - added tuner dataset generator
 - added information about the compiler and a list of target features at the startup
 - added diagnostic mode in search functions to gather statistics only if necessary
 - added hashfull in search output
 - added simple PGN parser
 - removed "tries_to_confirm" parameter from "test" command
 - removed arr_macro crate from dependencies
 - improved mobility evaluation, now the parameters are defined per piece instead of one value for all
 - improved null move reduction formula, now should be more aggressive
 - improved null move pruning, now it shouldn't be tried for hopeless positions
 - improved make-undo scheme performance
 - improved release scripts, now it's shorter and more flexible
 - improved error messages and made them more detailed
 - improved repetition draw detection
 - increased late move pruning max depth
 - increased memory amount allocated for pawn hashtable
 - adjusted evaluation parameters
 - made LMR less aggressive in PV nodes
 - made aging in the transposition table faster and more reliable
 - merged reduction pruning with late move pruning
 - decreased memory usage during tuner work
 - deferred evaluation of evasion mask
 - reduced amount of lazy evaluations
 - reduced amount of locks in UCI interface
 - removed duplicated search calls in PVS framework
 - fixed tuner and tester not being able to examine all positions when multithreading is enabled
 - fixed crash when "tuner" command had not enough parameters
 - fixed draw detection issue caused by transposition table
 - fixed crash when FEN didn't have information about halfmove clock and move number
 - fixed undefined behaviors and reduced the amount of unsafe code
 - fixed incorrect benchmark statistics
 - fixed a few edge cases in the short algebraic notation parser

**Strength**: 2800 Elo

# Version 1.0.1 (05-04-2022)
 - Added a new UCI option "Crash Files" (disabled by default)
 - Fixed move legality check which in rare cases was leading to engine crashes
 - Fixed PV lines being too long due to endless repetitions

**No change in Elo strength** 

# Version 1.0.0 (02-04-2022)
 - Initial release
 
**Strength**: 2750 Elo
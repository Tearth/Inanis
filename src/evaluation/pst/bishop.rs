// ------------------------------------------------------------------------- //
// Generated at 19-11-2024 23:40:19 UTC (e = 0.113270, k = 0.0077, r = 1.00) //
// ------------------------------------------------------------------------- //

use super::*;

#[rustfmt::skip]
pub const BISHOP_PST_PATTERN: [[[PackedEval; 64]; KING_BUCKETS_COUNT]; 2] =
[
    [
        [
            s!( 290,  296), s!( 342,  280), s!( 287,  307), s!( 308,  320), s!( 300,  328), s!( 306,  304), s!( 330,  288), s!( 321,  276),
            s!( 304,  298), s!( 327,  293), s!( 357,  297), s!( 343,  302), s!( 362,  298), s!( 360,  296), s!( 327,  311), s!( 279,  295),
            s!( 340,  301), s!( 354,  307), s!( 357,  298), s!( 378,  306), s!( 375,  309), s!( 431,  301), s!( 369,  320), s!( 368,  314),
            s!( 332,  301), s!( 356,  290), s!( 367,  300), s!( 393,  295), s!( 404,  286), s!( 383,  302), s!( 368,  313), s!( 348,  306),
            s!( 336,  289), s!( 348,  290), s!( 358,  318), s!( 372,  293), s!( 382,  294), s!( 370,  305), s!( 360,  293), s!( 366,  292),
            s!( 336,  288), s!( 349,  297), s!( 361,  314), s!( 363,  301), s!( 369,  319), s!( 368,  306), s!( 374,  287), s!( 369,  282),
            s!( 351,  273), s!( 342,  264), s!( 354,  282), s!( 348,  289), s!( 336,  297), s!( 358,  288), s!( 380,  280), s!( 379,  264),
            s!( 320,  271), s!( 359,  265), s!( 357,  253), s!( 344,  293), s!( 362,  295), s!( 356,  284), s!( 376,  257), s!( 329,  264),
        ],
        [
            s!( 318,  299), s!( 328,  295), s!( 311,  317), s!( 307,  321), s!( 302,  322), s!( 298,  323), s!( 338,  324), s!( 310,  288),
            s!( 302,  304), s!( 334,  296), s!( 342,  303), s!( 344,  311), s!( 358,  310), s!( 319,  315), s!( 298,  323), s!( 289,  303),
            s!( 347,  317), s!( 355,  317), s!( 376,  312), s!( 399,  288), s!( 388,  303), s!( 422,  315), s!( 372,  333), s!( 352,  327),
            s!( 344,  312), s!( 354,  317), s!( 364,  315), s!( 386,  309), s!( 378,  302), s!( 367,  318), s!( 352,  319), s!( 347,  314),
            s!( 356,  299), s!( 337,  312), s!( 364,  321), s!( 364,  315), s!( 381,  298), s!( 359,  320), s!( 353,  306), s!( 368,  290),
            s!( 341,  308), s!( 365,  313), s!( 363,  320), s!( 365,  317), s!( 364,  330), s!( 382,  304), s!( 374,  293), s!( 373,  300),
            s!( 370,  282), s!( 344,  298), s!( 364,  294), s!( 340,  307), s!( 357,  300), s!( 376,  295), s!( 398,  288), s!( 379,  244),
            s!( 337,  275), s!( 368,  270), s!( 346,  282), s!( 357,  306), s!( 347,  294), s!( 379,  286), s!( 349,  274), s!( 359,  259),
        ],
        [
            s!( 295,  294), s!( 339,  304), s!( 298,  327), s!( 308,  332), s!( 294,  330), s!( 313,  309), s!( 333,  316), s!( 324,  277),
            s!( 295,  301), s!( 338,  306), s!( 356,  317), s!( 334,  320), s!( 358,  320), s!( 339,  315), s!( 320,  313), s!( 282,  302),
            s!( 325,  319), s!( 368,  315), s!( 378,  312), s!( 394,  304), s!( 386,  311), s!( 441,  312), s!( 401,  315), s!( 358,  318),
            s!( 331,  320), s!( 380,  306), s!( 364,  320), s!( 407,  298), s!( 400,  302), s!( 406,  299), s!( 376,  312), s!( 376,  319),
            s!( 366,  310), s!( 361,  315), s!( 388,  307), s!( 403,  299), s!( 397,  296), s!( 379,  316), s!( 355,  307), s!( 382,  299),
            s!( 335,  310), s!( 383,  306), s!( 391,  315), s!( 383,  313), s!( 380,  320), s!( 411,  300), s!( 379,  297), s!( 363,  301),
            s!( 387,  288), s!( 361,  296), s!( 369,  302), s!( 365,  313), s!( 365,  298), s!( 372,  303), s!( 379,  288), s!( 359,  285),
            s!( 313,  291), s!( 364,  293), s!( 358,  288), s!( 345,  313), s!( 342,  307), s!( 370,  297), s!( 329,  282), s!( 334,  280),
        ],
        [
            s!( 275,  296), s!( 319,  309), s!( 280,  333), s!( 304,  328), s!( 282,  337), s!( 305,  323), s!( 333,  294), s!( 308,  283),
            s!( 291,  307), s!( 331,  309), s!( 318,  327), s!( 326,  342), s!( 337,  326), s!( 297,  326), s!( 336,  308), s!( 290,  310),
            s!( 352,  315), s!( 368,  319), s!( 355,  321), s!( 372,  319), s!( 357,  332), s!( 395,  323), s!( 358,  326), s!( 349,  323),
            s!( 326,  323), s!( 350,  324), s!( 358,  326), s!( 354,  313), s!( 360,  309), s!( 356,  323), s!( 346,  316), s!( 343,  322),
            s!( 359,  300), s!( 321,  322), s!( 361,  319), s!( 361,  315), s!( 372,  299), s!( 352,  322), s!( 352,  322), s!( 368,  290),
            s!( 332,  317), s!( 367,  309), s!( 368,  326), s!( 371,  318), s!( 358,  335), s!( 378,  307), s!( 366,  310), s!( 347,  314),
            s!( 381,  298), s!( 348,  306), s!( 374,  302), s!( 342,  316), s!( 364,  299), s!( 374,  297), s!( 375,  287), s!( 368,  291),
            s!( 354,  292), s!( 373,  295), s!( 325,  301), s!( 337,  318), s!( 357,  311), s!( 340,  286), s!( 359,  286), s!( 307,  282),
        ],
        [
            s!( 291,  285), s!( 341,  294), s!( 295,  320), s!( 309,  335), s!( 294,  330), s!( 316,  332), s!( 335,  292), s!( 320,  290),
            s!( 305,  289), s!( 336,  323), s!( 323,  327), s!( 331,  325), s!( 357,  319), s!( 357,  311), s!( 327,  313), s!( 288,  311),
            s!( 342,  318), s!( 375,  323), s!( 398,  322), s!( 371,  323), s!( 386,  307), s!( 427,  320), s!( 362,  314), s!( 383,  309),
            s!( 337,  320), s!( 384,  318), s!( 387,  320), s!( 386,  310), s!( 387,  298), s!( 394,  311), s!( 374,  320), s!( 357,  318),
            s!( 370,  313), s!( 355,  311), s!( 391,  318), s!( 395,  296), s!( 368,  306), s!( 384,  308), s!( 382,  323), s!( 369,  297),
            s!( 358,  314), s!( 373,  317), s!( 394,  312), s!( 407,  308), s!( 393,  323), s!( 397,  308), s!( 364,  304), s!( 362,  303),
            s!( 380,  306), s!( 365,  292), s!( 365,  307), s!( 385,  308), s!( 372,  307), s!( 373,  294), s!( 350,  306), s!( 364,  281),
            s!( 329,  295), s!( 373,  301), s!( 345,  309), s!( 351,  313), s!( 351,  300), s!( 345,  297), s!( 343,  294), s!( 325,  293),
        ],
        [
            s!( 295,  295), s!( 353,  309), s!( 292,  312), s!( 301,  315), s!( 294,  316), s!( 306,  306), s!( 335,  295), s!( 318,  263),
            s!( 290,  301), s!( 330,  322), s!( 325,  310), s!( 337,  321), s!( 343,  303), s!( 356,  308), s!( 329,  294), s!( 314,  297),
            s!( 315,  322), s!( 374,  327), s!( 372,  333), s!( 388,  302), s!( 392,  309), s!( 410,  305), s!( 381,  326), s!( 389,  295),
            s!( 345,  323), s!( 353,  335), s!( 352,  341), s!( 388,  308), s!( 386,  293), s!( 387,  307), s!( 367,  317), s!( 341,  309),
            s!( 359,  302), s!( 352,  322), s!( 363,  322), s!( 392,  308), s!( 372,  299), s!( 364,  305), s!( 374,  308), s!( 358,  297),
            s!( 375,  310), s!( 383,  305), s!( 390,  312), s!( 379,  322), s!( 372,  325), s!( 360,  323), s!( 358,  315), s!( 343,  305),
            s!( 360,  282), s!( 400,  292), s!( 409,  282), s!( 374,  304), s!( 347,  311), s!( 365,  281), s!( 340,  317), s!( 315,  296),
            s!( 326,  296), s!( 338,  299), s!( 384,  302), s!( 345,  312), s!( 365,  298), s!( 350,  291), s!( 345,  292), s!( 315,  290),
        ],
        [
            s!( 295,  304), s!( 343,  300), s!( 285,  308), s!( 312,  322), s!( 292,  316), s!( 307,  303), s!( 328,  280), s!( 324,  281),
            s!( 279,  288), s!( 336,  329), s!( 352,  320), s!( 337,  300), s!( 355,  313), s!( 366,  292), s!( 336,  291), s!( 294,  303),
            s!( 317,  333), s!( 362,  315), s!( 370,  338), s!( 382,  309), s!( 386,  290), s!( 416,  293), s!( 376,  317), s!( 385,  313),
            s!( 343,  306), s!( 370,  318), s!( 376,  316), s!( 402,  298), s!( 399,  282), s!( 396,  294), s!( 359,  310), s!( 344,  302),
            s!( 363,  306), s!( 360,  301), s!( 376,  318), s!( 395,  294), s!( 380,  307), s!( 350,  325), s!( 356,  300), s!( 354,  297),
            s!( 367,  295), s!( 381,  314), s!( 383,  309), s!( 381,  318), s!( 360,  322), s!( 353,  330), s!( 347,  292), s!( 339,  294),
            s!( 351,  261), s!( 408,  270), s!( 399,  276), s!( 363,  300), s!( 342,  311), s!( 340,  297), s!( 324,  298), s!( 316,  280),
            s!( 335,  276), s!( 362,  285), s!( 377,  291), s!( 345,  304), s!( 364,  284), s!( 347,  282), s!( 331,  284), s!( 333,  281),
        ],
        [
            s!( 297,  292), s!( 342,  289), s!( 298,  312), s!( 308,  317), s!( 298,  331), s!( 310,  307), s!( 332,  290), s!( 324,  278),
            s!( 303,  291), s!( 347,  312), s!( 338,  320), s!( 332,  307), s!( 371,  322), s!( 371,  307), s!( 319,  290), s!( 282,  295),
            s!( 336,  320), s!( 356,  313), s!( 380,  305), s!( 384,  298), s!( 387,  292), s!( 431,  299), s!( 367,  301), s!( 353,  311),
            s!( 338,  317), s!( 373,  321), s!( 393,  312), s!( 424,  296), s!( 382,  279), s!( 384,  296), s!( 354,  307), s!( 354,  316),
            s!( 375,  301), s!( 372,  295), s!( 384,  293), s!( 401,  288), s!( 371,  284), s!( 361,  316), s!( 372,  296), s!( 332,  291),
            s!( 369,  302), s!( 382,  286), s!( 387,  308), s!( 370,  310), s!( 370,  307), s!( 356,  322), s!( 340,  295), s!( 342,  285),
            s!( 358,  273), s!( 391,  281), s!( 385,  296), s!( 355,  290), s!( 343,  295), s!( 335,  285), s!( 320,  279), s!( 344,  270),
            s!( 324,  274), s!( 366,  281), s!( 357,  293), s!( 374,  308), s!( 364,  287), s!( 330,  277), s!( 336,  292), s!( 329,  281),
        ],
    ],
    [
        [
            s!(   1,    2), s!(  -2,    2), s!(   5,   12), s!(   2,    2), s!(   5,    6), s!(   5,    4), s!(   4,    8), s!(  -0,    0),
            s!(   3,    2), s!(  -3,    7), s!(  -3,    2), s!(  10,    6), s!(  -7,   -4), s!(   1,   14), s!(  -5,   -4), s!( -14,    3),
            s!(   6,   10), s!(   6,   -5), s!( -15,    9), s!(   2,   -5), s!( -15,   12), s!(  -2,    9), s!(  -2,    2), s!(  -5,   16),
            s!( -10,    1), s!(   1,    2), s!(  -7,   -7), s!( -12,   18), s!(   8,    2), s!(   5,   -2), s!(   4,   -3), s!(  -5,    3),
            s!(  -8,   -1), s!(  -7,  -14), s!(  -7,    5), s!(  10,    1), s!(  -3,   12), s!(  -4,   -4), s!(   4,   -2), s!( -12,   -0),
            s!(  -9,  -10), s!(  -4,    5), s!(   3,    6), s!(  13,    2), s!(  -3,   -4), s!(   1,    5), s!( -13,    8), s!(  -3,    5),
            s!( -12,   -2), s!(   4,   -1), s!(   4,    3), s!(  -1,   -1), s!(   7,    7), s!( -12,    1), s!(  -5,    0), s!(  -8,  -18),
            s!(   2,   -7), s!(  10,    7), s!(  -3,  -15), s!(   6,   -0), s!(  -6,   -8), s!(   0,    4), s!(  -5,   -4), s!(  -7,   -0),
        ],
        [
            s!(  -6,    1), s!(  -3,   10), s!(  -1,   12), s!(   3,    6), s!(  -2,   -5), s!( -11,  -11), s!(   1,   -5), s!(   3,    0),
            s!( -10,   19), s!( -12,   16), s!(   3,   10), s!(  -8,   11), s!(  -9,   12), s!( -17,  -25), s!(  -8,   -9), s!( -17,  -11),
            s!( -10,    7), s!( -13,   19), s!(  -8,    9), s!( -14,    9), s!(  14,    7), s!(  12,    8), s!(  -8,   12), s!(   5,    2),
            s!(   6,    0), s!(  -5,    9), s!( -10,   10), s!(   2,    6), s!(  -9,    3), s!(  -7,   10), s!(  -8,   -2), s!(  -0,    3),
            s!(  -7,    7), s!(  -2,    4), s!(  -1,   14), s!(   3,    3), s!(  -9,    3), s!( -10,   -1), s!(  -7,   -2), s!(  -6,   -2),
            s!(   2,    2), s!(   4,    7), s!(  -1,    7), s!(   0,    6), s!(  -6,    9), s!( -10,    6), s!( -11,    3), s!(  -8,   11),
            s!(   2,   -8), s!(   7,   -5), s!(   5,   -7), s!(  -1,   -0), s!(  -4,    8), s!(  -8,    7), s!( -13,    2), s!( -10,    9),
            s!(   1,  -13), s!(   7,  -20), s!(   5,  -11), s!(  -2,    3), s!(   1,    4), s!(  -5,   -0), s!(   1,    0), s!(  -3,    7),
        ],
        [
            s!(   4,   11), s!(   2,    7), s!(   2,    6), s!(   0,    8), s!(  -1,    4), s!(   1,    1), s!(  -2,   -5), s!(  -0,   -2),
            s!(  10,   14), s!(  -9,    4), s!(  11,    3), s!(   3,   -4), s!(  -2,    0), s!( -14,   -6), s!(  -5,  -10), s!(   2,   -5),
            s!(   9,   -0), s!(  -0,    4), s!(   2,   -2), s!(   3,   -6), s!(  -1,  -11), s!(  12,    1), s!(  11,   -2), s!(   2,  -16),
            s!(  -5,    9), s!( -16,    3), s!(  19,    2), s!(   7,   -5), s!(  11,  -11), s!(   6,   -7), s!(   8,  -11), s!(  -6,    2),
            s!(   3,    3), s!(   2,   -3), s!(   1,   -3), s!(  21,   -4), s!( -13,   -3), s!(  12,   -8), s!(   3,   -1), s!(   3,    5),
            s!(  14,    3), s!(  14,   -8), s!(   7,   -4), s!(   1,    1), s!(   9,   -6), s!(   0,   -5), s!(  -9,    7), s!( -10,   -2),
            s!(  -7,  -10), s!(   5,   -6), s!(   3,   -6), s!(   5,   -2), s!(  -9,    3), s!(   6,    4), s!( -11,    3), s!(   5,    6),
            s!(  -9,  -10), s!(  -8,  -10), s!(   4,   -2), s!(  -7,    0), s!(   0,    5), s!( -12,    3), s!(  -5,   -3), s!(  -6,    5),
        ],
        [
            s!(   5,    1), s!(  -2,   -2), s!(   0,   -5), s!(   0,    5), s!(   1,    1), s!(   1,    4), s!(  -1,  -10), s!(  -6,   -5),
            s!(  10,   10), s!(  -3,    1), s!(   6,   -7), s!(  -1,   -3), s!(  -5,   -0), s!( -14,  -14), s!(  -2,   -7), s!(  12,   -7),
            s!(  -6,   -1), s!(  -6,    2), s!(   5,   -5), s!(  22,   -1), s!(  11,   -2), s!(   2,   -3), s!(   9,  -10), s!(  21,   -8),
            s!(   3,   -3), s!(  -5,   -2), s!(  28,  -10), s!(   5,  -19), s!(  -3,   -9), s!(   2,  -11), s!( -10,   10), s!(   7,  -10),
            s!(   5,   -6), s!(   3,    5), s!( -11,   -4), s!(  -7,   -5), s!( -21,  -10), s!( -14,   -3), s!( -11,    3), s!(  -4,    3),
            s!(  14,    2), s!(   4,  -13), s!(  -8,   -6), s!( -17,   -2), s!( -15,    5), s!( -18,   -3), s!(  -7,   -3), s!(  -0,   -1),
            s!(   8,   -7), s!(   4,   -9), s!(  -2,   -3), s!(  -4,   -3), s!( -18,   -0), s!(  -7,   -4), s!( -15,    2), s!(  -4,    1),
            s!(   3,   -6), s!(   3,  -10), s!(   2,    6), s!(   3,   -5), s!(  -8,   -7), s!(  -9,    1), s!(   7,   -1), s!( -13,    6),
        ],
        [
            s!(  -3,   -1), s!(  -2,   -5), s!(  -1,   -6), s!(   1,    2), s!(  -2,   -3), s!(   0,   -1), s!(  -1,   -4), s!(  -4,    0),
            s!(  -0,    6), s!(   3,    6), s!(  -6,   -6), s!(  -1,   -3), s!(  -3,   -3), s!(   4,    6), s!(   7,    1), s!(   1,    4),
            s!(  -2,   -2), s!(  -3,   -7), s!(   2,    4), s!(  -5,   -2), s!(  -3,   -5), s!(  -6,   -2), s!(  -3,   -3), s!(   4,   -2),
            s!(   1,   11), s!(   9,   -1), s!(   5,   -5), s!(  -8,  -15), s!(   8,   -7), s!(  -0,   -3), s!(   0,   -1), s!(   6,    0),
            s!(   3,   -4), s!(   0,    2), s!(  -0,   -7), s!(  -2,   -4), s!(   6,  -11), s!(  12,   -0), s!(  -3,   -4), s!(  -5,    4),
            s!(  -8,  -11), s!(   3,   -4), s!(  -1,   -9), s!(   1,   -1), s!(   2,   -7), s!(  -7,   -1), s!(   8,   -1), s!(  -4,   -0),
            s!(  -7,   -6), s!( -10,   -1), s!(  -2,    4), s!(  -0,    4), s!(   1,   -4), s!(  -2,   -3), s!(  13,    8), s!(   0,    2),
            s!(  -1,    3), s!(   0,    8), s!( -14,   12), s!(  -2,    5), s!(  -1,    2), s!(  -7,    2), s!(  -2,   -0), s!(   0,   -2),
        ],
        [
            s!(  -1,   -6), s!(   1,    3), s!(  -1,   -1), s!(  -4,   -7), s!(   2,    7), s!(  -0,    7), s!(   1,    8), s!(  -2,    5),
            s!(  -4,  -22), s!(  -3,  -10), s!(  -7,   -2), s!(  -0,    3), s!(   2,    7), s!(   3,   12), s!(   2,    8), s!(  10,    4),
            s!(   4,   -2), s!(  18,    6), s!(   7,    5), s!(   6,    2), s!(   5,   -4), s!( -10,  -10), s!(  -3,    3), s!( -21,    4),
            s!(  -1,    1), s!(  15,   -7), s!( -16,   10), s!(   7,    1), s!(  -1,    1), s!(   8,    6), s!(   6,   -5), s!(  -9,    0),
            s!(   4,    9), s!(  -2,    5), s!(   5,    8), s!( -18,    1), s!(  26,    4), s!(  20,   -3), s!(  18,    6), s!(  27,    5),
            s!( -21,   -2), s!( -21,   13), s!(  -9,   11), s!(   7,    1), s!(   0,   -2), s!(  22,   -4), s!(  22,  -11), s!(  23,    9),
            s!(   1,    7), s!( -10,    6), s!( -14,   12), s!(  -7,    5), s!(   9,    4), s!(   7,   -6), s!(  21,    3), s!(  20,    7),
            s!(   4,   10), s!( -12,   13), s!(  -7,    4), s!(  -0,    7), s!(  -5,    7), s!(   8,    8), s!(   6,   -1), s!(  12,   -6),
        ],
        [
            s!(  -0,   -1), s!(   0,   -1), s!(  -0,   -2), s!(  -2,    1), s!(   2,    7), s!(   2,   12), s!(   1,   16), s!(  -2,    1),
            s!( -25,  -21), s!(   0,   -3), s!(  -0,   -2), s!(   2,    0), s!(   7,   16), s!(  14,   16), s!(   5,   20), s!(   7,   10),
            s!(   3,   -3), s!(   6,    0), s!(   8,    7), s!(   3,   10), s!(  -3,    3), s!( -22,   -3), s!(  -1,    9), s!(  -6,    4),
            s!(   3,   -5), s!(   7,    8), s!(  -4,    1), s!(   3,   14), s!(  14,    3), s!( -10,   11), s!(  14,    1), s!(  -2,   -0),
            s!(   0,    8), s!(  12,   22), s!(   4,   -7), s!(  -2,   -1), s!(  11,    5), s!(  27,   14), s!(  -1,   -3), s!(  -2,    1),
            s!(  -5,   22), s!( -21,   15), s!(  -7,   12), s!( -13,    8), s!(  19,   -2), s!(  11,   11), s!(  30,    0), s!(  -1,   -1),
            s!(   2,   14), s!( -25,   32), s!( -23,   10), s!(  -4,    4), s!(   4,   -0), s!(  24,    0), s!(  11,   -1), s!(   0,    7),
            s!(   4,   23), s!( -10,    7), s!(  -5,   10), s!(  -8,    4), s!(   4,   11), s!(   2,    4), s!(   6,    3), s!(   7,    2),
        ],
        [
            s!(   0,    1), s!(   2,    2), s!(   1,    2), s!(   3,    6), s!(   1,    6), s!(   2,    4), s!(   1,    2), s!(   1,    1),
            s!(   2,   -0), s!(   2,   -1), s!(   2,    5), s!(   2,    5), s!(   2,   -0), s!(   4,   15), s!(   0,    4), s!(   1,   -0),
            s!(   7,    2), s!(  -8,   -8), s!(  -1,   -0), s!(   3,    4), s!(   5,   11), s!(  -1,    7), s!(  -4,    7), s!( -11,   -2),
            s!(  -1,   -2), s!(   2,   -1), s!(   5,    7), s!(   4,    8), s!(   7,   13), s!(  -5,    2), s!(  15,   15), s!(   2,   -1),
            s!(   6,    3), s!(  -4,   -2), s!(   0,    2), s!(   3,    5), s!(  11,   10), s!(   3,   11), s!(  11,    3), s!(  -2,    0),
            s!(  -3,    7), s!(   1,    3), s!( -16,    3), s!( -17,   -4), s!(  28,   -5), s!(   6,   10), s!(   3,    1), s!( -14,   -7),
            s!(   2,    2), s!( -28,    9), s!(  -4,    8), s!(  10,    5), s!(   9,  -13), s!(  19,   11), s!(   9,   10), s!(  -0,   -2),
            s!(  -4,    3), s!(  -8,    4), s!(  -5,   11), s!(  -7,   -3), s!(  -5,    4), s!(   7,   -3), s!(   5,   -0), s!(  -4,   -4),
        ],
    ],
];

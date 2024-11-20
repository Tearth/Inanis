// ------------------------------------------------------------------------- //
// Generated at 19-11-2024 23:40:19 UTC (e = 0.113270, k = 0.0077, r = 1.00) //
// ------------------------------------------------------------------------- //

use super::*;

#[rustfmt::skip]
pub const PAWN_PST_PATTERN: [[[PackedEval; 64]; KING_BUCKETS_COUNT]; 2] =
[
    [
        [
            s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0),
            s!( 232,  227), s!( 222,  222), s!( 202,  196), s!( 199,  176), s!( 232,  177), s!( 163,  209), s!(  84,  305), s!(  89,  291),
            s!(  78,  162), s!( 107,  153), s!( 134,  137), s!( 147,  129), s!( 150,  122), s!( 149,  138), s!( 148,  187), s!( 123,  184),
            s!(  59,  114), s!(  71,  101), s!(  86,   79), s!(  92,   73), s!( 100,   86), s!(  99,  103), s!(  65,  133), s!( 102,  120),
            s!(  52,  100), s!(  54,   85), s!(  73,   72), s!(  82,   69), s!(  82,   75), s!(  88,   88), s!(  79,   95), s!( 100,   82),
            s!(  44,  100), s!(  39,   93), s!(  63,   82), s!(  73,   82), s!(  69,   82), s!(  84,   83), s!(  87,   78), s!( 113,   62),
            s!(  50,  103), s!(  40,   99), s!(  59,  100), s!(  37,  100), s!(  50,  107), s!(  79,   99), s!( 109,   80), s!( 109,   55),
            s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0),
        ],
        [
            s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0),
            s!( 204,  243), s!( 221,  223), s!( 191,  214), s!( 184,  184), s!( 217,  175), s!( 164,  246), s!( 104,  303), s!(  88,  289),
            s!(  72,  175), s!(  98,  159), s!( 135,  136), s!( 137,  121), s!( 134,  126), s!( 150,  154), s!( 144,  192), s!( 109,  187),
            s!(  52,  133), s!(  75,  108), s!(  82,   97), s!(  94,   85), s!(  86,  101), s!(  94,  112), s!(  93,  130), s!(  87,  131),
            s!(  49,  114), s!(  58,   98), s!(  74,   86), s!(  79,   80), s!(  79,   88), s!(  83,   96), s!( 110,   94), s!(  88,   91),
            s!(  43,  110), s!(  40,  105), s!(  61,   96), s!(  67,   95), s!(  73,   90), s!(  85,   89), s!( 133,   70), s!( 100,   73),
            s!(  45,  119), s!(  40,  113), s!(  53,  104), s!(  55,   97), s!(  62,  101), s!( 110,   88), s!( 154,   63), s!(  92,   72),
            s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0),
        ],
        [
            s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0),
            s!( 200,  233), s!( 220,  223), s!( 198,  203), s!( 212,  175), s!( 203,  216), s!( 169,  248), s!(  72,  287), s!(  66,  265),
            s!(  62,  175), s!(  78,  170), s!( 112,  152), s!( 164,  127), s!( 144,  147), s!( 186,  145), s!( 149,  179), s!(  72,  183),
            s!(  63,  129), s!(  53,  126), s!(  84,  103), s!(  98,   98), s!( 109,  103), s!( 125,  107), s!(  81,  128), s!(  58,  134),
            s!(  43,  120), s!(  56,  108), s!(  71,   98), s!(  92,   86), s!(  85,   90), s!( 119,   92), s!(  89,  100), s!(  51,  106),
            s!(  36,  118), s!(  45,  106), s!(  64,  103), s!(  79,   91), s!(  87,   81), s!( 123,   80), s!(  93,   78), s!(  45,   98),
            s!(  43,  126), s!(  47,  117), s!(  62,  111), s!(  65,  100), s!(  76,   92), s!( 136,   79), s!(  85,   82), s!(  29,  109),
            s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0),
        ],
        [
            s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0),
            s!( 179,  244), s!( 200,  214), s!( 155,  231), s!( 230,  215), s!( 194,  235), s!( 158,  245), s!(  74,  271), s!(  54,  259),
            s!(  48,  187), s!(  93,  182), s!( 105,  179), s!( 120,  166), s!( 119,  167), s!( 159,  157), s!( 101,  175), s!(  57,  173),
            s!(  48,  144), s!(  67,  127), s!(  74,  120), s!(  92,  108), s!( 103,  112), s!(  94,  108), s!(  61,  134), s!(  48,  133),
            s!(  49,  123), s!(  54,  116), s!(  85,  101), s!(  82,   96), s!(  91,   94), s!(  85,   97), s!(  73,  115), s!(  42,  113),
            s!(  42,  120), s!(  49,  110), s!(  71,  106), s!(  62,   94), s!(  89,   89), s!(  79,   89), s!(  76,   96), s!(  43,  107),
            s!(  43,  128), s!(  53,  116), s!(  65,  115), s!(  52,   99), s!(  70,   92), s!(  89,   86), s!(  78,   99), s!(  46,  111),
            s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0),
        ],
        [
            s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0),
            s!( 205,  248), s!( 234,  241), s!( 195,  254), s!( 239,  242), s!( 210,  239), s!( 183,  245), s!(  72,  251), s!(  34,  278),
            s!(  81,  211), s!(  98,  210), s!( 144,  193), s!( 157,  179), s!( 148,  161), s!( 151,  146), s!( 113,  153), s!(  77,  170),
            s!(  51,  154), s!(  58,  155), s!( 109,  126), s!( 114,  111), s!( 105,  103), s!( 109,  107), s!(  62,  121), s!(  52,  129),
            s!(  34,  138), s!(  68,  127), s!(  99,  101), s!( 121,   85), s!(  82,   92), s!(  70,  107), s!(  49,  111), s!(  28,  120),
            s!(  34,  130), s!(  68,  109), s!(  97,   90), s!( 126,   81), s!(  96,   78), s!(  63,  100), s!(  42,  104), s!(  13,  119),
            s!(  31,  139), s!(  60,  119), s!(  90,   97), s!(  99,   79), s!(  67,   78), s!(  70,  106), s!(  39,  114), s!(  16,  130),
            s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0),
        ],
        [
            s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0),
            s!( 215,  261), s!( 233,  264), s!( 208,  293), s!( 211,  236), s!( 190,  220), s!( 166,  201), s!(  88,  269), s!(  19,  281),
            s!(  94,  233), s!( 112,  228), s!( 151,  207), s!( 101,  185), s!( 127,  146), s!( 146,  140), s!(  98,  152), s!(  57,  174),
            s!(  85,  166), s!(  84,  155), s!( 106,  129), s!(  67,  120), s!(  83,  103), s!( 103,  102), s!(  52,  123), s!(  46,  133),
            s!(  94,  133), s!(  73,  127), s!( 108,  103), s!(  76,   99), s!(  66,   98), s!(  73,   99), s!(  53,  105), s!(  31,  117),
            s!(  96,  113), s!( 109,   97), s!(  94,   99), s!(  61,   99), s!(  66,   97), s!(  62,   96), s!(  22,  114), s!(  14,  117),
            s!( 101,  119), s!( 129,   90), s!( 113,  100), s!(  63,  111), s!(  25,  106), s!(  54,  108), s!(  24,  127), s!(  12,  129),
            s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0),
        ],
        [
            s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0),
            s!( 246,  287), s!( 239,  290), s!( 229,  296), s!( 214,  219), s!( 200,  214), s!( 174,  225), s!(  81,  285), s!(   8,  280),
            s!( 116,  238), s!( 124,  238), s!( 121,  209), s!( 126,  177), s!( 140,  148), s!( 145,  124), s!(  77,  165), s!(  61,  180),
            s!(  71,  184), s!(  92,  159), s!(  72,  142), s!(  69,  106), s!(  83,   99), s!(  97,   89), s!(  50,  122), s!(  43,  142),
            s!(  95,  135), s!(  86,  131), s!(  90,  108), s!(  78,   90), s!(  68,   89), s!(  64,   96), s!(  53,  100), s!(  38,  111),
            s!( 110,  108), s!( 128,   94), s!(  85,   96), s!(  70,   94), s!(  51,   97), s!(  49,   96), s!(  19,  111), s!(  22,  114),
            s!( 103,  111), s!( 150,   90), s!( 110,  100), s!(  41,  109), s!(  31,  110), s!(  37,  112), s!(  15,  126), s!(  22,  128),
            s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0),
        ],
        [
            s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0),
            s!( 231,  334), s!( 239,  324), s!( 205,  294), s!( 221,  207), s!( 201,  215), s!( 179,  233), s!(  75,  272), s!(  29,  278),
            s!( 105,  252), s!( 108,  250), s!( 159,  204), s!( 131,  136), s!( 156,  144), s!( 105,  127), s!( 106,  154), s!(  55,  170),
            s!(  65,  182), s!( 108,  166), s!(  95,  126), s!(  85,   82), s!(  64,  107), s!(  92,   95), s!(  31,  131), s!(  41,  140),
            s!(  90,  130), s!(  78,  132), s!(  86,   98), s!(  84,   89), s!(  74,   77), s!(  53,   97), s!(  52,  107), s!(  35,  122),
            s!( 112,   97), s!( 111,   99), s!(  93,   98), s!(  71,   92), s!(  49,   85), s!(  49,   88), s!(  19,   99), s!(  20,  110),
            s!(  92,  108), s!( 164,   86), s!( 101,  102), s!(  51,   99), s!(  39,   87), s!(  46,  102), s!(  23,  117), s!(  22,  123),
            s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0),
        ],
    ],
    [
        [
            s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0),
            s!(  26,    3), s!(  28,   21), s!(  21,   44), s!(  18,   67), s!(  15,   45), s!( -13,   -2), s!(  -6,  -23), s!( -81, -104),
            s!(  15,   36), s!(  -1,   42), s!( -24,   61), s!(   1,   58), s!(  -7,   31), s!(  -9,  -23), s!(  15,  -52), s!( -28,  -66),
            s!(  10,   17), s!( -13,   30), s!( -16,   28), s!( -14,   25), s!(  -3,   -3), s!(  -5,  -19), s!(  30,  -33), s!( -18,  -35),
            s!(   3,    1), s!(  -2,    6), s!(  -8,    3), s!( -11,    1), s!(  -6,  -13), s!(   8,  -14), s!(  12,  -22), s!( -17,  -19),
            s!(   8,  -12), s!(   9,  -12), s!(  -5,   -5), s!(  -2,   -2), s!(   5,   -1), s!(   9,   -3), s!(  11,   -7), s!( -12,   -6),
            s!(  11,  -16), s!(   5,  -13), s!(   4,  -10), s!(  -1,    3), s!(  -3,   11), s!(   5,    9), s!(  10,    1), s!(  -4,  -16),
            s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0),
        ],
        [
            s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0),
            s!(  28,   15), s!(  10,   37), s!(   1,   52), s!( -14,   49), s!(   0,   15), s!( -17,  -46), s!( -51,  -87), s!( -18,  -84),
            s!(  10,   34), s!(  -3,   41), s!(   4,   52), s!(  -7,   33), s!(  -1,   -1), s!(  33,  -42), s!(  -4,  -53), s!(  13,  -49),
            s!(  15,   17), s!(  -1,   26), s!(   2,   25), s!(   3,    4), s!(  14,  -14), s!(  12,  -26), s!( -11,  -20), s!(  -0,  -33),
            s!(  13,    3), s!(   6,   12), s!(   6,    3), s!(   6,   -4), s!(  13,   -8), s!(  12,  -14), s!(  -5,  -19), s!(  -6,  -14),
            s!(  14,   -5), s!(  18,   -4), s!(  10,   -3), s!(   7,   -4), s!(   5,    3), s!(   6,   -1), s!( -11,   -4), s!(  -8,    1),
            s!(  16,   -9), s!(  17,   -8), s!(  11,   -5), s!(   5,   -6), s!(   4,    5), s!(   6,    5), s!(  -8,   -3), s!(  -8,   -4),
            s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0),
        ],
        [
            s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0),
            s!(   6,   12), s!(  10,   16), s!(   3,   13), s!( -12,  -10), s!( -21,  -67), s!( -17,  -52), s!(  -5,  -53), s!(   8,  -28),
            s!(  -8,   25), s!(  -3,   24), s!( -14,   18), s!(  -7,   -7), s!(   9,  -45), s!(   3,  -39), s!(  10,  -57), s!(  -7,  -33),
            s!(  -7,   21), s!( -16,   18), s!( -13,   12), s!( -19,    9), s!(  29,  -17), s!( -10,  -19), s!(  23,  -31), s!(  10,  -20),
            s!( -10,    9), s!( -26,   15), s!( -20,   10), s!(  -8,    6), s!(   7,    0), s!(  -9,  -10), s!(   3,   -7), s!(   8,  -10),
            s!(  -2,    1), s!( -14,    3), s!(  -6,   -3), s!(   3,   -1), s!(   9,    1), s!( -11,   -1), s!(  -4,    1), s!(  -6,    3),
            s!(  -7,    2), s!(  -8,   -1), s!(  -5,   -3), s!( -11,   -6), s!(  11,   -3), s!( -13,    2), s!(  -8,    6), s!(  -4,    3),
            s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0),
        ],
        [
            s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0),
            s!(  -2,   21), s!(  -0,    6), s!(  -5,  -38), s!( -19,  -69), s!( -11,  -56), s!( -10,  -37), s!(   6,   -1), s!(  13,   17),
            s!( -17,    9), s!(  -8,  -11), s!(   2,  -22), s!(  -5,  -44), s!(   8,  -46), s!( -16,  -27), s!(  -9,  -12), s!(  11,  -13),
            s!( -11,    6), s!( -10,    1), s!(  -4,   -1), s!(  -5,  -12), s!( -16,   -4), s!(  -2,  -11), s!(  -7,   -4), s!(   1,   -3),
            s!( -15,    5), s!(  -5,    2), s!( -13,    5), s!(  -3,    3), s!( -14,    5), s!(  -6,    1), s!( -11,    3), s!(   2,   -1),
            s!( -11,   -1), s!(   1,   -3), s!(  -9,    3), s!(  -9,    7), s!( -28,   12), s!( -16,   10), s!( -12,    7), s!(  -1,    3),
            s!(  -9,    1), s!(  -1,   -1), s!(  -7,    1), s!(  -3,   -4), s!( -21,    8), s!( -23,   10), s!( -13,   10), s!(   0,    4),
            s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0),
        ],
        [
            s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0),
            s!(   7,    8), s!(  -9,  -32), s!( -12,  -74), s!( -14,  -63), s!(  -4,  -25), s!(   4,   14), s!(  13,   32), s!(  22,   36),
            s!(  -3,  -16), s!(  11,  -30), s!(   2,  -59), s!( -16,  -55), s!(   3,   -7), s!(  -5,   17), s!(   8,   15), s!(  -1,   27),
            s!( -18,   -4), s!(  12,  -16), s!(  22,  -24), s!(  -6,  -13), s!(  -4,   -1), s!( -11,    9), s!(  -2,    6), s!(   3,   19),
            s!( -21,    4), s!(   0,    2), s!(   6,    3), s!( -26,    8), s!(   5,    7), s!( -11,   12), s!( -11,   10), s!( -11,   16),
            s!( -14,    3), s!( -22,    4), s!(   1,    3), s!( -32,   12), s!(  -8,    5), s!( -14,    8), s!( -10,    9), s!( -11,   11),
            s!( -20,    5), s!( -11,    9), s!(  -0,    4), s!( -19,    8), s!(   1,    3), s!( -12,    6), s!(   3,    6), s!( -21,   17),
            s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0),
        ],
        [
            s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0),
            s!(  -1,  -23), s!( -11,  -63), s!( -11,  -52), s!(  -5,  -24), s!(   3,   16), s!(   8,   47), s!(  20,   38), s!(  36,   42),
            s!(  -1,  -41), s!(   8,  -54), s!(  -3,  -57), s!(  -3,  -13), s!( -16,   22), s!( -18,   44), s!(  -5,   55), s!(  14,   49),
            s!( -31,  -11), s!(   3,  -26), s!(  -5,  -17), s!(  28,  -14), s!(  -9,   14), s!( -12,   23), s!(  -0,   29), s!(  14,   34),
            s!( -18,    2), s!(   2,   -6), s!( -14,   -1), s!(  19,    2), s!(  -0,    7), s!(  -9,   15), s!(   7,   12), s!(  18,   24),
            s!( -27,   14), s!( -18,   14), s!( -13,    7), s!(   5,    8), s!(  -2,   -1), s!(  -4,    4), s!(  14,    6), s!(  21,    9),
            s!( -27,   14), s!( -24,   13), s!( -17,    8), s!(   1,    6), s!(   3,  -10), s!(  -1,   -2), s!(  15,    1), s!(  21,    7),
            s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0),
        ],
        [
            s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0),
            s!( -12,  -49), s!( -33,  -44), s!(  -3,  -17), s!(  -0,   12), s!(  12,   42), s!(  25,   50), s!(  37,   45), s!(  41,   49),
            s!(   3,  -55), s!(  -6,  -47), s!(   4,  -41), s!(   8,    2), s!(  -0,   42), s!( -12,   65), s!(   9,   70), s!(   9,   65),
            s!( -12,  -29), s!(  -5,  -26), s!(  10,  -24), s!(   8,    1), s!( -10,   17), s!(   2,   36), s!(   1,   45), s!(  12,   44),
            s!( -13,   -2), s!(  -5,  -17), s!(   7,   -6), s!(   7,   -4), s!(  -8,    2), s!(  -2,   16), s!(   9,   24), s!(  23,   21),
            s!( -31,   16), s!( -24,   10), s!(  -3,    3), s!(   1,   -0), s!(   7,   -9), s!(  12,   -6), s!(  19,    0), s!(  21,    0),
            s!( -29,   19), s!( -30,   18), s!(  -9,    8), s!(   4,   11), s!(   9,  -13), s!(  15,  -13), s!(  14,    0), s!(  22,    0),
            s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0),
        ],
        [
            s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0),
            s!( -49,  -42), s!(  -1,   -8), s!(   1,    3), s!(   6,   13), s!(   9,   23), s!(   9,   25), s!(  11,   26), s!(  22,   31),
            s!( -23,  -55), s!(  -5,  -16), s!(  15,  -10), s!(  11,   18), s!(   2,   48), s!(   2,   49), s!(   9,   51), s!(  -2,   52),
            s!( -19,  -20), s!(   9,  -16), s!( -17,  -13), s!(  -5,    6), s!( -18,   26), s!(   8,   47), s!(   8,   34), s!(   2,   37),
            s!(  -4,   -5), s!(  -1,  -11), s!(   6,   -9), s!(  -4,   -3), s!( -17,    5), s!(  -7,   20), s!(  18,   17), s!(  19,   24),
            s!(  -4,   -2), s!(  -0,   10), s!( -11,   10), s!(   6,  -14), s!(   8,  -10), s!(   7,   -4), s!(  15,    4), s!(  16,   -2),
            s!(  -6,   10), s!( -10,   15), s!( -11,   13), s!(   7,    6), s!(   1,  -10), s!(  14,  -22), s!(  14,   -9), s!(   0,    4),
            s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0),
        ],
    ],
];

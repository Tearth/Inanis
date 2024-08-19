// --------------------------------------------------- //
// Generated at 09-08-2024 10:19:09 UTC (e = 0.114457) //
// --------------------------------------------------- //

use super::*;

impl EvaluationParameters {
    #[rustfmt::skip]
    pub const PAWN_PST_PATTERN: [[[i16; 64]; 2]; KING_BUCKETS_COUNT] =
    [
        [
            [
                   0,    0,    0,    0,    0,    0,    0,    0,
                 126,  120,  101,  111,  122,   66,  -32,  -25,
                 -12,   11,   36,   53,   55,   61,   43,    9,
                 -34,  -35,  -14,   -8,    9,    9,  -33,   -9,
                 -41,  -46,  -27,  -19,  -12,   -7,  -23,  -11,
                 -51,  -49,  -33,  -20,  -31,   -9,  -20,    1,
                 -46,  -52,  -35,  -63,  -47,  -17,    4,   -4,
                   0,    0,    0,    0,    0,    0,    0,    0,
            ],
            [
                   0,    0,    0,    0,    0,    0,    0,    0,
                 148,  141,  120,   86,   83,  109,  198,  196,
                  82,   72,   60,   39,   17,   22,   72,   77,
                  29,   19,   -6,  -19,  -24,  -16,   18,    5,
                   4,   -5,  -23,  -30,  -27,  -19,  -14,  -24,
                  -4,  -13,  -18,  -24,  -12,  -12,  -20,  -33,
                   0,   -5,   -3,   -8,   10,    7,  -15,  -42,
                   0,    0,    0,    0,    0,    0,    0,    0,
            ],
        ],
        [
            [
                   0,    0,    0,    0,    0,    0,    0,    0,
                 107,  119,   98,   93,  107,   63,   -7,  -39,
                 -22,   -4,   37,   33,   33,   53,   37,    4,
                 -46,  -33,  -26,  -14,  -13,   -7,  -16,  -23,
                 -52,  -47,  -35,  -26,  -23,  -17,    1,  -21,
                 -58,  -56,  -43,  -38,  -34,  -20,   19,  -13,
                 -54,  -58,  -51,  -50,  -46,    0,   44,  -21,
                   0,    0,    0,    0,    0,    0,    0,    0,
            ],
            [
                   0,    0,    0,    0,    0,    0,    0,    0,
                 161,  144,  133,   88,   77,  135,  191,  195,
                  93,   78,   52,   31,   19,   35,   78,   77,
                  44,   23,   14,   -7,   -7,   -5,   18,   17,
                  17,    6,   -7,  -18,  -15,  -11,  -15,  -12,
                   6,    2,   -5,   -3,   -3,   -8,  -29,  -20,
                  15,    8,    2,   -8,    9,   -5,  -33,  -26,
                   0,    0,    0,    0,    0,    0,    0,    0,
            ],
        ],
        [
            [
                   0,    0,    0,    0,    0,    0,    0,    0,
                  99,  112,   92,  116,  104,   71,  -28,  -42,
                 -33,  -18,   20,   67,   44,   75,   30,  -24,
                 -28,  -42,  -12,   -1,   15,   30,  -21,  -36,
                 -45,  -38,  -22,   -4,  -13,   20,  -10,  -50,
                 -50,  -41,  -28,  -11,  -21,   25,  -13,  -53,
                 -44,  -38,  -29,  -33,  -27,   37,  -20,  -63,
                   0,    0,    0,    0,    0,    0,    0,    0,
            ],
            [
                   0,    0,    0,    0,    0,    0,    0,    0,
                 149,  138,  112,   78,  114,  144,  190,  170,
                  90,   82,   60,   28,   42,   39,   77,   78,
                  38,   33,   11,   -1,   -5,   -4,   17,   23,
                  20,   12,   -1,  -16,  -13,  -12,   -8,    3,
                  11,    1,   -1,  -13,  -14,  -20,  -20,    1,
                  19,   10,    6,    2,   -5,  -19,  -14,    5,
                   0,    0,    0,    0,    0,    0,    0,    0,
            ],
        ],
        [
            [
                   0,    0,    0,    0,    0,    0,    0,    0,
                  89,  110,   64,  133,   98,   62,  -26,  -57,
                 -42,   -5,   15,   18,   17,   54,    1,  -30,
                 -41,  -27,  -18,   -5,    6,    3,  -33,  -43,
                 -42,  -33,   -8,   -7,   -1,   -2,  -18,  -49,
                 -47,  -33,  -16,  -29,  -11,  -13,  -20,  -49,
                 -45,  -28,  -22,  -38,  -29,   -8,  -14,  -46,
                   0,    0,    0,    0,    0,    0,    0,    0,
            ],
            [
                   0,    0,    0,    0,    0,    0,    0,    0,
                 153,  124,  139,  121,  135,  146,  175,  169,
                  97,   89,   82,   66,   63,   53,   75,   70,
                  49,   30,   21,    5,    6,    0,   26,   27,
                  25,   17,    1,   -9,   -9,  -10,    7,    7,
                  18,    6,    2,   -5,   -9,  -13,   -7,    5,
                  25,   12,   10,   -4,   -8,  -15,   -4,    7,
                   0,    0,    0,    0,    0,    0,    0,    0,
            ],
        ],
        [
            [
                   0,    0,    0,    0,    0,    0,    0,    0,
                 106,  136,   97,  137,  107,   82,  -27,  -62,
                 -16,    3,   36,   49,   46,   53,    7,  -28,
                 -37,  -28,   13,   16,   10,    7,  -39,  -49,
                 -50,  -23,   -1,   24,  -17,  -23,  -47,  -68,
                 -58,  -26,   -2,   19,   -3,  -28,  -51,  -80,
                 -55,  -30,   -8,  -11,  -38,  -23,  -53,  -74,
                   0,    0,    0,    0,    0,    0,    0,    0,
            ],
            [
                   0,    0,    0,    0,    0,    0,    0,    0,
                 151,  147,  164,  144,  136,  142,  167,  185,
                 114,  110,  100,   76,   53,   45,   57,   72,
                  51,   48,   21,    4,    0,    5,   20,   29,
                  33,   20,   -2,  -17,   -9,    3,    9,   13,
                  29,    5,  -13,  -17,  -23,   -6,   -1,   11,
                  34,   14,   -7,  -12,  -21,    2,    9,   22,
                   0,    0,    0,    0,    0,    0,    0,    0,
            ],
        ],
        [
            [
                   0,    0,    0,    0,    0,    0,    0,    0,
                 113,  132,  104,  116,   90,   67,  -18,  -84,
                 -14,   11,   44,    5,   29,   48,   -9,  -44,
                   0,  -10,    9,  -29,  -19,    0,  -64,  -59,
                  10,  -20,   21,  -22,  -28,  -29,  -58,  -74,
                  13,    6,    2,  -37,  -32,  -44,  -84,  -91,
                  23,   36,   19,  -32,  -76,  -48,  -82,  -91,
                   0,    0,    0,    0,    0,    0,    0,    0,
            ],
            [
                   0,    0,    0,    0,    0,    0,    0,    0,
                 158,  165,  196,  131,  120,  111,  177,  184,
                 131,  125,  108,   75,   37,   40,   60,   72,
                  53,   43,   23,   11,    5,    3,   29,   33,
                  23,   17,   -6,   -4,   -3,   -2,   10,   10,
                  11,   -5,   -7,   -3,   -8,   -7,   10,   10,
                  11,  -16,   -5,    6,    4,    6,   24,   24,
                   0,    0,    0,    0,    0,    0,    0,    0,
            ],
        ],
        [
            [
                   0,    0,    0,    0,    0,    0,    0,    0,
                 132,  138,  115,  119,  101,   78,  -20,  -84,
                   8,   22,   28,   34,   43,   48,  -16,  -45,
                 -15,   -3,  -24,  -26,  -25,   -5,  -68,  -65,
                   6,  -11,   -2,  -20,  -31,  -42,  -58,  -76,
                  21,   27,  -12,  -26,  -44,  -55,  -87,  -87,
                  13,   57,   10,  -50,  -67,  -65,  -94,  -83,
                   0,    0,    0,    0,    0,    0,    0,    0,
            ],
            [
                   0,    0,    0,    0,    0,    0,    0,    0,
                 182,  192,  195,  116,  114,  131,  195,  178,
                 135,  136,  101,   69,   43,   26,   64,   75,
                  68,   49,   31,   -2,    7,   -6,   30,   44,
                  26,   23,   -1,  -13,  -10,   -2,    6,   10,
                   9,   -9,   -5,  -10,  -10,   -6,    5,    6,
                   9,  -15,    0,    5,    4,    8,   20,   24,
                   0,    0,    0,    0,    0,    0,    0,    0,
            ],
        ],
        [
            [
                   0,    0,    0,    0,    0,    0,    0,    0,
                 130,  136,  102,  126,  103,   78,  -21,  -60,
                   3,    5,   48,   34,   55,   20,    5,  -40,
                 -30,   10,   -1,  -21,  -41,   -1,  -64,  -57,
                 -16,  -22,  -17,  -15,  -27,  -44,  -46,  -71,
                  11,    7,    0,  -30,  -38,  -47,  -75,  -79,
                 -15,   61,    3,  -45,  -57,  -47,  -77,  -77,
                   0,    0,    0,    0,    0,    0,    0,    0,
            ],
            [
                   0,    0,    0,    0,    0,    0,    0,    0,
                 232,  215,  196,  120,  119,  133,  176,  193,
                 155,  154,  114,   48,   41,   26,   63,   72,
                  75,   57,   19,  -14,   12,   -1,   24,   39,
                  24,   29,   -4,  -13,  -21,   -4,    4,   13,
                  -2,    0,   -1,   -8,  -23,  -17,  -11,    1,
                  10,  -13,    4,   -2,  -10,   -7,    5,   19,
                   0,    0,    0,    0,    0,    0,    0,    0,
            ],
        ],
    ];
}

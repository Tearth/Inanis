// --------------------------------------------------- //
// Generated at 09-08-2024 10:19:09 UTC (e = 0.114457) //
// --------------------------------------------------- //

use super::*;

impl EvaluationParameters {
    #[rustfmt::skip]
    pub const ROOK_PST_PATTERN: [[[i16; 64]; 2]; KING_BUCKETS_COUNT] =
    [
        [
            [
                 -35,  -22,  -39,   -6,   -9,  -28,  -13,   -7,
                 -58,  -35,  -41,  -20,   -8,    9,   -7,  -24,
                 -58,  -38,  -43,  -20,   -6,   13,   48,  -26,
                 -69,  -70,  -70,  -37,  -40,  -30,    2,  -42,
                 -88,  -81,  -74,  -65,  -45,  -49,  -10,  -58,
                -101,  -84,  -79,  -75,  -64,  -56,  -33,  -57,
                 -94,  -70,  -70,  -63,  -65,  -52,  -28,  -68,
                 -77,  -74,  -74,  -69,  -62,  -49,  -43,  -90,
            ],
            [
                  40,   49,   53,   38,   47,   65,   61,   59,
                  58,   56,   60,   59,   53,   43,   64,   51,
                  53,   51,   52,   39,   45,   40,   20,   35,
                  59,   61,   55,   40,   37,   46,   37,   30,
                  52,   50,   52,   40,   36,   41,   19,   22,
                  35,   32,   34,   35,   31,   30,   17,    8,
                  26,   29,   28,   33,   22,   24,    2,   34,
                  24,   28,   24,   24,   22,   12,   15,    4,
            ],
        ],
        [
            [
                 -19,  -23,  -26,  -16,   -3,  -34,  -19,  -14,
                 -41,  -39,  -29,  -11,   -6,  -19,   -1,   -7,
                 -45,  -30,  -26,   -6,    6,    6,   12,  -14,
                 -62,  -44,  -44,  -33,  -34,  -40,  -38,  -36,
                 -72,  -79,  -66,  -52,  -60,  -75,  -36,  -68,
                 -84,  -71,  -67,  -55,  -61,  -58,  -38,  -75,
                 -82,  -66,  -66,  -55,  -61,  -54,  -15, -113,
                 -61,  -53,  -56,  -49,  -45,  -46,  -55, -130,
            ],
            [
                  53,   55,   61,   48,   47,   67,   83,   57,
                  67,   73,   67,   60,   53,   55,   65,   47,
                  63,   53,   59,   47,   39,   39,   43,   39,
                  58,   58,   57,   46,   44,   45,   50,   23,
                  47,   57,   56,   43,   47,   45,   26,   21,
                  32,   37,   38,   31,   32,   20,    7,    6,
                  29,   31,   36,   29,   35,   20,    7,   23,
                  31,   23,   29,   26,   23,   22,   17,   45,
            ],
        ],
        [
            [
                 -28,  -31,  -42,  -15,  -22,  -42,  -18,  -17,
                 -34,  -31,  -19,    8,   -4,   -8,  -23,  -14,
                 -59,  -34,  -36,  -15,  -26,  -20,   25,  -27,
                 -72,  -38,  -46,  -24,  -46,  -29,  -29,  -42,
                 -93,  -84,  -58,  -38,  -45,  -52,  -33,  -54,
                 -97,  -64,  -69,  -47,  -51,  -43,  -23,  -44,
                 -93,  -67,  -56,  -40,  -36,  -44,  -37,  -85,
                 -78,  -57,  -74,  -55,  -55,  -78,  -48,  -73,
            ],
            [
                  52,   55,   62,   51,   52,   86,   67,   59,
                  64,   67,   58,   52,   48,   65,   65,   56,
                  71,   54,   64,   50,   54,   58,   38,   51,
                  65,   54,   59,   44,   53,   56,   49,   48,
                  51,   62,   44,   40,   29,   50,   31,   32,
                  36,   39,   31,   25,   24,   25,   12,   10,
                  34,   28,   29,   28,   13,   30,   24,   34,
                  33,   29,   42,   35,   23,   39,   26,   25,
            ],
        ],
        [
            [
                 -16,  -25,  -17,  -10,   -4,  -38,   -6,   -2,
                 -34,  -50,  -42,  -11,  -13,  -37,  -50,  -24,
                 -46,  -20,  -52,  -29,  -10,  -30,    8,  -40,
                 -68,  -61,  -54,  -48,  -34,  -23,  -62,  -45,
                 -57,  -77,  -56,  -68,  -34,  -81,  -37,  -62,
                 -76,  -63,  -54,  -68,  -44,  -70,  -45,  -63,
                 -78,  -55,  -47,  -60,  -58,  -85,  -64,  -74,
                 -46,  -43,  -46,  -53,  -69,  -86,  -47,  -43,
            ],
            [
                  49,   48,   53,   50,   70,   64,   52,   57,
                  66,   74,   67,   59,   70,   58,   73,   55,
                  64,   54,   64,   54,   67,   58,   55,   60,
                  63,   66,   64,   63,   61,   48,   59,   57,
                  51,   55,   47,   46,   55,   54,   53,   46,
                  37,   33,   30,   30,   33,   37,   24,   25,
                  35,   20,   33,   22,   31,   33,   31,   31,
                  26,   34,   32,   32,   36,   42,   27,    4,
            ],
        ],
        [
            [
                 -13,  -20,  -20,   -1,  -11,  -36,   -3,   -9,
                 -17,  -19,  -17,   -6,   -4,  -22,  -21,   -5,
                 -51,  -15,  -31,   -8,  -20,  -18,   36,  -38,
                 -49,  -52,  -30,  -30,  -48,  -30,  -47,  -51,
                 -76,  -66,  -69,  -45,  -61,  -67,  -45,  -63,
                 -72,  -54,  -64,  -60,  -43,  -53,  -36,  -81,
                 -75,  -71,  -62,  -65,  -50,  -43,  -49,  -86,
                 -91,  -76,  -71,  -82,  -65,  -64,  -41,  -71,
            ],
            [
                  29,   45,   39,   59,   51,   48,   40,   43,
                  60,   67,   60,   71,   61,   49,   48,   44,
                  64,   52,   58,   63,   52,   59,   39,   55,
                  60,   68,   57,   74,   56,   50,   59,   56,
                  51,   51,   52,   57,   47,   54,   43,   47,
                  33,   35,   28,   39,   21,   27,   29,   27,
                  31,   23,   31,   42,   24,   20,   23,   28,
                  47,   47,   43,   43,   26,   26,   21,   21,
            ],
        ],
        [
            [
                 -17,  -23,  -15,   -6,   -4,  -42,  -11,   -7,
                 -27,  -28,  -19,  -12,   -6,  -18,  -33,   -7,
                 -52,  -15,  -36,  -23,  -14,  -19,   10,  -10,
                 -57,  -53,  -34,  -60,  -51,  -37,  -22,  -49,
                 -85,  -76,  -52,  -69,  -62,  -82,  -52,  -62,
                 -79,  -86,  -40,  -71,  -68,  -66,  -35,  -68,
                 -80,  -58,  -67,  -62,  -55,  -74,  -41,  -86,
                -147, -111,  -78,  -42,  -46,  -49,  -31,  -55,
            ],
            [
                   9,   30,   65,   47,   49,   38,   33,   46,
                  59,   69,   67,   55,   49,   48,   49,   39,
                  62,   50,   62,   49,   45,   42,   25,   38,
                  59,   69,   84,   65,   46,   27,   44,   49,
                  43,   41,   54,   38,   40,   49,   36,   39,
                  33,   39,   37,   31,   29,   17,    5,   15,
                  34,   27,   46,   26,   26,   22,   16,   22,
                  59,   49,   39,   18,   23,   12,    4,   13,
            ],
        ],
        [
            [
                 -23,  -26,  -26,   -5,  -13,  -39,  -10,  -12,
                 -29,  -33,  -15,  -20,   -5,   -9,  -22,  -29,
                 -41,  -26,  -25,  -12,   -6,  -26,   20,  -16,
                 -73,  -50,  -29,  -24,  -30,  -24,  -19,  -40,
                 -79,  -60,  -60,  -65,  -63,  -67,  -43,  -76,
                 -94,  -67,  -54,  -61,  -53,  -51,  -57,  -75,
                 -91,  -56,  -66,  -69,  -51,  -42,  -59,  -77,
                -140,  -56,  -57,  -45,  -56,  -58,  -36,  -58,
            ],
            [
                  -8,   63,   52,   41,   29,   36,   37,   41,
                  51,   74,   57,   64,   49,   33,   46,   43,
                  52,   76,   64,   51,   33,   34,   20,   35,
                  46,   76,   62,   57,   50,   34,   39,   40,
                  47,   62,   54,   44,   41,   44,   36,   46,
                  37,   37,   44,   35,   22,   17,   24,   13,
                  28,   42,   34,   36,   19,    6,   13,   17,
                  46,   23,   15,   21,   15,    7,    4,   18,
            ],
        ],
        [
            [
                 -28,  -30,  -19,   -4,  -10,  -46,  -11,  -15,
                 -36,  -21,   -9,  -18,   -6,  -12,  -28,  -20,
                 -64,  -34,  -27,  -15,  -19,  -22,   27,  -26,
                 -76,  -50,  -37,  -32,  -43,  -37,  -25,  -57,
                 -84,  -58,  -51,  -36,  -48,  -68,  -38,  -72,
                 -86,  -58,  -45,  -55,  -58,  -42,  -50,  -75,
                 -89,  -54,  -46,  -49,  -57,  -48,  -46,  -97,
                 -97,  -51,  -42,  -60,  -60,  -49,  -52,  -76,
            ],
            [
                  38,   47,   47,   44,   42,   42,   45,   41,
                  40,   66,   65,   49,   38,   31,   47,   46,
                  45,   61,   51,   41,   42,   24,   18,   39,
                  44,   57,   60,   51,   51,   25,   48,   39,
                  45,   63,   63,   44,   45,   43,   38,   50,
                  40,   39,   43,   31,   21,   15,   28,   19,
                  16,   40,   43,   23,   20,    2,    8,   35,
                  10,   25,   19,   34,    4,   -8,   20,   10,
            ],
        ],
    ];
}

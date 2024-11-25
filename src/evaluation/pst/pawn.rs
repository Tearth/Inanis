// ------------------------------------------------------------------------- //
// Generated at 24-11-2024 15:06:24 UTC (e = 0.130552, k = 0.0077, r = 1.00) //
// ------------------------------------------------------------------------- //

use super::*;

#[rustfmt::skip]
pub const PAWN_PST_PATTERN: [[[PackedEval; 64]; KING_BUCKETS_COUNT]; 2] =
[
    [
        [
            s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0),
            s!( 210,  208), s!( 218,  213), s!( 197,  198), s!( 184,  191), s!( 227,  182), s!( 165,  212), s!( 111,  285), s!( 121,  300),
            s!(  65,  166), s!(  95,  160), s!( 143,  125), s!( 134,  124), s!( 134,  134), s!( 134,  146), s!( 145,  189), s!( 110,  188),
            s!(  54,  111), s!(  51,  101), s!(  72,   74), s!(  81,   63), s!(  92,   75), s!(  93,   92), s!(  64,  111), s!(  90,  110),
            s!(  48,   93), s!(  37,   83), s!(  68,   57), s!(  74,   51), s!(  82,   60), s!(  85,   79), s!(  76,   76), s!( 103,   64),
            s!(  35,   93), s!(  34,   89), s!(  61,   66), s!(  74,   64), s!(  68,   74), s!(  82,   75), s!(  98,   59), s!( 113,   47),
            s!(  45,   95), s!(  43,   95), s!(  61,   89), s!(  41,   89), s!(  58,   90), s!(  87,   91), s!( 123,   72), s!( 115,   39),
            s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0),
        ],
        [
            s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0),
            s!( 181,  232), s!( 192,  216), s!( 175,  206), s!( 178,  180), s!( 197,  182), s!( 145,  219), s!( 114,  296), s!( 141,  265),
            s!(  73,  173), s!(  86,  156), s!( 129,  140), s!( 128,  126), s!( 125,  133), s!( 136,  156), s!( 146,  193), s!( 120,  190),
            s!(  57,  128), s!(  60,  107), s!(  76,   83), s!(  89,   72), s!(  80,   91), s!(  83,  105), s!(  91,  115), s!(  88,  119),
            s!(  48,  107), s!(  51,   90), s!(  72,   71), s!(  73,   67), s!(  78,   74), s!(  83,   85), s!( 110,   74), s!(  90,   78),
            s!(  37,  103), s!(  38,   95), s!(  59,   83), s!(  68,   86), s!(  72,   82), s!(  85,   85), s!( 143,   55), s!( 101,   63),
            s!(  46,  109), s!(  48,  104), s!(  57,  100), s!(  56,   98), s!(  58,  102), s!( 113,   87), s!( 165,   52), s!( 102,   53),
            s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0),
        ],
        [
            s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0),
            s!( 206,  213), s!( 224,  201), s!( 202,  206), s!( 203,  175), s!( 198,  197), s!( 165,  238), s!(  88,  281), s!(  95,  270),
            s!(  57,  172), s!(  73,  160), s!( 109,  148), s!( 141,  133), s!( 147,  141), s!( 186,  138), s!( 145,  174), s!(  75,  188),
            s!(  48,  131), s!(  47,  114), s!(  81,   86), s!(  89,   84), s!( 106,   93), s!( 126,   97), s!(  88,  111), s!(  66,  124),
            s!(  36,  114), s!(  39,  101), s!(  76,   77), s!(  86,   74), s!(  92,   77), s!( 128,   72), s!(  88,   86), s!(  60,   92),
            s!(  33,  107), s!(  38,   97), s!(  72,   83), s!(  85,   77), s!(  98,   68), s!( 125,   70), s!(  96,   64), s!(  48,   86),
            s!(  39,  119), s!(  45,  112), s!(  65,  104), s!(  48,  108), s!(  82,   86), s!( 138,   70), s!(  82,   77), s!(  41,   94),
            s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0),
        ],
        [
            s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0),
            s!( 157,  243), s!( 166,  221), s!( 127,  235), s!( 208,  188), s!( 184,  222), s!( 144,  229), s!(  91,  267), s!(  70,  267),
            s!(  65,  177), s!(  87,  174), s!( 101,  160), s!( 115,  154), s!( 110,  156), s!( 131,  155), s!(  86,  176), s!(  64,  184),
            s!(  52,  132), s!(  62,  113), s!(  73,   97), s!(  87,   88), s!(  97,  103), s!(  89,   99), s!(  61,  122), s!(  43,  125),
            s!(  45,  112), s!(  53,   99), s!(  82,   86), s!(  80,   79), s!(  97,   77), s!(  86,   87), s!(  68,   99), s!(  42,  100),
            s!(  42,  107), s!(  49,   96), s!(  72,   91), s!(  69,   80), s!(  97,   77), s!(  86,   77), s!(  80,   81), s!(  43,   91),
            s!(  46,  115), s!(  64,  105), s!(  72,  107), s!(  45,  103), s!(  70,   87), s!(  94,   79), s!(  91,   86), s!(  56,   94),
            s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0),
        ],
        [
            s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0),
            s!( 201,  251), s!( 225,  232), s!( 185,  233), s!( 236,  219), s!( 205,  214), s!( 177,  227), s!(  78,  254), s!(  38,  278),
            s!(  64,  189), s!(  88,  188), s!( 144,  169), s!( 147,  163), s!( 154,  146), s!( 147,  143), s!( 121,  160), s!(  53,  180),
            s!(  38,  141), s!(  53,  129), s!(  96,  102), s!( 114,   89), s!( 103,   90), s!( 113,   91), s!(  65,  114), s!(  51,  127),
            s!(  36,  119), s!(  62,  108), s!(  95,   82), s!( 116,   71), s!(  97,   76), s!(  89,   86), s!(  47,  100), s!(  30,  112),
            s!(  28,  113), s!(  64,   96), s!(  86,   78), s!( 132,   55), s!(  96,   67), s!(  88,   82), s!(  40,   92), s!(  22,  104),
            s!(  19,  126), s!(  71,  101), s!(  84,   91), s!( 116,   67), s!(  70,   81), s!(  80,   94), s!(  50,  102), s!(  22,  115),
            s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0),
        ],
        [
            s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0),
            s!( 219,  258), s!( 233,  238), s!( 205,  272), s!( 194,  218), s!( 189,  209), s!( 167,  212), s!(  95,  254), s!(  40,  269),
            s!(  98,  202), s!( 113,  206), s!( 129,  185), s!( 120,  162), s!( 138,  146), s!( 135,  131), s!(  96,  166), s!(  41,  182),
            s!(  77,  145), s!(  89,  125), s!(  89,  109), s!(  67,  101), s!(  74,   99), s!(  98,   89), s!(  45,  120), s!(  36,  127),
            s!(  76,  114), s!(  82,  104), s!(  98,   74), s!(  73,   78), s!(  66,   81), s!(  80,   82), s!(  45,   94), s!(  23,  109),
            s!(  87,   96), s!( 122,   75), s!(  93,   85), s!(  66,   81), s!(  63,   84), s!(  71,   81), s!(  32,   91), s!(  12,  106),
            s!(  98,   95), s!( 138,   70), s!( 117,   86), s!(  58,   89), s!(  39,  103), s!(  63,  101), s!(  30,  111), s!(  17,  115),
            s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0),
        ],
        [
            s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0),
            s!( 263,  260), s!( 235,  293), s!( 230,  257), s!( 195,  207), s!( 192,  206), s!( 163,  212), s!(  89,  269), s!(  31,  287),
            s!( 113,  227), s!( 128,  229), s!( 106,  200), s!( 131,  152), s!( 129,  145), s!( 133,  133), s!(  80,  173), s!(  61,  183),
            s!(  77,  159), s!(  87,  137), s!(  71,  118), s!(  60,   99), s!(  72,   94), s!(  86,   87), s!(  51,  109), s!(  45,  129),
            s!(  93,  119), s!(  92,  101), s!(  73,   93), s!(  75,   73), s!(  63,   72), s!(  68,   80), s!(  45,   93), s!(  36,  107),
            s!(  99,   89), s!( 138,   70), s!(  82,   85), s!(  74,   86), s!(  49,   84), s!(  51,   82), s!(  21,   93), s!(  21,  100),
            s!( 105,   80), s!( 159,   70), s!( 108,   84), s!(  41,  119), s!(  28,  106), s!(  49,   97), s!(  22,  120), s!(  31,  116),
            s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0),
        ],
        [
            s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0),
            s!( 232,  317), s!( 242,  299), s!( 209,  276), s!( 206,  196), s!( 194,  211), s!( 184,  232), s!(  67,  268), s!(  31,  273),
            s!(  99,  243), s!( 114,  235), s!( 172,  177), s!( 138,  147), s!( 145,  148), s!(  96,  147), s!(  99,  155), s!(  59,  191),
            s!(  68,  161), s!( 100,  140), s!(  96,  115), s!(  75,   76), s!(  59,   95), s!(  73,   77), s!(  37,  112), s!(  37,  124),
            s!(  98,  103), s!(  61,  123), s!(  81,   91), s!(  77,   79), s!(  64,   75), s!(  45,   78), s!(  44,   87), s!(  35,  111),
            s!( 117,   76), s!( 120,   71), s!(  89,   88), s!(  74,   91), s!(  39,   78), s!(  38,   82), s!(  11,   90), s!(  14,  108),
            s!( 101,   90), s!( 152,   87), s!(  97,   88), s!(  38,  102), s!(  39,   87), s!(  40,   95), s!(  25,  109), s!(  28,  123),
            s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0),
        ],
        [
            s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0),
            s!( 228,  204), s!( 222,  216), s!( 202,  193), s!( 200,  172), s!( 233,  185), s!( 176,  273), s!(  98,  363), s!(  92,  324),
            s!(  80,  165), s!( 108,  154), s!( 134,  123), s!( 147,  133), s!( 154,  135), s!( 167,  204), s!( 174,  253), s!( 127,  217),
            s!(  58,  108), s!(  70,   96), s!(  83,   76), s!(  91,   71), s!( 104,  113), s!( 109,  149), s!(  95,  200), s!( 105,  169),
            s!(  36,   99), s!(  46,   79), s!(  64,   79), s!(  82,   75), s!(  83,   92), s!(  91,  109), s!(  89,  140), s!( 102,  122),
            s!(  32,  100), s!(  20,   92), s!(  60,   88), s!(  65,   82), s!(  72,   85), s!(  79,   86), s!(  87,  114), s!( 111,   98),
            s!(  23,  106), s!(  19,  102), s!(  51,  108), s!(  37,  105), s!(  46,   99), s!(  76,  101), s!(  96,   99), s!( 101,   88),
            s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0),
        ],
        [
            s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0),
            s!( 198,  199), s!( 220,  211), s!( 193,  207), s!( 190,  198), s!( 234,  249), s!( 182,  331), s!( 118,  350), s!( 104,  352),
            s!(  71,  165), s!(  99,  152), s!( 137,  129), s!( 142,  144), s!( 156,  199), s!( 187,  227), s!( 169,  256), s!( 121,  253),
            s!(  52,  112), s!(  74,   98), s!(  82,   96), s!(  95,   92), s!(  94,  123), s!( 127,  177), s!( 114,  201), s!(  92,  194),
            s!(  36,  104), s!(  46,   91), s!(  72,   71), s!(  78,  100), s!(  74,  106), s!(  95,  128), s!( 137,  125), s!(  71,  140),
            s!(  37,  108), s!(  26,  109), s!(  42,   91), s!(  59,   98), s!(  58,   87), s!(  78,  101), s!( 103,   99), s!(  84,  102),
            s!(  18,  121), s!(  16,  113), s!(  45,  105), s!(  54,   97), s!(  59,   93), s!( 103,   98), s!( 133,   87), s!(  80,   99),
            s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0),
        ],
        [
            s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0),
            s!( 198,  207), s!( 219,  206), s!( 202,  214), s!( 229,  230), s!( 226,  302), s!( 188,  315), s!(  86,  342), s!(  81,  319),
            s!(  61,  164), s!(  78,  172), s!( 117,  163), s!( 180,  180), s!( 170,  216), s!( 212,  217), s!( 164,  223), s!(  79,  234),
            s!(  60,  115), s!(  57,  118), s!(  84,  111), s!( 110,  120), s!( 125,  156), s!( 142,  167), s!(  93,  170), s!(  55,  160),
            s!(  38,  112), s!(  55,   90), s!(  73,   94), s!(  93,   89), s!(  83,  113), s!( 135,  110), s!(  77,  116), s!(  41,  122),
            s!(  17,  112), s!(  30,   95), s!(  59,  100), s!(  71,   96), s!(  79,   87), s!( 119,   89), s!(  83,   96), s!(  29,  100),
            s!(  21,  135), s!(  22,  117), s!(  53,  105), s!(  64,   98), s!(  72,   84), s!( 119,   90), s!(  72,   94), s!(  23,  108),
            s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0),
        ],
        [
            s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0),
            s!( 176,  212), s!( 203,  234), s!( 169,  271), s!( 249,  283), s!( 214,  297), s!( 167,  301), s!(  83,  310), s!(  60,  269),
            s!(  50,  189), s!(  99,  197), s!( 120,  226), s!( 145,  222), s!( 142,  229), s!( 176,  208), s!( 112,  214), s!(  62,  184),
            s!(  56,  144), s!(  71,  138), s!(  85,  143), s!( 109,  153), s!( 115,  175), s!( 110,  152), s!(  63,  143), s!(  48,  134),
            s!(  40,  111), s!(  57,  113), s!(  89,  108), s!(  79,  110), s!( 107,  113), s!(  67,  116), s!(  68,  121), s!(  29,  118),
            s!(  33,  119), s!(  43,  103), s!(  68,   95), s!(  65,  100), s!(  88,   86), s!(  78,   88), s!(  54,   91), s!(  30,  103),
            s!(  23,  125), s!(  27,  117), s!(  63,  107), s!(  51,   92), s!(  68,   89), s!(  81,  100), s!(  60,  105), s!(  30,  113),
            s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0),
        ],
        [
            s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0),
            s!( 209,  265), s!( 244,  294), s!( 218,  317), s!( 258,  296), s!( 226,  295), s!( 189,  271), s!(  77,  270), s!(  35,  268),
            s!(  86,  219), s!( 115,  268), s!( 164,  238), s!( 180,  241), s!( 168,  220), s!( 155,  186), s!( 119,  167), s!(  76,  169),
            s!(  53,  166), s!(  74,  169), s!( 123,  177), s!( 132,  185), s!( 117,  143), s!( 107,  120), s!(  60,  122), s!(  49,  121),
            s!(  18,  139), s!(  65,  128), s!(  93,  116), s!( 120,  107), s!(  69,  109), s!(  53,  103), s!(  38,  105), s!(  16,  106),
            s!(  14,  125), s!(  52,  100), s!(  89,   98), s!( 121,   86), s!(  91,   82), s!(  46,   92), s!(  26,   95), s!(   4,  111),
            s!(  20,  132), s!(  47,  119), s!(  86,  104), s!(  98,   71), s!(  64,   78), s!(  54,   95), s!(  24,  109), s!(  -8,  123),
            s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0),
        ],
        [
            s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0),
            s!( 221,  333), s!( 245,  343), s!( 230,  334), s!( 229,  314), s!( 199,  259), s!( 165,  202), s!(  88,  262), s!(  18,  269),
            s!( 102,  274), s!( 136,  269), s!( 177,  270), s!( 129,  249), s!( 136,  192), s!( 150,  160), s!( 101,  152), s!(  59,  171),
            s!(  80,  196), s!( 100,  191), s!( 125,  205), s!(  82,  169), s!(  87,  109), s!( 103,  112), s!(  52,  116), s!(  45,  125),
            s!(  72,  150), s!(  76,  138), s!( 118,  127), s!(  77,  120), s!(  54,   97), s!(  60,   97), s!(  43,   96), s!(  24,  109),
            s!(  80,  117), s!(  92,  107), s!(  83,   95), s!(  58,  101), s!(  58,   86), s!(  55,   94), s!(  16,   98), s!(   3,  115),
            s!(  80,  131), s!( 111,  106), s!( 109,   96), s!(  60,  105), s!(  21,  109), s!(  43,  100), s!(   9,  121), s!(  -4,  135),
            s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0),
        ],
        [
            s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0),
            s!( 253,  375), s!( 251,  336), s!( 243,  355), s!( 230,  267), s!( 204,  228), s!( 173,  223), s!(  80,  275), s!(   9,  273),
            s!( 124,  307), s!( 143,  295), s!( 151,  280), s!( 144,  231), s!( 144,  162), s!( 145,  123), s!(  75,  150), s!(  60,  168),
            s!(  79,  239), s!( 114,  223), s!(  94,  205), s!(  77,  144), s!(  81,   92), s!(  95,   87), s!(  52,  106), s!(  34,  113),
            s!(  77,  170), s!(  98,  147), s!(  93,  144), s!(  73,   97), s!(  59,   90), s!(  62,   84), s!(  48,   93), s!(  29,   97),
            s!(  96,  119), s!( 106,  102), s!(  73,  105), s!(  66,   88), s!(  52,   97), s!(  44,   98), s!(   5,  105), s!(  13,  110),
            s!(  91,  127), s!( 130,  103), s!(  98,   99), s!(  40,  107), s!(  27,  109), s!(  28,  118), s!(   2,  125), s!(   3,  133),
            s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0),
        ],
        [
            s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0),
            s!( 232,  352), s!( 243,  375), s!( 218,  327), s!( 224,  214), s!( 202,  214), s!( 179,  230), s!(  74,  267), s!(  29,  273),
            s!( 112,  267), s!( 119,  311), s!( 169,  251), s!( 133,  139), s!( 156,  142), s!( 103,  121), s!( 104,  148), s!(  53,  161),
            s!(  77,  211), s!( 126,  229), s!( 106,  174), s!(  89,  104), s!(  57,   96), s!(  89,   97), s!(  27,  111), s!(  40,  124),
            s!(  96,  161), s!(  83,  170), s!(  91,  120), s!(  85,  101), s!(  66,   79), s!(  44,   87), s!(  45,   97), s!(  33,   99),
            s!( 114,  114), s!(  99,  111), s!(  92,   91), s!(  67,   87), s!(  47,   81), s!(  43,   93), s!(  18,   94), s!(  17,  104),
            s!(  80,  110), s!( 156,  102), s!(  96,  102), s!(  52,   99), s!(  37,   79), s!(  41,  112), s!(  17,  124), s!(   6,  127),
            s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0),
        ],
    ],
    [
        [
            s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0),
            s!(  56,   -1), s!(  47,   41), s!(  15,   84), s!(  -5,  103), s!(   3,   70), s!( -33,    6), s!( -18,  -87), s!(-108, -152),
            s!(  -7,   43), s!( -16,   54), s!( -43,   75), s!( -15,   63), s!( -21,   37), s!(  -6,  -39), s!(  31, -109), s!( -33, -100),
            s!( -13,   21), s!(  -8,   23), s!( -19,   42), s!( -17,   25), s!(  -6,   -6), s!(   4,  -33), s!(  26,  -45), s!( -21,  -43),
            s!(  -6,    2), s!(  -9,    9), s!(  -8,   13), s!(  -7,    2), s!(  -8,   -8), s!(  11,  -16), s!(  10,  -21), s!( -14,  -23),
            s!(   1,  -10), s!(   1,   -9), s!(  -6,   -0), s!(   2,   -8), s!(  10,   -7), s!(  10,   -8), s!(   2,   -4), s!( -10,  -12),
            s!(  -6,  -19), s!(  -9,  -19), s!(  -3,  -14), s!( -10,   -1), s!(  -6,   21), s!(  -9,    9), s!(  -3,   -3), s!( -17,  -18),
            s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0),
        ],
        [
            s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0),
            s!(  60,   39), s!(  20,   75), s!(  -1,   87), s!( -43,   76), s!( -20,    7), s!( -31, -126), s!( -86, -140), s!(  -7, -152),
            s!(   1,   42), s!(   2,   49), s!( -14,   62), s!( -14,   35), s!(   4,  -14), s!(  40,  -73), s!(  -3,  -86), s!(  18,  -78),
            s!(   9,   16), s!(  10,   20), s!(   4,   33), s!(   4,    7), s!(  16,  -16), s!(  12,  -31), s!( -12,  -20), s!(  -1,  -37),
            s!(  12,    1), s!(   9,    8), s!(   5,    7), s!(  11,    0), s!(  14,   -7), s!(   8,  -11), s!(  -5,  -17), s!( -11,  -10),
            s!(  18,   -8), s!(  19,   -9), s!(  11,   -2), s!(   6,   -1), s!(   7,    6), s!(   4,   -3), s!( -18,   -1), s!( -12,   -2),
            s!(  12,  -16), s!(   9,  -14), s!(   6,  -12), s!(   7,   -6), s!(   6,   -1), s!(   0,   -4), s!( -14,  -11), s!( -23,   -8),
            s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0),
        ],
        [
            s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0),
            s!(  29,   28), s!(  34,   37), s!(   4,   20), s!( -20,  -28), s!( -30, -107), s!( -47, -111), s!(   9, -121), s!(  52,  -74),
            s!( -27,   27), s!( -25,   38), s!( -35,   20), s!(  -3,  -14), s!(  50,  -81), s!(  34,  -71), s!(  55,  -78), s!(  27,  -65),
            s!( -25,   21), s!( -16,   14), s!( -14,   19), s!( -19,   11), s!(  27,  -17), s!(  -7,  -20), s!(  34,  -33), s!(  15,  -28),
            s!( -19,   11), s!( -18,   11), s!( -15,   12), s!(  -4,    8), s!(  10,    3), s!(  -6,   -9), s!(   0,   -3), s!(   8,  -11),
            s!( -16,    6), s!( -16,    1), s!(  -6,   -3), s!(   4,   -4), s!(  10,    0), s!(  -6,   -4), s!(  -6,    2), s!(   2,    0),
            s!( -22,    2), s!( -17,   -6), s!(  -6,  -13), s!(  -6,   -4), s!(  -2,   -9), s!( -18,   -7), s!( -12,   -6), s!( -12,    1),
            s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0),
        ],
        [
            s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0),
            s!(  -6,   41), s!(   5,   15), s!(   6,  -47), s!( -23, -105), s!( -20, -121), s!( -21, -100), s!(  17,  -33), s!(  41,   16),
            s!(   6,   -1), s!( -18,   -9), s!(  -2,  -30), s!(   4,  -70), s!(  16,  -81), s!( -11,  -47), s!(   3,  -30), s!(  17,  -33),
            s!(  -0,    3), s!(  -7,   -1), s!(  -6,    8), s!(  -3,   -3), s!( -19,   -6), s!(   0,  -14), s!(  -9,   -6), s!(   1,   -8),
            s!(  -8,    2), s!(  -4,    2), s!( -14,    9), s!(  -0,   10), s!( -21,    4), s!(  -7,    5), s!(  -9,    1), s!(   0,   -1),
            s!(  -1,   -3), s!(   2,   -5), s!(  -7,    0), s!(  -8,    4), s!( -30,   12), s!( -18,    7), s!( -15,   10), s!(   1,    3),
            s!(  -7,   -9), s!(  -8,   -6), s!( -13,   -8), s!(  -2,  -19), s!( -26,    3), s!( -25,    2), s!( -19,   -1), s!(  -9,    1),
            s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0),
        ],
        [
            s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0),
            s!(  27,    4), s!( -15,  -45), s!(  -7, -113), s!( -21, -116), s!(  -7,  -74), s!(   7,  -15), s!(  36,   43), s!(  60,   51),
            s!(  -2,  -27), s!(  33,  -45), s!(  29,  -79), s!(  -5,  -78), s!(  16,  -41), s!( -25,   -9), s!(  15,   -3), s!( -12,   22),
            s!(  -7,   -7), s!(   2,   -8), s!(  14,  -17), s!( -11,  -12), s!( -10,   -1), s!(  -4,    3), s!(  -3,    2), s!(  11,    6),
            s!( -34,    7), s!(  -8,    4), s!(   4,    8), s!( -31,    7), s!(  -7,   14), s!( -25,    7), s!( -19,   11), s!( -25,   16),
            s!( -31,    7), s!( -15,    4), s!(  -1,    2), s!( -23,    2), s!( -11,   10), s!( -20,    7), s!(  -7,    5), s!( -10,   11),
            s!( -43,    5), s!( -22,    3), s!( -13,   -6), s!( -42,   11), s!(   2,   -9), s!( -16,   -3), s!( -16,   -1), s!( -26,   12),
            s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0),
        ],
        [
            s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0),
            s!(   9,  -59), s!( -16, -108), s!( -25, -128), s!( -11,  -81), s!(  -2,    4), s!(   6,   66), s!(  35,   50), s!(  72,   51),
            s!(  14,  -65), s!(  19,  -80), s!(  27,  -87), s!(  -5,  -45), s!( -20,   14), s!( -41,   32), s!( -31,   50), s!(   8,   45),
            s!(  -2,  -26), s!(  -7,  -21), s!(  -2,  -18), s!(  32,  -13), s!(  -9,   12), s!( -12,   23), s!(  10,   12), s!(  15,   23),
            s!( -27,   -3), s!(  -5,    2), s!( -19,   -2), s!(  18,    6), s!(   4,    7), s!(  -5,    8), s!(  -2,    9), s!(  19,   14),
            s!( -25,    7), s!( -36,   13), s!( -16,   -0), s!(   8,    4), s!(   2,    2), s!(  -2,    2), s!(   8,    1), s!(  21,    4),
            s!( -38,    2), s!( -38,    8), s!( -26,    3), s!(   1,  -18), s!(   0,  -11), s!(  -3,   -4), s!(  10,  -10), s!(  16,   -1),
            s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0),
        ],
        [
            s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0),
            s!( -29, -143), s!(-130, -123), s!( -11,  -70), s!(  -5,   14), s!(  18,   71), s!(  44,   81), s!(  86,   59), s!(  75,   62),
            s!(  25, -109), s!(  21,  -88), s!(  27,  -91), s!(  20,  -14), s!( -24,   47), s!( -45,   73), s!(  -7,   76), s!(  -5,   56),
            s!(  11,  -46), s!( -18,  -21), s!(  23,  -42), s!(   8,   -3), s!( -10,   15), s!( -12,   44), s!(  11,   39), s!(   7,   36),
            s!( -27,    7), s!( -15,  -14), s!(  -1,   -3), s!(   6,   -3), s!(  -7,   10), s!(  -9,   15), s!(   5,   18), s!(  20,   17),
            s!( -27,   15), s!( -41,   10), s!(  -8,   -3), s!(  -5,   10), s!(  11,   -7), s!(   4,   -0), s!(  15,   -1), s!(  20,    2),
            s!( -36,    8), s!( -45,    8), s!( -17,    4), s!(   3,    2), s!(   7,  -14), s!(   7,  -11), s!(   5,   -4), s!(  21,   -7),
            s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0),
        ],
        [
            s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0),
            s!(-150, -134), s!(  -5,  -36), s!(   5,    9), s!(  20,   41), s!(  19,   60), s!(  28,   75), s!(  34,   52), s!(  54,   57),
            s!( -33, -115), s!(  -7,  -53), s!(  38,  -35), s!(  21,   23), s!( -26,   78), s!( -23,   87), s!(  -0,   81), s!( -34,   72),
            s!( -18,  -50), s!(  22,  -35), s!( -22,  -19), s!( -12,   10), s!( -25,   35), s!( -17,   53), s!(   8,   49), s!( -12,   49),
            s!( -20,  -22), s!(   7,   -2), s!(  10,   -4), s!( -11,   -3), s!( -19,   11), s!(  -2,   22), s!(   5,   15), s!(  11,   26),
            s!( -37,    6), s!( -22,    9), s!( -22,   14), s!(  -1,  -14), s!(   2,   -4), s!(   4,    2), s!(  13,    1), s!(  12,    8),
            s!( -32,   -1), s!( -26,    9), s!(  -3,   -0), s!(   3,   -1), s!(  16,   -8), s!(   2,  -11), s!(  16,  -10), s!(   3,   -2),
            s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0),
        ],
        [
            s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0),
            s!(  14,   10), s!(  26,   39), s!(  21,   50), s!(  20,   76), s!(  18,   65), s!(  -7,   30), s!(  -4,   -1), s!( -70,  -56),
            s!(   4,   36), s!(  -4,   60), s!( -26,   62), s!(   4,   68), s!(  -0,   67), s!(  -1,   36), s!(  21,  -20), s!( -19,   -8),
            s!(  -5,   29), s!( -11,   36), s!( -18,   46), s!(  -4,   34), s!(   4,   24), s!(   1,   17), s!(  33,  -26), s!( -23,  -40),
            s!(   1,    3), s!(  -8,    8), s!(  -3,   14), s!( -13,   12), s!(  -6,   -4), s!(  14,   -3), s!(   6,  -23), s!( -24,  -24),
            s!(   1,  -17), s!(   6,  -16), s!(   5,   -1), s!(  -3,   -0), s!(   1,  -20), s!(   7,    1), s!(  10,   -9), s!( -13,  -39),
            s!(   6,  -20), s!(  -0,  -11), s!(   4,  -21), s!(  -0,    5), s!(  -4,  -10), s!(   8,    6), s!(  30,   29), s!(   8,   -7),
            s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0),
        ],
        [
            s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0),
            s!(  14,   23), s!(   6,   61), s!(   1,   57), s!(  -6,   89), s!(  11,   68), s!(  -4,   30), s!( -38,  -22), s!(  -5,  -11),
            s!(   4,   52), s!(  -2,   73), s!(   5,   73), s!(   1,   73), s!(  12,   68), s!(  40,   27), s!(  11,   25), s!(  14,    8),
            s!(  -3,   30), s!(   7,   44), s!(   7,   43), s!(  15,   40), s!(  17,   19), s!(  18,   -3), s!(  -5,  -20), s!(  -3,  -22),
            s!(  -3,    8), s!(   3,    8), s!(   1,   12), s!(   1,   12), s!(  14,   -8), s!(  11,  -30), s!( -10,  -18), s!(  16,  -28),
            s!(   1,  -22), s!(   2,  -21), s!(  -4,  -13), s!(  10,  -13), s!(   4,  -14), s!(   6,   -5), s!(  -5,  -13), s!( -12,   -6),
            s!(  -1,  -13), s!(   7,  -16), s!(   0,  -20), s!(   6,  -11), s!(   6,    0), s!(  24,   16), s!(  14,   20), s!(  -0,   17),
            s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0),
        ],
        [
            s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0),
            s!(  -5,   23), s!(  11,   29), s!(  10,   48), s!(  -4,   47), s!(  -5,   15), s!(  -5,   -1), s!(   2,   12), s!(  14,   13),
            s!( -15,   35), s!(   0,   58), s!(  -8,   53), s!(   3,   51), s!(  19,   25), s!(  11,   17), s!(  14,   -2), s!(  -2,   30),
            s!( -16,   32), s!( -13,   30), s!(  -7,   36), s!( -12,   29), s!(  26,  -12), s!(  -5,   -8), s!(  19,  -24), s!(   3,  -14),
            s!( -31,    6), s!( -33,    7), s!( -21,    7), s!( -18,   -2), s!(   6,  -17), s!( -14,  -18), s!(   1,  -25), s!(  -2,   -4),
            s!( -13,   -4), s!( -31,  -10), s!( -13,  -15), s!(   7,   -6), s!(  -1,  -12), s!( -16,  -21), s!(  -9,  -20), s!( -20,   -3),
            s!( -16,  -10), s!( -20,   -7), s!(  -9,  -18), s!( -12,   -6), s!(  14,    1), s!(   4,   -1), s!(  11,   16), s!(  -1,   15),
            s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0),
        ],
        [
            s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0),
            s!( -11,   28), s!(   0,   28), s!(   1,   10), s!(  -4,    8), s!(  -1,  -10), s!(   2,   33), s!(  10,   30), s!(   9,   34),
            s!( -16,   26), s!(  -4,   24), s!(   9,   34), s!(  10,   34), s!(  22,   28), s!(  -8,   34), s!(  -6,   28), s!(  15,   22),
            s!( -13,   15), s!(  -7,   12), s!(   1,   14), s!(  -5,    7), s!( -18,   -1), s!(  -4,    1), s!(   2,   -6), s!(   8,    6),
            s!( -32,    5), s!( -16,   -6), s!( -23,   -8), s!( -12,  -12), s!( -20,   -4), s!(  -0,  -20), s!( -12,   -1), s!(  -6,    4),
            s!( -35,   -7), s!( -16,   -4), s!( -24,   -8), s!(  -9,   -1), s!( -32,  -10), s!( -20,    2), s!( -21,    4), s!( -26,    6),
            s!( -41,   -2), s!( -20,   -4), s!(  -9,   11), s!(  -3,    2), s!( -21,    3), s!( -10,   19), s!( -23,   18), s!( -23,    1),
            s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0),
        ],
        [
            s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0),
            s!(   5,   29), s!(  -1,    7), s!(  -0,  -12), s!(  -4,  -16), s!(   4,   21), s!(  12,   65), s!(  18,   60), s!(  19,   41),
            s!(   1,   11), s!(  15,    2), s!(  13,    1), s!(   1,   19), s!(  13,   44), s!(   2,   63), s!(  16,   58), s!(  -3,   55),
            s!( -18,    7), s!(  10,  -18), s!(  22,  -16), s!( -10,  -13), s!(  -6,    4), s!(  -4,   16), s!(   7,   22), s!(   3,   25),
            s!( -22,   -5), s!( -10,  -18), s!(  -5,  -27), s!( -35,   -7), s!(  -2,  -12), s!( -11,    3), s!(  -5,    0), s!( -12,   20),
            s!( -28,  -10), s!( -25,   -5), s!(  -4,  -16), s!( -36,   -7), s!(  -8,    9), s!(  -6,    1), s!( -22,   -1), s!( -25,   11),
            s!( -33,   -1), s!( -14,   14), s!(   1,   20), s!( -15,   25), s!(   2,    1), s!( -14,    8), s!( -10,   -3), s!( -24,    4),
            s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0),
        ],
        [
            s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0),
            s!(   2,   16), s!(  -4,  -21), s!(  -3,  -14), s!(   6,   16), s!(  11,   60), s!(  10,   74), s!(  27,   60), s!(  34,   54),
            s!(   7,   10), s!(  17,  -18), s!(   8,  -17), s!(   4,   22), s!(  -4,   69), s!( -10,   94), s!(  -3,   96), s!(  17,   74),
            s!( -21,  -18), s!(   6,  -20), s!(  -6,  -14), s!(  29,    1), s!(  -5,   24), s!(  -6,   51), s!(  11,   42), s!(  11,   46),
            s!( -16,   -4), s!(   2,  -35), s!( -22,  -24), s!(   7,  -19), s!(   4,    5), s!(  -5,    2), s!(   4,   12), s!(   8,   21),
            s!( -25,    8), s!( -16,  -13), s!( -23,  -20), s!(   0,    0), s!( -13,   -9), s!(  -6,   -9), s!(   9,   -4), s!(   1,   10),
            s!( -25,   26), s!( -17,   17), s!( -14,   -0), s!(  -0,   15), s!(   3,  -15), s!( -17,  -12), s!(   2,   -5), s!(  22,   -1),
            s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0),
        ],
        [
            s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0),
            s!(  -9,  -12), s!( -26,  -14), s!(   3,   10), s!(   6,   44), s!(  21,   78), s!(  26,   70), s!(  45,   68), s!(  40,   51),
            s!(  10,  -13), s!(   3,  -18), s!(  12,   -6), s!(  12,   37), s!(   6,   76), s!(  -3,  105), s!(  12,  104), s!(   8,   90),
            s!(  -8,  -24), s!(  -1,  -25), s!(  16,  -12), s!(  12,   11), s!(  -4,   49), s!(   7,   61), s!(  12,   61), s!(  12,   47),
            s!(  -8,  -28), s!(  -5,  -29), s!(  11,  -28), s!(   9,  -11), s!(  -8,   -8), s!(  -1,   10), s!(  11,   15), s!(  13,   17),
            s!( -36,   -3), s!( -20,  -13), s!(  -7,  -14), s!(  -2,  -13), s!(   0,  -23), s!(  11,  -19), s!(   6,   -8), s!(  15,   -4),
            s!( -18,   20), s!( -16,   19), s!(  -9,   11), s!(   4,    2), s!(   5,  -21), s!(  -1,  -24), s!(   4,   -9), s!(  21,   -5),
            s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0),
        ],
        [
            s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0),
            s!( -45,  -25), s!(   1,    3), s!(   2,    9), s!(   7,   20), s!(  14,   45), s!(  11,   45), s!(  13,   50), s!(  24,   46),
            s!( -18,  -40), s!(  -3,    1), s!(  16,   -2), s!(  14,   38), s!(   8,   74), s!(   9,   90), s!(  10,   80), s!(  -2,   74),
            s!( -18,  -20), s!(  10,  -14), s!( -13,   -6), s!(  -1,   27), s!( -18,   38), s!(   7,   67), s!(   9,   48), s!(  -9,   39),
            s!(  -8,  -14), s!(  -3,  -34), s!(   9,  -11), s!(  -5,   -5), s!( -17,   12), s!(  -5,   26), s!(  14,   12), s!(   8,   12),
            s!( -11,  -31), s!(  -4,  -10), s!( -20,  -10), s!(  -0,  -35), s!(   2,  -27), s!(  11,   -5), s!(   9,   -8), s!(  15,    0),
            s!(  -4,   -4), s!(   0,   36), s!(  -6,   13), s!(   6,    2), s!(   6,   -8), s!(  16,  -31), s!(  12,  -16), s!(   5,  -11),
            s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0),
        ],
    ],
];

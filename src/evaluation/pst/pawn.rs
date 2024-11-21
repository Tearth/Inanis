// ------------------------------------------------------------------------- //
// Generated at 21-11-2024 07:47:21 UTC (e = 0.112638, k = 0.0077, r = 1.00) //
// ------------------------------------------------------------------------- //

use super::*;

#[rustfmt::skip]
pub const PAWN_PST_PATTERN: [[[PackedEval; 64]; KING_BUCKETS_COUNT]; 2] =
[
    [
        [
            s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0),
            s!( 230,  220), s!( 220,  209), s!( 201,  186), s!( 191,  180), s!( 238,  174), s!( 164,  200), s!( 112,  290), s!( 115,  287),
            s!(  76,  158), s!( 114,  145), s!( 143,  129), s!( 147,  128), s!( 146,  131), s!( 140,  138), s!( 142,  174), s!( 126,  177),
            s!(  55,  112), s!(  68,   94), s!(  84,   68), s!(  90,   63), s!(  98,   80), s!(  95,   94), s!(  66,  114), s!( 105,  109),
            s!(  52,   95), s!(  51,   78), s!(  72,   60), s!(  80,   58), s!(  82,   64), s!(  90,   81), s!(  80,   82), s!( 105,   67),
            s!(  43,   94), s!(  43,   85), s!(  64,   74), s!(  74,   76), s!(  69,   72), s!(  84,   77), s!(  91,   67), s!( 116,   52),
            s!(  54,  103), s!(  48,   95), s!(  63,   98), s!(  37,  107), s!(  52,  106), s!(  83,   97), s!( 116,   74), s!( 117,   48),
            s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0),
        ],
        [
            s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0),
            s!( 191,  247), s!( 217,  216), s!( 185,  211), s!( 182,  185), s!( 226,  162), s!( 171,  221), s!( 126,  289), s!( 127,  253),
            s!(  70,  171), s!( 102,  155), s!( 139,  131), s!( 136,  121), s!( 133,  127), s!( 148,  145), s!( 142,  185), s!( 111,  178),
            s!(  47,  131), s!(  73,  101), s!(  80,   86), s!(  92,   76), s!(  84,   94), s!(  92,  100), s!(  95,  114), s!(  92,  119),
            s!(  48,  110), s!(  56,   91), s!(  73,   75), s!(  77,   71), s!(  79,   78), s!(  87,   85), s!( 113,   78), s!(  93,   79),
            s!(  42,  104), s!(  45,   97), s!(  62,   89), s!(  68,   89), s!(  72,   83), s!(  86,   81), s!( 139,   58), s!( 102,   63),
            s!(  49,  119), s!(  48,  109), s!(  56,  103), s!(  56,  101), s!(  64,  100), s!( 115,   85), s!( 163,   56), s!( 101,   65),
            s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0),
        ],
        [
            s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0),
            s!( 205,  228), s!( 229,  210), s!( 201,  195), s!( 204,  170), s!( 199,  188), s!( 158,  221), s!(  74,  261), s!(  82,  251),
            s!(  58,  170), s!(  80,  164), s!( 107,  150), s!( 161,  124), s!( 145,  135), s!( 194,  132), s!( 161,  171), s!(  72,  175),
            s!(  59,  126), s!(  51,  118), s!(  83,   88), s!(  96,   86), s!( 110,   91), s!( 125,   93), s!(  83,  112), s!(  59,  126),
            s!(  42,  115), s!(  51,  101), s!(  70,   86), s!(  91,   74), s!(  87,   75), s!( 124,   78), s!(  89,   89), s!(  54,   98),
            s!(  36,  111), s!(  48,   98), s!(  65,   95), s!(  79,   84), s!(  88,   71), s!( 124,   72), s!(  97,   69), s!(  45,   92),
            s!(  47,  124), s!(  54,  113), s!(  64,  109), s!(  67,  101), s!(  79,   88), s!( 140,   74), s!(  92,   77), s!(  35,  104),
            s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0),
        ],
        [
            s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0),
            s!( 156,  251), s!( 178,  203), s!( 135,  217), s!( 224,  184), s!( 178,  208), s!( 152,  223), s!(  76,  252), s!(  80,  258),
            s!(  44,  178), s!(  95,  171), s!( 105,  166), s!( 120,  155), s!( 115,  155), s!( 161,  149), s!( 100,  168), s!(  50,  177),
            s!(  44,  138), s!(  64,  114), s!(  71,  105), s!(  90,   90), s!( 101,  100), s!(  91,   96), s!(  61,  123), s!(  48,  130),
            s!(  48,  117), s!(  51,  107), s!(  83,   87), s!(  80,   82), s!(  91,   78), s!(  87,   89), s!(  73,  105), s!(  44,  106),
            s!(  40,  111), s!(  52,  101), s!(  70,   98), s!(  63,   86), s!(  89,   79), s!(  80,   83), s!(  80,   88), s!(  43,  100),
            s!(  47,  126), s!(  60,  111), s!(  67,  114), s!(  53,  101), s!(  72,   89), s!(  92,   84), s!(  85,   93), s!(  52,  106),
            s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0),
        ],
        [
            s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0),
            s!( 201,  244), s!( 226,  222), s!( 182,  216), s!( 235,  211), s!( 208,  214), s!( 184,  245), s!(  72,  232), s!(  35,  285),
            s!(  72,  200), s!(  86,  189), s!( 154,  169), s!( 162,  159), s!( 147,  143), s!( 154,  136), s!( 124,  151), s!(  81,  167),
            s!(  37,  148), s!(  44,  141), s!( 106,  104), s!( 110,   95), s!( 104,   90), s!( 110,   96), s!(  60,  114), s!(  46,  129),
            s!(  27,  131), s!(  57,  118), s!(  95,   86), s!( 121,   69), s!(  82,   79), s!(  72,   97), s!(  44,  105), s!(  25,  116),
            s!(  26,  123), s!(  63,  102), s!(  95,   82), s!( 128,   73), s!(  96,   69), s!(  61,   93), s!(  40,   97), s!(  10,  114),
            s!(  28,  138), s!(  61,  115), s!(  91,   96), s!( 107,   77), s!(  71,   74), s!(  71,  103), s!(  44,  108), s!(  19,  128),
            s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0),
        ],
        [
            s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0),
            s!( 219,  242), s!( 229,  225), s!( 203,  266), s!( 197,  201), s!( 186,  209), s!( 169,  206), s!( 107,  262), s!(  38,  284),
            s!(  96,  208), s!( 102,  209), s!( 151,  192), s!( 102,  163), s!( 127,  140), s!( 144,  140), s!( 105,  151), s!(  54,  176),
            s!(  78,  150), s!(  82,  131), s!( 103,  109), s!(  67,   97), s!(  82,   93), s!( 101,   93), s!(  53,  116), s!(  47,  133),
            s!(  89,  122), s!(  69,  114), s!( 106,   81), s!(  74,   85), s!(  67,   86), s!(  77,   89), s!(  54,   98), s!(  35,  112),
            s!(  88,  103), s!( 111,   86), s!(  93,   88), s!(  62,   90), s!(  66,   87), s!(  65,   86), s!(  27,  106), s!(  16,  111),
            s!(  98,  114), s!( 134,   84), s!( 114,   98), s!(  63,  115), s!(  27,  102), s!(  60,  104), s!(  33,  120), s!(  20,  124),
            s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0),
        ],
        [
            s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0),
            s!( 260,  259), s!( 235,  292), s!( 239,  264), s!( 198,  206), s!( 196,  209), s!( 168,  219), s!(  91,  273), s!(  22,  291),
            s!( 114,  219), s!( 122,  218), s!( 107,  198), s!( 121,  170), s!( 138,  149), s!( 139,  130), s!(  68,  177), s!(  60,  184),
            s!(  71,  164), s!(  92,  139), s!(  73,  115), s!(  68,   87), s!(  82,   91), s!(  96,   80), s!(  51,  113), s!(  44,  143),
            s!(  98,  116), s!(  86,  111), s!(  88,   93), s!(  75,   77), s!(  68,   78), s!(  67,   87), s!(  53,   93), s!(  41,  107),
            s!( 110,   94), s!( 134,   80), s!(  84,   87), s!(  70,   90), s!(  52,   85), s!(  52,   87), s!(  24,  102), s!(  24,  108),
            s!( 108,  102), s!( 157,   83), s!( 112,   97), s!(  38,  118), s!(  31,  112), s!(  43,  106), s!(  24,  118), s!(  30,  125),
            s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0),
        ],
        [
            s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0),
            s!( 235,  340), s!( 242,  306), s!( 206,  278), s!( 214,  198), s!( 196,  212), s!( 182,  234), s!(  69,  269), s!(  28,  272),
            s!( 105,  249), s!( 108,  223), s!( 174,  180), s!( 130,  134), s!( 155,  151), s!(  85,  139), s!( 108,  151), s!(  47,  182),
            s!(  59,  175), s!( 104,  144), s!(  87,  107), s!(  84,   68), s!(  66,   96), s!(  86,   88), s!(  29,  126), s!(  42,  139),
            s!(  97,  116), s!(  77,  119), s!(  78,   91), s!(  78,   78), s!(  74,   62), s!(  56,   86), s!(  52,   99), s!(  35,  122),
            s!( 114,   88), s!( 115,   90), s!(  86,   93), s!(  71,   85), s!(  43,   82), s!(  47,   82), s!(  20,   91), s!(  15,  115),
            s!( 102,  102), s!( 166,   84), s!( 100,  101), s!(  39,  101), s!(  35,   89), s!(  48,  102), s!(  29,  115), s!(  26,  124),
            s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0),
        ],
        [
            s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0),
            s!( 232,  222), s!( 222,  217), s!( 201,  194), s!( 200,  171), s!( 232,  180), s!( 171,  251), s!(  93,  335), s!(  86,  288),
            s!(  80,  161), s!( 108,  154), s!( 134,  127), s!( 147,  133), s!( 153,  131), s!( 163,  193), s!( 171,  233), s!( 125,  208),
            s!(  61,  115), s!(  68,   82), s!(  83,   76), s!(  91,   74), s!( 102,  103), s!( 108,  138), s!(  86,  178), s!( 101,  159),
            s!(  44,  103), s!(  46,   73), s!(  69,   90), s!(  86,   83), s!(  84,   93), s!(  86,   98), s!(  85,  134), s!(  90,  118),
            s!(  33,   97), s!(  27,   90), s!(  62,   89), s!(  68,   87), s!(  73,   88), s!(  82,   87), s!(  85,  103), s!( 109,   77),
            s!(  26,   95), s!(  24,  108), s!(  55,  115), s!(  36,  104), s!(  46,  102), s!(  80,   98), s!( 100,  100), s!( 100,   75),
            s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0),
        ],
        [
            s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0),
            s!( 199,  208), s!( 221,  216), s!( 193,  212), s!( 189,  199), s!( 230,  233), s!( 175,  297), s!( 114,  338), s!( 103,  334),
            s!(  72,  177), s!(  99,  155), s!( 137,  131), s!( 141,  140), s!( 152,  189), s!( 176,  212), s!( 164,  246), s!( 118,  238),
            s!(  53,  120), s!(  77,  110), s!(  82,  103), s!(  94,   98), s!(  95,  138), s!( 119,  161), s!( 111,  209), s!(  85,  189),
            s!(  40,  108), s!(  51,   93), s!(  75,   92), s!(  79,  101), s!(  74,  101), s!(  87,  129), s!( 127,  139), s!(  69,  129),
            s!(  44,  110), s!(  29,  109), s!(  47,  103), s!(  61,   95), s!(  58,   85), s!(  76,   99), s!( 104,   91), s!(  84,   97),
            s!(  29,  132), s!(  23,  124), s!(  45,  109), s!(  54,   98), s!(  58,   89), s!( 106,  103), s!( 141,   89), s!(  86,   97),
            s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0),
        ],
        [
            s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0),
            s!( 199,  215), s!( 219,  208), s!( 201,  208), s!( 225,  221), s!( 222,  281), s!( 182,  300), s!(  84,  321), s!(  78,  297),
            s!(  61,  172), s!(  77,  179), s!( 116,  160), s!( 177,  177), s!( 167,  203), s!( 207,  202), s!( 162,  214), s!(  79,  234),
            s!(  62,  117), s!(  55,  116), s!(  89,  135), s!( 108,  119), s!( 120,  140), s!( 142,  184), s!(  90,  177), s!(  56,  160),
            s!(  40,  114), s!(  59,   99), s!(  75,   94), s!(  96,  101), s!(  84,  112), s!( 131,  124), s!(  82,  123), s!(  40,  127),
            s!(  26,  122), s!(  35,  107), s!(  64,  105), s!(  73,  105), s!(  81,   92), s!( 122,   89), s!(  85,   92), s!(  33,  105),
            s!(  33,  133), s!(  31,  127), s!(  54,  109), s!(  64,   99), s!(  72,   83), s!( 122,   93), s!(  79,   99), s!(  28,  121),
            s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0),
        ],
        [
            s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0),
            s!( 175,  208), s!( 202,  226), s!( 166,  262), s!( 245,  270), s!( 211,  290), s!( 164,  280), s!(  80,  297), s!(  57,  255),
            s!(  50,  198), s!(  99,  199), s!( 119,  229), s!( 137,  208), s!( 137,  224), s!( 173,  190), s!( 112,  209), s!(  60,  181),
            s!(  53,  141), s!(  72,  146), s!(  84,  149), s!( 104,  158), s!( 115,  189), s!( 110,  141), s!(  63,  149), s!(  48,  130),
            s!(  44,  120), s!(  56,  117), s!(  92,  118), s!(  78,  113), s!( 103,  131), s!(  70,  112), s!(  69,  122), s!(  31,  116),
            s!(  37,  119), s!(  44,  104), s!(  70,  110), s!(  66,  105), s!(  87,  101), s!(  81,   99), s!(  57,   97), s!(  33,  107),
            s!(  28,  134), s!(  35,  130), s!(  63,  110), s!(  50,   89), s!(  68,   88), s!(  82,   96), s!(  65,  115), s!(  35,  128),
            s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0),
        ],
        [
            s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0),
            s!( 205,  246), s!( 240,  269), s!( 213,  300), s!( 254,  289), s!( 223,  283), s!( 186,  259), s!(  76,  262), s!(  34,  266),
            s!(  87,  233), s!( 112,  262), s!( 155,  238), s!( 175,  236), s!( 166,  204), s!( 155,  185), s!( 116,  157), s!(  76,  175),
            s!(  55,  176), s!(  74,  186), s!( 122,  179), s!( 132,  192), s!( 115,  137), s!( 107,  116), s!(  61,  122), s!(  50,  125),
            s!(  21,  145), s!(  66,  136), s!(  95,  129), s!( 119,  111), s!(  73,  116), s!(  59,  112), s!(  42,  105), s!(  24,  116),
            s!(  19,  133), s!(  55,  106), s!(  94,  110), s!( 120,   87), s!(  93,   93), s!(  52,  103), s!(  31,  104), s!(   5,  115),
            s!(  22,  141), s!(  54,  125), s!(  88,  104), s!(  98,   70), s!(  65,   79), s!(  58,  109), s!(  30,  123), s!(  -3,  126),
            s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0),
        ],
        [
            s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0),
            s!( 219,  303), s!( 241,  312), s!( 223,  322), s!( 224,  292), s!( 198,  250), s!( 164,  197), s!(  87,  261), s!(  19,  276),
            s!( 104,  292), s!( 131,  267), s!( 169,  246), s!( 121,  232), s!( 133,  180), s!( 146,  151), s!( 100,  145), s!(  57,  171),
            s!(  82,  216), s!( 100,  215), s!( 122,  199), s!(  83,  176), s!(  85,  110), s!( 103,  109), s!(  48,  112), s!(  44,  118),
            s!(  79,  170), s!(  78,  151), s!( 115,  141), s!(  74,  115), s!(  56,  101), s!(  63,  105), s!(  44,   89), s!(  29,  110),
            s!(  87,  126), s!(  94,  117), s!(  86,  111), s!(  57,   94), s!(  58,   97), s!(  57,   99), s!(  16,  107), s!(   6,  116),
            s!(  88,  146), s!( 117,  112), s!( 108,  101), s!(  60,  103), s!(  22,  110), s!(  44,  100), s!(  14,  135), s!(   1,  133),
            s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0),
        ],
        [
            s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0),
            s!( 253,  329), s!( 243,  300), s!( 240,  333), s!( 226,  258), s!( 204,  228), s!( 173,  222), s!(  80,  279), s!(   9,  277),
            s!( 124,  283), s!( 138,  276), s!( 142,  243), s!( 140,  219), s!( 143,  158), s!( 144,  120), s!(  75,  148), s!(  60,  175),
            s!(  81,  240), s!( 110,  215), s!(  89,  188), s!(  76,  148), s!(  82,  101), s!(  95,   93), s!(  54,  109), s!(  34,  120),
            s!(  84,  181), s!(  91,  162), s!(  89,  135), s!(  77,  111), s!(  58,   87), s!(  66,   91), s!(  48,   82), s!(  33,  100),
            s!( 102,  134), s!( 110,  106), s!(  71,  101), s!(  66,   86), s!(  50,  105), s!(  47,  101), s!(  10,  104), s!(  17,  117),
            s!(  94,  130), s!( 132,  103), s!( 101,  107), s!(  40,  109), s!(  28,  111), s!(  26,  117), s!(   6,  137), s!(   8,  123),
            s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0),
        ],
        [
            s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0),
            s!( 228,  332), s!( 240,  352), s!( 214,  316), s!( 222,  207), s!( 202,  213), s!( 179,  233), s!(  74,  268), s!(  29,  277),
            s!( 108,  254), s!( 117,  287), s!( 164,  232), s!( 132,  135), s!( 156,  142), s!( 103,  122), s!( 105,  148), s!(  53,  158),
            s!(  75,  213), s!( 122,  211), s!( 103,  170), s!(  88,   97), s!(  60,  105), s!(  89,  100), s!(  27,  118), s!(  41,  129),
            s!(  90,  152), s!(  82,  160), s!(  87,  106), s!(  87,  103), s!(  71,   90), s!(  50,  111), s!(  47,  103), s!(  33,  106),
            s!( 111,  104), s!( 102,  107), s!(  93,  100), s!(  68,   94), s!(  50,   92), s!(  45,   93), s!(  23,  110), s!(  18,   92),
            s!(  81,  106), s!( 152,   87), s!(  96,   97), s!(  52,  100), s!(  37,   81), s!(  41,  108), s!(  18,  120), s!(  10,  124),
            s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0),
        ],
    ],
    [
        [
            s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0),
            s!(  59,   -5), s!(  54,   25), s!(  25,   57), s!(   5,   86), s!(   8,   60), s!( -25,   -3), s!( -16,  -74), s!( -96, -158),
            s!(  16,   44), s!(  -8,   51), s!( -35,   76), s!(  -1,   61), s!(  -6,   24), s!(   9,  -39), s!(  50,  -84), s!( -23,  -74),
            s!(  14,   19), s!( -11,   36), s!( -13,   37), s!( -11,   33), s!(   1,   -1), s!(   1,  -15), s!(  32,  -28), s!( -19,  -31),
            s!(   4,    5), s!(   0,   13), s!(  -6,   13), s!(  -8,   10), s!(  -6,   -4), s!(   8,   -9), s!(  13,  -14), s!( -20,  -13),
            s!(  10,   -6), s!(   5,   -5), s!(  -5,    1), s!(  -3,    4), s!(   6,    7), s!(   9,    2), s!(   8,   -2), s!( -13,   -1),
            s!(   7,  -16), s!(  -2,  -10), s!(   1,   -9), s!(  -1,   -1), s!(  -6,   14), s!(   2,    9), s!(   4,    2), s!( -10,  -14),
            s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0),
        ],
        [
            s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0),
            s!(  53,   11), s!(  17,   46), s!(  -1,   63), s!( -19,   56), s!(   2,    7), s!( -27, -111), s!( -69, -142), s!( -19, -136),
            s!(  12,   40), s!(  -7,   45), s!(   1,   56), s!(  -5,   32), s!(   4,  -12), s!(  46,  -53), s!(   4,  -63), s!(  16,  -54),
            s!(  20,   17), s!(   2,   31), s!(   6,   36), s!(   6,   12), s!(  17,  -10), s!(  16,  -20), s!( -11,  -10), s!(  -2,  -28),
            s!(  14,    8), s!(   9,   18), s!(   8,   13), s!(   9,    5), s!(  13,    2), s!(   9,   -6), s!(  -5,  -10), s!( -10,   -5),
            s!(  15,    1), s!(  14,    3), s!(  10,    4), s!(   7,    1), s!(   6,   11), s!(   5,    5), s!( -15,    3), s!( -10,    7),
            s!(  12,   -9), s!(  10,   -6), s!(   8,   -4), s!(   5,  -10), s!(   2,    7), s!(   1,    6), s!( -15,    0), s!( -15,   -0),
            s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0),
        ],
        [
            s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0),
            s!(  18,   14), s!(  28,   29), s!(   8,   13), s!( -15,  -19), s!( -23,  -93), s!( -28,  -98), s!(   1, -126), s!(  37,  -65),
            s!( -10,   29), s!( -10,   25), s!( -28,   22), s!(  -2,  -15), s!(  47,  -70), s!(  31,  -56), s!(  48,  -77), s!(   7,  -47),
            s!(  -2,   22), s!( -13,   22), s!(  -8,   19), s!( -16,   16), s!(  33,  -14), s!(  -6,  -14), s!(  28,  -25), s!(  14,  -19),
            s!(  -9,   12), s!( -24,   21), s!( -19,   22), s!(  -5,   14), s!(   6,   11), s!( -11,   -3), s!(   2,    3), s!(   8,   -6),
            s!(  -2,    7), s!( -20,   12), s!(  -6,    3), s!(   1,    5), s!(  10,    8), s!( -13,    6), s!(  -7,    6), s!(  -5,    7),
            s!( -11,    1), s!( -16,    2), s!( -10,   -2), s!( -13,  -11), s!(  10,   -4), s!( -18,    4), s!( -14,    7), s!(  -8,    4),
            s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0),
        ],
        [
            s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0),
            s!(  -8,   32), s!(  -0,   21), s!(  10,  -47), s!( -20,  -84), s!( -10,  -99), s!( -25,  -96), s!(  15,  -31), s!(  33,    8),
            s!( -18,   14), s!( -13,   -8), s!(   6,  -27), s!(  -2,  -54), s!(  20,  -64), s!( -12,  -46), s!( -10,  -26), s!(  18,  -29),
            s!(  -7,    9), s!(  -7,    7), s!(  -1,   10), s!(  -3,   -5), s!( -13,    1), s!(   1,   -8), s!(  -7,    0), s!(   1,   -3),
            s!( -15,   10), s!(  -2,    9), s!( -12,   16), s!(  -1,   14), s!( -15,   14), s!(  -9,    9), s!( -11,   10), s!(  -1,    4),
            s!( -10,    6), s!(  -2,    3), s!(  -9,    8), s!( -10,    9), s!( -27,   20), s!( -18,   15), s!( -16,   12), s!(  -1,    7),
            s!( -13,    1), s!(  -8,   -0), s!( -10,   -0), s!(  -4,  -11), s!( -23,    9), s!( -27,   10), s!( -20,   10), s!(  -6,    5),
            s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0),
        ],
        [
            s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0),
            s!(  22,    3), s!( -16,  -41), s!( -11, -107), s!( -22, -102), s!(  -5,  -65), s!(   5,  -16), s!(  29,   39), s!(  47,   41),
            s!(  -1,  -17), s!(  38,  -38), s!(  28,  -68), s!( -16,  -66), s!(  11,  -31), s!( -18,    1), s!(  16,   -8), s!(  -7,   19),
            s!( -28,    2), s!(  15,   -6), s!(  33,  -17), s!(  -9,   -2), s!(  -6,    3), s!( -18,   13), s!(  -5,    5), s!(   2,   16),
            s!( -35,   13), s!(  -2,   12), s!(   2,   20), s!( -31,   19), s!(  -1,   19), s!( -20,   18), s!( -21,   18), s!( -21,   20),
            s!( -26,   13), s!( -32,   14), s!(   0,    8), s!( -44,   20), s!( -15,   12), s!( -25,   14), s!( -21,   14), s!( -20,   18),
            s!( -37,   10), s!( -23,   11), s!(  -4,    1), s!( -38,   11), s!(  -5,    4), s!( -24,    6), s!( -11,    8), s!( -35,   19),
            s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0),
        ],
        [
            s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0),
            s!(   9,  -53), s!( -17, -101), s!( -21, -116), s!( -12,  -70), s!(   3,    8), s!(   6,   58), s!(  39,   53), s!(  74,   49),
            s!(  -0,  -43), s!(  32,  -62), s!(  13,  -64), s!(   1,  -31), s!( -25,   11), s!( -29,   29), s!( -18,   46), s!(  20,   37),
            s!( -35,   -3), s!(   4,  -14), s!(  -4,   -5), s!(  32,  -10), s!(  -6,   17), s!(  -7,   20), s!(   3,   27), s!(  19,   26),
            s!( -23,    9), s!(   3,    6), s!( -13,    9), s!(  22,   12), s!(  -1,   16), s!( -12,   19), s!(   9,   14), s!(  18,   23),
            s!( -31,   22), s!( -23,   20), s!( -14,   14), s!(   5,    9), s!(  -2,    7), s!(  -6,   11), s!(  11,    9), s!(  22,   12),
            s!( -35,   15), s!( -32,   13), s!( -20,    8), s!(   1,   -1), s!(   0,   -8), s!(  -7,    0), s!(  10,    1), s!(  17,    7),
            s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0),
        ],
        [
            s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0),
            s!( -23, -108), s!( -83, -103), s!(  -9,  -58), s!(  -4,   13), s!(  16,   61), s!(  51,   75), s!(  84,   61), s!(  73,   65),
            s!(  11,  -64), s!(   2,  -59), s!(  27,  -73), s!(  20,  -16), s!( -11,   40), s!( -30,   65), s!(   0,   70), s!(   8,   63),
            s!( -12,  -21), s!(  -4,  -15), s!(  14,  -16), s!(  13,    7), s!(  -4,   13), s!(  11,   30), s!(   5,   43), s!(  15,   40),
            s!( -17,   11), s!(  -4,   -8), s!(   8,    9), s!(  11,    3), s!(  -7,   11), s!(  -1,   15), s!(  11,   28), s!(  24,   21),
            s!( -33,   25), s!( -30,   19), s!(  -2,    7), s!(   1,    3), s!(   8,   -2), s!(  12,   -2), s!(  18,    5), s!(  24,    3),
            s!( -35,   21), s!( -38,   22), s!( -11,    5), s!(   1,   14), s!(   7,  -12), s!(  12,  -12), s!(  10,    1), s!(  18,    1),
            s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0),
        ],
        [
            s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0),
            s!(-124, -108), s!(  -5,  -32), s!(   4,   10), s!(  19,   35), s!(  19,   47), s!(  22,   55), s!(  28,   50), s!(  51,   58),
            s!( -24,  -84), s!(  -6,  -44), s!(  43,  -27), s!(  22,   29), s!( -22,   77), s!( -14,   70), s!(   4,   70), s!( -24,   76),
            s!( -26,  -14), s!(  18,  -18), s!( -18,   -1), s!(  -5,   14), s!( -22,   39), s!(   9,   57), s!(  14,   37), s!(  -1,   49),
            s!(  -9,    1), s!(  -2,    1), s!(   7,    3), s!(  -3,    6), s!( -18,   16), s!(  -6,   21), s!(  27,   18), s!(  18,   31),
            s!( -10,   11), s!(  -6,   20), s!( -15,   21), s!(   6,  -10), s!(  11,   -4), s!(   8,    3), s!(  18,    8), s!(  17,    4),
            s!( -16,   12), s!( -21,   17), s!( -17,   14), s!(  16,   10), s!(  -0,  -13), s!(  11,  -16), s!(  15,   -6), s!(  -5,   11),
            s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0),
        ],
        [
            s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0),
            s!(  18,    8), s!(  26,   23), s!(  21,   50), s!(  20,   74), s!(  17,   55), s!( -10,   14), s!(  -5,  -12), s!( -75,  -82),
            s!(   4,   15), s!(  -4,   47), s!( -27,   48), s!(   4,   67), s!(  -2,   58), s!(  -3,   24), s!(  18,  -36), s!( -25,  -41),
            s!(   1,   26), s!( -12,   34), s!( -17,   39), s!(  -6,   24), s!(   2,   23), s!(  -3,    6), s!(  32,  -31), s!( -22,  -46),
            s!(  -0,    7), s!( -10,   -0), s!(  -1,   20), s!( -12,    9), s!(  -9,   -6), s!(  14,   -4), s!(   3,  -32), s!( -19,  -31),
            s!(   6,   -5), s!(   6,  -16), s!(   2,   -7), s!(  -7,  -11), s!(   3,   -6), s!(   8,   -8), s!(  13,  -16), s!( -11,  -27),
            s!(  11,   -8), s!(   1,  -13), s!(   2,  -23), s!(   0,    7), s!(  -5,  -13), s!(  14,   14), s!(  26,   25), s!(   5,  -12),
            s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0),
        ],
        [
            s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0),
            s!(  21,   22), s!(   7,   50), s!(   3,   67), s!( -10,   68), s!(   8,   58), s!(  -7,   19), s!( -43,  -54), s!(  -8,  -33),
            s!(   4,   27), s!(  -2,   57), s!(   4,   63), s!(  -3,   53), s!(   9,   58), s!(  40,   14), s!(   7,    3), s!(  13,  -11),
            s!(   3,   40), s!(   8,   46), s!(   6,   34), s!(  13,   32), s!(  16,   19), s!(  14,  -20), s!(  -8,  -32), s!(  -2,  -33),
            s!(  -1,   15), s!(   2,   22), s!(   3,   19), s!(   4,   13), s!(  10,  -13), s!(  10,  -30), s!( -10,  -26), s!(  12,  -39),
            s!(   6,   -3), s!(   7,  -10), s!(  -2,   -3), s!(  11,   -1), s!(   8,   -1), s!(   4,  -19), s!(  -7,  -17), s!( -12,  -27),
            s!(   4,   -2), s!(  14,  -11), s!(   4,  -17), s!(   7,  -10), s!(   6,   -2), s!(  19,   17), s!(  10,    9), s!(  -0,   12),
            s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0),
        ],
        [
            s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0),
            s!(   1,   20), s!(   9,    9), s!(   9,   35), s!(  -9,   25), s!(  -9,   -7), s!(  -9,  -23), s!(   2,    3), s!(  11,    1),
            s!( -13,   24), s!(   0,   43), s!( -11,   35), s!(  -1,   32), s!(  17,    8), s!(   7,  -11), s!(  11,  -28), s!(  -5,   16),
            s!( -14,   26), s!( -14,   32), s!( -10,   32), s!( -11,   31), s!(  27,  -21), s!(  -5,  -10), s!(  20,  -39), s!(   3,  -14),
            s!( -25,   13), s!( -34,   13), s!( -23,   -0), s!( -15,   10), s!(   7,  -19), s!( -13,  -24), s!(  -2,  -32), s!(  -0,  -11),
            s!( -12,  -13), s!( -23,  -16), s!( -12,    2), s!(   6,  -10), s!(  -0,  -15), s!( -15,  -14), s!(  -8,  -15), s!( -19,    3),
            s!(  -9,   -1), s!( -19,  -11), s!( -10,  -16), s!( -11,   -2), s!(  15,    4), s!(  -3,    6), s!(   9,   22), s!(  -0,   17),
            s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0),
        ],
        [
            s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0),
            s!( -11,   15), s!(  -3,    2), s!(  -4,  -22), s!(  -8,  -12), s!(  -6,  -38), s!(   1,   20), s!(  10,   27), s!(   9,   26),
            s!( -18,    9), s!(  -8,   -8), s!(   7,    9), s!(   5,    9), s!(  17,   -1), s!( -11,   16), s!(  -7,   14), s!(  15,   16),
            s!( -11,    6), s!(  -3,    9), s!(  -3,    2), s!(  -5,    1), s!( -16,   -8), s!(  -3,   -3), s!(   2,    2), s!(   8,    4),
            s!( -27,    1), s!( -13,   -7), s!( -18,  -11), s!(  -7,  -15), s!( -17,    2), s!(   3,  -18), s!( -14,   -5), s!(  -4,   -6),
            s!( -29,  -13), s!( -10,   -9), s!( -22,    4), s!(  -9,   -7), s!( -32,   -1), s!( -20,    3), s!( -17,    2), s!( -20,    4),
            s!( -34,   -2), s!( -13,    5), s!(  -7,   14), s!(  -4,   -3), s!( -21,   -2), s!( -13,   22), s!( -16,   23), s!( -12,    7),
            s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0),
        ],
        [
            s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0),
            s!(   6,   21), s!(  -4,  -12), s!(  -4,  -33), s!(  -9,  -39), s!(   1,    6), s!(  11,   58), s!(  14,   40), s!(  19,   33),
            s!(   0,    5), s!(  16,   -4), s!(  10,  -17), s!(  -4,   -4), s!(  11,   35), s!(   2,   54), s!(  14,   44), s!(  -5,   35),
            s!( -16,    7), s!(   5,  -30), s!(  22,  -25), s!(  -7,  -11), s!(  -5,   -1), s!(  -7,   12), s!(   6,   28), s!(   2,   28),
            s!( -22,   -3), s!(  -9,  -14), s!(  -2,  -22), s!( -34,  -10), s!(  -3,  -12), s!( -10,    6), s!(  -2,    1), s!( -12,   23),
            s!( -26,   -1), s!( -22,  -10), s!(  -3,  -13), s!( -36,   -2), s!( -11,  -12), s!(  -5,   10), s!( -21,    7), s!( -19,    4),
            s!( -26,   -1), s!( -13,   14), s!(   0,   15), s!( -16,   20), s!(   2,    3), s!( -16,   14), s!(  -1,    2), s!( -19,   11),
            s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0),
        ],
        [
            s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0),
            s!(   2,    2), s!(  -7,  -40), s!(  -8,  -39), s!(   3,    3), s!(   7,   40), s!(  10,   73), s!(  23,   44), s!(  34,   37),
            s!(   7,    3), s!(  14,  -34), s!(   4,  -37), s!(   4,   20), s!(  -7,   59), s!( -10,   86), s!(  -6,   77), s!(  14,   53),
            s!( -21,  -13), s!(   3,  -41), s!(  -5,  -11), s!(  30,   -2), s!(  -5,   19), s!(  -7,   46), s!(   9,   41), s!(  13,   51),
            s!( -18,   -2), s!(   3,  -34), s!( -18,  -17), s!(   8,  -17), s!(   2,   -6), s!(  -3,   18), s!(   2,   13), s!(  10,   26),
            s!( -22,   10), s!( -15,  -10), s!( -22,  -14), s!(   3,    5), s!( -15,  -14), s!( -10,   -3), s!(  13,    3), s!(   5,    5),
            s!( -22,   35), s!( -17,   36), s!( -17,    9), s!(   0,   15), s!(   3,  -18), s!( -14,  -17), s!(   9,    1), s!(  28,    3),
            s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0),
        ],
        [
            s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0),
            s!(  -9,  -15), s!( -29,  -27), s!(   0,   -1), s!(   4,   34), s!(  18,   66), s!(  23,   50), s!(  40,   43), s!(  41,   44),
            s!(  10,  -19), s!(  -0,  -34), s!(   9,  -18), s!(  12,   33), s!(   6,   72), s!(  -7,   85), s!(   9,   78), s!(   3,   63),
            s!( -10,  -36), s!(  -2,  -32), s!(  16,  -14), s!(  12,   10), s!(  -3,   50), s!(   5,   59), s!(   5,   53), s!(  11,   49),
            s!(  -8,  -25), s!(  -5,  -31), s!(   9,  -31), s!(  10,  -11), s!(  -7,   -3), s!(   1,   29), s!(   9,   22), s!(  13,   25),
            s!( -36,   -9), s!( -17,   -2), s!(  -3,   -8), s!(   1,   -5), s!(   2,  -16), s!(  10,  -10), s!(   5,   -5), s!(  19,   -0),
            s!( -20,   23), s!( -17,   17), s!(  -5,   26), s!(   4,    7), s!(   6,  -21), s!(  -2,  -22), s!(  11,   -1), s!(  20,   -2),
            s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0),
        ],
        [
            s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0),
            s!( -47,  -31), s!(  -0,   -2), s!(   2,    8), s!(   7,   18), s!(  13,   32), s!(   9,   30), s!(  11,   34), s!(  24,   39),
            s!( -21,  -51), s!(  -3,   -4), s!(  16,   -5), s!(  13,   28), s!(   7,   65), s!(   6,   70), s!(  10,   68), s!(  -1,   57),
            s!( -18,  -23), s!(  10,  -12), s!( -14,  -11), s!(  -2,   22), s!( -15,   41), s!(   7,   62), s!(   7,   45), s!(  -8,   30),
            s!(  -6,  -13), s!(  -2,  -32), s!(   7,  -17), s!(  -4,   -5), s!( -17,    6), s!(  -6,   41), s!(  16,   22), s!(  12,   27),
            s!(  -9,  -38), s!(  -4,  -11), s!( -16,   -2), s!(  -0,  -29), s!(   4,  -12), s!(   9,  -10), s!(  15,    5), s!(  18,    3),
            s!(  -1,   14), s!(  -0,   29), s!(  -4,   18), s!(   7,    7), s!(   6,   -7), s!(  16,  -34), s!(  13,  -17), s!(   4,    1),
            s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0), s!(   0,    0),
        ],
    ],
];

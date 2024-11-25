// ------------------------------------------------------------------------- //
// Generated at 24-11-2024 15:06:24 UTC (e = 0.130552, k = 0.0077, r = 1.00) //
// ------------------------------------------------------------------------- //

use super::*;

#[rustfmt::skip]
pub const ROOK_PST_PATTERN: [[[PackedEval; 64]; KING_BUCKETS_COUNT]; 2] =
[
    [
        [
            s!( 473,  558), s!( 493,  565), s!( 462,  581), s!( 515,  555), s!( 504,  557), s!( 496,  573), s!( 496,  583), s!( 500,  575),
            s!( 468,  580), s!( 486,  581), s!( 481,  581), s!( 493,  571), s!( 523,  564), s!( 524,  552), s!( 518,  569), s!( 505,  560),
            s!( 450,  576), s!( 477,  570), s!( 469,  574), s!( 501,  556), s!( 504,  550), s!( 526,  552), s!( 567,  542), s!( 495,  551),
            s!( 434,  569), s!( 444,  578), s!( 440,  583), s!( 475,  554), s!( 477,  558), s!( 485,  565), s!( 509,  557), s!( 480,  548),
            s!( 419,  577), s!( 425,  574), s!( 438,  576), s!( 457,  563), s!( 451,  565), s!( 459,  572), s!( 504,  543), s!( 459,  545),
            s!( 409,  556), s!( 433,  557), s!( 433,  555), s!( 444,  552), s!( 451,  560), s!( 461,  553), s!( 476,  539), s!( 456,  542),
            s!( 421,  551), s!( 436,  549), s!( 433,  553), s!( 443,  545), s!( 451,  548), s!( 461,  547), s!( 496,  525), s!( 452,  541),
            s!( 440,  543), s!( 445,  548), s!( 437,  550), s!( 443,  554), s!( 450,  548), s!( 464,  538), s!( 472,  550), s!( 422,  526),
        ],
        [
            s!( 472,  587), s!( 479,  587), s!( 472,  593), s!( 492,  582), s!( 524,  573), s!( 486,  592), s!( 491,  604), s!( 509,  583),
            s!( 472,  602), s!( 482,  606), s!( 479,  603), s!( 499,  594), s!( 503,  592), s!( 491,  585), s!( 529,  581), s!( 510,  578),
            s!( 476,  588), s!( 483,  590), s!( 482,  593), s!( 503,  574), s!( 495,  573), s!( 512,  569), s!( 514,  573), s!( 503,  562),
            s!( 442,  583), s!( 464,  582), s!( 458,  590), s!( 479,  572), s!( 462,  581), s!( 469,  582), s!( 452,  587), s!( 463,  567),
            s!( 434,  573), s!( 438,  585), s!( 450,  583), s!( 460,  578), s!( 448,  577), s!( 447,  579), s!( 459,  567), s!( 451,  559),
            s!( 430,  563), s!( 447,  559), s!( 439,  570), s!( 451,  564), s!( 445,  573), s!( 448,  565), s!( 456,  558), s!( 435,  546),
            s!( 439,  557), s!( 441,  555), s!( 443,  569), s!( 456,  558), s!( 445,  566), s!( 454,  558), s!( 485,  547), s!( 378,  571),
            s!( 457,  555), s!( 463,  557), s!( 459,  557), s!( 464,  556), s!( 462,  561), s!( 464,  551), s!( 464,  541), s!( 370,  575),
        ],
        [
            s!( 496,  573), s!( 481,  582), s!( 465,  589), s!( 499,  574), s!( 486,  583), s!( 463,  600), s!( 493,  587), s!( 509,  586),
            s!( 492,  587), s!( 499,  591), s!( 496,  592), s!( 531,  577), s!( 501,  583), s!( 510,  579), s!( 524,  577), s!( 506,  579),
            s!( 460,  588), s!( 480,  584), s!( 476,  590), s!( 496,  577), s!( 495,  575), s!( 488,  578), s!( 547,  560), s!( 492,  563),
            s!( 442,  580), s!( 463,  581), s!( 462,  582), s!( 492,  567), s!( 477,  575), s!( 484,  568), s!( 489,  563), s!( 464,  574),
            s!( 427,  575), s!( 424,  585), s!( 448,  584), s!( 478,  574), s!( 467,  569), s!( 462,  579), s!( 473,  558), s!( 462,  559),
            s!( 412,  568), s!( 432,  568), s!( 438,  576), s!( 457,  562), s!( 449,  561), s!( 458,  557), s!( 483,  535), s!( 458,  550),
            s!( 431,  561), s!( 446,  551), s!( 450,  559), s!( 475,  543), s!( 464,  548), s!( 476,  554), s!( 462,  542), s!( 426,  576),
            s!( 434,  555), s!( 446,  554), s!( 440,  564), s!( 451,  563), s!( 458,  550), s!( 437,  559), s!( 456,  547), s!( 433,  551),
        ],
        [
            s!( 506,  572), s!( 487,  585), s!( 493,  584), s!( 507,  583), s!( 506,  593), s!( 484,  585), s!( 519,  586), s!( 525,  579),
            s!( 470,  588), s!( 459,  604), s!( 472,  595), s!( 512,  583), s!( 495,  592), s!( 487,  583), s!( 473,  596), s!( 489,  576),
            s!( 467,  581), s!( 499,  577), s!( 477,  587), s!( 489,  577), s!( 514,  579), s!( 476,  580), s!( 517,  566), s!( 466,  577),
            s!( 445,  577), s!( 453,  589), s!( 461,  584), s!( 466,  579), s!( 476,  574), s!( 475,  569), s!( 457,  582), s!( 459,  577),
            s!( 440,  574), s!( 447,  583), s!( 457,  580), s!( 463,  574), s!( 480,  575), s!( 435,  581), s!( 457,  573), s!( 437,  566),
            s!( 439,  559), s!( 450,  564), s!( 451,  569), s!( 447,  569), s!( 460,  573), s!( 445,  553), s!( 466,  552), s!( 432,  538),
            s!( 437,  569), s!( 455,  550), s!( 463,  556), s!( 458,  549), s!( 444,  578), s!( 431,  558), s!( 450,  555), s!( 430,  548),
            s!( 465,  549), s!( 467,  554), s!( 460,  559), s!( 462,  551), s!( 441,  560), s!( 431,  559), s!( 459,  544), s!( 459,  532),
        ],
        [
            s!( 527,  565), s!( 516,  581), s!( 522,  578), s!( 514,  589), s!( 514,  578), s!( 491,  582), s!( 520,  579), s!( 529,  577),
            s!( 513,  571), s!( 511,  590), s!( 514,  578), s!( 509,  591), s!( 504,  579), s!( 502,  584), s!( 504,  587), s!( 530,  567),
            s!( 472,  578), s!( 513,  578), s!( 492,  576), s!( 514,  574), s!( 512,  571), s!( 506,  582), s!( 546,  558), s!( 469,  576),
            s!( 473,  573), s!( 474,  584), s!( 496,  574), s!( 476,  594), s!( 457,  579), s!( 487,  572), s!( 472,  580), s!( 468,  570),
            s!( 451,  575), s!( 456,  580), s!( 449,  587), s!( 457,  588), s!( 462,  573), s!( 463,  579), s!( 458,  577), s!( 455,  562),
            s!( 433,  569), s!( 466,  559), s!( 453,  565), s!( 456,  569), s!( 474,  548), s!( 462,  561), s!( 477,  555), s!( 437,  560),
            s!( 427,  552), s!( 440,  553), s!( 454,  557), s!( 436,  569), s!( 465,  552), s!( 471,  551), s!( 455,  561), s!( 441,  553),
            s!( 427,  566), s!( 445,  576), s!( 439,  572), s!( 432,  568), s!( 459,  551), s!( 447,  557), s!( 450,  555), s!( 427,  568),
        ],
        [
            s!( 530,  552), s!( 511,  569), s!( 505,  588), s!( 513,  572), s!( 506,  569), s!( 488,  579), s!( 513,  569), s!( 515,  560),
            s!( 496,  569), s!( 513,  572), s!( 486,  587), s!( 511,  571), s!( 507,  574), s!( 500,  573), s!( 478,  583), s!( 495,  575),
            s!( 472,  569), s!( 510,  552), s!( 464,  578), s!( 480,  573), s!( 500,  561), s!( 501,  562), s!( 517,  559), s!( 487,  561),
            s!( 458,  564), s!( 465,  565), s!( 459,  575), s!( 455,  579), s!( 474,  572), s!( 475,  561), s!( 483,  561), s!( 465,  557),
            s!( 421,  565), s!( 445,  564), s!( 467,  571), s!( 451,  569), s!( 435,  580), s!( 448,  578), s!( 470,  561), s!( 453,  550),
            s!( 438,  551), s!( 429,  564), s!( 479,  551), s!( 460,  552), s!( 445,  560), s!( 461,  537), s!( 480,  533), s!( 447,  533),
            s!( 426,  557), s!( 455,  549), s!( 443,  569), s!( 452,  557), s!( 460,  552), s!( 440,  552), s!( 470,  539), s!( 436,  535),
            s!( 349,  594), s!( 386,  588), s!( 431,  573), s!( 474,  542), s!( 468,  551), s!( 462,  540), s!( 473,  538), s!( 458,  548),
        ],
        [
            s!( 529,  518), s!( 484,  585), s!( 506,  570), s!( 515,  557), s!( 511,  558), s!( 487,  576), s!( 501,  569), s!( 503,  565),
            s!( 488,  558), s!( 479,  577), s!( 503,  574), s!( 491,  578), s!( 504,  565), s!( 517,  556), s!( 489,  563), s!( 480,  577),
            s!( 477,  542), s!( 493,  573), s!( 487,  569), s!( 501,  553), s!( 516,  544), s!( 494,  562), s!( 513,  554), s!( 506,  551),
            s!( 444,  563), s!( 455,  578), s!( 491,  559), s!( 476,  567), s!( 485,  565), s!( 484,  553), s!( 483,  552), s!( 483,  548),
            s!( 444,  548), s!( 454,  578), s!( 459,  571), s!( 452,  565), s!( 453,  567), s!( 445,  556), s!( 458,  553), s!( 439,  558),
            s!( 415,  557), s!( 448,  564), s!( 475,  558), s!( 462,  549), s!( 461,  548), s!( 469,  535), s!( 460,  537), s!( 444,  528),
            s!( 394,  543), s!( 463,  571), s!( 445,  561), s!( 445,  555), s!( 463,  539), s!( 466,  526), s!( 450,  534), s!( 446,  528),
            s!( 344,  583), s!( 456,  547), s!( 471,  549), s!( 471,  547), s!( 464,  544), s!( 459,  536), s!( 477,  531), s!( 462,  545),
        ],
        [
            s!( 497,  570), s!( 493,  570), s!( 504,  574), s!( 517,  552), s!( 509,  562), s!( 475,  565), s!( 509,  573), s!( 504,  569),
            s!( 476,  559), s!( 492,  566), s!( 503,  576), s!( 508,  563), s!( 524,  550), s!( 503,  562), s!( 505,  565), s!( 486,  576),
            s!( 455,  566), s!( 484,  585), s!( 507,  562), s!( 501,  554), s!( 505,  565), s!( 494,  542), s!( 534,  539), s!( 497,  559),
            s!( 432,  569), s!( 476,  571), s!( 477,  582), s!( 479,  572), s!( 478,  573), s!( 483,  544), s!( 482,  553), s!( 459,  557),
            s!( 436,  568), s!( 475,  579), s!( 460,  573), s!( 482,  551), s!( 467,  557), s!( 448,  560), s!( 467,  561), s!( 446,  565),
            s!( 436,  573), s!( 478,  556), s!( 477,  569), s!( 469,  549), s!( 454,  546), s!( 468,  533), s!( 466,  546), s!( 455,  546),
            s!( 429,  538), s!( 480,  570), s!( 475,  560), s!( 480,  545), s!( 456,  543), s!( 470,  517), s!( 475,  526), s!( 445,  549),
            s!( 413,  529), s!( 475,  550), s!( 480,  539), s!( 462,  551), s!( 461,  543), s!( 462,  513), s!( 457,  547), s!( 448,  539),
        ],
        [
            s!( 493,  567), s!( 505,  558), s!( 479,  561), s!( 519,  549), s!( 511,  568), s!( 504,  588), s!( 503,  556), s!( 511,  578),
            s!( 476,  580), s!( 499,  569), s!( 485,  553), s!( 503,  557), s!( 517,  565), s!( 538,  556), s!( 521,  565), s!( 503,  575),
            s!( 471,  585), s!( 486,  567), s!( 483,  566), s!( 501,  553), s!( 517,  562), s!( 537,  556), s!( 572,  542), s!( 498,  568),
            s!( 457,  584), s!( 452,  575), s!( 447,  565), s!( 485,  562), s!( 478,  556), s!( 495,  573), s!( 529,  555), s!( 492,  558),
            s!( 435,  574), s!( 440,  574), s!( 444,  560), s!( 450,  550), s!( 481,  551), s!( 475,  559), s!( 513,  537), s!( 466,  541),
            s!( 420,  560), s!( 438,  553), s!( 437,  549), s!( 445,  550), s!( 457,  546), s!( 465,  558), s!( 484,  543), s!( 460,  533),
            s!( 427,  558), s!( 447,  551), s!( 441,  541), s!( 452,  550), s!( 452,  542), s!( 468,  550), s!( 497,  527), s!( 456,  555),
            s!( 429,  544), s!( 445,  541), s!( 444,  542), s!( 449,  545), s!( 450,  544), s!( 464,  530), s!( 470,  547), s!( 421,  524),
        ],
        [
            s!( 505,  558), s!( 499,  558), s!( 494,  562), s!( 508,  549), s!( 526,  541), s!( 499,  575), s!( 499,  594), s!( 509,  545),
            s!( 491,  582), s!( 498,  576), s!( 502,  562), s!( 518,  556), s!( 523,  554), s!( 503,  552), s!( 529,  567), s!( 513,  541),
            s!( 482,  576), s!( 493,  564), s!( 498,  567), s!( 518,  553), s!( 527,  558), s!( 537,  557), s!( 532,  559), s!( 512,  551),
            s!( 459,  569), s!( 482,  557), s!( 475,  558), s!( 486,  551), s!( 491,  559), s!( 497,  576), s!( 488,  571), s!( 491,  539),
            s!( 449,  557), s!( 447,  563), s!( 456,  560), s!( 468,  556), s!( 465,  558), s!( 449,  557), s!( 490,  550), s!( 455,  536),
            s!( 439,  550), s!( 452,  537), s!( 455,  546), s!( 471,  541), s!( 464,  558), s!( 467,  544), s!( 484,  544), s!( 449,  539),
            s!( 437,  549), s!( 454,  540), s!( 453,  560), s!( 461,  541), s!( 456,  549), s!( 471,  540), s!( 510,  537), s!( 410,  564),
            s!( 455,  548), s!( 470,  535), s!( 465,  533), s!( 471,  537), s!( 478,  553), s!( 477,  537), s!( 475,  526), s!( 388,  542),
        ],
        [
            s!( 501,  558), s!( 491,  558), s!( 482,  564), s!( 510,  548), s!( 494,  544), s!( 470,  575), s!( 499,  547), s!( 503,  546),
            s!( 498,  572), s!( 501,  564), s!( 504,  536), s!( 545,  539), s!( 511,  528), s!( 511,  552), s!( 512,  546), s!( 504,  540),
            s!( 459,  567), s!( 479,  558), s!( 480,  548), s!( 499,  532), s!( 496,  546), s!( 497,  558), s!( 543,  527), s!( 492,  547),
            s!( 446,  564), s!( 474,  552), s!( 470,  558), s!( 495,  536), s!( 470,  545), s!( 487,  553), s!( 494,  549), s!( 469,  549),
            s!( 425,  551), s!( 432,  563), s!( 458,  534), s!( 478,  526), s!( 469,  526), s!( 465,  551), s!( 483,  537), s!( 469,  541),
            s!( 417,  552), s!( 448,  529), s!( 438,  517), s!( 475,  516), s!( 466,  529), s!( 466,  530), s!( 499,  511), s!( 467,  527),
            s!( 427,  537), s!( 454,  538), s!( 452,  523), s!( 483,  532), s!( 483,  517), s!( 474,  544), s!( 482,  533), s!( 432,  544),
            s!( 428,  541), s!( 454,  520), s!( 440,  533), s!( 456,  533), s!( 464,  524), s!( 434,  546), s!( 469,  545), s!( 435,  542),
        ],
        [
            s!( 516,  557), s!( 494,  550), s!( 503,  550), s!( 512,  549), s!( 514,  581), s!( 477,  540), s!( 513,  540), s!( 527,  558),
            s!( 492,  561), s!( 477,  565), s!( 501,  560), s!( 509,  543), s!( 506,  567), s!( 483,  544), s!( 478,  560), s!( 504,  554),
            s!( 472,  568), s!( 504,  550), s!( 472,  554), s!( 489,  550), s!( 511,  564), s!( 484,  555), s!( 527,  556), s!( 473,  560),
            s!( 456,  563), s!( 455,  557), s!( 469,  558), s!( 468,  554), s!( 483,  564), s!( 492,  554), s!( 454,  568), s!( 479,  569),
            s!( 467,  547), s!( 439,  540), s!( 464,  539), s!( 455,  539), s!( 484,  554), s!( 438,  559), s!( 485,  548), s!( 462,  551),
            s!( 442,  547), s!( 446,  519), s!( 465,  520), s!( 447,  541), s!( 468,  539), s!( 450,  543), s!( 482,  534), s!( 458,  530),
            s!( 437,  547), s!( 464,  527), s!( 471,  527), s!( 460,  539), s!( 447,  532), s!( 429,  543), s!( 448,  547), s!( 448,  537),
            s!( 456,  541), s!( 471,  532), s!( 465,  534), s!( 454,  534), s!( 443,  540), s!( 435,  554), s!( 471,  548), s!( 473,  538),
        ],
        [
            s!( 517,  531), s!( 505,  546), s!( 505,  539), s!( 518,  577), s!( 511,  558), s!( 486,  552), s!( 518,  546), s!( 520,  560),
            s!( 506,  557), s!( 507,  556), s!( 509,  550), s!( 511,  573), s!( 515,  553), s!( 503,  552), s!( 506,  558), s!( 531,  554),
            s!( 465,  560), s!( 502,  551), s!( 489,  549), s!( 515,  570), s!( 504,  555), s!( 502,  565), s!( 558,  549), s!( 478,  567),
            s!( 475,  562), s!( 472,  575), s!( 491,  557), s!( 481,  579), s!( 468,  571), s!( 492,  567), s!( 470,  562), s!( 466,  556),
            s!( 449,  569), s!( 449,  549), s!( 450,  555), s!( 462,  566), s!( 459,  558), s!( 455,  561), s!( 469,  556), s!( 460,  549),
            s!( 444,  539), s!( 469,  539), s!( 459,  539), s!( 455,  546), s!( 477,  534), s!( 466,  544), s!( 481,  536), s!( 437,  544),
            s!( 444,  546), s!( 452,  549), s!( 458,  544), s!( 441,  554), s!( 469,  545), s!( 477,  537), s!( 463,  522), s!( 438,  540),
            s!( 415,  567), s!( 435,  550), s!( 441,  543), s!( 431,  564), s!( 446,  535), s!( 452,  545), s!( 469,  535), s!( 438,  547),
        ],
        [
            s!( 508,  493), s!( 501,  528), s!( 502,  576), s!( 513,  553), s!( 515,  566), s!( 486,  560), s!( 510,  538), s!( 516,  556),
            s!( 487,  548), s!( 492,  545), s!( 481,  563), s!( 509,  559), s!( 511,  561), s!( 499,  556), s!( 484,  566), s!( 512,  552),
            s!( 466,  558), s!( 501,  549), s!( 474,  583), s!( 487,  575), s!( 507,  559), s!( 504,  558), s!( 528,  533), s!( 507,  543),
            s!( 463,  564), s!( 468,  581), s!( 480,  600), s!( 460,  581), s!( 467,  560), s!( 481,  540), s!( 496,  555), s!( 470,  555),
            s!( 432,  555), s!( 442,  551), s!( 469,  571), s!( 447,  548), s!( 446,  564), s!( 437,  556), s!( 464,  558), s!( 463,  549),
            s!( 439,  539), s!( 435,  557), s!( 483,  549), s!( 453,  541), s!( 446,  549), s!( 459,  537), s!( 484,  524), s!( 451,  527),
            s!( 442,  557), s!( 463,  543), s!( 449,  563), s!( 466,  551), s!( 466,  543), s!( 439,  551), s!( 480,  528), s!( 439,  535),
            s!( 364,  571), s!( 386,  566), s!( 436,  555), s!( 478,  543), s!( 472,  534), s!( 465,  539), s!( 483,  522), s!( 458,  544),
        ],
        [
            s!( 503,  469), s!( 496,  586), s!( 494,  546), s!( 515,  548), s!( 507,  544), s!( 482,  541), s!( 509,  544), s!( 507,  556),
            s!( 484,  541), s!( 477,  571), s!( 495,  543), s!( 494,  558), s!( 518,  563), s!( 521,  544), s!( 497,  558), s!( 488,  560),
            s!( 479,  559), s!( 487,  580), s!( 495,  566), s!( 504,  560), s!( 523,  543), s!( 496,  557), s!( 528,  540), s!( 505,  553),
            s!( 447,  561), s!( 467,  597), s!( 490,  568), s!( 486,  571), s!( 488,  561), s!( 494,  547), s!( 496,  537), s!( 481,  550),
            s!( 446,  565), s!( 456,  572), s!( 462,  559), s!( 449,  566), s!( 456,  552), s!( 455,  558), s!( 467,  538), s!( 443,  571),
            s!( 423,  548), s!( 449,  554), s!( 461,  552), s!( 457,  543), s!( 464,  541), s!( 470,  537), s!( 460,  527), s!( 445,  536),
            s!( 419,  546), s!( 464,  562), s!( 449,  549), s!( 447,  552), s!( 470,  538), s!( 483,  525), s!( 462,  534), s!( 447,  540),
            s!( 364,  561), s!( 463,  555), s!( 468,  524), s!( 474,  548), s!( 459,  526), s!( 459,  534), s!( 478,  533), s!( 454,  554),
        ],
        [
            s!( 492,  545), s!( 492,  559), s!( 502,  557), s!( 518,  565), s!( 509,  553), s!( 474,  551), s!( 509,  557), s!( 506,  555),
            s!( 483,  569), s!( 501,  574), s!( 510,  565), s!( 503,  557), s!( 521,  552), s!( 512,  548), s!( 501,  558), s!( 492,  557),
            s!( 459,  580), s!( 483,  576), s!( 502,  565), s!( 508,  568), s!( 504,  553), s!( 499,  535), s!( 547,  536), s!( 498,  556),
            s!( 443,  573), s!( 474,  576), s!( 486,  594), s!( 493,  573), s!( 477,  568), s!( 486,  542), s!( 503,  567), s!( 470,  562),
            s!( 436,  566), s!( 467,  570), s!( 468,  576), s!( 484,  561), s!( 470,  563), s!( 455,  568), s!( 472,  560), s!( 448,  567),
            s!( 436,  565), s!( 471,  558), s!( 474,  552), s!( 465,  548), s!( 455,  542), s!( 474,  533), s!( 466,  543), s!( 449,  544),
            s!( 432,  539), s!( 469,  550), s!( 475,  548), s!( 471,  544), s!( 457,  538), s!( 470,  518), s!( 471,  529), s!( 431,  550),
            s!( 413,  525), s!( 476,  543), s!( 480,  531), s!( 453,  545), s!( 463,  530), s!( 471,  516), s!( 465,  543), s!( 436,  517),
        ],
    ],
    [
        [
            s!(  -4,  -30), s!( -39,    0), s!( -50,    6), s!( -36,   12), s!(  -8,    1), s!(  10,    2), s!(  -6,   11), s!(  -4,   -8),
            s!( -16,    4), s!( -10,   -1), s!(  -2,   -2), s!(   4,   -2), s!(  21,  -11), s!(  46,  -15), s!(  63,   -6), s!(   2,    2),
            s!( -16,    1), s!( -26,   -3), s!( -17,   -6), s!(  -7,   -7), s!(  17,  -14), s!(  48,  -12), s!(  10,   -2), s!(   0,   -9),
            s!( -14,    8), s!( -23,   -2), s!(   2,  -10), s!(  -6,    4), s!(  31,  -14), s!(   4,   -9), s!(  23,   -6), s!(  22,  -32),
            s!(  -6,    6), s!( -21,    6), s!( -21,   10), s!( -14,    6), s!(  -1,   -3), s!(  11,   -9), s!(   2,    2), s!(  21,  -30),
            s!( -24,   10), s!( -20,   14), s!(  -8,   11), s!( -18,   22), s!(  -0,    4), s!(   9,   -0), s!(   1,   12), s!(  19,  -11),
            s!( -18,   16), s!( -16,   17), s!( -10,   16), s!( -18,   14), s!(  -7,   19), s!(  -2,    7), s!(   8,    6), s!(  18,  -15),
            s!( -18,    2), s!( -21,    9), s!( -14,    8), s!( -17,   19), s!( -12,   11), s!(  -5,    6), s!(   0,    8), s!(  -9,   -1),
        ],
        [
            s!(  35,   -5), s!(  29,    9), s!(  23,   15), s!(  23,   10), s!(  29,    1), s!( -11,   12), s!(   1,    3), s!( -13,    8),
            s!( -23,   25), s!( -32,   27), s!( -19,   23), s!( -27,   27), s!( -28,   20), s!( -19,   27), s!(  -1,   -3), s!( -16,   17),
            s!( -27,   22), s!( -17,   14), s!( -25,   22), s!( -25,   17), s!(  10,    7), s!(  -7,   15), s!(   6,   -3), s!(  -7,   -7),
            s!( -13,   26), s!( -28,   22), s!( -12,   21), s!( -18,   22), s!( -13,   15), s!( -25,   17), s!(   2,  -27), s!( -16,   -4),
            s!( -13,   23), s!( -25,   20), s!( -22,   21), s!( -20,   19), s!( -10,    9), s!( -15,    7), s!(  -8,   -1), s!( -16,   -0),
            s!(  -9,   25), s!( -19,   26), s!( -12,   21), s!( -18,   21), s!( -15,   19), s!(  -7,    9), s!(   4,   -7), s!(  -7,   16),
            s!( -11,   32), s!(  -9,   25), s!( -15,   29), s!( -16,   33), s!( -11,   22), s!( -14,   22), s!(  -0,   -3), s!( -10,   13),
            s!( -15,   25), s!( -18,   22), s!( -14,   28), s!( -13,   25), s!(  -8,   17), s!(  -7,   16), s!(  -8,   12), s!( -13,    3),
        ],
        [
            s!(  10,    9), s!( -10,   12), s!(  -8,   10), s!(   6,    5), s!( -36,   21), s!(   5,    6), s!( -14,    6), s!(  20,   -3),
            s!(  11,    4), s!(  26,   -6), s!(  12,   -2), s!(  31,   -6), s!( -33,    7), s!(   5,    6), s!(  -3,    7), s!(  38,   -5),
            s!(  18,    2), s!( -10,    6), s!(  10,   -4), s!(   1,   -3), s!( -29,   14), s!(  10,   -5), s!(   7,   -6), s!(  29,   -4),
            s!( -14,   15), s!( -17,    8), s!(  -5,    5), s!(   5,   -1), s!( -34,   14), s!( -13,   -9), s!( -19,    3), s!(  -0,   -3),
            s!( -20,    4), s!( -18,   -1), s!( -14,    2), s!( -20,   -2), s!( -31,    8), s!(  22,  -24), s!( -10,   -0), s!(  12,  -10),
            s!( -10,    5), s!( -20,   -1), s!( -16,   -3), s!( -11,    5), s!( -16,    4), s!(   4,  -16), s!( -11,   -1), s!(   7,    9),
            s!( -20,   12), s!( -10,    4), s!( -22,    8), s!( -20,   10), s!( -20,    7), s!(  -2,   -5), s!( -29,   11), s!(   4,   -0),
            s!( -31,   23), s!( -21,    9), s!( -23,   13), s!( -16,    8), s!( -10,   -0), s!(  -3,    2), s!( -17,    9), s!( -17,   17),
        ],
        [
            s!(  28,   23), s!(   3,   22), s!(  11,    6), s!( -23,   14), s!(   2,   14), s!(  -1,   13), s!( -17,    9), s!(  -1,   16),
            s!(  28,    1), s!(  15,   -2), s!(  22,   -7), s!( -13,   -2), s!(   2,    3), s!(  -5,    1), s!( -30,    4), s!( -10,   14),
            s!(   3,   11), s!( -20,    3), s!(  -7,    2), s!( -32,   12), s!(   4,   -7), s!( -15,    6), s!( -10,   10), s!( -32,   17),
            s!( -20,   19), s!( -30,   10), s!(  -8,    1), s!( -43,   17), s!( -26,   -8), s!( -34,   13), s!( -35,    6), s!( -25,   15),
            s!( -25,    9), s!( -12,   -0), s!( -17,    7), s!( -22,   -2), s!( -23,   -7), s!( -45,    7), s!( -49,    1), s!( -39,   15),
            s!( -31,   17), s!(  -9,   -7), s!( -25,    2), s!( -21,    5), s!( -14,  -15), s!( -43,    8), s!( -36,    4), s!( -33,   18),
            s!(  -9,   -9), s!(  -9,   -3), s!( -15,   -8), s!(  -7,   -9), s!( -10,  -17), s!( -25,    3), s!( -25,   -6), s!( -31,    0),
            s!( -31,   13), s!( -23,    8), s!( -13,    2), s!( -18,   -1), s!(  -9,    4), s!( -22,    5), s!( -29,    5), s!( -34,   16),
        ],
        [
            s!( -10,   26), s!(   6,   15), s!( -10,   12), s!(   5,    6), s!( -11,   20), s!(  11,    6), s!(   8,   17), s!(  -0,   30),
            s!(  -2,    9), s!(  19,   -6), s!(  -5,    2), s!(   3,    5), s!(  -4,   11), s!(  20,    4), s!(  37,    1), s!(   4,   14),
            s!(   8,    4), s!(   5,    1), s!(  -1,   12), s!(   4,  -11), s!( -14,   15), s!( -10,   11), s!( -11,    9), s!(  22,   -1),
            s!( -18,    6), s!(  -8,   -7), s!( -19,   10), s!(   2,   -7), s!( -18,   18), s!(  -7,    8), s!( -32,    9), s!(  -5,   19),
            s!(   2,   -4), s!(   0,   -6), s!( -14,    6), s!(  15,  -13), s!( -16,    7), s!(   5,  -12), s!( -21,    2), s!(  -6,    6),
            s!(  -2,    1), s!( -23,    3), s!(  -4,    7), s!(  20,  -21), s!( -17,    1), s!(  -2,   -4), s!( -12,   -1), s!(  -8,   -0),
            s!(  -0,  -12), s!(  -8,   -3), s!( -15,   -4), s!(  -1,   -4), s!(  -2,    2), s!(   4,   -7), s!(   2,  -11), s!(  10,    2),
            s!( -28,   21), s!( -10,    0), s!(  -8,    7), s!(   9,   -3), s!(  -7,   11), s!(  -9,   10), s!( -16,   12), s!( -34,   30),
        ],
        [
            s!(   7,   -3), s!(  -2,    9), s!(   1,    5), s!(   1,   -1), s!(   2,    4), s!(  10,   15), s!(   2,    6), s!(  14,    8),
            s!(  20,  -11), s!(  24,  -12), s!(  -1,   -9), s!(  -9,   -3), s!(  18,   -4), s!( -31,   22), s!( -10,   10), s!(   0,    9),
            s!(  31,  -23), s!(  28,  -19), s!(   8,   -7), s!( -11,   13), s!(  -6,   -8), s!( -16,    3), s!( -25,    5), s!(  -8,    6),
            s!(  -1,   -4), s!(   5,  -11), s!(   7,   -7), s!( -10,   -4), s!(  -9,   -0), s!( -20,   15), s!( -23,    4), s!(  -9,    9),
            s!( -15,   -8), s!(  10,  -15), s!(  15,  -40), s!(  -8,   -1), s!( -19,   -3), s!( -18,   11), s!( -23,    0), s!( -17,   15),
            s!(  -2,   -0), s!(   4,  -12), s!(  16,  -20), s!(  -0,   -0), s!( -32,   15), s!( -16,   14), s!( -30,   13), s!( -18,   17),
            s!( -26,    6), s!(   8,  -13), s!(  11,  -23), s!(   0,   -4), s!( -13,    9), s!( -11,   18), s!( -23,   15), s!(   6,   10),
            s!( -18,   -1), s!(  -5,   -3), s!(  10,  -13), s!( -13,    7), s!( -13,    9), s!( -18,   15), s!( -26,   21), s!( -24,   20),
        ],
        [
            s!(  -9,   19), s!(   0,    2), s!(   5,   -7), s!(   7,  -10), s!(   8,  -13), s!(  10,   -6), s!(   2,  -14), s!(  15,  -19),
            s!(   9,  -13), s!(  -2,   -8), s!( -26,   -8), s!(  19,  -17), s!(  -6,    6), s!( -12,    0), s!(   2,   -7), s!( -20,   -1),
            s!(   4,  -11), s!(  26,   -3), s!(  -4,  -15), s!(  22,  -10), s!(   2,   -8), s!( -41,    3), s!( -27,    3), s!( -27,    7),
            s!(  32,  -36), s!(  27,  -25), s!(   2,   -9), s!(  14,   -5), s!(  -5,   -1), s!(   6,    7), s!( -11,    1), s!(  -3,   13),
            s!(   9,   -7), s!(  19,  -26), s!(   2,  -15), s!(  18,   -3), s!(   1,    4), s!(   6,    1), s!( -14,   -3), s!(  -1,   -7),
            s!(   3,   -3), s!(  29,  -25), s!(  -4,    1), s!(  12,  -13), s!(  -4,    9), s!(  -6,   11), s!(  -8,    9), s!( -19,    9),
            s!( -12,   10), s!(  14,   -5), s!( -15,    8), s!(  14,   -6), s!( -12,   13), s!(   4,    9), s!( -14,   13), s!(  -5,   22),
            s!( -10,    5), s!(   7,   -8), s!(  -2,   -3), s!(  -7,   15), s!(  -6,   16), s!( -11,   23), s!( -16,   21), s!( -11,   21),
        ],
        [
            s!(   1,    5), s!(   9,    6), s!(   4,   -3), s!(  -0,  -17), s!(  -6,  -13), s!( -14,  -25), s!(  -9,  -19), s!( -11,  -13),
            s!(   0,   -4), s!(  10,  -10), s!(   1,   -7), s!(  -5,   -6), s!(   7,   -8), s!(  -0,   -1), s!( -10,    6), s!( -14,   -5),
            s!(  -6,   -5), s!(   9,  -18), s!(  17,  -13), s!(   2,  -18), s!(  -0,  -11), s!(  -8,  -10), s!( -23,   -6), s!( -14,    4),
            s!(   0,  -18), s!(  35,  -31), s!( -14,  -11), s!(  -1,   -5), s!(  16,   -9), s!(  30,   -4), s!(   2,    6), s!( -12,    3),
            s!(   7,  -14), s!(  14,   -8), s!(   3,   -6), s!(   6,  -12), s!(   0,   -4), s!(   1,    7), s!( -16,    4), s!(  -1,   -5),
            s!(  12,  -26), s!(  14,  -11), s!(   1,   -7), s!(   8,    2), s!(   5,    6), s!(   4,   11), s!(   1,    6), s!(  -4,   -6),
            s!(  19,   -8), s!( -21,   27), s!(  -9,    1), s!( -12,    7), s!(  -4,   18), s!(  -3,    7), s!(  -6,   -7), s!( -14,   -1),
            s!(  -9,   10), s!(   3,   13), s!(  -3,   -3), s!( -10,   11), s!( -18,   21), s!(  -9,   19), s!(  -6,    9), s!(  -6,   12),
        ],
        [
            s!(  -6,  -24), s!( -20,   -9), s!( -20,   -8), s!( -13,   -3), s!(  -4,   -2), s!(  12,    6), s!(  -4,   -8), s!(  -1,  -11),
            s!(  -5,  -16), s!(  -9,   -7), s!(  -0,  -15), s!(   3,  -16), s!(   9,  -16), s!(  23,  -23), s!(  25,  -25), s!(  -2,    3),
            s!(  -2,  -19), s!( -12,   -6), s!( -14,   -9), s!(  -3,   -9), s!(   6,  -12), s!(  22,  -10), s!(   2,   -1), s!(  -2,   -2),
            s!(  -2,   -1), s!(  -4,  -15), s!( -10,   -7), s!(  -0,  -20), s!(  12,  -14), s!(  10,    2), s!(  21,   -4), s!(  15,   -8),
            s!(  -0,  -10), s!(  -1,    2), s!(  -9,    2), s!( -14,    5), s!(   3,   -2), s!(  15,    2), s!(   4,   10), s!(  15,   -9),
            s!( -10,   -8), s!( -12,    1), s!(  -4,   -5), s!(  -8,    9), s!(   6,    4), s!(  17,    8), s!(  12,   13), s!(   0,    0),
            s!(   4,    1), s!(  -8,   -6), s!(   2,   -9), s!( -10,    3), s!(  -0,   -5), s!(   9,    0), s!(   7,   19), s!(  22,   -9),
            s!(   0,  -14), s!( -11,   -2), s!( -11,   -5), s!( -12,   -0), s!(  -2,    6), s!(   9,    6), s!(  10,   20), s!(  11,  -10),
        ],
        [
            s!(  20,  -31), s!(  19,  -31), s!(  11,  -34), s!(  13,  -13), s!(  14,  -13), s!(  -4,  -23), s!(   0,   -4), s!(   2,   -7),
            s!( -27,  -26), s!( -25,  -24), s!( -18,  -27), s!( -18,  -23), s!( -14,  -23), s!(  -3,  -17), s!(   2,    3), s!(   4,  -19),
            s!( -23,  -21), s!(  -3,  -13), s!(  -5,  -18), s!(  -2,  -14), s!(   9,  -20), s!(   9,  -21), s!(  11,   -1), s!(   2,  -13),
            s!( -12,  -15), s!( -20,  -16), s!( -12,  -32), s!( -14,  -28), s!(  -6,   -8), s!(  -5,   -0), s!(  10,   -7), s!(   1,  -14),
            s!(  -5,   -7), s!( -12,    2), s!(  -8,  -13), s!(   6,   -6), s!(  -6,   -2), s!(  -6,    3), s!(   7,   -1), s!(  -4,    0),
            s!(   2,   -4), s!(  -9,   -9), s!(  -4,   -4), s!(  -3,   -6), s!(   5,   11), s!(   8,   10), s!(  12,   -9), s!(   5,   10),
            s!(   0,   -0), s!(   3,  -10), s!(   7,   -6), s!(   2,    5), s!(  -5,   -5), s!(   6,    7), s!(   6,  -12), s!(   1,    4),
            s!(  -0,  -24), s!(  -8,  -12), s!(  -3,  -18), s!(   0,  -14), s!(   9,  -16), s!(   8,   -7), s!(  12,    1), s!(  13,    3),
        ],
        [
            s!(   2,  -44), s!(  -8,  -33), s!(  -5,  -11), s!(   3,  -11), s!( -11,  -27), s!(   4,    6), s!(  -1,  -19), s!(   6,  -16),
            s!(  -8,  -36), s!(   9,  -45), s!(  -4,  -49), s!(   3,  -46), s!( -13,  -42), s!(  -4,  -20), s!(  -2,  -43), s!(   8,  -32),
            s!(  -8,  -32), s!( -20,  -33), s!(   3,  -35), s!(  -3,  -29), s!( -17,  -30), s!(  -2,  -15), s!(   2,  -23), s!(   9,  -24),
            s!(  -2,  -33), s!(  -7,  -36), s!(  -6,  -32), s!(   2,  -35), s!( -17,  -23), s!(  -7,   -8), s!(   2,  -23), s!(  -5,  -23),
            s!(  -9,  -21), s!(  -6,  -28), s!(  -5,  -29), s!(  -5,  -12), s!( -14,   -2), s!(   6,  -16), s!(  -1,   -6), s!(  10,  -17),
            s!(  -2,   -9), s!(  -9,  -36), s!(  -8,  -33), s!(  -8,  -24), s!(  -1,   -5), s!(   1,  -13), s!( -12,   -5), s!(   5,   -7),
            s!(  -5,   -7), s!(  -9,  -16), s!( -15,  -17), s!(  -7,  -18), s!( -13,  -13), s!(  -6,  -17), s!( -13,  -14), s!(  -4,  -18),
            s!(  -8,  -23), s!( -26,  -18), s!( -13,  -33), s!(  -3,  -25), s!(  -8,  -15), s!(   2,  -11), s!(  -8,  -24), s!(  -3,   -9),
        ],
        [
            s!(   6,  -17), s!(  -2,  -11), s!(   3,  -12), s!( -11,  -32), s!(  -1,   -1), s!(  -2,  -23), s!(  -7,  -22), s!(  -3,  -20),
            s!(   9,  -32), s!(  13,  -29), s!(  21,  -34), s!(  -9,  -41), s!(  -1,  -11), s!(  -5,  -43), s!( -16,  -29), s!( -10,  -25),
            s!( -10,  -29), s!( -14,  -19), s!( -14,  -25), s!( -19,  -29), s!(  -3,   -9), s!(  -7,  -18), s!(  -6,  -22), s!( -15,  -19),
            s!( -10,  -25), s!( -22,  -27), s!( -13,  -28), s!( -25,  -19), s!(  -1,   -1), s!( -10,   -8), s!( -22,  -19), s!( -17,  -27),
            s!(  -5,  -16), s!(  -9,  -21), s!(  -9,  -17), s!( -10,   -4), s!(  -4,   -6), s!( -11,    2), s!( -14,  -15), s!( -14,  -13),
            s!( -14,   -3), s!(   2,  -19), s!(  -5,  -12), s!(  -5,  -13), s!(  -2,  -13), s!( -18,    3), s!( -13,   -7), s!( -10,    2),
            s!(   2,   -4), s!(   4,  -16), s!(   2,  -17), s!(   0,  -23), s!(  -7,  -14), s!( -19,   -9), s!(  -7,   -2), s!(  -5,   -3),
            s!(  -9,  -19), s!(  -6,  -16), s!(   9,  -22), s!(   8,  -25), s!(   1,  -14), s!(  -5,  -17), s!(  -9,  -10), s!( -10,  -16),
        ],
        [
            s!(  -1,   -3), s!(   1,  -12), s!(  -3,  -30), s!(   0,   -9), s!(  -2,  -18), s!(  -0,  -25), s!(  -1,  -18), s!(  -4,  -16),
            s!(   0,  -17), s!(   6,  -26), s!(  -3,  -29), s!(   1,   -8), s!(  -4,  -23), s!(   2,  -17), s!(   7,  -25), s!(   1,   -5),
            s!(  -5,  -17), s!(  -7,  -16), s!(  -1,  -24), s!(   2,   -3), s!(  -3,  -16), s!(  -4,   -5), s!( -10,   -5), s!(   2,   -7),
            s!( -12,  -23), s!(  -5,  -23), s!(  -8,  -19), s!(  -1,   -6), s!(  -9,  -11), s!(  -5,  -17), s!( -12,  -19), s!(  -5,   -4),
            s!(   6,   -1), s!(   0,  -16), s!(  -7,   -8), s!(  -5,  -20), s!(  -1,    3), s!(   6,   -5), s!(  -8,   -9), s!(   1,   -3),
            s!(  -2,   -1), s!(  -8,   -4), s!(  -4,   -3), s!(   2,  -14), s!(  -1,    2), s!(  -1,   -7), s!(  -3,  -11), s!(   0,   -3),
            s!(   4,    2), s!(  -4,  -14), s!(  -2,   -5), s!(  -1,   -4), s!(   3,   -9), s!(   5,    1), s!(   4,   -9), s!(   0,   -6),
            s!(  -5,  -18), s!(   2,  -14), s!(   4,  -13), s!(  12,  -11), s!(   8,  -16), s!(  -1,  -17), s!(  -6,   -3), s!( -19,    0),
        ],
        [
            s!(   1,   -9), s!(   2,  -16), s!(  -2,   -9), s!(   3,  -20), s!(  -9,  -24), s!(  -1,  -16), s!(  -0,  -22), s!(   1,  -20),
            s!(   7,  -18), s!(  11,  -37), s!(  -1,   -8), s!(  -0,  -28), s!(   4,  -18), s!( -15,  -14), s!(   0,  -18), s!(  -3,  -15),
            s!(   7,  -12), s!(   7,  -28), s!(   1,  -10), s!(  -1,  -19), s!(  -4,  -22), s!(  -8,  -11), s!( -16,   -8), s!(  -7,  -20),
            s!(  -0,  -21), s!(   1,  -15), s!(   6,   -1), s!(  -3,  -10), s!(   1,  -10), s!( -12,  -16), s!(  -8,  -13), s!(   4,   -8),
            s!(  -1,   -3), s!(   7,   -8), s!(  18,   -6), s!(   4,   -1), s!(   7,   -6), s!(  -8,  -11), s!(   2,   -4), s!(  -1,   -3),
            s!(  -0,    1), s!(   7,   -3), s!(   3,  -15), s!(  -3,  -18), s!(  -9,   14), s!(  -3,   -5), s!( -11,   -1), s!( -15,    5),
            s!( -17,  -12), s!(   8,   -8), s!(  -4,  -23), s!(   0,  -23), s!(   1,   -7), s!(  -9,   -1), s!( -10,   -5), s!(   9,   -0),
            s!(  -7,   -4), s!(  11,  -18), s!(  10,  -11), s!(  -6,  -18), s!(  -1,   -2), s!( -15,  -18), s!( -11,  -15), s!( -13,   -3),
        ],
        [
            s!(   1,    1), s!(  -1,   -7), s!(   2,  -25), s!(  -0,  -13), s!(  -1,  -15), s!(  -3,  -18), s!(  -3,  -18), s!(   3,  -18),
            s!(   6,  -17), s!(  -3,  -13), s!(  -2,  -24), s!(   1,  -25), s!(  -7,  -10), s!( -18,  -23), s!(   1,  -15), s!( -16,  -14),
            s!(   4,  -19), s!(  10,   -2), s!(   1,  -21), s!(   3,  -16), s!(  -2,   -9), s!( -23,   -4), s!( -13,   -4), s!( -12,  -10),
            s!(  21,  -16), s!(  15,   -8), s!(  -2,  -30), s!(   5,  -24), s!(  -6,  -14), s!(   8,  -16), s!(  -3,  -14), s!(  -3,  -16),
            s!(  10,  -13), s!(   5,   -8), s!(  12,   -5), s!(   1,  -13), s!(   7,   -6), s!(   2,  -16), s!(  -2,    1), s!(   5,  -15),
            s!(   2,  -13), s!(  16,   -5), s!(   2,   -7), s!(  12,   -9), s!(  -2,    2), s!(  -7,  -18), s!(   3,    0), s!(  -4,   -2),
            s!( -10,  -11), s!(   9,    2), s!( -13,   -3), s!(   4,   -7), s!(  -7,   -5), s!(  13,    1), s!(  -0,   10), s!(  -5,   -2),
            s!(  -3,   -5), s!(  17,  -17), s!(  -4,   -1), s!(  -8,  -11), s!(  -2,  -11), s!(  -1,   -8), s!( -15,   -8), s!(  -7,  -11),
        ],
        [
            s!(   1,    5), s!(   3,  -12), s!(  -5,  -10), s!(  -0,   -3), s!(  -6,  -15), s!(  -7,  -19), s!(  -3,   -8), s!(  -3,  -10),
            s!(  -0,   -4), s!(   2,  -20), s!(  -4,  -20), s!(  -4,  -13), s!(  -3,  -11), s!(   0,   -1), s!(  -8,   -9), s!( -11,  -14),
            s!(  -2,   -4), s!(   9,   -7), s!(   5,  -12), s!(  -0,   -7), s!(   1,   -7), s!(  -0,   -7), s!( -13,   -8), s!(  -7,  -10),
            s!(  -4,   -8), s!(  18,    1), s!(   1,   -3), s!(  -4,   -9), s!(   5,   -8), s!(  11,   -8), s!(  -1,   -7), s!( -10,  -13),
            s!(   3,   -3), s!(   7,    4), s!(   1,   -8), s!(   2,   -8), s!(   1,  -15), s!(   3,    3), s!(  -5,   -3), s!(  -0,    1),
            s!(   4,   -7), s!(  12,    2), s!(   2,   -0), s!(   4,   -0), s!(  -4,   -9), s!(  -0,    0), s!(   1,   -7), s!(  -4,  -10),
            s!(   6,   -6), s!( -12,   16), s!(  -3,    2), s!(  -8,    2), s!(  -3,    6), s!(  -2,   -6), s!(  -1,   -9), s!(  -6,   -5),
            s!( -11,   -8), s!(  12,    2), s!(  -5,   -4), s!(  -4,   -1), s!( -15,   -4), s!(  -3,   -6), s!(   0,   -2), s!(  10,   10),
        ],
    ],
];

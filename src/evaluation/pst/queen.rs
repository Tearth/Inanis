// ------------------------------------------------------------------------- //
// Generated at 19-11-2024 23:40:19 UTC (e = 0.113270, k = 0.0077, r = 1.00) //
// ------------------------------------------------------------------------- //

use super::*;

#[rustfmt::skip]
pub const QUEEN_PST_PATTERN: [[[PackedEval; 64]; KING_BUCKETS_COUNT]; 2] =
[
    [
        [
            s!(1045, 1082), s!(1031, 1113), s!(1053, 1096), s!(1085, 1096), s!(1073, 1104), s!(1105, 1091), s!(1119, 1083), s!(1085, 1093),
            s!(1039, 1077), s!(1015, 1108), s!(1039, 1103), s!(1046, 1113), s!(1044, 1118), s!(1085, 1087), s!(1067, 1108), s!(1105, 1060),
            s!(1043, 1049), s!(1044, 1077), s!(1058, 1046), s!(1064, 1078), s!(1074, 1078), s!(1111, 1071), s!(1095, 1065), s!(1067, 1096),
            s!(1046, 1053), s!(1053, 1059), s!(1029, 1078), s!(1051, 1064), s!(1088, 1056), s!(1077, 1082), s!(1074, 1095), s!(1079, 1078),
            s!(1058, 1033), s!(1042, 1073), s!(1058, 1057), s!(1058, 1054), s!(1071, 1047), s!(1080, 1051), s!(1087, 1056), s!(1083, 1057),
            s!(1032, 1019), s!(1060, 1013), s!(1059, 1044), s!(1059, 1039), s!(1064, 1048), s!(1085, 1031), s!(1075, 1049), s!(1066, 1027),
            s!(1045, 1066), s!(1058, 1035), s!(1061, 1020), s!(1059, 1034), s!(1070, 1029), s!(1075, 1015), s!(1084,  976), s!(1028, 1013),
            s!(1061, 1007), s!(1051, 1020), s!(1052,  985), s!(1066,  997), s!(1069,  991), s!(1058,  990), s!(1027, 1020), s!(1001, 1009),
        ],
        [
            s!(1059, 1083), s!(1033, 1112), s!(1060, 1091), s!(1078, 1080), s!(1082, 1094), s!(1089, 1091), s!(1124, 1067), s!(1084, 1093),
            s!(1040, 1090), s!(1024, 1121), s!(1054, 1084), s!(1057, 1103), s!(1038, 1130), s!(1068, 1097), s!(1052, 1130), s!(1108, 1076),
            s!(1072, 1044), s!(1055, 1076), s!(1065, 1085), s!(1057, 1096), s!(1066, 1096), s!(1086, 1105), s!(1108, 1091), s!(1073, 1098),
            s!(1066, 1048), s!(1063, 1076), s!(1069, 1067), s!(1062, 1088), s!(1077, 1075), s!(1072, 1093), s!(1072, 1099), s!(1075, 1093),
            s!(1073, 1054), s!(1066, 1063), s!(1074, 1047), s!(1066, 1064), s!(1070, 1062), s!(1065, 1076), s!(1085, 1060), s!(1074, 1061),
            s!(1069, 1016), s!(1072, 1038), s!(1080, 1034), s!(1074, 1032), s!(1074, 1043), s!(1083, 1043), s!(1084, 1048), s!(1077, 1036),
            s!(1088, 1019), s!(1079, 1031), s!(1082, 1016), s!(1077, 1015), s!(1086, 1014), s!(1098,  986), s!(1081,  988), s!(1029, 1036),
            s!(1100,  993), s!(1078, 1017), s!(1076,  999), s!(1085,  989), s!(1081,  988), s!(1074,  978), s!(1007, 1031), s!(1026, 1029),
        ],
        [
            s!( 992, 1059), s!(1023, 1121), s!(1037, 1089), s!(1064, 1085), s!(1079, 1081), s!(1077, 1085), s!(1106, 1061), s!(1055, 1073),
            s!( 997, 1056), s!(1006, 1091), s!(1016, 1088), s!(1028, 1107), s!(1022, 1122), s!(1061, 1080), s!(1035, 1100), s!(1071, 1062),
            s!(1028, 1024), s!(1025, 1050), s!(1050, 1052), s!(1054, 1076), s!(1072, 1076), s!(1096, 1055), s!(1087, 1066), s!(1052, 1084),
            s!(1015, 1031), s!(1028, 1040), s!(1030, 1066), s!(1040, 1077), s!(1076, 1054), s!(1064, 1075), s!(1064, 1092), s!(1066, 1084),
            s!(1028, 1025), s!(1012, 1053), s!(1050, 1039), s!(1056, 1054), s!(1070, 1050), s!(1073, 1061), s!(1067, 1058), s!(1066, 1068),
            s!(1017, 1010), s!(1030, 1011), s!(1054, 1031), s!(1050, 1031), s!(1045, 1035), s!(1079, 1026), s!(1063, 1041), s!(1055, 1061),
            s!(1015, 1056), s!(1049, 1024), s!(1055, 1005), s!(1057, 1001), s!(1063,  999), s!(1055,  989), s!(1048, 1012), s!(1044, 1042),
            s!(1033,  996), s!(1037,  995), s!(1021,  990), s!(1052,  953), s!(1026, 1004), s!(1033, 1006), s!(1019, 1046), s!(1028, 1021),
        ],
        [
            s!(1004, 1062), s!(1008, 1103), s!(1061, 1097), s!(1074, 1084), s!(1068, 1076), s!(1077, 1070), s!(1093, 1061), s!(1031, 1074),
            s!(1013, 1047), s!( 991, 1082), s!( 990, 1092), s!(1008, 1114), s!( 991, 1116), s!(1019, 1076), s!(1001, 1095), s!(1058, 1074),
            s!(1043, 1031), s!(1004, 1054), s!(1013, 1077), s!(1036, 1093), s!(1028, 1093), s!(1062, 1068), s!(1069, 1073), s!(1055, 1080),
            s!(1035, 1050), s!(1032, 1048), s!(1025, 1070), s!(1027, 1082), s!(1007, 1066), s!(1037, 1075), s!(1021, 1082), s!(1047, 1093),
            s!(1049, 1031), s!(1021, 1055), s!(1042, 1048), s!(1040, 1051), s!(1038, 1063), s!(1043, 1064), s!(1043, 1060), s!(1048, 1075),
            s!(1050, 1018), s!(1051,  999), s!(1068, 1030), s!(1054, 1016), s!(1052, 1041), s!(1061, 1010), s!(1055, 1029), s!(1050, 1045),
            s!(1034, 1044), s!(1064,  992), s!(1067,  961), s!(1067,  973), s!(1067,  978), s!(1076,  986), s!(1059, 1009), s!(1094, 1036),
            s!(1075, 1004), s!(1060, 1007), s!(1047,  968), s!(1059,  953), s!(1031, 1008), s!(1034,  991), s!(1013, 1046), s!(1025, 1029),
        ],
        [
            s!(1009, 1058), s!(1009, 1098), s!(1032, 1068), s!(1064, 1074), s!(1083, 1085), s!(1075, 1070), s!(1111, 1069), s!(1042, 1060),
            s!(1022, 1061), s!(1004, 1099), s!(1041, 1100), s!(1026, 1120), s!(1008, 1110), s!(1063, 1070), s!(1029, 1091), s!(1095, 1078),
            s!(1039, 1026), s!(1038, 1057), s!(1050, 1048), s!(1046, 1073), s!(1077, 1071), s!(1080, 1057), s!(1063, 1055), s!(1067, 1085),
            s!(1018, 1043), s!(1035, 1059), s!(1025, 1069), s!(1028, 1065), s!(1069, 1058), s!(1057, 1053), s!(1049, 1079), s!(1059, 1077),
            s!(1026, 1032), s!(1033, 1057), s!(1049, 1046), s!(1079, 1071), s!(1058, 1042), s!(1081, 1066), s!(1040, 1055), s!(1057, 1063),
            s!(1043, 1016), s!(1061, 1006), s!(1060, 1027), s!(1061, 1033), s!(1046, 1027), s!(1061, 1028), s!(1043, 1031), s!(1046, 1049),
            s!(1010, 1043), s!(1039, 1000), s!(1030,  996), s!(1052, 1003), s!(1063,  988), s!(1035,  999), s!(1067, 1021), s!(1057, 1043),
            s!(1042,  994), s!(1038, 1001), s!(1024, 1000), s!(1022,  987), s!(1015, 1009), s!(1025, 1004), s!(1015, 1047), s!(1018, 1022),
        ],
        [
            s!(1007, 1051), s!( 997, 1091), s!(1046, 1085), s!(1070, 1082), s!(1083, 1077), s!(1074, 1066), s!(1100, 1053), s!(1050, 1067),
            s!(1035, 1066), s!(1014, 1087), s!(1037, 1109), s!(1024, 1115), s!(1013, 1123), s!(1078, 1082), s!(1021, 1087), s!(1054, 1061),
            s!(1051, 1038), s!(1047, 1065), s!(1065, 1060), s!(1048, 1083), s!(1076, 1074), s!(1079, 1062), s!(1080, 1070), s!(1065, 1062),
            s!(1062, 1047), s!(1037, 1063), s!(1053, 1073), s!(1057, 1066), s!(1072, 1064), s!(1072, 1053), s!(1064, 1074), s!(1053, 1071),
            s!(1062, 1033), s!(1047, 1058), s!(1059, 1059), s!(1064, 1057), s!(1063, 1020), s!(1059, 1070), s!(1048, 1066), s!(1046, 1064),
            s!(1035, 1012), s!(1068, 1023), s!(1084, 1032), s!(1072, 1016), s!(1072, 1042), s!(1068, 1021), s!(1068, 1023), s!(1046, 1058),
            s!(1009, 1051), s!(1046, 1011), s!(1089, 1004), s!(1077, 1013), s!(1066, 1036), s!(1054, 1004), s!(1056, 1020), s!(1067, 1039),
            s!(1035,  994), s!(1025, 1002), s!(1050,  999), s!(1049,  979), s!(1048, 1007), s!(1041, 1003), s!(1022, 1047), s!(1035, 1026),
        ],
        [
            s!(1023, 1050), s!(1021, 1100), s!(1049, 1083), s!(1082, 1078), s!(1074, 1075), s!(1074, 1066), s!(1114, 1063), s!(1065, 1087),
            s!(1047, 1059), s!(1010, 1100), s!(1048, 1098), s!(1048, 1123), s!(1040, 1116), s!(1072, 1065), s!(1044, 1087), s!(1067, 1063),
            s!(1060, 1046), s!(1046, 1072), s!(1075, 1068), s!(1059, 1090), s!(1089, 1081), s!(1099, 1069), s!(1090, 1068), s!(1078, 1077),
            s!(1053, 1055), s!(1056, 1071), s!(1063, 1073), s!(1055, 1078), s!(1089, 1075), s!(1064, 1051), s!(1055, 1080), s!(1054, 1077),
            s!(1059, 1040), s!(1051, 1076), s!(1079, 1047), s!(1075, 1055), s!(1084, 1030), s!(1064, 1054), s!(1059, 1080), s!(1031, 1070),
            s!(1024, 1015), s!(1074, 1008), s!(1060, 1060), s!(1071, 1051), s!(1064, 1049), s!(1073, 1019), s!(1047, 1040), s!(1045, 1064),
            s!(1016, 1047), s!(1059, 1014), s!(1084,  986), s!(1071, 1025), s!(1061, 1025), s!(1069, 1000), s!(1055, 1026), s!(1064, 1048),
            s!(1049, 1001), s!(1053, 1005), s!(1061,  987), s!(1049,  993), s!(1050, 1021), s!(1033, 1002), s!(1037, 1056), s!(1004, 1016),
        ],
        [
            s!(1027, 1057), s!(1022, 1107), s!(1055, 1095), s!(1070, 1074), s!(1073, 1075), s!(1078, 1065), s!(1103, 1058), s!(1062, 1081),
            s!(1044, 1075), s!(1040, 1104), s!(1042, 1102), s!(1034, 1120), s!(1023, 1116), s!(1074, 1071), s!(1032, 1087), s!(1075, 1058),
            s!(1057, 1037), s!(1071, 1076), s!(1067, 1070), s!(1053, 1093), s!(1075, 1077), s!(1091, 1068), s!(1091, 1071), s!(1048, 1082),
            s!(1054, 1054), s!(1075, 1078), s!(1050, 1083), s!(1043, 1067), s!(1065, 1056), s!(1057, 1061), s!(1064, 1086), s!(1056, 1082),
            s!(1072, 1043), s!(1050, 1070), s!(1064, 1056), s!(1066, 1056), s!(1072, 1046), s!(1054, 1062), s!(1053, 1067), s!(1029, 1060),
            s!(1031, 1015), s!(1083, 1030), s!(1082, 1039), s!(1074, 1033), s!(1069, 1037), s!(1067, 1020), s!(1047, 1035), s!(1059, 1052),
            s!(1017, 1050), s!(1050, 1015), s!(1084, 1014), s!(1081, 1026), s!(1055, 1024), s!(1081,  992), s!(1064, 1027), s!(1061, 1057),
            s!(1063, 1005), s!(1050, 1005), s!(1054, 1003), s!(1092,  994), s!(1040, 1019), s!(1037,  998), s!(1015, 1048), s!(1014, 1024),
        ],
    ],
    [
        [
            s!( -21,   -9), s!( -18,  -12), s!(  -2,    1), s!( -10,   -1), s!(  -2,    5), s!(  -8,   -2), s!(   2,    3), s!(   2,    2),
            s!( -12,   -5), s!( -12,   -6), s!(  -8,   -2), s!(   2,  -10), s!(  15,   -1), s!(  16,   12), s!(   1,   -1), s!(   2,    3),
            s!(  -8,  -16), s!( -11,   -9), s!( -21,   -2), s!(  -6,  -11), s!(  -9,    9), s!(  25,    8), s!(  49,   10), s!(  -4,    7),
            s!( -10,   -7), s!( -24,   -2), s!(  -4,  -10), s!(  -4,    0), s!(   3,   -2), s!(  17,    4), s!(  28,   13), s!(   1,   -5),
            s!( -23,  -16), s!( -13,  -10), s!( -11,    5), s!(  11,  -14), s!(   4,    0), s!(  11,    4), s!( -12,   10), s!(   9,   -0),
            s!(  -6,   -8), s!( -14,   -1), s!(   1,   -9), s!(  -0,   -3), s!(  -1,    2), s!(   2,    3), s!(  -3,    4), s!(   8,   -4),
            s!( -26,   -8), s!( -10,   -8), s!(  -7,    4), s!(  -2,   -1), s!(  -2,   -7), s!(  -6,   16), s!(  12,   -6), s!(  -6,   -6),
            s!(  -7,   -9), s!( -14,   -5), s!(  -7,   -5), s!(  -4,  -14), s!(  -6,   -5), s!(  -4,   -7), s!(  -2,   -7), s!(   4,   -2),
        ],
        [
            s!( -11,    8), s!(   7,    0), s!(  -1,   -4), s!(   3,   -0), s!(  -9,   -2), s!(   9,   10), s!(   1,    2), s!(  14,   11),
            s!( -21,    8), s!( -13,   15), s!(  -3,    4), s!(   0,    4), s!(  -8,    9), s!(   4,    1), s!(   3,    2), s!(   7,    5),
            s!( -11,    3), s!( -13,    8), s!( -14,    5), s!(  -6,    5), s!(  18,    9), s!(  10,    4), s!( -13,    5), s!(  11,    1),
            s!(  -5,   -1), s!(  -8,   -1), s!( -14,    2), s!(  -3,    8), s!(  -7,   11), s!(  -4,   10), s!(  -0,   -0), s!(   1,   -3),
            s!( -13,    4), s!(  -6,   -1), s!(  -2,    3), s!(  -1,    6), s!(   1,    2), s!(   1,   -6), s!(   6,   -8), s!(   5,   -0),
            s!(  -1,   -9), s!(  -3,   -1), s!(  -4,   -2), s!(  -4,   11), s!(  -4,    3), s!(   1,   -1), s!(   5,   -5), s!(  12,   -7),
            s!(  -8,   -9), s!(  -9,    1), s!(  -2,    5), s!(   4,   -5), s!(  -2,   -1), s!(   2,  -13), s!(  12,  -18), s!(  13,    1),
            s!(  -9,  -12), s!(  -3,   -4), s!(  10,  -19), s!(  -2,    9), s!(   5,   -4), s!(  -2,  -10), s!(   7,   -5), s!(  -2,    0),
        ],
        [
            s!(  -3,   -1), s!(   3,    1), s!(   6,    3), s!(   4,    1), s!(   5,    5), s!(   5,    4), s!(   7,    6), s!(  13,    5),
            s!(  -6,   -1), s!(  14,    1), s!(   3,    1), s!(   5,   -3), s!(   1,   -1), s!(   2,    1), s!(   3,   -1), s!(  41,    5),
            s!(   6,    4), s!(   1,   -0), s!(  -1,    2), s!(   9,    5), s!(   1,   -3), s!(  17,   10), s!(  15,    5), s!(  19,   10),
            s!(  -1,    2), s!(   0,    2), s!(  -3,   -1), s!(   9,   -5), s!(  -3,   -1), s!(   2,   -1), s!(  23,   -4), s!(  22,   10),
            s!( -12,   -2), s!(  -3,   -1), s!(  -5,   -3), s!(  -1,    1), s!( -23,  -10), s!(   4,   -0), s!( -16,   -6), s!(  15,    5),
            s!(  -6,   -8), s!(  11,   -5), s!(   1,   -0), s!(   1,   -1), s!( -15,   -7), s!(   6,   -4), s!( -10,   -5), s!( -19,    3),
            s!(  -5,   -4), s!(  -4,   -2), s!( -12,   -4), s!(  -2,  -10), s!(  -6,    1), s!(   0,    2), s!(  -7,   -2), s!(   5,    0),
            s!(   1,   -0), s!(  -0,   -1), s!(   6,    3), s!(   2,   -2), s!(  -9,  -12), s!(  -1,   -4), s!(  -6,   -2), s!(   1,    1),
        ],
        [
            s!(   2,   -1), s!(   1,    0), s!(   0,   -1), s!(   2,    0), s!(   2,    1), s!(   4,    4), s!(  -4,   -5), s!(  -8,   -2),
            s!(  16,    3), s!(  25,   11), s!(  11,    4), s!(  -1,   -3), s!(   0,   -0), s!(   2,   -0), s!(   5,    9), s!(  -1,    5),
            s!(   4,    1), s!(  18,    5), s!(  25,   11), s!(   3,    5), s!(   5,    2), s!(  -3,    2), s!(   9,    1), s!(   4,   -3),
            s!(   2,    0), s!(  -3,    4), s!(   8,    6), s!(  -5,    1), s!(   1,    4), s!(   9,   -2), s!(  -7,   -1), s!(   4,    3),
            s!(  -6,    8), s!(   4,    4), s!(   3,    0), s!( -16,   -3), s!(   5,    5), s!(   6,   -3), s!(   1,   -3), s!(  -3,    3),
            s!(  -4,    4), s!(   3,   10), s!( -14,    0), s!( -10,  -12), s!(  -6,   -2), s!( -10,   -2), s!(   4,   -7), s!(   7,   -4),
            s!(   9,    2), s!(   3,    1), s!(  -6,   -9), s!(  -9,   -3), s!(  -3,   -2), s!( -10,    2), s!(   1,   -1), s!(   5,   -1),
            s!(  10,    4), s!(   4,    4), s!(   1,   -1), s!(  -9,   -5), s!(   4,   -5), s!(  -2,   -2), s!(   2,    2), s!(   9,    4),
        ],
        [
            s!(   5,    6), s!(  13,   10), s!(   2,    0), s!(  -0,   -0), s!(   0,   -2), s!(  -0,   -2), s!(  -2,   -1), s!(   4,   -1),
            s!(   7,    5), s!(  15,    4), s!(  -0,   -1), s!(   1,    0), s!(   4,    2), s!(  -1,   -6), s!(   5,    7), s!(  -4,   -2),
            s!(   9,    5), s!(   3,    1), s!(   4,    2), s!(   0,   -1), s!(  -1,   -1), s!(  -3,   -4), s!(   3,   -3), s!(  -7,   -4),
            s!(   4,    1), s!(  11,    6), s!(   4,    2), s!(   3,   -0), s!(  -7,   -3), s!(  -4,   -4), s!(  -8,   -5), s!(   9,    4),
            s!(  26,    6), s!(   9,    4), s!(  -4,   -1), s!(   3,    2), s!( -10,   -7), s!(  -1,    2), s!(  14,    3), s!(  -8,    0),
            s!(  -3,    1), s!(   4,    4), s!(   7,    4), s!(   8,    0), s!(   5,    3), s!(  -9,   -6), s!(  -4,   -2), s!(  -1,   -1),
            s!(   8,    4), s!(   5,    2), s!(   2,    2), s!(  -1,   -0), s!(   1,    3), s!(   4,   -1), s!(  -4,    0), s!(  -4,   -0),
            s!(   4,    3), s!(   2,   -0), s!(  -8,   -2), s!(   6,    4), s!(  -8,   -1), s!(   2,    1), s!(  -1,    0), s!(   0,    1),
        ],
        [
            s!(  27,    6), s!(   4,    3), s!(   1,    1), s!(   3,    2), s!(   3,   -0), s!(   3,    2), s!(  -4,   -0), s!( -19,  -11),
            s!(  60,   17), s!(   1,    0), s!(   1,    2), s!(   1,    0), s!(   1,   -0), s!(  -7,   -1), s!( -21,   -6), s!( -28,  -11),
            s!(  14,    5), s!(  14,    7), s!(  18,    9), s!(   9,    3), s!(   8,    2), s!( -18,   -4), s!( -21,   -7), s!( -16,   -4),
            s!(  28,   10), s!(  21,    6), s!(  10,    4), s!(  -4,    5), s!(   5,    1), s!(   0,   -3), s!( -29,   -7), s!( -22,   -3),
            s!(  29,   11), s!(  22,    7), s!(   8,    3), s!(   3,    2), s!(  -8,   -2), s!( -17,   -3), s!(  -1,    5), s!( -23,   -8),
            s!(  24,    9), s!(   1,    2), s!(   6,   -1), s!(   3,   -1), s!(   1,   -2), s!(  -0,    1), s!( -22,   -1), s!(  -4,   -1),
            s!(  10,    5), s!(   3,    1), s!(   6,  -10), s!( -11,   -6), s!(  -4,   -7), s!(   2,   -2), s!( -21,   -1), s!( -15,   -1),
            s!(  10,    4), s!(   1,    2), s!( -21,    0), s!( -10,  -19), s!( -13,    0), s!(   9,   -1), s!(   2,    1), s!(   1,    0),
        ],
        [
            s!(   7,    6), s!(   2,    2), s!(   4,    3), s!(   5,    5), s!(   2,    2), s!(  -3,   -4), s!(   1,    0), s!(  -5,   -4),
            s!(   1,    1), s!(  -0,   -0), s!(   2,    2), s!(   9,    7), s!(  -3,   -9), s!( -15,   -5), s!( -12,   -2), s!( -40,  -16),
            s!(  18,    5), s!(   5,    6), s!(  11,    8), s!(   7,    3), s!( -10,    3), s!( -25,   -9), s!( -34,  -13), s!( -19,  -15),
            s!(   3,    5), s!(  18,   -0), s!(  13,    9), s!(  14,    6), s!(   8,    0), s!( -17,    1), s!( -21,   -7), s!( -18,   -8),
            s!(  16,    0), s!(  -3,    2), s!(   5,   -2), s!(  -6,   -6), s!(  12,    3), s!(  -2,   -1), s!( -14,   -8), s!( -16,  -11),
            s!(   1,    2), s!(   8,   -3), s!(   2,    1), s!(   0,   -3), s!(   8,    2), s!(   2,   -5), s!(  -2,    2), s!( -10,   -6),
            s!(  16,    7), s!(  11,    1), s!(   8,    3), s!(   0,    5), s!(   2,    5), s!(   6,   -3), s!(  -3,    1), s!( -16,   -6),
            s!(   3,    3), s!(  12,   -2), s!(   2,    2), s!(  -9,    3), s!(  -2,    3), s!(   3,   -2), s!(   4,   -0), s!(   1,    1),
        ],
        [
            s!(   1,    1), s!(   2,   -0), s!(   4,    4), s!(   5,    5), s!(  -2,   -1), s!(  -1,   -2), s!(  -8,   -5), s!(  -8,   -7),
            s!(  -0,   -0), s!(   2,    3), s!(  17,    9), s!(   4,    2), s!(   2,   -1), s!( -12,   -7), s!(  -3,   -0), s!( -15,  -11),
            s!(  -3,   -2), s!(   5,    4), s!(   9,    6), s!(   4,    5), s!(  -3,   -7), s!( -14,   -5), s!( -12,   -8), s!(  -7,   -4),
            s!(  -3,    0), s!(  10,    3), s!(  13,    8), s!(  -2,   -4), s!(   8,    3), s!(  -8,   -2), s!(  -7,   -1), s!( -17,  -10),
            s!(   8,   -0), s!(   9,    7), s!(   2,    2), s!(   6,    3), s!(   2,   -1), s!(  -1,   -0), s!(  -8,   -4), s!(  -5,   -2),
            s!(  -6,    0), s!(  -5,    4), s!(  -1,    0), s!(   1,   -3), s!(  12,    4), s!(  -7,   -1), s!(  -4,    0), s!( -31,  -10),
            s!(  10,    2), s!(   3,    5), s!(   5,   -6), s!(   7,    1), s!(   4,    6), s!( -14,   -8), s!(   0,   -1), s!(  -2,   -4),
            s!(  -1,    0), s!(  -1,   -0), s!(  -2,    1), s!(  16,    1), s!(  -2,   -1), s!(  -1,   -0), s!(   2,    1), s!(  -1,   -0),
        ],
    ],
];

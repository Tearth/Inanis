// ------------------------------------------------------------------------- //
// Generated at 24-11-2024 15:06:24 UTC (e = 0.130552, k = 0.0077, r = 1.00) //
// ------------------------------------------------------------------------- //

use super::*;

#[rustfmt::skip]
pub const QUEEN_PST_PATTERN: [[[PackedEval; 64]; KING_BUCKETS_COUNT]; 2] =
[
    [
        [
            s!(1030, 1080), s!(1043, 1112), s!(1057, 1097), s!(1085, 1090), s!(1069, 1105), s!(1107, 1086), s!(1117, 1082), s!(1089, 1089),
            s!(1029, 1080), s!(1021, 1107), s!(1050, 1102), s!(1042, 1108), s!(1050, 1102), s!(1088, 1080), s!(1067, 1108), s!(1113, 1055),
            s!(1040, 1057), s!(1050, 1081), s!(1076, 1058), s!(1067, 1077), s!(1088, 1074), s!(1122, 1068), s!(1099, 1068), s!(1067, 1103),
            s!(1043, 1057), s!(1044, 1058), s!(1034, 1086), s!(1056, 1058), s!(1084, 1054), s!(1077, 1071), s!(1075, 1100), s!(1083, 1077),
            s!(1047, 1028), s!(1035, 1087), s!(1056, 1052), s!(1057, 1049), s!(1069, 1049), s!(1082, 1056), s!(1080, 1064), s!(1077, 1059),
            s!(1037, 1030), s!(1055, 1018), s!(1050, 1045), s!(1067, 1029), s!(1070, 1036), s!(1085, 1042), s!(1078, 1050), s!(1081, 1025),
            s!(1043, 1079), s!(1051, 1046), s!(1064, 1011), s!(1062, 1026), s!(1077, 1025), s!(1077, 1014), s!(1094,  975), s!(1031, 1002),
            s!(1059, 1007), s!(1048, 1028), s!(1054,  995), s!(1068,  994), s!(1077,  984), s!(1060,  981), s!(1025, 1012), s!( 997, 1006),
        ],
        [
            s!(1059, 1099), s!(1046, 1110), s!(1063, 1096), s!(1086, 1088), s!(1080, 1104), s!(1089, 1092), s!(1127, 1078), s!(1086, 1085),
            s!(1049, 1086), s!(1037, 1115), s!(1068, 1089), s!(1061, 1110), s!(1047, 1125), s!(1071, 1107), s!(1053, 1140), s!(1117, 1071),
            s!(1078, 1055), s!(1068, 1082), s!(1073, 1089), s!(1073, 1101), s!(1071, 1104), s!(1101, 1108), s!(1101, 1091), s!(1076, 1103),
            s!(1070, 1053), s!(1071, 1077), s!(1071, 1074), s!(1072, 1085), s!(1075, 1077), s!(1075, 1098), s!(1066, 1106), s!(1081, 1095),
            s!(1080, 1056), s!(1072, 1064), s!(1079, 1062), s!(1076, 1066), s!(1077, 1067), s!(1075, 1078), s!(1087, 1076), s!(1071, 1070),
            s!(1075, 1033), s!(1083, 1041), s!(1086, 1041), s!(1085, 1031), s!(1078, 1047), s!(1092, 1047), s!(1092, 1056), s!(1082, 1039),
            s!(1082, 1035), s!(1087, 1039), s!(1090, 1016), s!(1087, 1013), s!(1095, 1023), s!(1111,  972), s!(1096,  975), s!(1043, 1042),
            s!(1098, 1012), s!(1085, 1014), s!(1078, 1015), s!(1094,  998), s!(1086,  994), s!(1088,  971), s!(1003, 1026), s!(1033, 1035),
        ],
        [
            s!( 991, 1072), s!(1024, 1120), s!(1040, 1099), s!(1066, 1091), s!(1081, 1087), s!(1076, 1094), s!(1103, 1058), s!(1063, 1079),
            s!( 997, 1067), s!(1000, 1092), s!(1014, 1098), s!(1024, 1104), s!(1023, 1128), s!(1053, 1088), s!(1052, 1110), s!(1054, 1060),
            s!(1024, 1020), s!(1027, 1057), s!(1041, 1057), s!(1069, 1081), s!(1072, 1085), s!(1095, 1066), s!(1090, 1067), s!(1058, 1081),
            s!(1019, 1040), s!(1032, 1045), s!(1022, 1080), s!(1040, 1092), s!(1073, 1060), s!(1068, 1076), s!(1062, 1085), s!(1064, 1090),
            s!(1030, 1028), s!(1019, 1064), s!(1050, 1051), s!(1055, 1066), s!(1067, 1065), s!(1075, 1059), s!(1068, 1063), s!(1059, 1075),
            s!(1004, 1011), s!(1036, 1019), s!(1054, 1046), s!(1059, 1052), s!(1052, 1050), s!(1091, 1039), s!(1060, 1038), s!(1047, 1060),
            s!(1011, 1050), s!(1035, 1027), s!(1054, 1006), s!(1057, 1009), s!(1069, 1009), s!(1047,  985), s!(1048, 1006), s!(1044, 1041),
            s!(1035,  999), s!(1031,  998), s!(1019,  994), s!(1052,  954), s!(1029,  999), s!(1038, 1008), s!(1018, 1042), s!(1045, 1024),
        ],
        [
            s!( 983, 1065), s!(1001, 1100), s!(1054, 1095), s!(1067, 1078), s!(1069, 1081), s!(1088, 1077), s!(1083, 1061), s!(1027, 1083),
            s!(1013, 1044), s!( 982, 1090), s!( 994, 1098), s!(1013, 1119), s!( 994, 1129), s!(1026, 1087), s!( 996, 1101), s!(1055, 1076),
            s!(1042, 1024), s!(1001, 1059), s!(1012, 1101), s!(1032, 1086), s!(1023, 1107), s!(1052, 1077), s!(1060, 1079), s!(1055, 1074),
            s!(1034, 1052), s!(1028, 1056), s!(1025, 1071), s!(1022, 1091), s!(1021, 1090), s!(1034, 1087), s!(1036, 1084), s!(1047, 1084),
            s!(1055, 1023), s!(1031, 1059), s!(1039, 1056), s!(1050, 1061), s!(1038, 1087), s!(1046, 1066), s!(1053, 1055), s!(1042, 1066),
            s!(1046, 1008), s!(1061, 1008), s!(1064, 1034), s!(1060, 1019), s!(1053, 1051), s!(1065, 1017), s!(1057, 1029), s!(1052, 1031),
            s!(1050, 1041), s!(1065,  984), s!(1075,  963), s!(1080,  974), s!(1072,  986), s!(1084,  981), s!(1058, 1001), s!(1059, 1025),
            s!(1065, 1000), s!(1052,  999), s!(1053,  947), s!(1067,  949), s!(1031, 1003), s!(1036,  973), s!(1010, 1046), s!(1018, 1027),
        ],
        [
            s!(1004, 1056), s!(1004, 1094), s!(1033, 1070), s!(1066, 1080), s!(1084, 1086), s!(1074, 1071), s!(1114, 1073), s!(1031, 1053),
            s!(1034, 1066), s!(1005, 1090), s!(1039, 1102), s!(1028, 1119), s!(1010, 1111), s!(1048, 1065), s!(1023, 1085), s!(1089, 1074),
            s!(1037, 1022), s!(1040, 1055), s!(1059, 1046), s!(1054, 1085), s!(1071, 1069), s!(1078, 1055), s!(1067, 1053), s!(1049, 1073),
            s!(1011, 1041), s!(1038, 1060), s!(1043, 1077), s!(1031, 1068), s!(1070, 1065), s!(1064, 1058), s!(1045, 1076), s!(1049, 1071),
            s!(1032, 1028), s!(1042, 1062), s!(1052, 1054), s!(1093, 1077), s!(1057, 1050), s!(1060, 1062), s!(1035, 1058), s!(1052, 1056),
            s!(1044, 1018), s!(1038, 1005), s!(1075, 1030), s!(1051, 1036), s!(1056, 1036), s!(1059, 1033), s!(1053, 1045), s!(1041, 1049),
            s!(1004, 1042), s!(1039,  999), s!(1029, 1002), s!(1048,  997), s!(1050,  985), s!(1043, 1009), s!(1054, 1013), s!(1054, 1041),
            s!(1030,  990), s!(1031,  997), s!(1003,  996), s!(1012,  991), s!( 994, 1006), s!(1014, 1004), s!(1012, 1042), s!(1033, 1031),
        ],
        [
            s!(1012, 1052), s!( 993, 1092), s!(1053, 1092), s!(1068, 1084), s!(1091, 1079), s!(1079, 1069), s!(1095, 1048), s!(1042, 1059),
            s!(1049, 1060), s!(1015, 1082), s!(1021, 1105), s!(1023, 1118), s!(1020, 1122), s!(1079, 1080), s!(1015, 1088), s!(1046, 1060),
            s!(1047, 1030), s!(1049, 1063), s!(1061, 1069), s!(1063, 1083), s!(1073, 1079), s!(1079, 1068), s!(1068, 1068), s!(1071, 1048),
            s!(1058, 1044), s!(1047, 1068), s!(1061, 1071), s!(1049, 1067), s!(1059, 1075), s!(1071, 1058), s!(1063, 1072), s!(1054, 1065),
            s!(1058, 1029), s!(1045, 1050), s!(1058, 1063), s!(1064, 1064), s!(1074, 1018), s!(1058, 1087), s!(1054, 1064), s!(1068, 1059),
            s!(1043, 1006), s!(1056, 1016), s!(1077, 1034), s!(1068, 1016), s!(1070, 1031), s!(1072, 1015), s!(1060, 1025), s!(1047, 1055),
            s!(1014, 1053), s!(1056, 1018), s!(1085,  979), s!(1074, 1013), s!(1061, 1027), s!(1058, 1016), s!(1049, 1020), s!(1062, 1038),
            s!(1022,  993), s!(1024, 1003), s!(1049,  997), s!(1052,  986), s!(1066,  997), s!(1048, 1002), s!(1038, 1047), s!(1046, 1030),
        ],
        [
            s!(1037, 1057), s!(1037, 1109), s!(1058, 1081), s!(1091, 1083), s!(1081, 1082), s!(1072, 1065), s!(1108, 1063), s!(1056, 1083),
            s!(1057, 1054), s!(1026, 1103), s!(1055, 1097), s!(1057, 1119), s!(1047, 1108), s!(1074, 1064), s!(1047, 1089), s!(1054, 1063),
            s!(1052, 1042), s!(1047, 1082), s!(1074, 1066), s!(1056, 1088), s!(1072, 1074), s!(1084, 1067), s!(1090, 1059), s!(1071, 1059),
            s!(1055, 1057), s!(1063, 1082), s!(1064, 1071), s!(1060, 1079), s!(1075, 1070), s!(1070, 1047), s!(1055, 1078), s!(1058, 1078),
            s!(1060, 1049), s!(1053, 1077), s!(1071, 1050), s!(1072, 1050), s!(1079, 1021), s!(1069, 1053), s!(1048, 1079), s!(1055, 1082),
            s!(1039, 1019), s!(1081, 1018), s!(1065, 1068), s!(1071, 1050), s!(1072, 1038), s!(1079, 1009), s!(1050, 1046), s!(1057, 1067),
            s!(1013, 1046), s!(1066, 1009), s!(1087,  982), s!(1071, 1027), s!(1066, 1027), s!(1067, 1006), s!(1058, 1032), s!(1061, 1049),
            s!(1037,  999), s!(1052, 1004), s!(1071,  977), s!(1050,  995), s!(1063, 1019), s!(1024, 1003), s!(1034, 1054), s!( 999, 1014),
        ],
        [
            s!(1037, 1063), s!(1028, 1110), s!(1060, 1097), s!(1076, 1080), s!(1072, 1076), s!(1085, 1067), s!(1099, 1057), s!(1063, 1079),
            s!(1057, 1080), s!(1051, 1105), s!(1056, 1110), s!(1040, 1119), s!(1032, 1113), s!(1082, 1076), s!(1027, 1078), s!(1066, 1050),
            s!(1055, 1037), s!(1078, 1073), s!(1076, 1083), s!(1054, 1094), s!(1084, 1079), s!(1090, 1070), s!(1093, 1069), s!(1047, 1076),
            s!(1060, 1061), s!(1086, 1083), s!(1057, 1087), s!(1054, 1079), s!(1063, 1065), s!(1055, 1072), s!(1065, 1081), s!(1046, 1077),
            s!(1079, 1044), s!(1057, 1075), s!(1079, 1060), s!(1065, 1057), s!(1070, 1038), s!(1054, 1060), s!(1047, 1062), s!(1021, 1062),
            s!(1027, 1012), s!(1078, 1044), s!(1085, 1040), s!(1081, 1037), s!(1073, 1040), s!(1062, 1015), s!(1043, 1031), s!(1049, 1043),
            s!(1011, 1047), s!(1044, 1014), s!(1083, 1003), s!(1075, 1034), s!(1054, 1030), s!(1073,  984), s!(1070, 1027), s!(1066, 1063),
            s!(1066, 1006), s!(1042, 1000), s!(1062, 1006), s!(1085,  983), s!(1036, 1019), s!(1037,  999), s!(1021, 1049), s!(1022, 1031),
        ],
        [
            s!(1043, 1078), s!(1030, 1110), s!(1052, 1094), s!(1084, 1094), s!(1072, 1103), s!(1106, 1089), s!(1118, 1082), s!(1081, 1089),
            s!(1037, 1074), s!(1014, 1108), s!(1037, 1099), s!(1042, 1107), s!(1043, 1115), s!(1084, 1086), s!(1064, 1102), s!(1107, 1062),
            s!(1041, 1045), s!(1044, 1077), s!(1057, 1045), s!(1064, 1078), s!(1072, 1074), s!(1112, 1071), s!(1095, 1062), s!(1065, 1092),
            s!(1045, 1051), s!(1053, 1057), s!(1029, 1076), s!(1052, 1065), s!(1089, 1056), s!(1078, 1079), s!(1072, 1089), s!(1078, 1076),
            s!(1056, 1031), s!(1041, 1072), s!(1057, 1054), s!(1058, 1053), s!(1070, 1045), s!(1080, 1051), s!(1088, 1056), s!(1080, 1053),
            s!(1033, 1020), s!(1057, 1008), s!(1060, 1043), s!(1059, 1039), s!(1065, 1052), s!(1086, 1031), s!(1072, 1044), s!(1065, 1025),
            s!(1044, 1066), s!(1057, 1035), s!(1061, 1020), s!(1059, 1033), s!(1069, 1028), s!(1078, 1018), s!(1085,  977), s!(1027, 1012),
            s!(1062, 1010), s!(1050, 1019), s!(1053,  986), s!(1060,  995), s!(1069,  991), s!(1057,  989), s!(1027, 1019), s!(1000, 1008),
        ],
        [
            s!(1059, 1082), s!(1035, 1111), s!(1060, 1090), s!(1079, 1080), s!(1083, 1092), s!(1085, 1085), s!(1117, 1056), s!(1078, 1084),
            s!(1037, 1086), s!(1022, 1117), s!(1051, 1079), s!(1056, 1100), s!(1034, 1121), s!(1067, 1092), s!(1051, 1130), s!(1107, 1073),
            s!(1071, 1042), s!(1055, 1074), s!(1064, 1082), s!(1051, 1085), s!(1064, 1092), s!(1083, 1095), s!(1107, 1089), s!(1071, 1093),
            s!(1065, 1045), s!(1061, 1072), s!(1068, 1062), s!(1059, 1081), s!(1076, 1067), s!(1067, 1083), s!(1071, 1096), s!(1074, 1090),
            s!(1070, 1051), s!(1065, 1060), s!(1074, 1046), s!(1069, 1067), s!(1065, 1054), s!(1061, 1068), s!(1081, 1055), s!(1071, 1056),
            s!(1068, 1016), s!(1072, 1037), s!(1077, 1029), s!(1074, 1030), s!(1073, 1039), s!(1083, 1041), s!(1085, 1050), s!(1076, 1035),
            s!(1088, 1018), s!(1078, 1031), s!(1080, 1011), s!(1075, 1015), s!(1085, 1014), s!(1099,  987), s!(1081,  988), s!(1030, 1036),
            s!(1101,  994), s!(1076, 1013), s!(1077, 1001), s!(1081,  985), s!(1081,  988), s!(1074,  978), s!(1007, 1031), s!(1026, 1029),
        ],
        [
            s!( 993, 1062), s!(1024, 1121), s!(1033, 1084), s!(1060, 1079), s!(1075, 1074), s!(1078, 1084), s!(1104, 1057), s!(1053, 1071),
            s!( 997, 1056), s!(1004, 1087), s!(1014, 1087), s!(1029, 1107), s!(1025, 1123), s!(1060, 1080), s!(1033, 1097), s!(1070, 1061),
            s!(1027, 1022), s!(1025, 1050), s!(1049, 1053), s!(1053, 1071), s!(1068, 1067), s!(1095, 1054), s!(1084, 1061), s!(1051, 1083),
            s!(1014, 1029), s!(1026, 1035), s!(1029, 1064), s!(1035, 1067), s!(1073, 1047), s!(1064, 1074), s!(1062, 1088), s!(1063, 1081),
            s!(1028, 1024), s!(1010, 1050), s!(1047, 1034), s!(1054, 1048), s!(1069, 1045), s!(1070, 1055), s!(1069, 1061), s!(1066, 1067),
            s!(1016, 1008), s!(1029, 1011), s!(1051, 1026), s!(1046, 1026), s!(1046, 1035), s!(1075, 1023), s!(1061, 1039), s!(1054, 1059),
            s!(1014, 1055), s!(1050, 1024), s!(1055, 1005), s!(1058, 1002), s!(1060,  994), s!(1055,  989), s!(1050, 1014), s!(1043, 1039),
            s!(1033,  997), s!(1037,  995), s!(1021,  990), s!(1052,  953), s!(1026, 1004), s!(1033, 1007), s!(1019, 1045), s!(1028, 1020),
        ],
        [
            s!(1006, 1067), s!(1010, 1106), s!(1060, 1094), s!(1072, 1081), s!(1067, 1075), s!(1077, 1071), s!(1094, 1062), s!(1031, 1074),
            s!(1013, 1046), s!( 994, 1087), s!( 992, 1093), s!(1007, 1112), s!( 992, 1120), s!(1018, 1075), s!(1000, 1093), s!(1057, 1071),
            s!(1043, 1030), s!(1005, 1054), s!(1012, 1076), s!(1036, 1093), s!(1027, 1092), s!(1062, 1070), s!(1068, 1071), s!(1054, 1078),
            s!(1035, 1048), s!(1031, 1047), s!(1024, 1069), s!(1025, 1077), s!(1009, 1066), s!(1036, 1072), s!(1021, 1081), s!(1047, 1093),
            s!(1047, 1030), s!(1021, 1056), s!(1042, 1048), s!(1040, 1051), s!(1037, 1061), s!(1043, 1064), s!(1043, 1059), s!(1048, 1075),
            s!(1050, 1016), s!(1051,  999), s!(1069, 1031), s!(1053, 1014), s!(1051, 1040), s!(1061, 1010), s!(1054, 1027), s!(1048, 1042),
            s!(1034, 1042), s!(1062,  989), s!(1065,  959), s!(1067,  972), s!(1066,  977), s!(1075,  985), s!(1058, 1008), s!(1094, 1035),
            s!(1075, 1003), s!(1061, 1008), s!(1047,  967), s!(1059,  952), s!(1031, 1009), s!(1034,  990), s!(1013, 1045), s!(1025, 1030),
        ],
        [
            s!(1009, 1060), s!(1009, 1098), s!(1029, 1064), s!(1065, 1075), s!(1082, 1084), s!(1074, 1068), s!(1111, 1068), s!(1040, 1059),
            s!(1023, 1061), s!(1005, 1101), s!(1040, 1098), s!(1024, 1119), s!(1006, 1108), s!(1063, 1069), s!(1029, 1089), s!(1095, 1077),
            s!(1039, 1027), s!(1037, 1057), s!(1050, 1050), s!(1047, 1075), s!(1074, 1065), s!(1076, 1051), s!(1062, 1053), s!(1065, 1082),
            s!(1018, 1044), s!(1035, 1059), s!(1025, 1067), s!(1031, 1069), s!(1067, 1054), s!(1056, 1052), s!(1047, 1075), s!(1057, 1074),
            s!(1027, 1036), s!(1030, 1053), s!(1050, 1047), s!(1078, 1069), s!(1058, 1043), s!(1078, 1062), s!(1040, 1054), s!(1055, 1059),
            s!(1043, 1015), s!(1061, 1007), s!(1058, 1023), s!(1060, 1031), s!(1043, 1022), s!(1058, 1025), s!(1040, 1028), s!(1045, 1048),
            s!(1011, 1045), s!(1038,  999), s!(1028,  994), s!(1053, 1004), s!(1063,  987), s!(1036, 1000), s!(1067, 1020), s!(1057, 1043),
            s!(1043,  995), s!(1038, 1001), s!(1023,  997), s!(1022,  988), s!(1016, 1010), s!(1024, 1002), s!(1013, 1043), s!(1018, 1023),
        ],
        [
            s!(1008, 1052), s!( 995, 1090), s!(1044, 1082), s!(1068, 1080), s!(1080, 1072), s!(1075, 1067), s!(1099, 1051), s!(1050, 1065),
            s!(1034, 1064), s!(1013, 1085), s!(1033, 1106), s!(1025, 1118), s!(1011, 1119), s!(1077, 1078), s!(1021, 1086), s!(1053, 1058),
            s!(1049, 1036), s!(1046, 1063), s!(1065, 1062), s!(1047, 1080), s!(1075, 1069), s!(1075, 1054), s!(1079, 1069), s!(1064, 1060),
            s!(1060, 1046), s!(1038, 1063), s!(1055, 1074), s!(1055, 1062), s!(1066, 1052), s!(1067, 1045), s!(1063, 1072), s!(1053, 1071),
            s!(1062, 1033), s!(1049, 1060), s!(1057, 1058), s!(1061, 1050), s!(1061, 1015), s!(1057, 1067), s!(1049, 1065), s!(1045, 1062),
            s!(1034, 1010), s!(1067, 1022), s!(1081, 1026), s!(1069, 1011), s!(1071, 1041), s!(1068, 1021), s!(1067, 1021), s!(1046, 1058),
            s!(1009, 1051), s!(1045, 1010), s!(1088, 1004), s!(1076, 1012), s!(1066, 1036), s!(1054, 1002), s!(1056, 1019), s!(1066, 1036),
            s!(1035,  994), s!(1024, 1000), s!(1050,  998), s!(1049,  979), s!(1046, 1005), s!(1040, 1002), s!(1022, 1045), s!(1035, 1026),
        ],
        [
            s!(1024, 1050), s!(1016, 1090), s!(1045, 1077), s!(1077, 1069), s!(1076, 1077), s!(1073, 1064), s!(1114, 1062), s!(1064, 1082),
            s!(1045, 1057), s!(1009, 1099), s!(1046, 1092), s!(1044, 1112), s!(1039, 1114), s!(1070, 1062), s!(1043, 1086), s!(1067, 1064),
            s!(1058, 1044), s!(1043, 1067), s!(1079, 1073), s!(1059, 1089), s!(1086, 1074), s!(1097, 1065), s!(1087, 1062), s!(1075, 1074),
            s!(1054, 1055), s!(1053, 1066), s!(1061, 1071), s!(1055, 1076), s!(1085, 1066), s!(1062, 1048), s!(1052, 1074), s!(1052, 1074),
            s!(1058, 1037), s!(1051, 1077), s!(1076, 1041), s!(1075, 1051), s!(1084, 1029), s!(1061, 1047), s!(1058, 1078), s!(1029, 1067),
            s!(1024, 1016), s!(1076, 1009), s!(1059, 1060), s!(1066, 1044), s!(1066, 1049), s!(1074, 1021), s!(1044, 1035), s!(1044, 1062),
            s!(1016, 1047), s!(1060, 1015), s!(1083,  984), s!(1071, 1025), s!(1060, 1024), s!(1069, 1001), s!(1055, 1025), s!(1064, 1048),
            s!(1049, 1002), s!(1053, 1006), s!(1061,  987), s!(1049,  994), s!(1051, 1022), s!(1034, 1003), s!(1036, 1055), s!(1004, 1017),
        ],
        [
            s!(1023, 1052), s!(1021, 1107), s!(1053, 1091), s!(1070, 1075), s!(1074, 1077), s!(1076, 1061), s!(1103, 1058), s!(1061, 1080),
            s!(1043, 1073), s!(1041, 1102), s!(1040, 1096), s!(1032, 1115), s!(1023, 1116), s!(1072, 1068), s!(1032, 1085), s!(1074, 1057),
            s!(1058, 1039), s!(1069, 1072), s!(1069, 1071), s!(1055, 1097), s!(1076, 1078), s!(1091, 1067), s!(1088, 1068), s!(1048, 1080),
            s!(1052, 1052), s!(1074, 1077), s!(1048, 1079), s!(1041, 1065), s!(1066, 1055), s!(1055, 1058), s!(1063, 1085), s!(1055, 1081),
            s!(1070, 1041), s!(1053, 1072), s!(1064, 1054), s!(1063, 1052), s!(1071, 1044), s!(1054, 1061), s!(1054, 1068), s!(1029, 1059),
            s!(1031, 1015), s!(1083, 1030), s!(1080, 1037), s!(1074, 1032), s!(1069, 1038), s!(1069, 1022), s!(1047, 1035), s!(1059, 1052),
            s!(1017, 1050), s!(1049, 1014), s!(1084, 1015), s!(1078, 1023), s!(1055, 1023), s!(1080,  992), s!(1064, 1027), s!(1062, 1058),
            s!(1063, 1004), s!(1051, 1006), s!(1053, 1003), s!(1093,  996), s!(1039, 1019), s!(1037,  998), s!(1015, 1048), s!(1014, 1025),
        ],
    ],
    [
        [
            s!( -50,   -1), s!( -37,  -15), s!( -28,   -1), s!( -34,   -1), s!( -17,    6), s!( -26,    2), s!(   9,   13), s!(   4,    3),
            s!( -19,   -7), s!( -21,  -10), s!( -18,  -12), s!(  -5,  -18), s!(  26,  -13), s!(  23,   12), s!(   9,    6), s!(   7,    8),
            s!( -22,  -29), s!( -23,  -27), s!( -39,   -1), s!(  -6,  -21), s!( -22,   26), s!(  50,   -5), s!(  56,   -8), s!(  -7,   25),
            s!( -21,  -15), s!( -24,   -3), s!(  -1,  -16), s!( -14,    9), s!(   7,    1), s!(  22,   10), s!(  36,   18), s!(  13,  -17),
            s!( -23,  -32), s!( -14,  -18), s!( -14,    6), s!(   6,   -9), s!(   3,    2), s!(   7,    5), s!( -11,   23), s!(  17,   -1),
            s!( -11,  -25), s!( -22,   -6), s!(   1,  -18), s!(  -3,    8), s!(  -1,    5), s!(  -2,   14), s!(  -3,    2), s!(  18,  -23),
            s!( -31,   -6), s!( -17,  -11), s!( -12,   -1), s!(  -2,   -8), s!(  -5,   -4), s!(  -6,   20), s!(  10,  -19), s!(   8,  -13),
            s!( -19,  -19), s!( -27,    0), s!(  -8,  -11), s!( -11,    2), s!(  -3,  -15), s!(  -2,  -11), s!(  -4,  -22), s!(  15,   -9),
        ],
        [
            s!( -45,   29), s!(   4,   -5), s!(   6,    1), s!(   6,    4), s!(  -7,   19), s!(  30,   33), s!(  15,   19), s!(  41,   26),
            s!( -22,   13), s!( -15,   27), s!( -16,   15), s!( -12,   23), s!(  -9,   31), s!(  21,   13), s!(  15,   11), s!(  26,   21),
            s!( -22,    3), s!( -21,   17), s!( -31,   23), s!(  -8,    4), s!(  25,   23), s!(  11,    6), s!( -32,   30), s!(  26,  -22),
            s!(  -7,   -1), s!( -16,   12), s!( -13,   15), s!( -10,   24), s!( -11,   35), s!(  -6,   28), s!(  -0,    5), s!(  -1,    4),
            s!( -15,    9), s!( -14,    5), s!(  -7,   16), s!(  -4,   18), s!(  -4,   14), s!(  -0,    5), s!(   5,    3), s!(  12,    4),
            s!(  -2,   -9), s!(  -7,   14), s!(  -8,   11), s!( -10,   14), s!(   2,    2), s!(   1,   -0), s!(   5,    1), s!(  13,   -8),
            s!(  -1,   -6), s!( -11,   11), s!(  -3,    8), s!(   1,    3), s!(  -5,    2), s!(   3,  -13), s!(  33,  -45), s!(   3,   -1),
            s!(  -6,   -5), s!(   4,  -15), s!(  13,  -18), s!(  -4,    9), s!(   6,  -12), s!(   1,  -15), s!(  13,  -26), s!(  -3,    3),
        ],
        [
            s!(  -9,    1), s!(   3,   -4), s!(  15,    5), s!(  20,    8), s!(  24,   20), s!(  23,   20), s!(  26,   21), s!(  26,    8),
            s!(   6,   -1), s!(  22,  -13), s!(  14,    6), s!(  16,   -2), s!(   6,    3), s!(   8,    4), s!(  16,    1), s!(  45,  -18),
            s!(  -7,    4), s!(  -0,   -3), s!( -12,   10), s!(  16,    8), s!(  11,   -2), s!(  57,   33), s!(  30,   16), s!(  40,   17),
            s!(  -3,    7), s!(   3,    2), s!(   2,    5), s!(  22,  -14), s!(  -8,   14), s!(  12,   -3), s!(  35,  -13), s!(  33,   17),
            s!( -16,   -5), s!( -14,  -10), s!(   1,   -2), s!(   1,   12), s!(  -8,    2), s!(  13,    1), s!( -12,    1), s!(  24,    8),
            s!(  -1,  -23), s!(  -4,  -14), s!(   3,    2), s!(   4,   -1), s!(  -9,   -2), s!(  11,  -14), s!(  -8,    2), s!(  -9,   26),
            s!( -10,   -9), s!( -12,   -7), s!( -14,    2), s!(   2,  -15), s!(  -5,   -2), s!(  -1,    4), s!( -14,   -6), s!(  14,    3),
            s!( -12,   -5), s!( -18,   -8), s!(   5,   -1), s!(  -1,   -1), s!(   0,  -30), s!(  -5,  -17), s!( -18,   -5), s!(   6,    6),
        ],
        [
            s!(  -4,   -4), s!(  10,    7), s!(   6,   -4), s!(  15,    8), s!(   8,    5), s!(  14,   11), s!( -12,  -13), s!( -14,    5),
            s!(  14,   -4), s!(  30,   25), s!(  24,    7), s!(   1,   -2), s!(   0,   -0), s!(  11,    7), s!(  29,   21), s!( -15,    6),
            s!(  -2,    1), s!(  21,    6), s!(  13,   17), s!(  12,   30), s!(   7,   11), s!(  -3,   19), s!(  20,    1), s!(  11,   -6),
            s!(   1,    2), s!(  -1,   23), s!(  15,   17), s!(  -4,   10), s!(  14,   19), s!(  14,    3), s!(   1,    6), s!(  10,    8),
            s!(   0,   25), s!(  12,   18), s!(   8,    6), s!(  -9,    0), s!(  -2,   16), s!(   8,  -14), s!(  -3,    1), s!(   2,    6),
            s!(   6,   20), s!(  -2,   31), s!(  -8,   10), s!(  -6,  -25), s!(   6,   -5), s!(  -2,   -5), s!(  13,  -25), s!(   9,  -19),
            s!(  -3,    1), s!(   5,   -2), s!(  -0,  -23), s!(  -9,   -2), s!(   2,  -12), s!( -14,    6), s!(   9,   -7), s!(   1,    0),
            s!(  -7,    6), s!(  -4,    5), s!(   3,   -5), s!(  -7,  -21), s!(  11,  -18), s!(   7,   -7), s!(   2,    5), s!(  24,   11),
        ],
        [
            s!(  20,   20), s!(  39,   20), s!(  12,    5), s!(   1,    2), s!(   6,    1), s!(   1,   -5), s!(   2,    5), s!(  12,   -7),
            s!(  24,   21), s!(  46,    6), s!(   2,   -0), s!(   2,    1), s!(  14,   11), s!(   8,   -3), s!(  20,   19), s!( -14,   -7),
            s!(   9,   10), s!(  18,    8), s!(  24,   14), s!(   8,    3), s!(   7,   12), s!(  -4,   -5), s!(   7,   -8), s!( -13,   -7),
            s!(  16,    6), s!(  25,   14), s!(  14,    2), s!(  13,    2), s!( -16,    1), s!(  -2,   -4), s!(  -7,   -5), s!(  -0,    5),
            s!(  25,    2), s!(  31,   14), s!(   0,    7), s!(   5,    6), s!( -20,  -14), s!(  -5,    4), s!(  11,   -2), s!( -23,    6),
            s!(  -0,    8), s!(  19,   14), s!(  13,    8), s!(  16,   -4), s!(   2,    2), s!(   1,   -8), s!(  -6,   -5), s!(   6,    0),
            s!(  22,   11), s!(  14,    8), s!(  10,    1), s!(  10,   -4), s!(  -4,    2), s!(  13,   -3), s!( -14,   -0), s!( -12,   -1),
            s!(  -3,    1), s!(   6,   -1), s!( -22,   -5), s!(   9,    4), s!( -17,   -0), s!(  -2,   -2), s!(  -6,   -1), s!(   1,    3),
        ],
        [
            s!(  59,   -0), s!(  17,   14), s!(   3,    3), s!(  14,    6), s!(   8,    2), s!(  11,    6), s!( -23,   -4), s!( -49,  -20),
            s!(  83,   11), s!(   4,   -0), s!(   3,    5), s!(   4,    3), s!(   4,    1), s!( -22,   10), s!( -42,   -0), s!( -45,  -12),
            s!(  46,   17), s!(  28,   15), s!(  63,   30), s!(  28,   15), s!(  16,   -1), s!( -44,    7), s!( -25,   -6), s!( -52,   -9),
            s!(  39,   15), s!(  20,   11), s!(  15,    5), s!(  -4,   21), s!(  -6,    7), s!(  12,   -4), s!( -28,   -6), s!( -24,   -2),
            s!(  29,   12), s!(  28,    6), s!(   3,    8), s!(   7,    4), s!(  -5,    8), s!( -25,    4), s!(  -7,    9), s!( -33,  -12),
            s!(  29,   15), s!(   0,    6), s!(  -4,   -5), s!(   1,    4), s!(  -0,   -1), s!(  -4,   11), s!( -27,    8), s!(   1,   -4),
            s!(   6,    5), s!(   5,    3), s!(   7,  -26), s!( -12,  -11), s!(  -3,   -6), s!(   3,   -6), s!( -24,   10), s!( -29,    4),
            s!(   9,    6), s!(  -9,   -1), s!(  -8,    6), s!( -12,  -36), s!( -10,   -1), s!(  12,   -8), s!(   0,    4), s!(   7,    2),
        ],
        [
            s!(  26,   16), s!(   9,    8), s!(  13,    9), s!(  10,    7), s!(   1,    2), s!(  -9,  -11), s!( -13,   -9), s!( -22,  -17),
            s!(   4,    3), s!(   0,    1), s!(   6,    5), s!(  22,   14), s!(  20,  -10), s!( -33,   -2), s!( -38,   -0), s!( -54,  -12),
            s!(   9,   -1), s!(   7,   17), s!(  21,   14), s!(  26,    8), s!( -25,    9), s!( -47,  -10), s!( -52,   -8), s!( -24,  -34),
            s!( -12,   12), s!(  15,   -8), s!(  -1,   15), s!(  21,    7), s!(   2,   -3), s!( -22,   19), s!( -24,   -3), s!( -29,  -21),
            s!(   6,  -13), s!(   9,   10), s!(   0,   -2), s!(  14,  -10), s!(  11,    3), s!(  -9,   -3), s!( -13,  -13), s!( -12,  -19),
            s!(  -7,   11), s!(  11,  -12), s!(  -0,    0), s!(   8,   -0), s!(  18,    1), s!(   7,   -5), s!(  -0,    7), s!( -11,  -16),
            s!(   6,   14), s!(  11,   -5), s!(   2,   -3), s!(  -3,    6), s!(   0,    7), s!(   8,  -11), s!( -12,    2), s!( -27,  -15),
            s!(  -2,   11), s!(  12,  -16), s!(  -0,   -1), s!( -13,    6), s!( -11,   10), s!(   1,  -11), s!(  12,   -2), s!(   5,    5),
        ],
        [
            s!(   3,    4), s!(  12,    6), s!(   9,   11), s!(   8,    9), s!( -10,   -7), s!(  -3,   -7), s!( -30,  -18), s!( -24,  -20),
            s!(   0,   -1), s!(   5,    6), s!(  32,   15), s!(  11,    5), s!(  -0,   -9), s!( -26,  -14), s!( -21,   -8), s!( -41,  -34),
            s!(  -2,   -5), s!(  15,   16), s!(  32,   19), s!(   4,    8), s!(  -7,  -18), s!( -38,  -10), s!( -27,  -19), s!( -16,  -14),
            s!(   1,    2), s!(  28,    5), s!(  32,   23), s!(  10,    2), s!(   3,   -7), s!( -19,   -5), s!( -19,   -6), s!( -29,  -24),
            s!(  20,   -1), s!(  12,   15), s!(   2,    7), s!(  11,    1), s!(  12,    0), s!(  -3,   -3), s!(  -7,  -10), s!( -10,   -6),
            s!( -10,    2), s!(  -5,   14), s!(  -3,    1), s!(   4,   -5), s!(   8,    1), s!(  -8,    6), s!( -14,   -1), s!( -64,  -18),
            s!(  12,    2), s!(  -1,   12), s!(   4,  -26), s!(  -4,   -1), s!(   3,   10), s!( -13,  -17), s!(   5,   -2), s!(  -6,  -13),
            s!(  -0,    2), s!(  -3,   -0), s!(  -7,    0), s!(  10,   -7), s!(   6,    0), s!( -10,   -3), s!(   5,    2), s!(  -3,   -1),
        ],
        [
            s!( -17,   -2), s!( -22,  -18), s!(  -6,   -4), s!( -11,   -4), s!(  -3,    4), s!(  -8,   -4), s!(   2,    3), s!(   4,    5),
            s!( -13,   -6), s!( -14,   -9), s!( -10,   -4), s!(   1,  -16), s!(  13,   -5), s!(  17,   14), s!(  -0,   -7), s!(   3,    4),
            s!(  -9,  -18), s!( -15,  -14), s!( -24,   -5), s!(  -8,  -15), s!( -11,    7), s!(  26,    8), s!(  50,    8), s!(  -3,    8),
            s!( -11,   -9), s!( -26,   -4), s!(  -7,  -16), s!(  -5,   -3), s!(   5,   -2), s!(  16,   -3), s!(  26,   10), s!(   1,   -4),
            s!( -24,  -17), s!( -15,  -12), s!( -12,    4), s!(   7,  -22), s!(   2,   -7), s!(  13,    5), s!( -12,   10), s!(   9,   -0),
            s!(  -6,   -8), s!( -14,   -1), s!(   1,   -8), s!(   1,   -2), s!(   2,    4), s!(   5,    5), s!(  -3,    4), s!(   8,   -4),
            s!( -26,   -8), s!( -12,  -10), s!(  -6,    4), s!(  -0,   -1), s!(  -2,   -7), s!(  -4,   17), s!(  13,   -5), s!(  -6,   -6),
            s!(  -6,   -8), s!( -15,   -6), s!(  -6,   -5), s!(  -1,  -10), s!(  -3,   -1), s!(  -3,   -5), s!(  -2,   -8), s!(   5,   -1),
        ],
        [
            s!( -13,    6), s!(   3,   -7), s!(  -6,  -11), s!(  -3,   -9), s!(  -6,    0), s!(   7,    5), s!(  -0,    0), s!(  14,    6),
            s!( -22,    5), s!( -14,   12), s!(  -5,   -0), s!(  -6,   -8), s!( -10,    4), s!(   5,   -3), s!(   1,   -0), s!(  10,    3),
            s!( -11,    2), s!( -11,   10), s!( -17,   -1), s!(  -9,   -2), s!(  13,    0), s!(  10,    1), s!( -14,    5), s!(  14,   -3),
            s!(  -6,   -2), s!(  -9,   -4), s!( -13,    2), s!(  -6,   -2), s!( -12,   -1), s!(  -6,    6), s!(  -0,   -1), s!(  -1,   -8),
            s!( -14,    4), s!(  -7,   -1), s!(  -2,    3), s!(  -5,   -3), s!(  -1,   -4), s!(   3,   -4), s!(   6,   -8), s!(   5,   -1),
            s!(  -2,  -10), s!(  -3,   -2), s!(  -5,   -3), s!(  -7,    4), s!(  -1,    5), s!(   1,   -1), s!(   5,   -5), s!(  12,   -8),
            s!(  -7,   -8), s!(  -9,   -0), s!(  -1,    5), s!(   8,   -1), s!(  -2,   -2), s!(   7,   -5), s!(  12,  -18), s!(  13,    0),
            s!(  -8,  -10), s!(  -2,   -2), s!(  12,  -17), s!(  -1,   11), s!(   9,    2), s!(   0,   -5), s!(   7,   -5), s!(   0,    4),
        ],
        [
            s!(  -4,   -2), s!(   2,   -0), s!(   4,   -1), s!(  -0,   -6), s!(   2,   -1), s!(   4,    3), s!(   6,    2), s!(  12,    2),
            s!(  -7,   -3), s!(  11,   -5), s!(   2,   -3), s!(  -2,  -17), s!(   0,   -6), s!(   1,   -0), s!(   1,  -10), s!(  43,    4),
            s!(   6,    3), s!(   0,   -3), s!(  -2,   -5), s!(   8,    1), s!(  -2,  -13), s!(  15,    7), s!(  10,   -6), s!(  18,    7),
            s!(  -2,    1), s!(   1,    3), s!(  -7,   -8), s!(   8,  -11), s!(  -3,   -3), s!(   2,   -2), s!(  18,  -16), s!(  22,    8),
            s!( -12,   -2), s!(  -2,   -0), s!(  -4,   -3), s!(  -3,   -3), s!( -24,  -14), s!(   3,   -2), s!( -16,   -7), s!(  14,    1),
            s!(  -6,   -8), s!(  11,   -6), s!(   1,   -2), s!(  -0,   -7), s!( -15,   -8), s!(   6,   -4), s!(  -9,   -3), s!( -18,   -0),
            s!(  -5,   -5), s!(  -4,   -2), s!( -10,   -3), s!(   0,   -7), s!(  -6,    0), s!(   0,    2), s!(  -5,   -0), s!(   5,   -0),
            s!(   1,   -0), s!(   2,    3), s!(   7,    4), s!(   4,    1), s!(  -7,  -10), s!(  -1,   -3), s!(  -5,   -1), s!(   1,    0),
        ],
        [
            s!(   0,   -3), s!(  -3,   -7), s!(  -1,   -1), s!(  -1,   -4), s!(   2,    1), s!(   1,   -3), s!(  -6,   -8), s!( -13,  -10),
            s!(  15,    1), s!(  23,    8), s!(  11,    1), s!(  -5,  -14), s!(  -1,   -2), s!(   2,   -7), s!(   2,    3), s!(  -1,    3),
            s!(   4,   -1), s!(  18,    4), s!(  25,    9), s!(  -3,   -9), s!(   4,   -2), s!(  -6,   -8), s!(   8,   -3), s!(   4,   -7),
            s!(   0,   -5), s!(  -5,   -0), s!(   8,    1), s!(  -7,   -5), s!(   1,    5), s!(   4,  -11), s!( -12,  -12), s!(   1,   -2),
            s!(  -7,    5), s!(   4,    3), s!(   2,   -5), s!( -16,   -6), s!(   6,    6), s!(   4,  -10), s!(  -2,  -10), s!(  -2,    3),
            s!(  -5,    2), s!(   4,   12), s!( -12,    2), s!( -10,  -13), s!(  -6,   -2), s!( -10,   -1), s!(   5,   -7), s!(   6,   -6),
            s!(  10,    3), s!(   4,    2), s!(  -4,   -5), s!( -10,   -6), s!(  -3,   -2), s!( -10,    2), s!(   2,    0), s!(   4,   -4),
            s!(  12,    8), s!(   7,    8), s!(   0,   -2), s!(  -7,   -4), s!(   4,   -6), s!(  -1,   -2), s!(   2,    1), s!(  10,    6),
        ],
        [
            s!(   1,   -0), s!(  15,   11), s!(  -2,   -7), s!(  -1,   -2), s!(  -4,   -9), s!(  -2,   -9), s!(  -4,   -5), s!(   1,   -6),
            s!(   6,    2), s!(  14,    1), s!(  -2,   -9), s!(   2,   -0), s!(   3,   -4), s!(  -2,   -9), s!(   4,    4), s!(  -5,   -5),
            s!(   9,    3), s!(   3,   -1), s!(   1,   -5), s!(  -2,   -5), s!(  -5,  -13), s!(  -5,   -9), s!(   0,  -10), s!( -10,  -10),
            s!(   4,   -2), s!(  11,    5), s!(   5,    0), s!(   2,   -3), s!(  -8,   -7), s!(  -6,   -9), s!( -12,  -13), s!(   9,    2),
            s!(  27,    6), s!(  11,    7), s!(  -2,   -0), s!(   3,    2), s!(  -9,   -7), s!(  -1,   -1), s!(  11,   -3), s!(  -7,    1),
            s!(   0,    5), s!(   2,    1), s!(   8,    5), s!(   8,    0), s!(   5,    4), s!( -10,   -8), s!(  -3,   -1), s!(  -1,   -2),
            s!(   8,    4), s!(   6,    1), s!(   3,    2), s!(  -1,    1), s!(   2,    4), s!(   2,   -4), s!(  -2,    1), s!(  -4,   -0),
            s!(   6,    6), s!(   4,    2), s!(  -7,   -1), s!(   6,    4), s!(  -8,   -2), s!(   3,    2), s!(   1,    2), s!(   1,    2),
        ],
        [
            s!(  26,    5), s!(   0,   -6), s!(   2,    1), s!(   2,    1), s!(   1,   -4), s!(  -2,   -7), s!(  -7,   -6), s!( -22,  -15),
            s!(  59,   15), s!(   2,   -3), s!(   1,    1), s!(   1,   -6), s!(  -1,   -5), s!(  -9,   -6), s!( -23,  -11), s!( -28,  -13),
            s!(  14,    3), s!(  16,    5), s!(  20,   11), s!(   9,   -0), s!(   3,  -10), s!( -20,  -11), s!( -21,   -7), s!( -19,  -12),
            s!(  29,    8), s!(  19,    1), s!(  10,    2), s!(  -5,    2), s!(   1,  -11), s!(  -4,  -12), s!( -28,   -6), s!( -24,   -6),
            s!(  30,   12), s!(  23,    7), s!(   7,    1), s!(   3,    1), s!(  -8,   -5), s!( -17,   -6), s!(  -1,    4), s!( -23,  -10),
            s!(  28,   13), s!(   2,    3), s!(   6,   -1), s!(   3,   -1), s!(   0,   -4), s!(   0,   -1), s!( -21,   -1), s!(  -3,    1),
            s!(  12,    7), s!(   5,    4), s!(   5,  -10), s!( -10,   -4), s!(  -1,   -3), s!(   3,    0), s!( -20,    1), s!( -15,   -2),
            s!(  10,    3), s!(   3,    5), s!( -21,   -1), s!(  -7,  -15), s!( -10,    6), s!(  10,   -0), s!(   2,    2), s!(   1,   -1),
        ],
        [
            s!(   8,    8), s!(   2,    0), s!(   4,    2), s!(   4,    3), s!(  -0,   -2), s!(  -5,   -7), s!(  -3,   -7), s!(  -7,   -5),
            s!(   2,    1), s!(  -2,   -2), s!(   3,    2), s!(  12,    7), s!(  -7,  -16), s!( -17,  -11), s!( -15,   -8), s!( -37,  -13),
            s!(  17,    3), s!(   6,    6), s!(  12,    8), s!(   6,   -1), s!(  -9,    0), s!( -29,  -17), s!( -36,  -19), s!( -22,  -20),
            s!(   6,    9), s!(  19,    2), s!(  13,    7), s!(  11,   -2), s!(   5,  -11), s!( -18,   -3), s!( -24,  -13), s!( -19,   -9),
            s!(  17,    1), s!(  -3,    1), s!(   5,   -4), s!(  -7,  -10), s!(   8,   -5), s!(  -3,   -5), s!( -13,   -6), s!( -17,  -14),
            s!(   2,    3), s!(   8,   -3), s!(   2,    2), s!(   4,    1), s!(  10,    4), s!(   5,   -0), s!(  -0,    4), s!(  -9,   -5),
            s!(  15,    7), s!(  11,    2), s!(   6,    0), s!(   0,    6), s!(   5,    7), s!(   7,   -3), s!(  -2,    2), s!( -16,   -6),
            s!(   4,    4), s!(  12,   -2), s!(   4,    4), s!(  -8,    4), s!(  -1,    4), s!(   5,    1), s!(   4,   -0), s!(   1,    1),
        ],
        [
            s!(   1,    1), s!(   0,   -5), s!(   4,    3), s!(   6,    5), s!(  -3,   -3), s!(  -2,   -7), s!( -13,  -10), s!( -11,  -11),
            s!(   0,    0), s!(   2,    1), s!(  16,    7), s!(   3,    1), s!(   1,   -5), s!( -14,  -11), s!(  -5,   -3), s!( -14,  -11),
            s!(  -3,   -1), s!(   6,    2), s!(  10,    7), s!(   5,    5), s!(  -7,  -13), s!( -14,   -7), s!( -14,  -11), s!(  -7,   -4),
            s!(  -3,   -0), s!(  10,    3), s!(  13,    6), s!(  -4,  -12), s!(   7,   -3), s!( -10,   -7), s!(  -7,   -2), s!( -18,  -12),
            s!(   8,    0), s!(  10,    9), s!(   6,    7), s!(   7,    3), s!(   2,   -1), s!(  -3,   -2), s!(  -7,   -3), s!(  -6,   -2),
            s!(   0,    0), s!(  -5,    4), s!(  -1,   -1), s!(   2,   -3), s!(  12,    3), s!(  -8,   -2), s!(  -3,    2), s!( -30,   -9),
            s!(  10,    2), s!(   4,    8), s!(   8,   -5), s!(   8,    3), s!(   6,    6), s!( -12,   -5), s!(   1,   -2), s!(  -2,   -5),
            s!(  -1,    1), s!(   1,    2), s!(  -2,    2), s!(  18,    4), s!(  -1,    0), s!(   0,    1), s!(   2,    1), s!(  -1,    1),
        ],
    ],
];

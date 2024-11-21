// ------------------------------------------------------------------------- //
// Generated at 21-11-2024 07:47:21 UTC (e = 0.112638, k = 0.0077, r = 1.00) //
// ------------------------------------------------------------------------- //

use super::*;

#[rustfmt::skip]
pub const QUEEN_PST_PATTERN: [[[PackedEval; 64]; KING_BUCKETS_COUNT]; 2] =
[
    [
        [
            s!(1052, 1088), s!(1041, 1109), s!(1057, 1093), s!(1087, 1094), s!(1070, 1107), s!(1106, 1088), s!(1118, 1086), s!(1090, 1091),
            s!(1038, 1078), s!(1014, 1106), s!(1042, 1101), s!(1049, 1109), s!(1050, 1103), s!(1085, 1078), s!(1068, 1113), s!(1112, 1058),
            s!(1044, 1056), s!(1046, 1081), s!(1063, 1048), s!(1069, 1075), s!(1075, 1072), s!(1115, 1067), s!(1091, 1066), s!(1065, 1108),
            s!(1046, 1055), s!(1056, 1060), s!(1029, 1084), s!(1053, 1060), s!(1088, 1051), s!(1076, 1079), s!(1071, 1101), s!(1079, 1079),
            s!(1060, 1035), s!(1041, 1083), s!(1057, 1059), s!(1060, 1050), s!(1068, 1050), s!(1081, 1047), s!(1089, 1057), s!(1084, 1060),
            s!(1033, 1024), s!(1062, 1012), s!(1059, 1047), s!(1062, 1032), s!(1068, 1040), s!(1086, 1027), s!(1077, 1052), s!(1068, 1023),
            s!(1049, 1076), s!(1058, 1040), s!(1064, 1015), s!(1061, 1032), s!(1071, 1027), s!(1077, 1011), s!(1085,  969), s!(1029, 1002),
            s!(1067, 1009), s!(1053, 1020), s!(1052,  986), s!(1069,  992), s!(1072,  991), s!(1064,  985), s!(1031, 1016), s!( 998, 1007),
        ],
        [
            s!(1065, 1095), s!(1042, 1114), s!(1062, 1097), s!(1083, 1084), s!(1084, 1104), s!(1094, 1096), s!(1129, 1075), s!(1098, 1092),
            s!(1040, 1101), s!(1029, 1124), s!(1061, 1089), s!(1064, 1104), s!(1042, 1133), s!(1071, 1102), s!(1058, 1145), s!(1123, 1074),
            s!(1078, 1055), s!(1063, 1083), s!(1076, 1090), s!(1064, 1102), s!(1070, 1100), s!(1091, 1117), s!(1114, 1092), s!(1077, 1112),
            s!(1072, 1055), s!(1071, 1081), s!(1075, 1075), s!(1070, 1087), s!(1083, 1074), s!(1077, 1097), s!(1076, 1104), s!(1079, 1102),
            s!(1080, 1059), s!(1073, 1070), s!(1081, 1047), s!(1072, 1065), s!(1073, 1068), s!(1069, 1085), s!(1091, 1068), s!(1079, 1069),
            s!(1073, 1030), s!(1078, 1041), s!(1086, 1042), s!(1082, 1031), s!(1081, 1047), s!(1088, 1047), s!(1089, 1057), s!(1080, 1043),
            s!(1100, 1021), s!(1085, 1035), s!(1089, 1018), s!(1083, 1021), s!(1092, 1020), s!(1104,  994), s!(1087,  988), s!(1032, 1042),
            s!(1113,  991), s!(1084, 1021), s!(1078, 1017), s!(1092,  995), s!(1087,  998), s!(1085,  977), s!(1005, 1028), s!(1028, 1033),
        ],
        [
            s!( 985, 1066), s!(1030, 1127), s!(1040, 1097), s!(1071, 1096), s!(1083, 1088), s!(1080, 1098), s!(1104, 1058), s!(1058, 1077),
            s!( 998, 1063), s!(1012, 1097), s!(1016, 1099), s!(1031, 1107), s!(1020, 1124), s!(1058, 1086), s!(1043, 1107), s!(1063, 1062),
            s!(1030, 1022), s!(1032, 1060), s!(1058, 1057), s!(1065, 1078), s!(1075, 1081), s!(1097, 1062), s!(1090, 1066), s!(1048, 1084),
            s!(1015, 1037), s!(1029, 1043), s!(1028, 1081), s!(1034, 1090), s!(1080, 1058), s!(1062, 1078), s!(1065, 1087), s!(1058, 1096),
            s!(1029, 1027), s!(1013, 1060), s!(1049, 1044), s!(1053, 1065), s!(1063, 1063), s!(1076, 1062), s!(1065, 1058), s!(1059, 1078),
            s!(1016, 1014), s!(1028, 1020), s!(1054, 1039), s!(1047, 1039), s!(1043, 1046), s!(1075, 1031), s!(1063, 1045), s!(1045, 1061),
            s!(1020, 1056), s!(1049, 1029), s!(1054, 1011), s!(1057, 1003), s!(1064, 1006), s!(1051,  987), s!(1054, 1008), s!(1037, 1040),
            s!(1034, 1000), s!(1036,  998), s!(1018,  994), s!(1055,  944), s!(1031,  998), s!(1037, 1007), s!(1020, 1044), s!(1036, 1023),
        ],
        [
            s!(1003, 1067), s!(1007, 1105), s!(1063, 1099), s!(1073, 1081), s!(1069, 1076), s!(1085, 1074), s!(1088, 1060), s!(1027, 1081),
            s!(1011, 1043), s!( 986, 1086), s!( 993, 1096), s!(1008, 1114), s!( 990, 1126), s!(1018, 1082), s!( 996, 1104), s!(1054, 1076),
            s!(1046, 1025), s!( 998, 1055), s!(1010, 1100), s!(1040, 1091), s!(1028, 1105), s!(1068, 1073), s!(1068, 1077), s!(1053, 1078),
            s!(1034, 1059), s!(1032, 1056), s!(1028, 1069), s!(1030, 1088), s!(1006, 1081), s!(1033, 1081), s!(1020, 1079), s!(1047, 1086),
            s!(1049, 1036), s!(1022, 1058), s!(1040, 1055), s!(1040, 1052), s!(1034, 1076), s!(1040, 1069), s!(1045, 1053), s!(1048, 1068),
            s!(1049, 1010), s!(1050, 1002), s!(1069, 1027), s!(1054, 1016), s!(1053, 1043), s!(1060, 1009), s!(1055, 1027), s!(1048, 1035),
            s!(1040, 1042), s!(1065,  986), s!(1067,  965), s!(1066,  981), s!(1066,  985), s!(1077,  982), s!(1054, 1006), s!(1092, 1024),
            s!(1083, 1004), s!(1060, 1007), s!(1046,  950), s!(1060,  961), s!(1022, 1004), s!(1042,  976), s!(1009, 1045), s!(1027, 1029),
        ],
        [
            s!(1007, 1056), s!(1009, 1096), s!(1031, 1068), s!(1066, 1079), s!(1085, 1087), s!(1073, 1071), s!(1114, 1072), s!(1030, 1053),
            s!(1029, 1069), s!(1006, 1095), s!(1048, 1108), s!(1033, 1123), s!(1008, 1111), s!(1055, 1065), s!(1018, 1086), s!(1100, 1079),
            s!(1037, 1022), s!(1041, 1057), s!(1057, 1046), s!(1055, 1081), s!(1082, 1075), s!(1076, 1054), s!(1061, 1052), s!(1071, 1081),
            s!(1008, 1040), s!(1031, 1060), s!(1026, 1072), s!(1030, 1067), s!(1078, 1066), s!(1061, 1055), s!(1043, 1076), s!(1061, 1073),
            s!(1020, 1028), s!(1031, 1057), s!(1048, 1052), s!(1083, 1074), s!(1057, 1049), s!(1084, 1063), s!(1033, 1054), s!(1057, 1058),
            s!(1048, 1019), s!(1056, 1007), s!(1064, 1026), s!(1059, 1034), s!(1046, 1032), s!(1055, 1035), s!(1035, 1038), s!(1036, 1047),
            s!(1005, 1044), s!(1037,  999), s!(1028, 1002), s!(1047,  996), s!(1062,  983), s!(1022, 1001), s!(1070, 1019), s!(1049, 1041),
            s!(1035,  991), s!(1034, 1000), s!(1016, 1000), s!(1017,  992), s!( 991, 1005), s!(1009, 1003), s!(1011, 1043), s!(1031, 1029),
        ],
        [
            s!(1007, 1052), s!( 989, 1088), s!(1055, 1094), s!(1071, 1086), s!(1088, 1077), s!(1078, 1069), s!(1098, 1049), s!(1042, 1059),
            s!(1042, 1061), s!(1022, 1088), s!(1044, 1114), s!(1022, 1117), s!(1014, 1124), s!(1086, 1085), s!(1009, 1087), s!(1041, 1062),
            s!(1064, 1034), s!(1055, 1066), s!(1077, 1073), s!(1050, 1086), s!(1079, 1082), s!(1079, 1068), s!(1076, 1071), s!(1066, 1047),
            s!(1068, 1046), s!(1039, 1067), s!(1058, 1071), s!(1058, 1069), s!(1069, 1074), s!(1073, 1055), s!(1062, 1073), s!(1052, 1064),
            s!(1062, 1033), s!(1051, 1052), s!(1055, 1066), s!(1061, 1067), s!(1063, 1014), s!(1055, 1080), s!(1048, 1066), s!(1047, 1057),
            s!(1037, 1001), s!(1067, 1023), s!(1085, 1029), s!(1074, 1013), s!(1072, 1043), s!(1069, 1012), s!(1070, 1013), s!(1042, 1056),
            s!(1010, 1052), s!(1051, 1018), s!(1089, 1001), s!(1078, 1003), s!(1065, 1041), s!(1053, 1013), s!(1053, 1020), s!(1064, 1038),
            s!(1023,  993), s!(1024, 1004), s!(1048,  996), s!(1046,  983), s!(1050, 1001), s!(1040, 1001), s!(1032, 1045), s!(1043, 1029),
        ],
        [
            s!(1030, 1051), s!(1035, 1109), s!(1060, 1083), s!(1090, 1084), s!(1079, 1079), s!(1070, 1064), s!(1114, 1066), s!(1064, 1088),
            s!(1056, 1053), s!(1016, 1101), s!(1052, 1097), s!(1058, 1127), s!(1048, 1112), s!(1072, 1065), s!(1045, 1085), s!(1055, 1062),
            s!(1067, 1048), s!(1049, 1085), s!(1084, 1072), s!(1058, 1092), s!(1089, 1081), s!(1099, 1074), s!(1092, 1061), s!(1084, 1066),
            s!(1056, 1058), s!(1061, 1075), s!(1068, 1071), s!(1052, 1080), s!(1088, 1075), s!(1063, 1045), s!(1054, 1080), s!(1053, 1075),
            s!(1060, 1048), s!(1055, 1076), s!(1078, 1046), s!(1076, 1047), s!(1088, 1013), s!(1066, 1050), s!(1056, 1089), s!(1030, 1074),
            s!(1024, 1015), s!(1073, 1009), s!(1058, 1068), s!(1068, 1062), s!(1065, 1049), s!(1074, 1013), s!(1047, 1047), s!(1042, 1070),
            s!(1014, 1046), s!(1062, 1011), s!(1086,  973), s!(1071, 1022), s!(1061, 1024), s!(1071,  998), s!(1050, 1033), s!(1061, 1052),
            s!(1038,  999), s!(1052, 1004), s!(1065,  979), s!(1045,  993), s!(1052, 1018), s!(1026, 1002), s!(1040, 1056), s!( 995, 1013),
        ],
        [
            s!(1035, 1063), s!(1025, 1110), s!(1064, 1101), s!(1075, 1079), s!(1073, 1077), s!(1082, 1066), s!(1096, 1054), s!(1061, 1079),
            s!(1059, 1083), s!(1055, 1105), s!(1050, 1109), s!(1042, 1121), s!(1029, 1114), s!(1081, 1075), s!(1025, 1078), s!(1069, 1051),
            s!(1060, 1039), s!(1089, 1080), s!(1071, 1078), s!(1054, 1098), s!(1077, 1079), s!(1087, 1070), s!(1096, 1070), s!(1043, 1076),
            s!(1061, 1062), s!(1089, 1084), s!(1054, 1087), s!(1042, 1068), s!(1055, 1059), s!(1051, 1068), s!(1061, 1082), s!(1054, 1082),
            s!(1085, 1045), s!(1050, 1071), s!(1069, 1055), s!(1065, 1055), s!(1071, 1038), s!(1054, 1060), s!(1046, 1062), s!(1018, 1059),
            s!(1023, 1011), s!(1082, 1039), s!(1082, 1037), s!(1075, 1033), s!(1066, 1038), s!(1069, 1019), s!(1045, 1032), s!(1059, 1047),
            s!(1010, 1046), s!(1040, 1012), s!(1084, 1007), s!(1081, 1027), s!(1053, 1031), s!(1084,  984), s!(1064, 1026), s!(1059, 1059),
            s!(1063, 1005), s!(1047, 1002), s!(1056, 1002), s!(1095,  986), s!(1035, 1018), s!(1033,  998), s!(1022, 1048), s!(1018, 1027),
        ],
        [
            s!(1044, 1080), s!(1031, 1112), s!(1052, 1095), s!(1084, 1095), s!(1071, 1103), s!(1106, 1090), s!(1119, 1083), s!(1082, 1090),
            s!(1038, 1075), s!(1015, 1109), s!(1037, 1099), s!(1043, 1109), s!(1043, 1115), s!(1085, 1087), s!(1064, 1102), s!(1107, 1062),
            s!(1041, 1045), s!(1045, 1079), s!(1058, 1045), s!(1064, 1079), s!(1073, 1077), s!(1109, 1068), s!(1095, 1063), s!(1065, 1092),
            s!(1045, 1051), s!(1053, 1058), s!(1029, 1077), s!(1052, 1066), s!(1089, 1056), s!(1078, 1080), s!(1072, 1089), s!(1078, 1076),
            s!(1056, 1031), s!(1041, 1072), s!(1057, 1054), s!(1058, 1054), s!(1071, 1046), s!(1080, 1051), s!(1089, 1057), s!(1080, 1053),
            s!(1033, 1020), s!(1058, 1009), s!(1059, 1043), s!(1060, 1040), s!(1065, 1051), s!(1086, 1031), s!(1073, 1045), s!(1065, 1025),
            s!(1044, 1065), s!(1058, 1036), s!(1061, 1021), s!(1059, 1034), s!(1070, 1029), s!(1077, 1017), s!(1085,  978), s!(1027, 1012),
            s!(1062, 1009), s!(1050, 1019), s!(1053,  986), s!(1061,  995), s!(1069,  991), s!(1057,  989), s!(1027, 1020), s!(1000, 1008),
        ],
        [
            s!(1060, 1083), s!(1036, 1115), s!(1061, 1092), s!(1079, 1080), s!(1083, 1092), s!(1086, 1086), s!(1119, 1059), s!(1079, 1086),
            s!(1038, 1087), s!(1022, 1117), s!(1051, 1079), s!(1057, 1101), s!(1034, 1121), s!(1067, 1092), s!(1051, 1130), s!(1107, 1073),
            s!(1071, 1043), s!(1055, 1075), s!(1065, 1083), s!(1051, 1086), s!(1064, 1092), s!(1082, 1094), s!(1107, 1089), s!(1071, 1094),
            s!(1065, 1045), s!(1061, 1072), s!(1067, 1062), s!(1060, 1083), s!(1076, 1068), s!(1067, 1084), s!(1071, 1096), s!(1074, 1091),
            s!(1071, 1052), s!(1065, 1061), s!(1074, 1046), s!(1069, 1068), s!(1065, 1054), s!(1062, 1071), s!(1082, 1056), s!(1071, 1056),
            s!(1068, 1016), s!(1072, 1037), s!(1078, 1030), s!(1075, 1031), s!(1073, 1039), s!(1083, 1041), s!(1085, 1050), s!(1076, 1035),
            s!(1088, 1019), s!(1078, 1031), s!(1080, 1012), s!(1076, 1016), s!(1085, 1014), s!(1099,  987), s!(1081,  988), s!(1029, 1036),
            s!(1101,  994), s!(1076, 1013), s!(1077, 1001), s!(1082,  985), s!(1081,  988), s!(1074,  978), s!(1007, 1031), s!(1026, 1029),
        ],
        [
            s!( 993, 1062), s!(1024, 1122), s!(1033, 1085), s!(1060, 1080), s!(1076, 1075), s!(1078, 1085), s!(1104, 1058), s!(1054, 1072),
            s!( 997, 1056), s!(1004, 1088), s!(1014, 1086), s!(1029, 1107), s!(1026, 1127), s!(1061, 1081), s!(1034, 1098), s!(1070, 1061),
            s!(1027, 1022), s!(1025, 1050), s!(1048, 1050), s!(1053, 1073), s!(1068, 1068), s!(1095, 1054), s!(1085, 1062), s!(1051, 1083),
            s!(1014, 1029), s!(1027, 1038), s!(1029, 1064), s!(1036, 1068), s!(1074, 1051), s!(1064, 1074), s!(1063, 1091), s!(1064, 1082),
            s!(1028, 1025), s!(1011, 1051), s!(1047, 1034), s!(1055, 1051), s!(1069, 1046), s!(1071, 1056), s!(1069, 1061), s!(1066, 1068),
            s!(1016, 1008), s!(1029, 1011), s!(1052, 1027), s!(1046, 1025), s!(1046, 1034), s!(1075, 1023), s!(1062, 1040), s!(1054, 1060),
            s!(1015, 1056), s!(1050, 1025), s!(1055, 1005), s!(1058, 1002), s!(1060,  995), s!(1055,  989), s!(1050, 1014), s!(1043, 1040),
            s!(1033,  997), s!(1037,  995), s!(1021,  990), s!(1052,  953), s!(1026, 1004), s!(1033, 1007), s!(1019, 1046), s!(1028, 1020),
        ],
        [
            s!(1006, 1067), s!(1010, 1106), s!(1060, 1095), s!(1072, 1081), s!(1067, 1075), s!(1077, 1071), s!(1094, 1062), s!(1031, 1074),
            s!(1013, 1046), s!( 994, 1088), s!( 991, 1092), s!(1007, 1112), s!( 992, 1119), s!(1018, 1075), s!(1000, 1094), s!(1057, 1071),
            s!(1043, 1030), s!(1005, 1055), s!(1012, 1076), s!(1036, 1093), s!(1027, 1092), s!(1062, 1070), s!(1069, 1072), s!(1055, 1079),
            s!(1035, 1049), s!(1031, 1047), s!(1025, 1070), s!(1025, 1078), s!(1008, 1065), s!(1036, 1073), s!(1021, 1082), s!(1047, 1093),
            s!(1047, 1030), s!(1021, 1055), s!(1043, 1048), s!(1041, 1052), s!(1036, 1059), s!(1044, 1066), s!(1043, 1059), s!(1048, 1075),
            s!(1050, 1017), s!(1051,  999), s!(1069, 1031), s!(1053, 1014), s!(1051, 1040), s!(1061, 1010), s!(1054, 1027), s!(1048, 1042),
            s!(1034, 1043), s!(1063,  990), s!(1065,  959), s!(1067,  972), s!(1066,  977), s!(1075,  985), s!(1059, 1009), s!(1094, 1035),
            s!(1075, 1003), s!(1061, 1009), s!(1047,  968), s!(1059,  953), s!(1031, 1009), s!(1034,  990), s!(1013, 1045), s!(1025, 1029),
        ],
        [
            s!(1009, 1060), s!(1009, 1098), s!(1029, 1064), s!(1064, 1074), s!(1083, 1086), s!(1074, 1068), s!(1111, 1069), s!(1040, 1059),
            s!(1023, 1062), s!(1006, 1102), s!(1040, 1098), s!(1025, 1119), s!(1006, 1108), s!(1063, 1069), s!(1029, 1090), s!(1095, 1078),
            s!(1039, 1027), s!(1037, 1057), s!(1049, 1048), s!(1047, 1075), s!(1075, 1066), s!(1078, 1054), s!(1062, 1054), s!(1066, 1083),
            s!(1018, 1044), s!(1035, 1059), s!(1025, 1067), s!(1031, 1069), s!(1068, 1055), s!(1056, 1052), s!(1047, 1076), s!(1058, 1075),
            s!(1027, 1035), s!(1031, 1054), s!(1051, 1048), s!(1079, 1070), s!(1058, 1042), s!(1079, 1064), s!(1040, 1054), s!(1055, 1060),
            s!(1043, 1015), s!(1061, 1007), s!(1058, 1023), s!(1061, 1032), s!(1044, 1024), s!(1059, 1026), s!(1040, 1027), s!(1045, 1048),
            s!(1011, 1045), s!(1038,  999), s!(1028,  993), s!(1053, 1004), s!(1063,  988), s!(1036, 1000), s!(1067, 1021), s!(1057, 1043),
            s!(1043,  995), s!(1038, 1001), s!(1023,  998), s!(1022,  988), s!(1016, 1010), s!(1024, 1003), s!(1013, 1044), s!(1018, 1023),
        ],
        [
            s!(1007, 1050), s!( 996, 1091), s!(1044, 1082), s!(1068, 1080), s!(1080, 1072), s!(1075, 1067), s!(1100, 1053), s!(1051, 1068),
            s!(1035, 1065), s!(1013, 1085), s!(1033, 1106), s!(1025, 1117), s!(1013, 1122), s!(1077, 1078), s!(1021, 1087), s!(1053, 1058),
            s!(1050, 1037), s!(1046, 1063), s!(1063, 1058), s!(1047, 1080), s!(1076, 1072), s!(1075, 1055), s!(1080, 1070), s!(1064, 1061),
            s!(1061, 1046), s!(1037, 1062), s!(1054, 1073), s!(1055, 1062), s!(1066, 1053), s!(1067, 1046), s!(1063, 1072), s!(1053, 1071),
            s!(1062, 1033), s!(1049, 1060), s!(1058, 1058), s!(1060, 1049), s!(1061, 1015), s!(1057, 1067), s!(1049, 1066), s!(1045, 1062),
            s!(1034, 1010), s!(1067, 1022), s!(1082, 1027), s!(1069, 1011), s!(1071, 1040), s!(1068, 1021), s!(1067, 1022), s!(1046, 1058),
            s!(1009, 1051), s!(1045, 1010), s!(1088, 1004), s!(1077, 1013), s!(1066, 1036), s!(1054, 1003), s!(1056, 1019), s!(1066, 1036),
            s!(1035,  994), s!(1024, 1001), s!(1050,  998), s!(1049,  979), s!(1046, 1005), s!(1040, 1002), s!(1022, 1046), s!(1035, 1027),
        ],
        [
            s!(1025, 1051), s!(1017, 1093), s!(1045, 1078), s!(1078, 1070), s!(1075, 1076), s!(1074, 1066), s!(1114, 1063), s!(1064, 1083),
            s!(1046, 1058), s!(1009, 1099), s!(1046, 1093), s!(1044, 1113), s!(1038, 1114), s!(1070, 1062), s!(1043, 1086), s!(1067, 1064),
            s!(1059, 1045), s!(1043, 1067), s!(1078, 1072), s!(1059, 1089), s!(1087, 1077), s!(1097, 1065), s!(1087, 1063), s!(1076, 1075),
            s!(1054, 1055), s!(1053, 1066), s!(1061, 1071), s!(1056, 1078), s!(1086, 1070), s!(1062, 1048), s!(1053, 1075), s!(1052, 1075),
            s!(1058, 1038), s!(1052, 1078), s!(1076, 1042), s!(1075, 1052), s!(1084, 1029), s!(1061, 1048), s!(1058, 1078), s!(1029, 1067),
            s!(1024, 1016), s!(1076, 1009), s!(1060, 1061), s!(1066, 1044), s!(1065, 1048), s!(1074, 1021), s!(1044, 1035), s!(1044, 1062),
            s!(1016, 1047), s!(1060, 1015), s!(1083,  984), s!(1072, 1028), s!(1061, 1026), s!(1069, 1001), s!(1055, 1025), s!(1064, 1048),
            s!(1049, 1002), s!(1053, 1006), s!(1061,  987), s!(1049,  994), s!(1051, 1022), s!(1034, 1003), s!(1036, 1055), s!(1004, 1017),
        ],
        [
            s!(1024, 1053), s!(1020, 1105), s!(1053, 1091), s!(1070, 1074), s!(1073, 1076), s!(1077, 1062), s!(1103, 1058), s!(1061, 1080),
            s!(1044, 1074), s!(1040, 1101), s!(1040, 1097), s!(1032, 1116), s!(1023, 1116), s!(1071, 1067), s!(1032, 1086), s!(1074, 1057),
            s!(1058, 1039), s!(1069, 1072), s!(1068, 1070), s!(1055, 1097), s!(1076, 1078), s!(1091, 1068), s!(1088, 1068), s!(1048, 1081),
            s!(1052, 1052), s!(1074, 1077), s!(1048, 1080), s!(1041, 1065), s!(1065, 1054), s!(1055, 1058), s!(1063, 1085), s!(1056, 1082),
            s!(1071, 1042), s!(1051, 1071), s!(1064, 1055), s!(1063, 1052), s!(1072, 1045), s!(1055, 1062), s!(1054, 1068), s!(1029, 1059),
            s!(1031, 1015), s!(1083, 1030), s!(1081, 1038), s!(1074, 1032), s!(1069, 1038), s!(1069, 1022), s!(1047, 1035), s!(1059, 1052),
            s!(1017, 1050), s!(1049, 1014), s!(1084, 1015), s!(1078, 1023), s!(1055, 1023), s!(1080,  992), s!(1064, 1027), s!(1061, 1057),
            s!(1063, 1004), s!(1051, 1006), s!(1053, 1003), s!(1093,  996), s!(1039, 1019), s!(1037,  998), s!(1015, 1048), s!(1014, 1025),
        ],
    ],
    [
        [
            s!( -42,   -8), s!( -32,  -11), s!(  -8,    4), s!( -24,    7), s!(  -9,   10), s!( -17,    4), s!(   6,    9), s!(   5,    4),
            s!( -15,   -8), s!( -16,   -7), s!( -16,   -2), s!(   6,  -19), s!(  24,   -7), s!(  18,   17), s!(   3,    1), s!(   6,    7),
            s!(  -9,  -34), s!( -16,  -16), s!( -34,    3), s!(  -8,  -18), s!( -16,   20), s!(  43,   -4), s!(  62,   -4), s!(  -9,   24),
            s!( -14,  -10), s!( -32,    3), s!(  -6,  -18), s!( -10,    6), s!(   3,   -3), s!(  18,    3), s!(  29,   16), s!(   1,  -12),
            s!( -26,  -27), s!( -18,  -16), s!( -16,   12), s!(  12,  -19), s!(   3,    1), s!(  10,    4), s!( -19,   21), s!(   7,   -4),
            s!(  -7,  -20), s!( -18,    2), s!(  -1,  -14), s!(  -6,    4), s!(  -6,    7), s!(  -2,   10), s!(  -7,    7), s!(  11,  -12),
            s!( -37,  -10), s!( -12,  -13), s!( -11,   11), s!(  -6,    1), s!(  -6,   -5), s!( -14,   31), s!(  17,  -17), s!(  -4,  -13),
            s!( -13,  -22), s!( -19,   -5), s!(  -8,  -12), s!(  -9,  -14), s!( -10,   -9), s!(  -9,  -13), s!(  -1,  -20), s!(  11,   -8),
        ],
        [
            s!( -27,   27), s!(  14,   -1), s!(   5,   -1), s!(   8,    3), s!( -12,    8), s!(  24,   25), s!(   4,    8), s!(  38,   26),
            s!( -21,   21), s!( -15,   29), s!(  -6,   13), s!(  -3,   18), s!( -12,   31), s!(  13,    6), s!(  12,    9), s!(  22,   17),
            s!( -13,   11), s!( -20,   21), s!( -20,   15), s!(  -8,   14), s!(  24,   17), s!(  15,    8), s!( -13,   27), s!(  18,   -0),
            s!(  -5,    5), s!(  -9,    5), s!( -14,    8), s!(  -5,   23), s!(  -9,   29), s!(  -3,   25), s!(   3,    7), s!(   4,    3),
            s!( -13,   10), s!(  -7,    3), s!(  -0,   12), s!(  -0,   19), s!(   4,    9), s!(   3,   -0), s!(   8,   -7), s!(   6,    6),
            s!(   2,  -13), s!(  -2,    7), s!(  -4,    4), s!(  -4,   24), s!(  -4,   13), s!(   2,   10), s!(   7,   -2), s!(  18,   -8),
            s!(  -9,  -14), s!(  -7,    7), s!(  -2,   15), s!(   5,    1), s!(  -1,    6), s!(   4,  -11), s!(  22,  -33), s!(  20,    2),
            s!(  -9,  -17), s!(  -2,   -2), s!(  15,  -27), s!(  -2,   15), s!(   5,   -2), s!(  -1,  -11), s!(  18,  -17), s!(  -5,    2),
        ],
        [
            s!(  -9,   -3), s!(   6,   -0), s!(  15,    5), s!(  14,    2), s!(  17,   17), s!(  18,   16), s!(  17,   13), s!(  28,    6),
            s!(  -9,    0), s!(  27,   -5), s!(   6,   -0), s!(  18,   -2), s!(   3,    0), s!(   8,    3), s!(  12,    0), s!(  55,   -9),
            s!(   7,    7), s!(  -0,   -2), s!(  -6,    4), s!(  21,    9), s!(  10,   -4), s!(  48,   28), s!(  34,   17), s!(  34,   16),
            s!(  -4,    6), s!(  -3,    4), s!(  -6,    3), s!(  21,  -15), s!(  -4,    3), s!(   7,   -2), s!(  44,  -18), s!(  27,   17),
            s!( -13,   -2), s!(  -6,   -4), s!(  -4,   -3), s!(  -2,    7), s!( -27,   -5), s!(   8,   -2), s!( -18,   -3), s!(  20,    9),
            s!(  -6,  -22), s!(  13,  -15), s!(   1,   -1), s!(  -0,    5), s!( -15,   -6), s!(  10,  -11), s!(  -9,   -8), s!( -30,   23),
            s!( -14,  -12), s!(  -6,   -4), s!( -12,   -4), s!(   2,  -20), s!(  -7,    5), s!(  -2,    6), s!( -10,   -2), s!(  13,   -1),
            s!(  -4,   -3), s!(  -4,   -5), s!(   9,    4), s!(   2,   -6), s!(  -8,  -31), s!(  -2,  -13), s!( -20,   -6), s!(   2,    3),
        ],
        [
            s!(   4,   -6), s!(   2,    2), s!(   2,   -5), s!(   8,    2), s!(   7,    4), s!(  14,   15), s!(  -6,  -14), s!( -14,    2),
            s!(  26,    0), s!(  30,   21), s!(  18,    3), s!(   0,   -3), s!(  -0,   -0), s!(   8,    6), s!(   7,   24), s!(  -3,   15),
            s!(   2,    0), s!(  34,    7), s!(  32,   20), s!(   2,   22), s!(  13,    6), s!(  -6,   13), s!(  21,    1), s!(   9,  -10),
            s!(   2,    1), s!(  -5,   14), s!(   7,   15), s!(  -8,    7), s!(  -0,   13), s!(  16,   -4), s!(  -5,    6), s!(   5,    9),
            s!(  -7,   20), s!(   2,    9), s!(   5,    2), s!( -15,   -3), s!(   6,   12), s!(   9,   -8), s!(   1,   -5), s!(  -3,   10),
            s!(  -4,   14), s!(   3,   21), s!( -14,    0), s!(  -9,  -26), s!(  -6,   -5), s!(  -8,   -2), s!(   6,  -23), s!(  12,  -13),
            s!(  11,    1), s!(   3,    1), s!(  -4,  -21), s!(  -9,   -4), s!(  -2,   -4), s!( -11,    7), s!(   5,   -5), s!(  10,   -2),
            s!(  11,    5), s!(   3,    9), s!(   4,   -4), s!(  -9,   -6), s!(   5,  -16), s!(  -6,   -6), s!(   5,    6), s!(  26,   10),
        ],
        [
            s!(  14,   17), s!(  37,   23), s!(   8,    4), s!(   0,    1), s!(   2,   -3), s!(  -0,   -5), s!(  -4,    2), s!(  13,   -3),
            s!(  20,   15), s!(  37,    4), s!(   1,    0), s!(   2,    1), s!(  12,   10), s!(   4,  -13), s!(   8,   16), s!(  -8,   -2),
            s!(  19,   11), s!(  11,    5), s!(  15,    9), s!(   2,    0), s!(   2,    6), s!(  -6,   -7), s!(  12,   -6), s!( -13,   -6),
            s!(  13,    5), s!(  29,   15), s!(  11,    3), s!(   9,    1), s!( -18,   -5), s!(  -9,   -8), s!( -14,   -6), s!(  16,    9),
            s!(  43,    8), s!(  23,    8), s!( -10,   -1), s!(   5,    5), s!( -23,  -17), s!(  -4,    8), s!(  35,    7), s!( -18,    3),
            s!( -10,    0), s!(   8,   12), s!(  16,    7), s!(  17,   -3), s!(   8,    4), s!( -11,  -13), s!(  -7,   -6), s!(   1,   -2),
            s!(  21,   11), s!(  13,    5), s!(   7,    6), s!(  -1,   -2), s!(   3,    6), s!(   9,   -4), s!( -13,   -0), s!( -14,   -1),
            s!(  11,    6), s!(   4,   -3), s!( -19,   -5), s!(   8,   10), s!( -19,    0), s!(   5,    1), s!(  -5,   -1), s!(  -0,    2),
        ],
        [
            s!(  62,    8), s!(  15,   13), s!(   3,    3), s!(  10,    5), s!(   9,   -1), s!(   8,    6), s!( -14,    1), s!( -47,  -22),
            s!(  78,   19), s!(   3,   -0), s!(   3,    5), s!(   3,    3), s!(   1,   -2), s!( -13,    2), s!( -37,   -5), s!( -45,  -14),
            s!(  39,   15), s!(  30,   14), s!(  53,   25), s!(  25,    6), s!(  19,    6), s!( -36,    2), s!( -40,   -9), s!( -25,   -2),
            s!(  39,   16), s!(  24,   12), s!(  14,    5), s!( -12,   18), s!(   7,    5), s!(   0,   -5), s!( -41,  -10), s!( -29,    1),
            s!(  24,   23), s!(  32,    7), s!(   5,    6), s!(   1,    5), s!( -11,   -0), s!( -23,   -1), s!(  -8,   14), s!( -29,  -16),
            s!(  33,   16), s!(  -1,    1), s!(   4,   -7), s!(  -1,   -4), s!(  -2,   -6), s!(  -3,    0), s!( -28,    4), s!(  -5,   -3),
            s!(  13,    8), s!(  -0,   -2), s!(   5,  -26), s!( -13,  -15), s!(  -6,  -19), s!(  -1,  -10), s!( -32,    7), s!( -34,    2),
            s!(  19,    9), s!(  -3,    2), s!( -26,    7), s!( -12,  -41), s!( -18,   -1), s!(  11,   -9), s!(   3,    3), s!(   3,    2),
        ],
        [
            s!(  22,   15), s!(   8,    7), s!(  10,    8), s!(  15,   12), s!(   3,    2), s!(  -8,  -11), s!(   1,    0), s!( -14,  -11),
            s!(   3,    2), s!(   0,    1), s!(   5,    5), s!(  19,   12), s!(  -1,  -20), s!( -23,    1), s!( -27,    2), s!( -60,  -21),
            s!(  23,    8), s!(   8,   17), s!(  20,   12), s!(  20,    5), s!( -23,   12), s!( -41,   -7), s!( -52,  -13), s!( -22,  -32),
            s!(   1,   12), s!(  21,  -12), s!(  13,   16), s!(  23,    7), s!(  15,   -0), s!( -26,   16), s!( -29,   -3), s!( -22,  -15),
            s!(  13,   -3), s!( -10,    7), s!(   5,   -7), s!(  -6,  -10), s!(  13,    2), s!(  -4,   -2), s!( -18,  -17), s!( -20,  -23),
            s!(  -1,    6), s!(   7,  -11), s!(  -0,    1), s!(  -3,   -7), s!(   6,   -2), s!(   3,  -16), s!(  -6,    5), s!(  -9,  -12),
            s!(  17,   13), s!(  10,   -3), s!(   5,    7), s!(  -3,   10), s!(  -2,    7), s!(   6,  -15), s!(  -4,    6), s!( -34,  -15),
            s!(  -3,    7), s!(  16,  -13), s!(   2,    3), s!( -14,    8), s!(  -9,    8), s!(   3,  -10), s!(  11,   -1), s!(   4,    5),
        ],
        [
            s!(   4,    5), s!(   7,    2), s!(  12,   12), s!(  13,   13), s!(  -5,   -3), s!(  -5,   -7), s!( -20,  -10), s!( -22,  -16),
            s!(  -0,   -1), s!(   6,    8), s!(  39,   17), s!(  10,    4), s!(   7,   -4), s!( -23,  -10), s!( -10,   -1), s!( -33,  -30),
            s!(  -6,   -6), s!(  11,   13), s!(  25,   14), s!(   7,   10), s!(  -2,  -13), s!( -31,   -6), s!( -23,  -17), s!( -15,  -13),
            s!(  -7,    1), s!(  23,    4), s!(  27,   19), s!(  -3,   -7), s!(  18,    3), s!( -14,   -2), s!( -14,   -2), s!( -28,  -23),
            s!(  14,   -3), s!(  16,   15), s!(   2,    3), s!(  13,    6), s!(   4,   -5), s!(  -1,    0), s!( -16,  -10), s!( -10,   -5),
            s!( -14,    2), s!(  -9,   14), s!(  -4,    1), s!(   1,   -9), s!(  17,    2), s!( -11,    3), s!(  -8,    2), s!( -67,  -19),
            s!(  24,    5), s!(  -1,   14), s!(   5,  -22), s!(   4,   -2), s!(  -1,   13), s!( -18,  -20), s!(   4,   -1), s!(  -1,  -10),
            s!(  -7,   -1), s!(  -5,   -1), s!(  -6,    2), s!(  13,   -6), s!(  -7,   -3), s!(  -5,    0), s!(   6,    3), s!(  -4,   -2),
        ],
        [
            s!( -17,   -2), s!( -21,  -17), s!(  -6,   -3), s!( -11,   -3), s!(  -3,    4), s!(  -8,   -3), s!(   2,    3), s!(   4,    5),
            s!( -12,   -5), s!( -13,   -8), s!(  -9,   -3), s!(   3,  -12), s!(  13,   -4), s!(  17,   14), s!(  -0,   -5), s!(   3,    5),
            s!(  -9,  -18), s!( -14,  -13), s!( -24,   -6), s!(  -7,  -13), s!(  -7,   13), s!(  26,    9), s!(  50,   10), s!(  -2,    9),
            s!( -11,   -9), s!( -25,   -3), s!(  -7,  -16), s!(  -5,   -2), s!(   4,   -1), s!(  17,    0), s!(  27,   11), s!(   1,   -4),
            s!( -23,  -16), s!( -14,  -11), s!( -12,    4), s!(   8,  -20), s!(   2,   -6), s!(  13,    6), s!( -12,   10), s!(   9,    0),
            s!(  -6,   -8), s!( -14,   -1), s!(   1,   -8), s!(   1,   -2), s!(   2,    4), s!(   2,    1), s!(  -3,    4), s!(   8,   -4),
            s!( -26,   -8), s!( -12,  -10), s!(  -6,    4), s!(  -1,   -1), s!(  -1,   -6), s!(  -5,   17), s!(  12,   -6), s!(  -6,   -6),
            s!(  -6,   -8), s!( -15,   -6), s!(  -7,   -5), s!(  -2,  -11), s!(  -4,   -2), s!(  -3,   -5), s!(  -2,   -7), s!(   5,   -1),
        ],
        [
            s!( -12,    8), s!(   4,   -4), s!(  -5,  -10), s!(  -2,   -7), s!(  -6,    1), s!(   8,    7), s!(   0,    0), s!(  15,    9),
            s!( -22,    5), s!( -14,   12), s!(  -4,    3), s!(  -5,   -4), s!(  -8,    8), s!(   5,   -2), s!(   2,    1), s!(  10,    6),
            s!( -11,    3), s!( -11,   11), s!( -17,   -0), s!(  -9,    1), s!(  16,    6), s!(  10,    2), s!( -14,    5), s!(  14,    1),
            s!(  -6,   -2), s!(  -9,   -4), s!( -12,    5), s!(  -5,    2), s!( -10,    4), s!(  -5,    7), s!(   0,   -0), s!(  -1,   -7),
            s!( -14,    4), s!(  -7,   -1), s!(  -2,    3), s!(  -5,   -2), s!(  -1,   -4), s!(   3,   -4), s!(   6,   -8), s!(   5,   -1),
            s!(  -2,  -11), s!(  -3,   -2), s!(  -5,   -3), s!(  -6,    5), s!(  -2,    5), s!(   1,   -1), s!(   5,   -5), s!(  12,   -8),
            s!(  -7,   -7), s!(  -9,    1), s!(  -1,    6), s!(   7,   -2), s!(  -2,   -2), s!(   7,   -6), s!(  12,  -18), s!(  13,    0),
            s!(  -8,  -10), s!(  -2,   -2), s!(  12,  -17), s!(  -1,   11), s!(   9,    2), s!(   0,   -6), s!(   7,   -5), s!(  -0,    3),
        ],
        [
            s!(  -4,   -2), s!(   3,    1), s!(   5,    0), s!(  -0,   -6), s!(   3,    1), s!(   4,    3), s!(   9,    7), s!(  12,    2),
            s!(  -7,   -2), s!(  11,   -5), s!(   2,   -2), s!(  -2,  -16), s!(  -0,   -5), s!(   2,    1), s!(   2,   -6), s!(  42,    3),
            s!(   6,    3), s!(  -0,   -3), s!(  -0,   -1), s!(   8,    2), s!(  -1,  -10), s!(  16,    8), s!(   9,   -7), s!(  18,    8),
            s!(  -1,    1), s!(   1,    3), s!(  -7,   -8), s!(   8,  -10), s!(  -3,   -3), s!(   2,   -2), s!(  19,  -14), s!(  23,   10),
            s!( -12,   -2), s!(  -2,    0), s!(  -4,   -3), s!(  -3,   -3), s!( -24,  -14), s!(   3,   -2), s!( -16,   -6), s!(  14,    2),
            s!(  -6,   -8), s!(  11,   -6), s!(   1,   -1), s!(  -1,   -7), s!( -15,   -8), s!(   6,   -4), s!(  -9,   -3), s!( -18,   -0),
            s!(  -5,   -4), s!(  -4,   -2), s!( -11,   -4), s!(   0,   -7), s!(  -6,    0), s!(   0,    2), s!(  -6,   -1), s!(   5,   -0),
            s!(   1,    0), s!(   2,    2), s!(   7,    4), s!(   4,    1), s!(  -8,  -10), s!(  -1,   -3), s!(  -5,   -1), s!(   1,    0),
        ],
        [
            s!(   1,   -2), s!(  -2,   -5), s!(  -1,   -1), s!(   0,   -2), s!(   2,    1), s!(   2,    0), s!(  -5,   -6), s!( -12,   -9),
            s!(  15,    1), s!(  24,    9), s!(  11,    2), s!(  -4,  -11), s!(   0,   -0), s!(   2,   -6), s!(   2,    3), s!(  -1,    3),
            s!(   4,   -0), s!(  18,    5), s!(  26,   12), s!(  -2,   -5), s!(   5,    1), s!(  -5,   -5), s!(   9,   -0), s!(   4,   -6),
            s!(   1,   -4), s!(  -5,    1), s!(   9,    4), s!(  -7,   -3), s!(   1,    5), s!(   4,  -11), s!( -11,   -9), s!(   1,   -1),
            s!(  -7,    5), s!(   4,    3), s!(   2,   -4), s!( -16,   -5), s!(   6,    6), s!(   4,   -9), s!(  -2,   -9), s!(  -2,    3),
            s!(  -5,    2), s!(   4,   12), s!( -13,    2), s!( -10,  -13), s!(  -6,   -2), s!( -10,   -1), s!(   4,   -8), s!(   6,   -6),
            s!(  10,    3), s!(   3,    1), s!(  -5,   -6), s!( -10,   -6), s!(  -3,   -2), s!( -10,    2), s!(   2,    0), s!(   4,   -4),
            s!(  12,    7), s!(   6,    7), s!(   0,   -2), s!(  -7,   -4), s!(   4,   -6), s!(  -2,   -3), s!(   2,    1), s!(  10,    6),
        ],
        [
            s!(   2,    1), s!(  15,   12), s!(  -1,   -5), s!(  -1,   -2), s!(  -3,   -7), s!(  -0,   -5), s!(  -4,   -5), s!(   2,   -4),
            s!(   6,    2), s!(  14,    2), s!(  -1,   -5), s!(   2,   -0), s!(   5,   -0), s!(  -2,   -9), s!(   5,    5), s!(  -5,   -4),
            s!(   9,    4), s!(   3,   -1), s!(   2,   -2), s!(  -1,   -4), s!(  -5,  -12), s!(  -5,   -8), s!(   1,   -8), s!( -10,   -9),
            s!(   3,   -2), s!(  11,    6), s!(   5,    2), s!(   2,   -2), s!(  -8,   -7), s!(  -6,   -8), s!( -11,  -12), s!(   9,    3),
            s!(  27,    6), s!(  11,    7), s!(  -2,    0), s!(   3,    2), s!(  -9,   -7), s!(  -1,   -0), s!(  11,   -3), s!(  -7,    1),
            s!(   0,    5), s!(   2,    1), s!(   8,    5), s!(   8,    0), s!(   5,    4), s!( -10,   -8), s!(  -3,   -1), s!(  -1,   -2),
            s!(   8,    4), s!(   6,    1), s!(   3,    2), s!(  -1,    1), s!(   2,    4), s!(   2,   -5), s!(  -2,    2), s!(  -4,    0),
            s!(   6,    6), s!(   3,    1), s!(  -8,   -2), s!(   6,    4), s!(  -8,   -2), s!(   3,    2), s!(   0,    1), s!(   1,    2),
        ],
        [
            s!(  27,    6), s!(   2,   -1), s!(   2,    2), s!(   3,    2), s!(   2,   -3), s!(  -1,   -5), s!(  -5,   -2), s!( -22,  -15),
            s!(  60,   17), s!(   3,   -0), s!(   1,    2), s!(   2,   -2), s!(   1,   -1), s!(  -8,   -5), s!( -23,  -10), s!( -28,  -12),
            s!(  14,    3), s!(  17,   10), s!(  20,   11), s!(   9,    1), s!(   4,   -6), s!( -20,  -10), s!( -22,   -8), s!( -18,   -8),
            s!(  29,    9), s!(  20,    2), s!(  10,    3), s!(  -5,    3), s!(   1,   -8), s!(  -3,   -9), s!( -28,   -5), s!( -24,   -5),
            s!(  30,   12), s!(  23,    8), s!(   7,    1), s!(   3,    2), s!(  -8,   -4), s!( -17,   -5), s!(  -1,    5), s!( -23,   -9),
            s!(  28,   13), s!(   2,    3), s!(   6,   -1), s!(   3,   -1), s!(   0,   -5), s!(  -0,   -1), s!( -21,   -1), s!(  -3,    1),
            s!(  12,    8), s!(   5,    4), s!(   5,  -10), s!( -10,   -4), s!(  -2,   -4), s!(   3,   -0), s!( -20,    1), s!( -15,   -1),
            s!(  10,    3), s!(   3,    5), s!( -21,   -1), s!(  -8,  -16), s!( -10,    5), s!(  10,   -0), s!(   2,    2), s!(   1,   -1),
        ],
        [
            s!(   9,    9), s!(   2,    1), s!(   5,    4), s!(   4,    4), s!(   1,    1), s!(  -4,   -5), s!(  -0,   -3), s!(  -6,   -3),
            s!(   2,    2), s!(  -1,   -1), s!(   3,    3), s!(  13,   11), s!(  -6,  -14), s!( -17,  -10), s!( -15,   -7), s!( -37,  -12),
            s!(  17,    3), s!(   6,    7), s!(  13,   12), s!(   7,    3), s!(  -8,    5), s!( -26,  -13), s!( -36,  -17), s!( -21,  -19),
            s!(   5,    8), s!(  19,    2), s!(  13,    7), s!(  14,    4), s!(   5,   -8), s!( -17,   -2), s!( -23,  -12), s!( -18,   -8),
            s!(  17,    1), s!(  -3,    1), s!(   5,   -4), s!(  -7,  -10), s!(   9,   -1), s!(  -3,   -4), s!( -13,   -6), s!( -17,  -13),
            s!(   2,    3), s!(   8,   -3), s!(   2,    2), s!(   3,   -0), s!(  10,    4), s!(   4,   -1), s!(  -0,    5), s!(  -9,   -5),
            s!(  15,    6), s!(  11,    2), s!(   6,   -0), s!(   0,    6), s!(   5,    7), s!(   7,   -2), s!(  -2,    2), s!( -16,   -6),
            s!(   4,    4), s!(  12,   -2), s!(   4,    4), s!(  -8,    4), s!(  -1,    4), s!(   4,    0), s!(   4,    0), s!(   1,    1),
        ],
        [
            s!(   1,    1), s!(   1,   -3), s!(   5,    4), s!(   6,    5), s!(  -3,   -3), s!(  -0,   -3), s!( -12,   -9), s!( -10,  -10),
            s!(  -0,    0), s!(   3,    6), s!(  17,    8), s!(   4,    2), s!(   1,   -4), s!( -13,   -9), s!(  -4,   -2), s!( -14,  -11),
            s!(  -3,   -1), s!(   5,    1), s!(  10,    8), s!(   6,    8), s!(  -7,  -13), s!( -13,   -4), s!( -14,  -11), s!(  -6,   -3),
            s!(  -3,   -0), s!(  10,    3), s!(  13,    6), s!(  -3,   -8), s!(   8,    1), s!(  -9,   -3), s!(  -6,   -0), s!( -18,  -12),
            s!(   8,    0), s!(  10,    8), s!(   6,    7), s!(   7,    4), s!(   2,   -0), s!(  -2,   -1), s!(  -7,   -3), s!(  -6,   -2),
            s!(   0,    0), s!(  -5,    4), s!(  -1,   -1), s!(   2,   -2), s!(  12,    4), s!(  -8,   -2), s!(  -3,    2), s!( -30,   -9),
            s!(  10,    2), s!(   4,    7), s!(   7,   -5), s!(   8,    3), s!(   6,    7), s!( -12,   -5), s!(   1,   -1), s!(  -2,   -5),
            s!(  -1,    1), s!(   1,    2), s!(  -2,    2), s!(  18,    4), s!(  -1,    0), s!(  -0,    1), s!(   2,    1), s!(  -1,    1),
        ],
    ],
];

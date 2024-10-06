#[derive(Clone)]
pub struct SParams {
    pub aspwin_delta: i16,
    pub aspwin_min_depth: i8,
    pub aspwin_max_width: i16,

    pub iir_min_depth: i8,
    pub iir_reduction_base: i8,
    pub iir_reduction_step: i8,
    pub iir_max_reduction: i8,

    pub razoring_min_depth: i8,
    pub razoring_max_depth: i8,
    pub razoring_depth_margin_base: i16,
    pub razoring_depth_margin_multiplier: i16,

    pub snmp_min_depth: i8,
    pub snmp_max_depth: i8,
    pub snmp_depth_margin_base: i16,
    pub snmp_depth_margin_multiplier: i16,

    pub nmp_min_depth: i8,
    pub nmp_min_game_phase: u8,
    pub nmp_margin: i16,
    pub nmp_depth_base: i8,
    pub nmp_depth_divider: i8,

    pub lmp_min_depth: i8,
    pub lmp_max_depth: i8,
    pub lmp_move_index_margin_base: usize,
    pub lmp_move_index_margin_multiplier: usize,
    pub lmp_max_score: i16,

    pub lmr_min_depth: i8,
    pub lmr_max_score: i16,
    pub lmr_min_move_index: usize,
    pub lmr_reduction_base: usize,
    pub lmr_reduction_step: usize,
    pub lmr_max_reduction: i8,
    pub lmr_pv_min_move_index: usize,
    pub lmr_pv_reduction_base: usize,
    pub lmr_pv_reduction_step: usize,
    pub lmr_pv_max_reduction: i8,

    pub q_score_pruning_treshold: i16,
    pub q_futility_pruning_margin: i16,
}

#[allow(non_upper_case_globals)]
impl SParams {
    pub const aspwin_delta: i16 = 25;
    pub const aspwin_min_depth: i8 = 5;
    pub const aspwin_max_width: i16 = 400;

    pub const iir_min_depth: i8 = 4;
    pub const iir_reduction_base: i8 = 1;
    pub const iir_reduction_step: i8 = 99;
    pub const iir_max_reduction: i8 = 3;

    pub const razoring_min_depth: i8 = 1;
    pub const razoring_max_depth: i8 = 5;
    pub const razoring_depth_margin_base: i16 = 260;
    pub const razoring_depth_margin_multiplier: i16 = 260;

    pub const snmp_min_depth: i8 = 1;
    pub const snmp_max_depth: i8 = 8;
    pub const snmp_depth_margin_base: i16 = 135;
    pub const snmp_depth_margin_multiplier: i16 = 55;

    pub const nmp_min_depth: i8 = 2;
    pub const nmp_min_game_phase: u8 = 3;
    pub const nmp_margin: i16 = 60;
    pub const nmp_depth_base: i8 = 2;
    pub const nmp_depth_divider: i8 = 5;

    pub const lmp_min_depth: i8 = 1;
    pub const lmp_max_depth: i8 = 3;
    pub const lmp_move_index_margin_base: usize = 2;
    pub const lmp_move_index_margin_multiplier: usize = 5;
    pub const lmp_max_score: i16 = -55;

    pub const lmr_min_depth: i8 = 2;
    pub const lmr_max_score: i16 = 90;
    pub const lmr_min_move_index: usize = 2;
    pub const lmr_reduction_base: usize = 1;
    pub const lmr_reduction_step: usize = 4;
    pub const lmr_max_reduction: i8 = 3;
    pub const lmr_pv_min_move_index: usize = 2;
    pub const lmr_pv_reduction_base: usize = 1;
    pub const lmr_pv_reduction_step: usize = 8;
    pub const lmr_pv_max_reduction: i8 = 2;

    pub const q_score_pruning_treshold: i16 = 0;
    pub const q_futility_pruning_margin: i16 = 100;
}

impl Default for SParams {
    /// Constructs a default instance of [SearchParams] with default elements.
    fn default() -> Self {
        Self {
            aspwin_delta: Self::aspwin_delta,
            aspwin_min_depth: Self::aspwin_min_depth,
            aspwin_max_width: Self::aspwin_max_width,

            iir_min_depth: Self::iir_min_depth,
            iir_reduction_base: Self::iir_reduction_base,
            iir_reduction_step: Self::iir_reduction_step,
            iir_max_reduction: Self::iir_max_reduction,

            razoring_min_depth: Self::razoring_min_depth,
            razoring_max_depth: Self::razoring_max_depth,
            razoring_depth_margin_base: Self::razoring_depth_margin_base,
            razoring_depth_margin_multiplier: Self::razoring_depth_margin_multiplier,

            snmp_min_depth: Self::snmp_min_depth,
            snmp_max_depth: Self::snmp_max_depth,
            snmp_depth_margin_base: Self::snmp_depth_margin_base,
            snmp_depth_margin_multiplier: Self::snmp_depth_margin_multiplier,

            nmp_min_depth: Self::nmp_min_depth,
            nmp_min_game_phase: Self::nmp_min_game_phase,
            nmp_margin: Self::nmp_margin,
            nmp_depth_base: Self::nmp_depth_base,
            nmp_depth_divider: Self::nmp_depth_divider,

            lmp_min_depth: Self::lmp_min_depth,
            lmp_max_depth: Self::lmp_max_depth,
            lmp_move_index_margin_base: Self::lmp_move_index_margin_base,
            lmp_move_index_margin_multiplier: Self::lmp_move_index_margin_multiplier,
            lmp_max_score: Self::lmp_max_score,

            lmr_min_depth: Self::lmr_min_depth,
            lmr_max_score: Self::lmr_max_score,
            lmr_min_move_index: Self::lmr_min_move_index,
            lmr_reduction_base: Self::lmr_reduction_base,
            lmr_reduction_step: Self::lmr_reduction_step,
            lmr_max_reduction: Self::lmr_max_reduction,
            lmr_pv_min_move_index: Self::lmr_pv_min_move_index,
            lmr_pv_reduction_base: Self::lmr_pv_reduction_base,
            lmr_pv_reduction_step: Self::lmr_pv_reduction_step,
            lmr_pv_max_reduction: Self::lmr_pv_max_reduction,

            q_score_pruning_treshold: Self::q_score_pruning_treshold,
            q_futility_pruning_margin: Self::q_futility_pruning_margin,
        }
    }
}

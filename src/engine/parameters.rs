#[derive(Clone)]
pub struct SearchParameters {
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

impl Default for SearchParameters {
    fn default() -> Self {
        Self {
            iir_min_depth: 4,
            iir_reduction_base: 1,
            iir_reduction_step: 99,
            iir_max_reduction: 3,

            razoring_min_depth: 1,
            razoring_max_depth: 5,
            razoring_depth_margin_base: 260,
            razoring_depth_margin_multiplier: 260,

            snmp_min_depth: 1,
            snmp_max_depth: 8,
            snmp_depth_margin_base: 135,
            snmp_depth_margin_multiplier: 55,

            nmp_min_depth: 2,
            nmp_min_game_phase: 3,
            nmp_margin: 60,
            nmp_depth_base: 2,
            nmp_depth_divider: 5,

            lmp_min_depth: 1,
            lmp_max_depth: 3,
            lmp_move_index_margin_base: 2,
            lmp_move_index_margin_multiplier: 5,
            lmp_max_score: -55,

            lmr_min_depth: 2,
            lmr_max_score: 90,
            lmr_min_move_index: 2,
            lmr_reduction_base: 1,
            lmr_reduction_step: 4,
            lmr_max_reduction: 3,
            lmr_pv_min_move_index: 2,
            lmr_pv_reduction_base: 1,
            lmr_pv_reduction_step: 8,
            lmr_pv_max_reduction: 2,

            q_score_pruning_treshold: 0,
            q_futility_pruning_margin: 100,
        }
    }
}

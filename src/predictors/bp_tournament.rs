use super::{BranchPredictor, Counter2};

pub struct BPTournament {
    // ghistory_bits: usize,
    // lhistory_bits: usize,
    // pc_index_bits: usize,
    global_table: Vec<Counter2>,
    chooser_table: Vec<Counter2>,
    local_pattern: Vec<usize>,
    local_table: Vec<Counter2>,
    ghr: usize,
    mask_global: usize,
    mask_pc: usize,
    mask_local: usize,
}

impl BPTournament {
    pub fn new(ghistory_bits: usize, lhistory_bits: usize, pc_index_bits: usize) -> Self {
        Self {
            // ghistory_bits,
            // lhistory_bits,
            // pc_index_bits,
            ghr: 0,
            global_table: vec![Counter2::WeakNot; 1 << ghistory_bits],
            chooser_table: vec![Counter2::WeakNot; 1 << ghistory_bits],
            local_pattern: vec![0; 1 << pc_index_bits],
            local_table: vec![Counter2::WeakNot; 1 << lhistory_bits],
            mask_global: (1 << ghistory_bits) - 1,
            mask_pc: (1 << pc_index_bits) - 1,
            mask_local: (1 << lhistory_bits) - 1,
        }
    }
}

impl BranchPredictor for BPTournament {
    fn predict(&mut self, pc: usize) -> bool {
        let g_idx = self.ghr & self.mask_global;
        let g_pred = self.global_table[g_idx].predict();

        let pc_idx = (pc >> 2) & self.mask_pc;
        let l_pattern = self.local_pattern[pc_idx] & self.mask_local;
        let l_pred = self.local_table[l_pattern].predict();

        let c = self.chooser_table[g_idx];

        if c.predict() {
            g_pred
        }
        else {
            l_pred
        }
    }

    fn update(&mut self, _i: usize, pc: usize, outcome: bool, _pred: bool) {
        let g_idx = self.ghr & self.mask_global;
        let g_pred = self.global_table[g_idx].predict();

        let pc_idx = (pc >> 2) & self.mask_pc;
        let l_pattern = self.local_pattern[pc_idx] & self.mask_local;
        let l_pred = self.local_table[l_pattern].predict();

        self.global_table[g_idx] = self.global_table[g_idx].update(outcome);
        self.local_table[l_pattern] = self.local_table[l_pattern].update(outcome);
        self.local_pattern[pc_idx] = ((self.local_pattern[pc_idx] << 1) | (outcome as usize)) & self.mask_local;
        if g_pred == outcome && l_pred != outcome {
            self.chooser_table[g_idx] = self.chooser_table[g_idx].plus();
        }
        else if g_pred != outcome && l_pred == outcome {
            self.chooser_table[g_idx] = self.chooser_table[g_idx].minus();
        }
        self.ghr = ((self.ghr << 1) | (outcome as usize)) & self.mask_global;
    }
}

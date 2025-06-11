// run  := cargo build && bunzip2 -kc ../../traces/mm_1.bz2 | /home/jyh/ucsd/sp25/cse240A/bp/target/debug/bp --custom
// run  := cargo build && /usr/bin/cat ../data1.in | /home/jyh/ucsd/sp25/cse240A/bp/target/debug/bp --custom
// dir  := .
// kid  :=

use super::{BranchPredictor, Counter2, CounterN};

const CHOOSER_BITS: u8 = 2;

#[derive(Copy, Clone)]
struct LoopEntry {
    iter: u16,
    trip: u16,
    dir: bool,
    conf: CounterN<CHOOSER_BITS>
}
impl Default for LoopEntry {
    fn default() -> Self {
        Self {
            iter: 0,
            trip: 0,
            dir: true,
            conf: CounterN::<CHOOSER_BITS>::default()
        }
    }
}

pub struct BPCustom {
    global_table: Vec<Counter2>,
    chooser_table: Vec<CounterN<3>>,
    loop_table: Vec<LoopEntry>,
    local_pattern: Vec<usize>,
    local_table: Vec<Counter2>,
    ghr: usize,
    loop_history_bits: usize,
    mask_global: usize,
    mask_pc: usize,
    mask_local: usize,
    mask_loop: usize,
    verbose: bool,
}

impl BPCustom {
    pub fn new(verbose: bool) -> Self {
        let ghistory_bits = 12;
        let lhistory_bits = 12;
        let pc_index_bits = 11;
        let n_loop = 9;
        let loop_history_bits = 10;

        let total_bits = 
            2 * (1 << ghistory_bits) + // global_table
            3 * (1 << ghistory_bits) + // chooser_table
            (loop_history_bits + loop_history_bits + CHOOSER_BITS as usize + 1) * (1 << n_loop) + // loop_table
            lhistory_bits * (1 << pc_index_bits) + // local_pattern
            2 * (1 << lhistory_bits); // local_table
        println!("BPCustom: total bits = {}", total_bits);
        assert!(total_bits <= (1 << 16) + 256);

        Self {
            verbose,
            ghr: 0,
            loop_history_bits,
            global_table: vec![Counter2::WeakNot; 1 << ghistory_bits],
            chooser_table: vec![CounterN::<3>::default(); 1 << ghistory_bits],
            loop_table: vec![LoopEntry::default(); 1 << n_loop],
            local_pattern: vec![0; 1 << pc_index_bits],
            local_table: vec![Counter2::WeakNot; 1 << lhistory_bits],
            mask_global: (1 << ghistory_bits) - 1,
            mask_pc: (1 << pc_index_bits) - 1,
            mask_local: (1 << lhistory_bits) - 1,
            mask_loop: (1 << n_loop) - 1,
        }
    }

    fn lp_update(&mut self, pc: usize, taken: bool) {
        let idx = (pc >> 2) & self.mask_loop;
        let e = &mut self.loop_table[idx];

        if e.trip == 0 && e.iter == 0 && e.conf.get() == 0 {
            e.dir = taken;
            e.iter = 1;
            return;
        }

        if taken == e.dir {
            e.iter = (e.iter + 1).min((1 << self.loop_history_bits) - 1);
        }
        else {
            if e.iter != 0 {
                if e.trip == 0 {
                    e.trip = e.iter;
                    e.conf = CounterN::<CHOOSER_BITS>::default();
                }
                else if e.iter == e.trip {
                    e.conf.plus();
                }
                else {
                    e.trip = e.iter;
                    e.conf.minus();
                }
            }
            e.iter = 0;
        }
    }

    #[inline]
    fn lp_predict(&self, pc: usize) -> Option<bool> {
        let e = &self.loop_table[(pc >> 2) & self.mask_loop];
        if e.conf.get() >= e.conf.max() && (3..(1 << self.loop_history_bits)-1).contains(&e.trip) {
            if e.dir && e.iter == e.trip {
                return Some(false);
            } // T...T -> N
            if !e.dir && e.iter == e.trip {
                return Some(true);
            } // N...N -> T
        }
        None
    }
}

impl BranchPredictor for BPCustom {
    fn predict(&mut self, pc: usize) -> bool {
        if let Some(lp_pred) = self.lp_predict(pc) {
            return lp_pred;
        }
        let g_idx = self.ghr & self.mask_global;
        let g_pred = self.global_table[g_idx].predict();

        let pc_idx = (pc >> 2) & self.mask_pc;
        let l_pattern = self.local_pattern[pc_idx] & self.mask_local;
        let l_pred = self.local_table[l_pattern].predict();

        let c = self.chooser_table[g_idx];

        if c.predict() { g_pred } else { l_pred }
    }

    fn update(&mut self, i: usize, pc: usize, outcome: bool, pred: bool) {
        self.lp_update(pc, outcome);
        let g_idx = self.ghr & self.mask_global;
        let g_pred = self.global_table[g_idx];

        let pc_idx = (pc >> 2) & self.mask_pc;
        let l_pattern = self.local_pattern[pc_idx] & self.mask_local;
        let l_pred = self.local_table[l_pattern];

        if self.verbose {
            let c = self.chooser_table[g_idx];
            if pred != outcome {
                print!("Misprediction: ");
                println!("BPCustom: access {i} - PC {:#x}: predicted {}, actual {}, chooser {}, local {}, global {}, g_idx {}", pc, pred, outcome, c.as_u8(), l_pred.as_u8(), g_pred.as_u8(), g_idx);
            }
        }

        let g_pred = g_pred.predict();
        let l_pred = l_pred.predict();

        self.global_table[g_idx] = self.global_table[g_idx].update(outcome);
        self.local_table[l_pattern] = self.local_table[l_pattern].update(outcome);
        self.local_pattern[pc_idx] = ((self.local_pattern[pc_idx] << 1) | (outcome as usize)) & self.mask_local;
        if g_pred == outcome && l_pred != outcome {
            self.chooser_table[g_idx].plus();
        } else if g_pred != outcome && l_pred == outcome {
            self.chooser_table[g_idx].minus();
        }
        self.ghr = ((self.ghr << 1) | (outcome as usize)) & self.mask_global;
    }
}

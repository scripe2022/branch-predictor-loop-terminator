use super::{BranchPredictor, Counter2};

pub struct BPGShare {
    ghistory_bits: usize,
    ghr: usize,
    table: Vec<Counter2>,
    verbose: bool
}

impl BPGShare {
    pub fn new(ghistory_bits: usize, verbose: bool) -> Self {
        Self {
            ghistory_bits,
            ghr: 0,
            table: vec![Counter2::WeakNot; 1 << ghistory_bits],
            verbose
        }
    }
}

impl BranchPredictor for BPGShare {
    fn predict(&mut self, pc: usize) -> bool {
        let mask = (1 << self.ghistory_bits) - 1;
        let index = (pc ^ self.ghr) & mask;
        self.table[index].predict()
    }

    fn update(&mut self, i: usize, pc: usize, outcome: bool, _pred: bool) {
        let mask = (1 << self.ghistory_bits) - 1;
        let index = (pc ^ self.ghr) & mask;
        let pred = self.table[index];

        // if self.verbose && i >= 1458820 && i <= 1464498 {
            if self.verbose && pred.predict() != outcome {
                println!("BPGShare: access {i} - Misprediction at PC {:#x}: index {:#x}, predicted {}, actual {}", pc, index, pred.as_u8(), outcome);
                // println!("{:#x},{:#x},{},{}", pc, index, pred.as_u8(), outcome);
            }
            else {
                println!("BPGShare: access {i} - PC {:#x}: index {:#x}, predicted {}, actual {}", pc, index, pred.as_u8(), outcome);
            }
        // }

        self.table[index] = self.table[index].update(outcome);
        self.ghr = ((self.ghr << 1) | (outcome as usize)) & mask;
    }
}

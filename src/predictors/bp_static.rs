use super::BranchPredictor;

pub struct BPStatic {
}

impl BPStatic {
    pub fn new() -> Self {
        BPStatic {}
    }
}

impl BranchPredictor for BPStatic {
    fn predict(&mut self, _pc: usize) -> bool {
        // Always predict not taken
        true
    }

    fn update(&mut self, _i: usize,_pc: usize, _outcome: bool, _pred: bool) {
        // Static predictor does not update
    }
}

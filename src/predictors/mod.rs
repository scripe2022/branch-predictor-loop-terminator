pub mod bp_gshare;
pub use bp_gshare::BPGShare;

pub mod bp_static;
pub use bp_static::BPStatic;

pub mod bp_tournament;
pub use bp_tournament::BPTournament;

pub mod bp_custom;
pub use bp_custom::BPCustom;

pub trait BranchPredictor {
    fn predict(&mut self, pc: usize) -> bool;
    fn update(&mut self, _i: usize, _pc: usize, _actual: bool, _pred: bool) {}
}

#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Counter2 {
    StrongNot = 0,
    WeakNot   = 1,
    WeakTake  = 2,
    StrongTake= 3,
}

impl Counter2 {
    #[inline]
    pub fn plus(self) -> Self {
        use Counter2::*;
        match self {
            StrongNot => WeakNot,
            WeakNot   => WeakTake,
            WeakTake  => StrongTake,
            StrongTake=> StrongTake,
        }
    }

    #[inline]
    pub fn minus(self) -> Self {
        use Counter2::*;
        match self {
            StrongNot => StrongNot,
            WeakNot   => StrongNot,
            WeakTake  => WeakNot,
            StrongTake=> WeakTake,
        }
    }

    #[inline] pub fn predict(self) -> bool {
        matches!(self, Counter2::WeakTake | Counter2::StrongTake)
    }

    #[inline]
    pub fn update(self, taken: bool) -> Self {
        if taken { self.plus() } else { self.minus() }
    }

    #[inline] pub fn as_u8(self) -> u8 { self as u8 }

    // #[inline]
    // pub fn from_u8(v: u8) -> Self {
    //     use Counter2::*;
    //     match v & 0b11 {
    //         0 => StrongNot,
    //         1 => WeakNot,
    //         2 => WeakTake,
    //         _ => StrongTake,
    //     }
    // }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct CounterN<const N: u8> {
    val: u8,
}

impl<const N: u8> Default for CounterN<N> {
    fn default() -> Self { Self::new() }
}

impl<const N: u8> CounterN<N> {
    const MAX: u8 = (1 << N) - 1;

    #[inline]
    pub const fn new() -> Self {
        Self { val: (1 << (N - 1)) - 1 }
    }

    #[inline]
    pub fn plus(&mut self) {
        if self.val < Self::MAX { self.val += 1; }
    }

    #[inline]
    pub fn minus(&mut self) {
        if self.val > 0 { self.val -= 1; }
    }

    #[inline]
    pub fn _update(&mut self, taken: bool) {
        if taken { self.plus() } else { self.minus() }
    }

    #[inline]
    pub fn predict(&self) -> bool {
        self.val >= (1 << (N - 1))
    }

    #[inline] pub fn get(self) -> u8 { self.val }

    #[inline]
    pub fn _set(&mut self, v: u8) {
        self.val = v & Self::MAX;
    }

    #[inline]
    pub fn as_u8(self) -> u8 { self.val }

    #[inline]
    pub fn max(&self) -> u8 { Self::MAX }
}

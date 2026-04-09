use crate::DualState;

pub type LatticeSlices = Vec<Vec<DualState>>;

#[derive(Debug, Default)]
pub struct LatticeHistory {
    pub sample_period: usize,
    pub lattice_slices: LatticeSlices,
}

impl LatticeHistory {
    pub fn set_sample_period(&mut self, sample_period: usize) {
        self.sample_period = sample_period;
    }

    pub fn record<F: FnOnce() -> Vec<DualState>>(&mut self, lattice_fn: F, iteration: usize) {
        if self.sample_period > 0 && iteration.is_multiple_of(self.sample_period) {
            self.lattice_slices.push(lattice_fn());
        }
    }

    pub fn len(&self) -> usize {
        self.lattice_slices.len()
    }

    pub fn take(self) -> LatticeSlices {
        self.lattice_slices
    }
}

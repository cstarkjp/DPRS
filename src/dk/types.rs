// #![warn(missing_docs)]
// //!
// //!

use crate::{dk::traits::HasMean, sim_parameters::DualState};

pub type LatticeSlices = Vec<Vec<DualState>>;
pub type Tracking = Vec<Vec<f64>>;

#[derive(Debug, Default)]
pub struct LatticeHistory {
    pub sample_period: usize,
    pub lattice_slices: LatticeSlices,
}

impl LatticeHistory {
    pub fn set_sample_period(&mut self, sample_period: usize) {
        self.sample_period = sample_period;
    }

    pub fn record(&mut self, lattice: &Vec<DualState>, i: usize) {
        if self.sample_period > 0 && i.is_multiple_of(self.sample_period) {
            self.lattice_slices.push(lattice.clone());
        }
    }

    pub fn len(&self) -> usize {
        self.lattice_slices.len()
    }

    pub fn take(self) -> LatticeSlices {
        self.lattice_slices
    }
}

#[derive(Debug)]
pub struct TrackingHistory {
    pub tracking: Tracking,
}

impl Default for TrackingHistory {
    fn default() -> Self {
        let mut tracking = Vec::new();
        let t_tracking = Vec::new();
        let rho_mean_tracking = Vec::new();
        let radius_mean_tracking = Vec::new();
        let radius_stddev_tracking = Vec::new();
        tracking.push(t_tracking);
        tracking.push(rho_mean_tracking);
        tracking.push(radius_mean_tracking);
        tracking.push(radius_stddev_tracking);
        Self { tracking }
    }
}

impl TrackingHistory {
    pub fn update<T: HasMean>(&mut self, i: usize, lattice_model: &T) {
        let t = i as f64;
        self.tracking[0].push(t);
        let rho_mean = lattice_model.mean();
        self.tracking[1].push(rho_mean);
    }

    pub fn take(self) -> Tracking {
        self.tracking
    }
}

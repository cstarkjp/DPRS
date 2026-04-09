use super::{CellDim, DramaticallySimulatable};

/// Statistics gathered for each iteration of a simulation (if the lattice model
/// provides them)
#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct Statistics {
    /// Iteration number for the statistic
    pub iteration: u32,

    /// The total mass, i.e. number of active cells, in the lattice
    pub mass: f32,

    /// The mean mass of the lattice (mass divided by lattice size)
    pub mean_rho: f32,

    /// The mean radius mumble
    pub mean_radius: f32,
}

/// The tracking history for a simulation of a lattice model
#[derive(Debug, Default, Clone)]
pub struct TrackingHistory {
    pub tracking: Vec<Statistics>,
}

impl std::ops::Deref for TrackingHistory {
    type Target = [Statistics];
    fn deref(&self) -> &Self::Target {
        &self.tracking
    }
}

impl TrackingHistory {
    pub fn update<D: CellDim, T: DramaticallySimulatable<D>>(
        &mut self,
        iteration: usize,
        lattice_model: &T,
    ) {
        let mut statistics = Statistics::default();
        let lm_s = lattice_model.statistics();
        statistics.mass = lm_s.0 as f32;
        statistics.mean_rho = lm_s.1 as f32;
        statistics.mean_radius = lm_s.2 as f32;
        statistics.iteration = iteration as u32;
        self.tracking.push(statistics);
    }

    pub fn take(self) -> Vec<Statistics> {
        self.tracking
    }
}

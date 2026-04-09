use super::{CellDim, DramaticallySimulatable};

pub type Tracking = Vec<Vec<f64>>;

#[derive(Debug)]
pub struct TrackingHistory {
    pub tracking: Tracking,
}

impl Default for TrackingHistory {
    fn default() -> Self {
        let mut tracking = Vec::new();
        // TODO: I don't think this is the right way to do this.
        // Rust can't know the number of tracked statistic vectors
        // at compile time, so update() can incur a runtime error
        // if the tracking[i] index i is out of bounds.
        let t_tracking = Vec::new();
        let mass_tracking = Vec::new();
        let mean_rho_tracking = Vec::new();
        let mean_radius_tracking = Vec::new();
        tracking.push(t_tracking);
        tracking.push(mass_tracking);
        tracking.push(mean_rho_tracking);
        tracking.push(mean_radius_tracking);
        Self { tracking }
    }
}

impl TrackingHistory {
    pub fn update<D: CellDim, T: DramaticallySimulatable<D>>(
        &mut self,
        iteration: usize,
        lattice_model: &T,
    ) {
        let t = iteration as f64;
        let statistics = lattice_model.statistics();
        self.tracking[0].push(t);
        self.tracking[1].push(statistics.0);
        self.tracking[2].push(statistics.1);
        self.tracking[3].push(statistics.2);
    }

    pub fn take(self) -> Tracking {
        self.tracking
    }
}

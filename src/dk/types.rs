// #![warn(missing_docs)]
// //!
// //!

use crate::sim_parameters::DualState;

pub type LatticeHistory = Vec<Vec<DualState>>;
pub type TrackingHistory = Vec<Vec<f64>>;

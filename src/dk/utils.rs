// #![warn(missing_docs)]
// //!
// //!

use crate::dk::traits::HasMean;

pub fn update_statistics<T: HasMean>(i: usize, lm: &T, tracking: &mut Vec<Vec<f64>>) {
    let t = i as f64;
    tracking[0].push(t);
    let rho_mean = lm.mean();
    tracking[1].push(rho_mean);
}

pub fn do_slice(i: usize, sample_period: usize) -> bool {
    sample_period > 0 && i.is_multiple_of(sample_period)
}

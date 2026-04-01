// #![warn(missing_docs)]
// //!
// //!

pub trait HasMean {
    fn mean(&self) -> f64;
}

// pub trait HasLattice: Sync {
//     fn lattice<T>(&self) -> &Vec<T>;
// }
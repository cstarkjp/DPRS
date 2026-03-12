// #![warn(missing_docs)]
// //!
// //!
use pyo3::{FromPyObject, PyAny, PyErr};

/// Lattice dimension, auto-computed from presence of n_y, n_z kwarg parameters.
#[derive(PartialEq, Debug, Clone)]
#[repr(u8)]
pub enum Dimension {
    D1,
    D2,
    D3,
}

impl Dimension {
    // This can be created from num-derive
    fn from_u8(value: u8) -> Option<Self> {
        if value == (Dimension::D1 as u8) {
            Some(Dimension::D1)
        } else if value == (Dimension::D2 as u8) {
            Some(Dimension::D2)
        } else if value == (Dimension::D3 as u8) {
            Some(Dimension::D3)
        } else {
            None
        }
    }
}
impl FromPyObject<'_, '_> for Dimension {
    type Error = PyErr;
    fn extract(ob: pyo3::Borrowed<'_, '_, PyAny>) -> Result<Self, PyErr> {
        let value: u8 = ob.extract().unwrap();
        let opcode = Dimension::from_u8(value).unwrap();
        Ok(opcode)
    }
}

/// Choice of processing type: will become a Py-passable parameter
#[derive(PartialEq, Debug, Clone)]
#[repr(u8)]
pub enum Processing {
    Serial,
    Parallel,
    ParallelChunked,
}

impl Processing {
    // This can be created from num-derive
    fn from_u8(value: u8) -> Option<Self> {
        if value == (Processing::Serial as u8) {
            Some(Processing::Serial)
        } else if value == (Processing::Parallel as u8) {
            Some(Processing::Parallel)
        } else if value == (Processing::ParallelChunked as u8) {
            Some(Processing::ParallelChunked)
        } else {
            None
        }
    }
}
impl FromPyObject<'_, '_> for Processing {
    type Error = PyErr;
    fn extract(ob: pyo3::Borrowed<'_, '_, PyAny>) -> Result<Self, PyErr> {
        let value: u8 = ob.extract().unwrap();
        let opcode = Processing::from_u8(value).unwrap();
        Ok(opcode)
    }
}

/// Model parameter bundle derived from Python kwarg dict.
#[derive(FromPyObject, Debug, Clone)]
pub struct Parameters {
    pub dim: Dimension,
    pub n_x: usize,
    pub n_y: usize,
    pub n_z: usize,
    pub p: f64,
    pub n_iterations: usize,
    pub sample_rate: usize,
    pub processing: Processing,
    pub n_threads: usize,
    pub serial_skip: usize,
    pub do_buffering: bool,
}

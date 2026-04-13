use directed_percolation as dprs;
use directed_percolation::{BoundaryCondition, Topology};

use wasm_bindgen::prelude::wasm_bindgen;

use crate::TopoBc;

#[wasm_bindgen]
#[derive(Default, Clone)]
pub struct Parameters(dprs::Parameters);

#[wasm_bindgen]
impl Parameters {
    /// Create a new [Parameters]
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        let mut s = Self::default();
        s.0.do_edge_buffering = true;
        s.0.n_threads = 1;
        s.0.processing = directed_percolation::Processing::Serial;
        s
    }

    pub fn sim_dimension(&self) -> usize {
        if self.0.n_y < 2 { 1 } else { 2 }
    }

    pub(crate) fn sim_parameters(&self) -> &dprs::Parameters {
        &self.0
    }

    fn get_topo_bc(
        &self,
        value: &TopoBc,
    ) -> (Topology, BoundaryCondition, BoundaryCondition, bool, bool) {
        let topology = if value.periodic {
            Topology::Periodic
        } else {
            Topology::Unspecified
        };
        let bc_0 = if value.fix_min {
            BoundaryCondition::Pinned
        } else {
            BoundaryCondition::Floating
        };
        let bc_1 = if value.fix_max {
            BoundaryCondition::Pinned
        } else {
            BoundaryCondition::Floating
        };
        (topology, bc_0, bc_1, value.fix_value, value.fix_value)
    }

    #[wasm_bindgen(getter)]
    pub fn topo_bc_x(&mut self) -> TopoBc {
        TopoBc {
            periodic: self.0.topology_x.is_periodic(),
            ..Default::default()
        }
    }

    #[wasm_bindgen(setter)]
    pub fn set_topo_bc_x(&mut self, value: &TopoBc) {
        let (topology, bc0, bc1, bc_v_0, bc_v_1) = self.get_topo_bc(value);
        self.0.topology_x = topology;
        self.0.bcs_x = (bc0, bc1);
        self.0.bc_values_x = (bc_v_0.into(), bc_v_1.into());
    }

    #[wasm_bindgen(setter)]
    pub fn set_topo_bc_y(&mut self, value: &TopoBc) {
        let (topology, bc0, bc1, bc_v_0, bc_v_1) = self.get_topo_bc(value);
        self.0.topology_y = topology;
        self.0.bcs_y = (bc0, bc1);
        self.0.bc_values_y = (bc_v_0.into(), bc_v_1.into());
    }

    #[wasm_bindgen(setter)]
    pub fn set_topo_bc_z(&mut self, value: &TopoBc) {
        let (topology, bc0, bc1, bc_v_0, bc_v_1) = self.get_topo_bc(value);
        self.0.topology_z = topology;
        self.0.bcs_z = (bc0, bc1);
        self.0.bc_values_z = (bc_v_0.into(), bc_v_1.into());
    }
}

field_getter_setter! {Parameters, u32, n_x, {|a| a as u32}, set_n_x, {|a| a as usize}}
field_getter_setter! {Parameters, u32, n_y, {|a| a as u32}, set_n_y, {|a| a as usize}}
field_getter_setter! {Parameters, u32, n_z, {|a| a as u32}, set_n_z, {|a| a as usize}}

field_getter_setter! {Parameters, f64, p_initial, {|a| a}, set_p_initial, {|a|a}}
field_getter_setter! {Parameters, f64, p_1, {|a| a}, set_p_1, {|a| a}}
field_getter_setter! {Parameters, f64, p_2, {|a| a}, set_p_2, {|a| a}}

field_getter_setter! {Parameters, u32, n_iterations, {|a| a as u32}, set_n_iterations, {|a| a as usize}}
field_getter_setter! {Parameters, u32, sample_period, {|a| a as u32}, set_sample_period, {|a| a as usize}}
field_getter_setter! {Parameters, u32, random_seed, {|a| a as u32}, set_random_seed, {|a| a as usize}}
field_getter_setter! {Parameters, bool, initial_condition, {|a| matches![
    a,
    directed_percolation::InitialCondition::CentralSeed
]}, set_initial_condition, {|a| if a {directed_percolation::InitialCondition::CentralSeed} else {directed_percolation::InitialCondition::Randomized}}}
// field_getter_setter! {Parameters, crate::SimulationKind, simulation_kind, {|a| (&a).into()}, set_simulation_kind, {|a| a.into() }}

#!/usr/bin/env python3

# CPS note: this doesn't work for me
# Note this imports 'sim', as the library now has a 'pymodule' called sim, which means that it has a PyInit_sim() function
#
# maturin builds a library with sublibrary of 'sim', one of 'viz', etc; so presumably 'sim' is magically added to the 'python/dprs' directory

try:
    from dprs import sim
    from dprs.utils import DUAL
except Exception:
    import pathlib
    import sys

    this_dir = pathlib.Path(__file__).parent.parent.resolve()
    sys.path.append(this_dir.joinpath("target", "release").__str__())
    import sim #type: ignore


class Parameters:
    growth_model = sim.GrowthModel.DomanyKinzel
    dim = sim.Dimension.D2
    n_x: int = 1_000
    n_y: int = 1_000
    n_z: int = 1
    p_1: float = 0.163145
    p_initial: float = 0.99
    random_seed: int = 1
    n_iterations: int = 1000
    sample_period: int  = 1000
    axis_topology_x = sim.Topology.Periodic
    axis_topology_y = sim.Topology.Periodic
    axis_topology_z = sim.Topology.Unspecified
    axis_bcs_x = (sim.BoundaryCondition.Floating, sim.BoundaryCondition.Floating)
    axis_bcs_y = (sim.BoundaryCondition.Floating, sim.BoundaryCondition.Floating)
    axis_bcs_z = (sim.BoundaryCondition.Unspecified, sim.BoundaryCondition.Unspecified)
    axis_bc_values_x = (DUAL.OCCUPIED.state, DUAL.OCCUPIED.state)
    axis_bc_values_y = (DUAL.OCCUPIED.state, DUAL.OCCUPIED.state)
    axis_bc_values_z = (DUAL.EMPTY.state, DUAL.EMPTY.state)
    do_edge_buffering: bool = True
    processing = sim.Processing.Parallel
    n_threads: int = 8


parameters = Parameters()

print(f"\n{sim}\n")
# help(sim)
(n_lattices, raw_lattices, raw_tracking, t_run_time) = sim.dk(parameters)
print(f"Total number of lattice time slices = {n_lattices}\n")

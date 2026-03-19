#!/usr/bin/env python3

# Note this imports 'sim', as the library now has a 'pymodule' called sim, which means that it has a PyInit_sim() function
#
# maturin builds a library with sublibrary of 'sim', one of 'viz', etc; so presumably 'sim' is magically added to the 'python/dprs' directory

try:
    import sim
except Exception:
    import pathlib
    import sys

    this_dir = pathlib.Path(__file__).parent.parent.resolve()
    sys.path.append(this_dir.joinpath("target", "release").__str__())
    import sim


class Parameters:
    p: float = 0.163156
    seed: int = 1
    n_iterations: int = 10_000
    dim = sim.Dimension.D2
    n_x: int = 8_000
    n_y: int = 8_000
    n_z: int = 1
    edge_topology_x = sim.Topology.Periodic
    edge_topology_y = sim.Topology.Periodic
    edge_topology_z = sim.Topology.Unspecified
    edge_bc_x = (sim.BoundaryCondition.Floating, sim.BoundaryCondition.Floating)
    edge_bc_y = (sim.BoundaryCondition.Floating, sim.BoundaryCondition.Floating)
    edge_bc_z = (sim.BoundaryCondition.Unspecified, sim.BoundaryCondition.Unspecified)
    edge_values_x = (True, True)
    edge_values_y = (True, True)
    edge_values_z = (False, False)
    do_edge_buffering: bool = True
    processing = sim.Processing.Parallel
    sample_rate: int = 10_000
    n_threads: int = 8


parameters = Parameters()

print(f"\n{sim}\n")
# help(sim)
(n_lattices, raw_lattices, raw_tracking, t_run_time) = sim.dp(parameters)
print(f"Total number of lattice time slices = {n_lattices}\n")

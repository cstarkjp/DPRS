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
    dim: sim.Dimension = sim.Dimension.D2
    processing: sim.Processing = sim.Processing.ParallelChunked
    n_x: int = 1_000
    n_y: int = 1_000
    n_z: int = 1
    p: float = 0.5
    n_iterations: int = 1
    sample_rate: int = 10
    n_threads: int = 1
    serial_skip: int = 1
    do_buffering: bool = True
    edge_topology_x = sim.Topology.Periodic
    edge_topology_y = sim.Topology.Open
    edge_topology_z = sim.Topology.Unspecified
    edge_bc_x = (sim.BoundaryCondition.Floating, sim.BoundaryCondition.Floating)
    edge_bc_y = (sim.BoundaryCondition.Pinned, sim.BoundaryCondition.Pinned)
    edge_bc_z = (sim.BoundaryCondition.Unspecified, sim.BoundaryCondition.Unspecified)
    edge_values_x = (False, False)
    edge_values_y = (True, True)
    edge_values_z = (False, False)
    seed: int = 1


print(f"\n{sim}\n")
# help(sim)
_ = sim.life(Parameters())

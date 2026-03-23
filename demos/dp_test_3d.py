#!/usr/bin/env python3

import pathlib
import sys

try:
    this_dir = pathlib.Path(__file__).parent.parent.resolve()
    sys.path.append(this_dir.joinpath("target", "release").__str__())
    sys.path.append(this_dir.joinpath("python").__str__())
    print(sys.path)
    import sim #type: ignore
except Exception:
    from dprs import sim
    from dprs.utils import DUAL

print(f"\n{sim}")


class Parameters:
    growth_model = sim.GrowthModel.DomanyKinzel
    dim = sim.Dimension.D3
    n_x: int = 100
    n_y: int = 100
    n_z: int = 100
    p_0: float = 0.05
    n_iterations: int = 1000
    sample_period: int  = 0
    initial_condition = sim.InitialCondition.Randomized
    p_initial: float = 0.99
    random_seed: int = 1
    axis_topology_x = sim.Topology.Periodic
    axis_topology_y = sim.Topology.Periodic
    axis_topology_z = sim.Topology.Periodic
    axis_bcs_x = (sim.BoundaryCondition.Floating, sim.BoundaryCondition.Floating)
    axis_bcs_y = (sim.BoundaryCondition.Floating, sim.BoundaryCondition.Floating)
    axis_bcs_z = (sim.BoundaryCondition.Floating, sim.BoundaryCondition.Floating)
    axis_bc_values_x = (DUAL.OCCUPIED.state, DUAL.OCCUPIED.state)
    axis_bc_values_y = (DUAL.OCCUPIED.state, DUAL.OCCUPIED.state)
    axis_bc_values_z = (DUAL.OCCUPIED.state, DUAL.OCCUPIED.state)
    do_edge_buffering: bool = True
    processing = sim.Processing.Parallel
    n_threads: int = 16


parameters = Parameters()

_ = sim.dk(parameters)

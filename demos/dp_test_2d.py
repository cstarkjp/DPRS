#!/usr/bin/env python3

from dprs import sim
from dprs.utils import DUAL

print(f"\n{sim}")

class Parameters:
    growth_model = sim.GrowthModel.SimplifiedDomanyKinzel
    dim = sim.Dimension.D2
    n_x: int = 1_000
    n_y: int = 1_000
    n_z: int = 1
    p: float = 0.163145
    p0: float = 0.99
    seed: int = 1
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
    n_threads: int = 16
parameters = Parameters()

_ = sim.dk(parameters)
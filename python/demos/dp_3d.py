#!/usr/bin/env python3

from dprs import sim
from dprs.utils import DUAL

print(f"\n{sim}")

class Parameters:
    growth_model_choice = sim.GrowthModelChoice.SimplifiedDomanyKinzel
    dim = sim.Dimension.D3
    n_x: int = 100
    n_y: int = 100
    n_z: int = 100
    p_1: float = 0.05
    p_2: float = 0
    n_iterations: int = 1000
    sample_period: int  = 0
    initial_condition = sim.InitialCondition.Randomized
    p_initial: float = 0.99
    random_seed: int = 1
    topology_x = sim.Topology.Periodic
    topology_y = sim.Topology.Periodic
    topology_z = sim.Topology.Periodic
    bcs_x = (sim.BoundaryCondition.Floating, sim.BoundaryCondition.Floating)
    bcs_y = (sim.BoundaryCondition.Floating, sim.BoundaryCondition.Floating)
    bcs_z = (sim.BoundaryCondition.Floating, sim.BoundaryCondition.Floating)
    bc_values_x = (DUAL.OCCUPIED.state, DUAL.OCCUPIED.state)
    bc_values_y = (DUAL.OCCUPIED.state, DUAL.OCCUPIED.state)
    bc_values_z = (DUAL.OCCUPIED.state, DUAL.OCCUPIED.state)
    do_edge_buffering: bool = True
    processing = sim.Processing.Parallel
    n_threads: int = 16


parameters = Parameters()

_ = sim.dk(parameters)

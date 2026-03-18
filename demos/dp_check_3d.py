from dprs import sim

print(f"\n{sim}")

class Parameters:
    dim = sim.Dimension.D3
    n_x: int = 300
    n_y: int = 300
    n_z: int = 300
    p: float = 0.05
    p0: float = 0.99
    seed: int = 1
    n_iterations: int = 1_000
    sample_rate: int  = 1_000
    axis_topology_x = sim.Topology.Periodic
    axis_topology_y = sim.Topology.Periodic
    axis_topology_z = sim.Topology.Periodic
    axis_bcs_x = (sim.BoundaryCondition.Floating, sim.BoundaryCondition.Floating)
    axis_bcs_y = (sim.BoundaryCondition.Floating, sim.BoundaryCondition.Floating)
    axis_bcs_z = (sim.BoundaryCondition.Floating, sim.BoundaryCondition.Floating)
    axis_bc_values_x = (True, True)
    axis_bc_values_y = (True, True)
    axis_bc_values_z = (True, True)
    do_edge_buffering: bool = True
    processing = sim.Processing.Parallel
    n_threads: int = 16
parameters = Parameters()

_ = sim.dp(parameters)
from dprs import sim

print(f"\n{sim}\n")

class Parameters:
    dim = sim.Dimension.D2
    processing = sim.Processing.Parallel
    n_x: int = 81 #10_000
    n_y: int = 51 #10_000
    n_z: int = 1
    edge_topology_x = sim.Topology.Periodic
    edge_topology_y = sim.Topology.Open
    edge_topology_z = sim.Topology.Unspecified
    edge_bc_x = (sim.BoundaryCondition.Floating, sim.BoundaryCondition.Floating)
    edge_bc_y = (sim.BoundaryCondition.Pinned, sim.BoundaryCondition.Pinned)
    edge_bc_z = (sim.BoundaryCondition.Unspecified, sim.BoundaryCondition.Unspecified)
    edge_values_x = (False, False)
    edge_values_y = (True, True)
    edge_values_z = (False, False)
    p: float = 0.5
    n_iterations: int = 100
    sample_rate: int = 10
    n_threads: int = 8
    serial_skip: int = 10
    do_buffering: bool = True
parameters = Parameters()

_ = sim.dp(parameters)

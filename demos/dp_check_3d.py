import pathlib
import sys

this_dir = pathlib.Path(__file__).parent.parent.resolve()
sys.path.append(this_dir.joinpath("target", "release").__str__())
sys.path.append(this_dir.joinpath("python").__str__())
print(sys.path)
import sim

# from dprs import sim
from dprs.utils import DP

print(f"\n{sim}")


class Parameters:
    dim = sim.Dimension.D3
    n_x: int = 100
    n_y: int = 100
    n_z: int = 100
    p: float = 0.05
    p0: float = 0.99
    seed: int = 1
    n_iterations: int = 1_00
    sample_rate: int  = 1_00
    axis_topology_x = sim.Topology.Periodic
    axis_topology_y = sim.Topology.Periodic
    axis_topology_z = sim.Topology.Periodic
    axis_bcs_x = (sim.BoundaryCondition.Floating, sim.BoundaryCondition.Floating)
    axis_bcs_y = (sim.BoundaryCondition.Floating, sim.BoundaryCondition.Floating)
    axis_bcs_z = (sim.BoundaryCondition.Floating, sim.BoundaryCondition.Floating)
    axis_bc_values_x = (DP.OCCUPIED.state, DP.OCCUPIED.state)
    axis_bc_values_y = (DP.OCCUPIED.state, DP.OCCUPIED.state)
    axis_bc_values_z = (DP.OCCUPIED.state, DP.OCCUPIED.state)
    do_edge_buffering: bool = True
    processing = sim.Processing.Serial
    n_threads: int = 16


parameters = Parameters()

_ = sim.dp(parameters)

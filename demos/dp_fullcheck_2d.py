import numpy as np
from scipy.stats import linregress
from numpy.typing import NDArray
from dprs import sim

print(f"\n{sim}")

class Parameters:
    dim = sim.Dimension.D2
    n_x: int = 5_000
    n_y: int = 5_000
    n_z: int = 1
    p: float = 0.163140
    p0: float = 0.99
    seed: int = 1
    n_iterations: int = 5_000
    sample_rate: int  = 5_000
    axis_topology_x = sim.Topology.Periodic
    axis_topology_y = sim.Topology.Periodic
    axis_topology_z = sim.Topology.Unspecified
    axis_bcs_x = (sim.BoundaryCondition.Floating, sim.BoundaryCondition.Floating)
    axis_bcs_y = (sim.BoundaryCondition.Floating, sim.BoundaryCondition.Floating)
    axis_bcs_z = (sim.BoundaryCondition.Unspecified, sim.BoundaryCondition.Unspecified)
    axis_bc_values_x = (True, True)
    axis_bc_values_y = (True, True)
    axis_bc_values_z = (False, False)
    do_edge_buffering: bool = True
    processing = sim.Processing.Parallel
    n_threads: int = 16
parameters = Parameters()
# Just in case we forget to update sample_rate to match n_iterations
if parameters.sample_rate > parameters.n_iterations:
    parameters.sample_rate = parameters.n_iterations

n_lattices: int
raw_lattices: list[list[bool]] 
raw_tracking: list[list, list]
t_run_time: float
(n_lattices, raw_lattices, raw_tracking, t_run_time)= sim.dp(parameters)
lattices: NDArray = np.array(raw_lattices, dtype=np.bool,).reshape(
    n_lattices, parameters.n_y, parameters.n_x,
).T
tracking: NDArray = np.array(raw_tracking, dtype=np.float64,) 

i_offset: int = parameters.n_iterations//3
t: NDArray = tracking[0][i_offset:]
ρ_mean: NDArray = tracking[1][i_offset:]
(exponent, scale, r_value, p_value, std_err) \
    = linregress(np.log(t), np.log(ρ_mean))

print(rf"Estimated t-decay exponent:  δ = {exponent:0.3f}")
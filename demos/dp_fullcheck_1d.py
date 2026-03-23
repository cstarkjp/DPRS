from functools import partial
import numpy as np
from scipy.stats import linregress
import matplotlib.pyplot as plt
from numpy.typing import NDArray
from dprs import sim
from dprs.viz import Viz
from dprs.utils import make_name, make_title, DUAL

print(f"\n{sim}")

class Parameters:
    dim = sim.Dimension.D1
    n_x: int = 10_000
    n_y: int = 1
    n_z: int = 1
    p: float = 0.538910
    p0: float = 0.99
    seed: int = 5
    n_iterations: int = 1000
    sample_period: int  = 0
    axis_topology_x = sim.Topology.Periodic
    axis_topology_y = sim.Topology.Unspecified
    axis_topology_z = sim.Topology.Unspecified
    axis_bcs_x = (sim.BoundaryCondition.Floating, sim.BoundaryCondition.Floating)
    axis_bcs_y = (sim.BoundaryCondition.Unspecified, sim.BoundaryCondition.Unspecified)
    axis_bcs_z = (sim.BoundaryCondition.Unspecified, sim.BoundaryCondition.Unspecified)
    axis_bc_values_x = (DUAL.OCCUPIED.state, DUAL.OCCUPIED.state)
    axis_bc_values_y = (DUAL.EMPTY.state, DUAL.EMPTY.state)
    axis_bc_values_z = (DUAL.EMPTY.state, DUAL.EMPTY.state)
    do_edge_buffering: bool = True
    processing = sim.Processing.Parallel
    n_threads: int = 16
parameters = Parameters()
# Just in case we forget to update sample_period to match n_iterations
if parameters.sample_period > parameters.n_iterations:
    parameters.sample_period = parameters.n_iterations

n_lattices: int
raw_lattices: list[list[bool]] 
raw_tracking: list[list, list]
t_run_time: float
(n_lattices, raw_lattices, raw_tracking, t_run_time)= sim.dp(parameters)
print(f"Total number of lattice time slices = {n_lattices}\n")
tracking: NDArray = np.array(raw_tracking, dtype=np.float64,) 

viz = Viz(dpi=250)
name: str
δ = 0.1594646
ρ_mean_ref = 0.563

name = make_name(parameters, "ρmean", None, )
print(name)
viz.plot_ρmean(
    name,
    make_title(parameters, None),
    tracking,
    δ, 
    ρ_mean_ref,
    fig_size=(6,4,),
    i_offset=1,
    do_ref_curve=True,
)
plt.show()

i_offset: int = parameters.n_iterations//3
t: NDArray = tracking[0][i_offset:]
ρ_mean: NDArray = tracking[1][i_offset:]
(exponent, scale, r_value, p_value, std_err) \
    = linregress(np.log(t), np.log(ρ_mean))

print(rf"Estimated t-decay exponent:  δ = {exponent:0.3f}")
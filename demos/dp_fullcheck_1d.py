from collections.abc import Sequence
import numpy as np
from scipy.stats import linregress
import matplotlib.pyplot as plt
from numpy.typing import NDArray
from dprs import sim
from dprs.sim import GrowthModelChoice
from dprs.viz import Viz
from dprs.utils import make_name, make_title, DUAL, postprocessing

print(f"\n{sim}")

class Parameters:
    growth_model_choice = sim.GrowthModelChoice.StaggeredDomanyKinzel
    # growth_model_choice = sim.GrowthModelChoice.SimplifiedDomanyKinzel
    dim = sim.Dimension.D1
    n_x: int = 10_000
    n_y: int = 1
    n_z: int = 1
    p_1: float = 0.705485152
    p_2: float = 0.705485152
    n_iterations: int = 1000
    sample_period: int  = 0
    initial_condition = sim.InitialCondition.Randomized
    p_initial: float = 0.99
    random_seed: int = 5
    topology_x = sim.Topology.Periodic
    topology_y = sim.Topology.Unspecified
    topology_z = sim.Topology.Unspecified
    bcs_x = (sim.BoundaryCondition.Floating, sim.BoundaryCondition.Floating)
    bcs_y = (sim.BoundaryCondition.Unspecified, sim.BoundaryCondition.Unspecified)
    bcs_z = (sim.BoundaryCondition.Unspecified, sim.BoundaryCondition.Unspecified)
    bc_values_x = (DUAL.OCCUPIED.state, DUAL.OCCUPIED.state)
    bc_values_y = (DUAL.EMPTY.state, DUAL.EMPTY.state)
    bc_values_z = (DUAL.EMPTY.state, DUAL.EMPTY.state)
    do_edge_buffering: bool = True
    processing = sim.Processing.Parallel
    n_threads: int = 16
parameters = Parameters()
# Just in case we forget to update sample_period to match n_iterations
if parameters.sample_period > parameters.n_iterations:
    parameters.sample_period = parameters.n_iterations

n_lattices: int
raw_lattices: list[list[bool]]
raw_tracking: Sequence[list]
t_run_time: float
(n_lattices, raw_lattices, raw_tracking, t_run_time)= sim.dk(parameters)

lattices: NDArray
tracking_array: NDArray
(lattices, tracking) \
    = postprocessing(parameters, n_lattices, raw_lattices, raw_tracking,)

viz = Viz(dpi=250)
name: str
δ = 0.1594646
scale = 0.752
# scale = 0.01

name = make_name(parameters, "ρmean", None, )
print(name)
viz.plot_lattice_statistic(
    name,
    make_title(parameters, None),
    tracking,
    choices=("time", "ρ_mean"),
    labels=(
        "Order parameter  $\\widebar{\\rho}(t)$", 
        "$\\widebar{\\rho}(t) \\sim t^{-\\delta}$",
        "${\\delta}$",
    ),
    exponent=-δ, 
    scale=scale,
    i_offset=1,
    do_ref_curve=True,
)
plt.show()

i_offset: int = parameters.n_iterations//3
t: NDArray = tracking["time"][i_offset:]
ρ_mean: NDArray = tracking["ρ_mean"][i_offset:]
(exponent, scale, r_value, p_value, std_err) \
    = linregress(np.log(t), np.log(ρ_mean))
print(rf"Estimated t-decay exponent:  δ = {exponent:0.3f}")
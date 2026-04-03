from functools import partial
from collections.abc import Sequence
import numpy as np
from scipy.stats import linregress
import matplotlib.pyplot as plt
from numpy.typing import NDArray
from dprs import sim
from dprs.viz import Viz
from dprs.utils import make_name, make_title, DUAL

print(f"\n{sim}")

class Parameters:
    growth_model_choice = sim.GrowthModelChoice.SimplifiedDomanyKinzel
    dim = sim.Dimension.D2
    n_x: int = 1000
    n_y: int = 1000
    n_z: int = 1
    p_1: float = 0.163145
    p_2: float = 0
    n_iterations: int = 1000
    sample_period: int  = 1000
    initial_condition = sim.InitialCondition.Randomized
    p_initial: float = 0.99
    random_seed: int = 1
    topology_x = sim.Topology.Periodic
    topology_y = sim.Topology.Periodic
    topology_z = sim.Topology.Unspecified
    bcs_x = (sim.BoundaryCondition.Floating, sim.BoundaryCondition.Floating)
    bcs_y = (sim.BoundaryCondition.Floating, sim.BoundaryCondition.Floating)
    bcs_z = (sim.BoundaryCondition.Unspecified, sim.BoundaryCondition.Unspecified)
    bc_values_x = (DUAL.OCCUPIED.state, DUAL.OCCUPIED.state)
    bc_values_y = (DUAL.OCCUPIED.state, DUAL.OCCUPIED.state)
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
pruned_tracking: Sequence[list]
t_run_time: float
(n_lattices, raw_lattices, raw_tracking, t_run_time)= sim.dk(parameters)
lattices: NDArray
if n_lattices>0:
    lattices = np.array(raw_lattices, dtype=np.bool,).reshape(
        n_lattices, parameters.n_y, parameters.n_x,
    ).T
else:
    lattices = np.zeros((0,))
pruned_tracking = []
for data in raw_tracking:
    if len(data)>0:
        pruned_tracking.append(data)
tracking: NDArray = np.array(pruned_tracking, dtype=np.float64,) 

viz = Viz(dpi=250)
i_slice: int
name: str
image_lattice = partial(
    viz.image_lattice,
    lattices=lattices, 
    p=parameters, 
    x=min(300, parameters.n_x),
    y=min(200, parameters.n_y),
    fig_size=(6, 4,),
)
if n_lattices>0:
    i_slice = 0
    name = make_name(parameters, "lattice", i_slice,)
    print(name)
    image_lattice(
        name=name, title=make_title(parameters, i_slice), i_lattice=i_slice,
    )
    plt.show()
if n_lattices>=1:
    i_slice = (n_lattices-1)
    name = make_name(parameters, "lattice", i_slice,)
    print(name)
    image_lattice(
        name=name, title=make_title(parameters, i_slice), i_lattice=i_slice,
    )
    plt.show()

δ = 0.45051
ρ_mean_ref = 0.238
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
)
plt.show()

i_offset: int = parameters.n_iterations//3
t: NDArray = tracking[0][i_offset:]
ρ_mean: NDArray = tracking[1][i_offset:]
(exponent, scale, r_value, p_value, std_err) \
    = linregress(np.log(t), np.log(ρ_mean))
print(rf"Estimated t-decay exponent:  δ = {exponent:0.3f}")
"""
Useful functions.
"""
import warnings
from enum import Enum
from dataclasses import dataclass
from collections.abc import Sequence
import numpy as np
from numpy.typing import NDArray
from dprs import sim
from dprs.sim import (
    GrowthModelChoice, Dimension,
)
warnings.filterwarnings("ignore")

__all__ = [
    "DUAL",
    "Parameters"
    "make_title",
    "make_name",
]

@dataclass
class Parameters(): 
    """Dummy declaration: shadows definition in Rust."""
    growth_model: sim.GrowthModel.DomanyKinzel
    dim: sim.Dimension.D1
    n_x: int
    n_y: int
    n_z: int
    p_1: float
    p_2: float
    n_iterations: int
    sample_period: int
    initial_condition: sim.InitialCondition.Randomized
    p_initial: float
    random_seed: int
    topology_x: sim.Topology
    topology_y: sim.Topology
    topology_z: sim.Topology
    bcs_x: tuple[sim.BoundaryCondition, sim.BoundaryCondition]
    bcs_y: tuple[sim.BoundaryCondition, sim.BoundaryCondition]
    bcs_z: tuple[sim.BoundaryCondition, sim.BoundaryCondition]
    bc_values_x: tuple[bool, bool]
    bc_values_y: tuple[bool, bool]
    bc_values_z: tuple[bool, bool]
    do_edge_buffering: bool
    processing: sim.Processing
    n_threads: int

class DUAL(Enum):
    """Abstract DP cell state."""
    EMPTY = False
    OCCUPIED = True

    @property
    def state(self):
        """Convert to boolean."""
        if self is DUAL.EMPTY:
            return False
        elif self is DUAL.OCCUPIED:
            return True

def postprocessing(parameters, n_lattices, raw_lattices, raw_tracking,):
    n_lattices: int
    raw_lattices: list[list[bool]] 
    raw_tracking: Sequence[list]
    lattices: NDArray
    skip: int = (
        2 if parameters.growth_model_choice==GrowthModelChoice.StaggeredDomanyKinzel 
        else 1
    )
    lattices: NDArray 
    if n_lattices>0:
        lattices_all: NDArray 
        match parameters.dim:
            case Dimension.D1:
                lattices_all = np.array(raw_lattices, dtype=np.bool,).reshape(
                    n_lattices, parameters.n_x,
                ).T
            case Dimension.D2:
                lattices_all = np.array(raw_lattices, dtype=np.bool,).reshape(
                    n_lattices, parameters.n_y, parameters.n_x,
                ).T
            case Dimension.D3:
                lattices_all = np.array(raw_lattices, dtype=np.bool,).reshape(
                    n_lattices, parameters.n_z, parameters.n_y, parameters.n_x,
                ).T
            case _: 
                raise Exception
        lattices = lattices_all[:, ::skip]
    else:
        lattices = np.zeros((0,))


    pruned_tracking: Sequence[list] = []
    for data in raw_tracking:
        if len(data)>0:
            pruned_tracking.append(data)
    tracking_array: NDArray = np.array(pruned_tracking, dtype=np.float64,)[:, ::skip]
    tracking = dict(
        iteration = tracking_array[0],
        time = tracking_array[0]/skip,
        mass = tracking_array[1],
        ρ_mean = tracking_array[2],
        R_mean = tracking_array[3],
    )

    return (lattices, tracking)


def make_title(parameters: Parameters, i_slice: int|None = None, z_slice: int|None = None): 
    """Generate a string summarizing the sim for entitling plots."""
    model: str
    match parameters.growth_model_choice:
        case GrowthModelChoice.SimplifiedDomanyKinzel: model="Simplified D-K:"
        case GrowthModelChoice.StaggeredDomanyKinzel: model="Staggered D-K:"
        case _: model="Unspecified model"
    return (
        (
            model
        )
        + rf"   " +
        (
            rf"$p_1={parameters.p_1:0.7f}$" if parameters.dim==sim.Dimension.D3
            else rf"$p_1={parameters.p_1:0.6f}$"
        )
        + rf"   " +
        (
            rf"$p_2={parameters.p_2:0.7f}$" if parameters.dim==sim.Dimension.D3
            else rf"$p_2={parameters.p_2:0.6f}$"
        )
        + "\n" +
        (
            rf"$s={parameters.random_seed}$"
        )
        + (
            rf"   $n_x={parameters.n_x}$" if parameters.n_x>=10000
            else rf"   $n_x={parameters.n_x}$"
        )
        + (
            rf"   $n_y={parameters.n_y}$" if parameters.n_y>1
           else ""
        )
        + (
            rf"   $n_z={parameters.n_z}$" if parameters.n_z>1
           else ""
        )
        + ("\n" + rf"$i={i_slice*parameters.sample_period:0{5}}$" 
           if i_slice is not None else "")
        + (rf"   $z={z_slice}$" 
           if z_slice is not None else "")
    )

def make_name(parameters: Parameters, variable: str, i_slice: int|None = None): 
    """Generate a string summarizing the sim for file naming."""
    return (
          f"{variable}"
        + (
            f"_p{parameters.p_1:0.7f}".replace(".", "p") 
                if parameters.dim==sim.Dimension.D3
            else f"_p{parameters.p_1:0.6f}".replace(".", "p")
        )
        + f"_s{parameters.random_seed}"
        + f"_nx{parameters.n_x}"
        + (
            f"_ny{parameters.n_y}" if parameters.n_y>1
            else ""
        )        
        + (
            f"_nz{parameters.n_z}" if parameters.n_z>1
            else ""
        )        
        + (f"_i{i_slice*parameters.sample_period:0{5}}" 
           if i_slice is not None else "")
    )
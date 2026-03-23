"""
Useful functions.
"""
import warnings
from enum import Enum
from dataclasses import dataclass
from dprs import sim

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
    p_0: float
    n_iterations: int
    sample_period: int
    initial_condition: sim.InitialCondition.Randomized
    p_initial: float
    random_seed: int
    axis_topology_x: sim.Topology
    axis_topology_y: sim.Topology
    axis_topology_z: sim.Topology
    axis_bcs_x: tuple[sim.BoundaryCondition, sim.BoundaryCondition]
    axis_bcs_y: tuple[sim.BoundaryCondition, sim.BoundaryCondition]
    axis_bcs_z: tuple[sim.BoundaryCondition, sim.BoundaryCondition]
    axis_bc_values_x: tuple[bool, bool]
    axis_bc_values_y: tuple[bool, bool]
    axis_bc_values_z: tuple[bool, bool]
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

def make_title(parameters: Parameters, i_slice: int|None = None, z_slice: int|None = None): 
    """Generate a string summarizing the sim for entitling plots."""
    return (
        (
            rf"$p={parameters.p_0:0.7f}$" if parameters.dim==sim.Dimension.D3
            else rf"$p={parameters.p_0:0.6f}$"
        )
        + rf"   $s={parameters.random_seed}$"
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
            f"_p{parameters.p_0:0.7f}".replace(".", "p") 
                if parameters.dim==sim.Dimension.D3
            else f"_p{parameters.p_0:0.6f}".replace(".", "p")
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
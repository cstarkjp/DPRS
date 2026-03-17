"""
Useful functions.
"""
import warnings
from dprs import sim

warnings.filterwarnings("ignore")

__all__ = [
    "make_title",
    "make_name",
    "from_serializable",
]

class Parameters: {}

def make_title(parameters: Parameters, i_slice: int | None): 
    return (
        rf"$p={parameters.p:0.6f}$"
        + rf"   $s={parameters.seed}$"
        + rf"   $n_x={parameters.n_x}$"
        + rf"   $n_y={parameters.n_y}$"
        + (rf"   $i={i_slice*parameters.sample_rate:0{5}}$" 
           if i_slice is not None else "")
    )

def make_name(parameters: Parameters, variable: str, i_slice: int|None = None): 
    return (
          f"{variable}"
        + f"_p{parameters.p:0.6f}".replace(".", "p")
        + f"_s{parameters.seed}"
        + f"_nx{parameters.n_x}"
        + f"_ny{parameters.n_x}"
        + (f"_i{i_slice*parameters.sample_rate:0{5}}" 
           if i_slice is not None else "")
    )
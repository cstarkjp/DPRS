import os
from functools import partial
from collections.abc import Sequence
import numpy as np
from scipy.stats import linregress
import matplotlib.pyplot as plt
from numpy.typing import NDArray
import dprs
from dprs import sim
from dprs.sim import (
    GrowthModel, Dimension,
    Topology, InitialCondition, BoundaryCondition, Processing
)
from dprs.viz import Viz
from dprs.file import create_directories, export_plots
from dprs.utils import (
    make_name, make_title, DUAL, postprocessing,
)

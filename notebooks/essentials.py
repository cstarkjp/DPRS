import os
from functools import partial
import numpy as np
from scipy.stats import linregress
import matplotlib.pyplot as plt
from numpy.typing import NDArray
import dprs
from dprs import sim
from dprs.viz import Viz
from dprs.file import create_directories, export_plots
from dprs.utils import make_name, make_title, DP

#!/usr/bin/env python3
import pathlib
import sys
from enum import Enum

this_dir = pathlib.Path(__file__).parent.resolve()
sys.path.append(this_dir.joinpath("target", "release").__str__())

import dprs as sim


class Parameters:
    dim: sim.Dimension = sim.Dimension.D2
    processing: sim.Processing = sim.Processing.ParallelChunked
    n_x: int = 10000
    n_y: int = 10000
    n_z: int = 1
    p: float = 0.5
    n_iterations: int = 1
    sample_rate: int = 10
    n_threads: int = 1
    serial_skip: int = 1
    do_buffering: bool = True


print(f"\n{sim}\n")
# help(sim)
_ = sim.life(Parameters())

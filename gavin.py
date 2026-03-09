#!/usr/bin/env python3
import os
import pathlib
import sys

this_dir = pathlib.Path(__file__).parent.resolve()
sys.path.append(this_dir.joinpath("target", "release").__str__())

import sim

# x, y, iterations, slow factor, threads
sim.life(10000, 10000, 50, 10, 10)

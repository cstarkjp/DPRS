#!/usr/bin/env python3
import os
import pathlib
import sys

this_dir = pathlib.Path(__file__).parent.resolve()
sys.path.append(this_dir.joinpath("target", "release").__str__())

import life

os.environ["RAYON_NUM_THREADS"] = "2"
life.sim(10000, 10000, 100)

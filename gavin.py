#!/usr/bin/env python3
import os
import pathlib
import sys

this_dir = pathlib.Path(__file__).parent.resolve()
sys.path.append(this_dir.joinpath("target", "release").__str__())

import dprs as sim


class Parameters:
    # pub dim: MyDimension,
    dim: int = 0
    processing: int = 0
    n_x: int = 1
    n_y: int = 1
    n_z: int = 1
    p: float = 0.5
    n_iterations: int = 10000
    sample_rate: int = 10000
    # pub processing: MyProcessing,
    n_threads: int = 4
    serial_skip: int = 10
    do_buffering: bool = False


sim.blah(Parameters())

print(f"\n{sim}\n")

kwargs = dict(
    n_x=10_000,
    n_y=10_000,
    # n_z = 1,
    p=0.5,
    n_iterations=50,
    sample_rate=10,
    n_threads=8,
    serial_skip=10,
    do_buffering=0,  # currently booleans not correctly parsed
)

_ = sim.life(**kwargs)

# num_iter = 10
# thread_counts = range(1, 11)
# sizes = [1000, 2000, 3000, 4000]
# for size in sizes:
#    ns = life.serial(size, size, num_iter)[1]
#    print(f"{size}: {ns / (size * size)}ns per cell for {num_iter} iterations serially")
#    for num_threads in thread_counts:
#        ns = life.parallel(num_threads, size, size, num_iter)[1]
#        print(
#            f"{size}: {ns / (size * size)}ns per cell for {num_iter} iterations with {num_threads} threads"
#        )
#    for num_threads in thread_counts:
#        ns = life.parallel_chunked(num_threads, size, size, num_iter)[1]
#        print(
#            f"{size}: {ns / (size * size)}ns per cell for {num_iter} iterations with {num_threads} threads using chunking"
#        )
#        pass
#    pass

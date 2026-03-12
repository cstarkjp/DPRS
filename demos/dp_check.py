from dprs import sim

print(f"\n{sim}\n")


class Parameters:
    # dim: Dimension::D1,
    # processing: Processing::ParallelChunked,
    dim: int = 0
    processing: int = 2
    n_x: int = 1
    n_y: int = 1
    n_z: int = 1
    p: float = 0.5
    n_iterations: int = 1
    sample_rate: int = 10
    n_threads: int = 1
    serial_skip: int = 1
    do_buffering: bool = True


parameters = Parameters()
parameters.n_x = 10_000
parameters.n_y = 10_000
parameters.p = 0.5
parameters.n_iterations = 50
parameters.sample_rate = 10
parameters.n_threads = 8
parameters.serial_skip = 10

_ = sim.dp(parameters)

from dprs import sim


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


def demo() -> None:
    print(sim)

    parameters = Parameters()
    parameters.n_x = 10_000
    parameters.n_y = 10_000
    parameters.n_iterations = 100
    # ?? parameters.slow_factor = 10
    parameters.n_threads = 16

    print(sim.life(parameters))

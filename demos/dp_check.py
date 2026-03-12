from dprs import sim

print(f"\n{sim}\n")

class Parameters:
    dim: int = sim.Dimension.D2
    processing: int = sim.Processing.Parallel
    n_x: int = 81 #10_000
    n_y: int = 51 #10_000
    n_z: int = 1
    p: float = 0.5
    n_iterations: int = 100
    sample_rate: int = 10
    n_threads: int = 8
    serial_skip: int = 10
    do_buffering: bool = True
parameters = Parameters()

_ = sim.dp(parameters)

from dprs import sim

def demo() -> None:
    print(sim)

    n_x = 10_000
    n_y = 10_000
    n_iterations = 100
    slow_factor = 10
    n_threads = 16

    print(sim.life(n_x, n_y, n_iterations, slow_factor, n_threads,))
from dprs import sim

def demo() -> None:
    print(sim)

    n_x = 10
    n_y = n_x
    n_iterations = 200

    print(sim.dp(n_x, n_y, n_iterations))
from dprs import sim

print(f"\n{sim}\n")

n_x = 10_000
n_y = n_x
n_iterations = 50
serial_skip = 10
n_threads = 1

_ = sim.life(n_x, n_y, n_iterations, serial_skip, n_threads,)

from dprs import sim

print(f"\n{sim}\n")

n_x = 5_000
n_y = n_x
n_iterations = 200

_ = sim.dp(n_x, n_y, n_iterations)

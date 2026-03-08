from dprs import sim

print(f"\n{sim}\n")

n_x = 100 #5_000
n_y = n_x
n_iterations = 100_000 #200

_ = sim.life(n_x, n_y, n_iterations, 10, 16,)

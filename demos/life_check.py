from dprs import sim

print(f"\n{sim}\n")

kwargs = dict(
    n_x = 10_000,
    n_y = 10_000,
    # n_z = 1,
    p = 0.5,
    n_iterations = 50,
    sample_rate = 10,
    n_threads = 8,
    serial_skip = 10,
    do_buffering = 0, # currently booleans not correctly parsed
)

_ = sim.life(**kwargs)

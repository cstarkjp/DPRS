# Directed Percolation in Rust (and Python)

In this project, we implement directed percolation (DP) and similar lattice  models in Rust. The Rust code is accessed via a Python wrapper to make experimentation as convenient as possible. Jupyter notebooks are used to implement the Python-wrapped simulations. 

![1d DP evolution for $p_c \approx p=0.53891$, $n_x=300$, $t=200$](docs/images/lattice_p0p538910_s5_nx300.png){width=500}

We have two motivations for adopting Rust: one is to ensure maximum performance; another is to achieve this in a memory-safe and bug-free fashion (which is not easy to do in C or C++). 

Fast run times are achieved through parallelization using the [`Rayon`](https://docs.rs/rayon/latest/rayon/) crate. 
We anticipate boosting performance further with GPU-compute using [`wgpu`](https://wgpu.rs/).

See [here](docs/HOWTO.md) for some rough "how-to" notes on wrapping Rust with Python.

## Demos

For now, only DP has been implemented.  A series of related models are in development.

### DP

![2d DP $t$-decay of mean order parameter $\overline\rho(t)$, for $p_c \approx p=0.163145$, $n_x=30000$, $n_y=30000$, $t=50000$](docs/images/ρmean_p0p163145_s1_nx30000_ny30000.png){width=500}

Directed-percolation model simulations in 2d are demonstrated in the following Jupyter notebook:

 - [Jupyter demo](notebooks/dp_2d.ipynb)

and pure Python demos can be found here:

 - [Python demos](demos/)


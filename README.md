# Directed Percolation in Rust and Python

[![](https://github.com/cstarkjp/DPRS/actions/workflows/PyPI.yml/badge.svg?style=cache-control=no-cache)](https://github.com/cstarkjp/DPRS/actions/workflows/PyPI.yml)
[![](https://github.com/cstarkjp/DPRS/actions/workflows/unittest-macos.yml/badge.svg?style=cache-control=no-cache)](https://github.com/cstarkjp/DPRS/actions/workflows/unittest-macos.yml)
[![](https://github.com/cstarkjp/DPRS/actions/workflows/unittest-linux.yml/badge.svg?style=cache-control=no-cache)](https://github.com/cstarkjp/DPRS/actions/workflows/unittest-linux.yml)
[![](https://github.com/cstarkjp/DPRS/actions/workflows/unittest-windows.yml/badge.svg?style=cache-control=no-cache)](https://github.com/cstarkjp/DPRS/actions/workflows/unittest-windows.yml)

In this project, we implement directed percolation models in Rust. The Rust code is accessed via a Python wrapper to make experimentation as convenient as possible. Jupyter notebooks are used to implement the Python-wrapped simulations. 

![1d DP-class, simplified Domany-Kinzel evolution for p_c ~ p=0.53891, n_x=300, t=200](https://raw.githubusercontent.com/cstarkjp/DPRS/main/docs/images/lattice_p0p538910_s5_nx300.png)

We have two motivations for adopting Rust: one is to ensure maximum performance; another is to achieve this in a memory-safe and bug-free fashion (which is not easy to do in C or C++). 
Fast run times are achieved through parallelization using the [`Rayon`](https://docs.rs/rayon/latest/rayon/) crate. 
We anticipate boosting performance further with GPU-compute using [`wgpu`](https://wgpu.rs/).

See [here](docs/HOWTO.md) for some rough "how-to" notes on wrapping Rust with Python.


![2d DP-class, simplified Domany-Kinzel t-decay of mean order parameter mean ρ(t), for p_c ~ p=0.163145, n_x=30000, n_y=30000, t=50000](https://raw.githubusercontent.com/cstarkjp/DPRS/main/docs/images/ρmean_p0p163145_s1_nx30000_ny30000.png)

## Demos

<!-- For now, only a simplified form of Domany-Kinzel has been implemented.  A series of related models are in development. -->


### Domany-Kinzel

Simplified DP-class Domany-Kinzel model simulations are demonstrated in the following Jupyter notebooks:

- [**1d DP** for a small lattice, to visualize t evolution](https://github.com/cstarkjp/DPRS/tree/main/notebooks/dp_1d_quick.ipynb)

- [**1d DP** for a large lattice and large number of iterations, to validate t-decay of mean ρ(t)](https://github.com/cstarkjp/DPRS/tree/main/notebooks/dp_1d.ipynb)

- [**2d DP** for a large lattice and large number of iterations, to validate t-decay of mean ρ(t)](https://github.com/cstarkjp/DPRS/tree/main/notebooks/dp_2d.ipynb)

Related pure-Python demos can be found here:

 - [Python demos](https://github.com/cstarkjp/DPRS/tree/main/demos/)




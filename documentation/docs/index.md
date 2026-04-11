# [**DPRS PyPI package**](https://pypi.org/project/dprs/)

###  _Directed percolation-type models in Rust_

[![](https://github.com/cstarkjp/DPRS/actions/workflows/PyPI.yml/badge.svg?style=cache-control=no-cache)](https://github.com/cstarkjp/DPRS/actions/workflows/PyPI.yml)
[![](https://github.com/cstarkjp/DPRS/actions/workflows/unittest-macos.yml/badge.svg?style=cache-control=no-cache)](https://github.com/cstarkjp/DPRS/actions/workflows/unittest-macos.yml)
[![](https://github.com/cstarkjp/DPRS/actions/workflows/unittest-linux.yml/badge.svg?style=cache-control=no-cache)](https://github.com/cstarkjp/DPRS/actions/workflows/unittest-linux.yml)
[![](https://github.com/cstarkjp/DPRS/actions/workflows/unittest-windows.yml/badge.svg?style=cache-control=no-cache)](https://github.com/cstarkjp/DPRS/actions/workflows/unittest-windows.yml)

[Live WASM demo](live-demo/http)

In this project, we implement directed percolation models in Rust. The [Rust code](https://github.com/cstarkjp/DPRS/tree/main/src) is accessed via a [Python wrapper](https://github.com/cstarkjp/DPRS/tree/main/src/sim.rs) to make experimentation as convenient as possible. Jupyter notebooks are used to implement the Python-wrapped simulations. 

![1d DP-class, simplified Domany-Kinzel evolution for $p_c \approx p=0.53891$, $n_x=300$, $t=200$](images/lattice_p0p538910_s5_nx300.png){width=600}

We have two motivations for adopting Rust: one is to ensure maximum performance; another is to achieve this in a memory-safe and bug-free fashion (which is not easy to do in C or C++). 
Fast run times are achieved through parallelization using the [`Rayon` crate](https://docs.rs/rayon/latest/rayon/). 
We anticipate boosting performance further with GPU-compute using [`wgpu`](https://wgpu.rs/).

<!-- See [here](demos-reference.md) for some demos.

See [here](HOWTO.md) for some rough "how-to" notes on wrapping Rust with Python. -->

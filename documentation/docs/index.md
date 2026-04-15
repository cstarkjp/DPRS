# [**Directed percolation solver in Rust/Python**](https://pypi.org/project/dprs/)

[![](https://github.com/cstarkjp/DPRS/actions/workflows/PyPI.yml/badge.svg?style=cache-control=no-cache)](https://github.com/cstarkjp/DPRS/actions/workflows/PyPI.yml)
[![](https://github.com/cstarkjp/DPRS/actions/workflows/unittest-macos.yml/badge.svg?style=cache-control=no-cache)](https://github.com/cstarkjp/DPRS/actions/workflows/unittest-macos.yml)
[![](https://github.com/cstarkjp/DPRS/actions/workflows/unittest-linux.yml/badge.svg?style=cache-control=no-cache)](https://github.com/cstarkjp/DPRS/actions/workflows/unittest-linux.yml)
[![](https://github.com/cstarkjp/DPRS/actions/workflows/unittest-windows.yml/badge.svg?style=cache-control=no-cache)](https://github.com/cstarkjp/DPRS/actions/workflows/unittest-windows.yml)

![1d DP-class, simplified Domany-Kinzel evolution for $p_c \approx p=0.53891$, $n_x=300$, $t=200$](images/lattice_p0p538910_s5_nx300.png){width=600}


##  DPRS

DPRS implements solvers for a variety of directed-percolation class models. The code is largely written in Rust.
The [solver](https://github.com/cstarkjp/DPRS/tree/main/dprs_core/src) is accessed via a [wrapper](https://github.com/cstarkjp/DPRS/tree/main/py_dprs/src) that exposes it to Python. This wrapping makes experimentation more convenient. 

The Python wrapper is available as a [PyPI package called DPRS](https://pypi.org/project/dprs/) and can be installed using `pip`. It has multi-platform support.
Jupyter notebooks are used to implement the Python-wrapped simulations. 


## Live demos


You can experiment with [interactive demos of the DP solver here.](live-demos/index.md)
These demos run the same Rust code as the Python-wrapped solver, but instead are made available using WebAssembly and accessed via a Typescript/Javascript wrapper.



## Technical motivation

We have two motivations for adopting Rust: one is to ensure maximum performance; another is to achieve this in a memory-safe and bug-free fashion (which is not easy to do in C or C++). 
Fast run times are achieved through parallelization using the [`Rayon` crate](https://docs.rs/rayon/latest/rayon/). 
We anticipate boosting performance further with GPU-compute using [`wgpu`](https://wgpu.rs/).

<!-- See [here](demos-reference.md) for some demos.

See [here](HOWTO.md) for some rough "how-to" notes on wrapping Rust with Python. -->

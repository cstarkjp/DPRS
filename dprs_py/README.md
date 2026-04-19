# Directed Percolation in Rust and Python

[![](https://github.com/cstarkjp/DPRS/actions/workflows/PyPI.yml/badge.svg?style=cache-control=no-cache)](https://github.com/cstarkjp/DPRS/actions/workflows/PyPI.yml)
[![](https://github.com/cstarkjp/DPRS/actions/workflows/unittest-macos.yml/badge.svg?style=cache-control=no-cache)](https://github.com/cstarkjp/DPRS/actions/workflows/unittest-macos.yml)
[![](https://github.com/cstarkjp/DPRS/actions/workflows/unittest-linux.yml/badge.svg?style=cache-control=no-cache)](https://github.com/cstarkjp/DPRS/actions/workflows/unittest-linux.yml)
[![](https://github.com/cstarkjp/DPRS/actions/workflows/unittest-windows.yml/badge.svg?style=cache-control=no-cache)](https://github.com/cstarkjp/DPRS/actions/workflows/unittest-windows.yml)

In this project, we implement a variety of directed percolation models in Rust. The Rust code is accessed via a Python wrapper to make experimentation as convenient as possible. Jupyter notebooks are used to implement the Python-wrapped simulations. 

![1d DP-class, simplified Domany-Kinzel evolution for p_c ~ p=0.53891, n_x=300, t=200](https://raw.githubusercontent.com/cstarkjp/DPRS/main/web/docs/images/lattice_p0p538910_s5_nx300.png)

<!-- We have two motivations for adopting Rust: one is to ensure maximum performance; another is to achieve this in a memory-safe and bug-free fashion (which is not easy to do in C or C++). 
Fast run times are achieved through parallelization using the [`Rayon`](https://docs.rs/rayon/latest/rayon/) crate. 
We anticipate boosting performance further with GPU-compute using [`wgpu`](https://wgpu.rs/). -->

Follow these links for more information:

 - [live online demos](https://cstarkjp.github.io/DPRS/live-demos/)
    - [in 1d](https://cstarkjp.github.io/DPRS/live-demos/1d_DP_simulations/)
    - [in 2d](https://cstarkjp.github.io/DPRS/live-demos/2d_DP_simulations/)
 - [documentation](https://cstarkjp.github.io/DPRS/)
 - [PyPI package](https://pypi.org/project/dprs/)
 - [Jupyter notebooks](https://github.com/cstarkjp/DPRS/tree/main/dprs_py/notebooks)
 - [core Rust solver](https://github.com/cstarkjp/DPRS/tree/main/dprs_core/src)
 - [Python wrapper](https://github.com/cstarkjp/DPRS/tree/main/dprs_py/src)
 - [WebAssembly/TypeScript wrapper](https://github.com/cstarkjp/DPRS/tree/main/dprs_web/src)
 
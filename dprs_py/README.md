# Directed percolation solver in Rust/Python

[![](https://github.com/cstarkjp/DPRS/actions/workflows/PyPI.yml/badge.svg?style=cache-control=no-cache)](https://github.com/cstarkjp/DPRS/actions/workflows/PyPI.yml)
[![](https://github.com/cstarkjp/DPRS/actions/workflows/unittest-macos.yml/badge.svg?style=cache-control=no-cache)](https://github.com/cstarkjp/DPRS/actions/workflows/unittest-macos.yml)
[![](https://github.com/cstarkjp/DPRS/actions/workflows/unittest-linux.yml/badge.svg?style=cache-control=no-cache)](https://github.com/cstarkjp/DPRS/actions/workflows/unittest-linux.yml)
[![](https://github.com/cstarkjp/DPRS/actions/workflows/unittest-windows.yml/badge.svg?style=cache-control=no-cache)](https://github.com/cstarkjp/DPRS/actions/workflows/unittest-windows.yml)

DPRS implements solvers for a variety of directed-percolation (DP) class cellular automata models.

![1d DP-class, simplified Domany-Kinzel evolution for p_c ~ p=0.53891, n_x=300, t=200](https://raw.githubusercontent.com/cstarkjp/DPRS/main/web/docs/images/lattice_p0p538910_s5_nx300.png)

The [core code](https://github.com/cstarkjp/DPRS/tree/main/dprs_core/src) is written in Rust, as is a [Python wrapper](https://github.com/cstarkjp/DPRS/tree/main/dprs_py/src) that provides easy access to the solvers, and a [Typescript wrapper](https://github.com/cstarkjp/DPRS/tree/main/dprs_web/src)  that allow them to be run directly in a web browser via WebAssembly.  Jupyter notebooks are used to implement the DP simulations. 

Access to the Python wrapper is provided by the `pip`-installable PyPI package [DPRS](https://pypi.org/project/dprs/) provided here. This package has multi-platform support, and should run on macOS, Windows and Linux without any difficulty (`pip` can rebuild the package from source if necessary).

Follow these links for more information:

 - [live online demos](https://cstarkjp.github.io/DPRS/live-demos/)
    - [in 1d](https://cstarkjp.github.io/DPRS/live-demos/1d_DP_simulations/)
    - [in 2d](https://cstarkjp.github.io/DPRS/live-demos/2d_DP_simulations/)
 - [documentation](https://cstarkjp.github.io/DPRS/)
 - [PyPI package](https://pypi.org/project/dprs/)
 - [Jupyter notebooks](https://github.com/cstarkjp/DPRS/tree/main/dprs_py/notebooks)
 - [core Rust solver](https://github.com/cstarkjp/DPRS/tree/main/dprs_core)
 - [Python wrapper](https://github.com/cstarkjp/DPRS/tree/main/dprs_py)
 - [WebAssembly/TypeScript wrapper](https://github.com/cstarkjp/DPRS/tree/main/dprs_web)
 
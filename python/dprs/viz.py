"""
Data visualization.
"""
import warnings
import logging
import numpy as np
import matplotlib as mpl
import matplotlib.pyplot as plt
from matplotlib.colors import ListedColormap
from matplotlib.figure import Figure
from matplotlib.axes import Axes
from typing import Any, Callable
from collections.abc import Sequence
from numpy.typing import NDArray
from dprs import sim

warnings.filterwarnings("ignore")

__all__ = ["Viz"]

class Viz:
    """
    Provide a visualization class.

    Args:
        dpi:
            set resolution for rasterized images
        font_size:
            set mpl default font size
        font_family:
            set mpl default font family

    Attributes:
        dpi (int):
            rasterization resolution
        fdict  (dict):
            dictionary to which each figure is appended as it is generated
    """

    dpi: int
    fdict: dict[Any, Any]

    def __init__(
            self, dpi: int=150, font_size: int=11, font_family="Arial",
        ) -> None:
        self.dpi = dpi
        self.fdict = {}
        try:
            mpl.rc("font", size=font_size, family=font_family)
        except:
            mpl.rc("font", size=font_size, family="")


    def create_figure(
        self,
        fig_name: str,
        fig_size: tuple[float, float] | None = None,
        dpi: int | None = None,
    ) -> Figure:
        """
        Initialize a Pyplot figure.

        Set its size and DPI. Append it to the figures dictionary.


        Args:
            fig_name:
                name of figure; used as key in figures dictionary
            fig_size:
                optional width and height of figure in inches
            dpi:
                rasterization resolution

        Returns:
            figure:
                reference to MatPlotLib/Pyplot figure
        """
        fig_size_: tuple[float, float] = (
            (6, 4) if fig_size is None else fig_size
        )
        dpi_: float = self.dpi if dpi is None else dpi
        logging.info(
            "Viz:\n   "
            + f"Creating plot: {fig_name} size={fig_size_} @ {dpi_} dpi"
        )
        fig = plt.figure()
        self.fdict.update({fig_name: fig})
        if fig_size_ is not None:
            fig.set_size_inches(*fig_size_)
        fig.set_dpi(dpi_)
        return fig

    def image_lattice_history(
            self,
            name: str,
            title: str,
            lattices: NDArray|None, 
            p: sim.Parameters,
            x: int | None=None, 
            t: int | None=None,
            fig_size: tuple[float,float]=(6,4,),
        ) -> tuple[Figure, Any]:
        """
        Plot colorized image of 1d lattice history.
        """
        _ = self.create_figure(fig_name=name, fig_size=fig_size,)
        plt.title(title, fontdict={"fontsize": 11.5})
        color_map = ListedColormap(((0.9, 0.9, 0.9,), (0.65, 0, 0.65),))
        x = (lattices.shape[0] if x is None else min(x, lattices.shape[0]))
        t = (lattices.shape[1] if t is None else min(t, lattices.shape[1]))
        plt.imshow(
            lattices[0:x, 0:t, ].T, 
            vmin=0, vmax=1,
            cmap=color_map, 
            extent=(0, x, t-0.5, -0.5),
        )
        color_bar = plt.colorbar(
            ticks=(0.25, 0.75,), 
            shrink=0.5*(t/x)**0.25, 
            aspect=15,
            label="cell state",
        )
        color_bar.set_ticklabels((0, 1,),)
        plt.xlabel(r"$x$")
        plt.ylabel(r"$t$")
        plt.grid(ls=":")
        # plt.close()

    def image_lattice(
            self,
            name: str,
            title: str,
            lattices: NDArray|None, 
            p: sim.Parameters,
            i_lattice: int | None = 0, 
            x: int | None=None, 
            y: int | None=None,
            fig_size: tuple[float,float]=(6,4,),
        ) -> tuple[Figure, Any]:
        """
        Plot colorized image of lattice.
        """
        _ = self.create_figure(fig_name=name, fig_size=fig_size,)
        plt.title(title, fontdict={"fontsize": 11.5})
        color_map = ListedColormap(((0.9, 0.9, 0.9,), (0.65, 0, 0.65),))
        x = (lattices.shape[0] if x is None else min(x, lattices.shape[0]))
        y = (lattices.shape[1] if y is None else min(y, lattices.shape[1]))
        plt.imshow(
            lattices[0:x, 0:y, i_lattice,].T, 
            vmin=0, vmax=1,
            cmap=color_map, 
            origin="lower",
            extent=(0, x, y, 0),
        )
        color_bar = plt.colorbar(
            ticks=(0.25, 0.75,), 
            shrink=0.5*(y/x)**0.25, 
            aspect=15,
            label="cell state",
        )
        color_bar.set_ticklabels((0, 1,),)
        plt.xlabel(r"$x$")
        plt.ylabel(r"$y$")
        plt.grid(ls=":")
        # plt.close()


    def plot_ρmean(
            self,
            name: str,
            title: str,
            tracking: NDArray,
            δ: float, 
            ρ_mean_ref: float,
            fig_size: tuple[float,float]=(6,4,),
            i_offset: int=3,
            do_ref_curve: bool=True,
        ) -> tuple[Figure, Any]:
        """
        Plot time evolution of mean order parameter.
        """
        _ = self.create_figure(fig_name=name, fig_size=fig_size,)
        plt.title(title, fontdict={"fontsize": 13})
        t: NDArray = tracking[0][i_offset:]
        ρ_mean: NDArray = tracking[1][i_offset:]
        ρ_mean_fn = lambda t: ρ_mean_ref*t**(-δ)
        plt.plot(
            t, ρ_mean, lw=0.4, color="k",
        )
        if do_ref_curve:
            plt.plot(
                t, ρ_mean_fn(t), color="blue", alpha=0.5, 
                label=r"$\widebar\rho(t) \sim t^{-\delta}$" + rf"$\quad\delta={δ}$",
            )
        plt.legend()
        axes = plt.gca()
        axes.autoscale(enable=True, axis="both", tight=True)
        plt.loglog()
        # plt.ylim(0,)
        # plt.xlim(0,)
        plt.ylabel(r"Mean order parameter  $\widebar\rho(t)$")
        plt.xlabel(r"Time  $t$")
        plt.grid(ls=":")
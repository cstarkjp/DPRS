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
from dprs.sim import InitialCondition

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
            self, dpi: int=150, font_size: int=11, font_family: str="Arial",
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

    def lattice_history(
            self,
            name: str,
            title: str,
            lattices: NDArray|None, 
            p: sim.Parameters,
            x: int | None=None, 
            t: int | None=None,
            fig_size: tuple[float,float]=(6,4,),
        ) -> None:
        """
        Plot colorized image of 1d lattice history.
        """
        _ = self.create_figure(fig_name=name, fig_size=fig_size,)
        plt.title(title, fontdict={"fontsize": 11.5})
        color_map = ListedColormap(((0.9, 0.9, 0.9,), (0.65, 0, 0.65),))
        x_span = (lattices.shape[0] if x is None else min(x, lattices.shape[0]))
        t = (lattices.shape[1] if t is None else min(t, lattices.shape[1]))
        match p.initial_condition:
            case InitialCondition.CentralCell: 
                x = (lattices.shape[0]//2-x_span, lattices.shape[0]//2+x_span,)
            case InitialCondition.EdgeCell: 
                x = (0, lattices.shape[0]//2+x_span,)
            case InitialCondition.Randomized: 
                x = (0, x_span,)
            case _:
                raise Exception
        plt.imshow(
            lattices[x[0]:x[1], 0:t, ].T, 
            vmin=0, vmax=1,
            cmap=color_map, 
            extent=(*x, t-0.5, -0.5),
        )
        color_bar = plt.colorbar(
            ticks=(0.25, 0.75,), 
            shrink=min(0.35, 0.5*(t/x_span)**0.25), 
            aspect=15,
            label="cell state",
        )
        color_bar.set_ticklabels((0, 1,),)
        plt.xlabel(r"$x$")
        plt.ylabel(r"$t$")
        plt.grid(ls=":")
        # plt.close()

    def lattice(
            self,
            name: str,
            title: str,
            lattices: NDArray|None, 
            p: sim.Parameters,
            i_lattice: int | None = 0, 
            x: int | None=None, 
            y: int | None=None,
            z: int | None=None,
            fig_size: tuple[float,float]=(6,4,),
        ) -> None:
        """
        Plot colorized image of lattice.
        """
        _ = self.create_figure(fig_name=name, fig_size=fig_size,)
        plt.title(title, fontdict={"fontsize": 11.5})
        color_map = ListedColormap(((0.9, 0.9, 0.9,), (0.65, 0, 0.65),))
        x = (lattices.shape[0] if x is None else min(x, lattices.shape[0]))
        y = (lattices.shape[1] if y is None else min(y, lattices.shape[1]))
        z = (lattices.shape[2] if z is None else min(z, lattices.shape[2]))
        if p.dim == sim.Dimension.D3:
            lattice_slice = lattices[0:x, 0:y, z, i_lattice,].T
        else:
            lattice_slice = lattices[0:x, 0:y, i_lattice,].T
        plt.imshow(
            lattice_slice, 
            vmin=0, vmax=1,
            cmap=color_map, 
            origin="lower",
            extent=(0, x, 0, y),
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

    def lattice_statistic(
            self,
            name: str,
            title: str,
            tracking: dict,
            choices: tuple[str, str],
            labels: Sequence[str],
            exponent: float, 
            scale: float,
            fig_size: tuple[float,float]=(6,4,),
            i_offset: int=3,
            do_ref_curve: bool=True,
        ) -> tuple[Figure, Any]:
        """
        Plot time evolution of mean cluster radius.
        """
        _ = self.create_figure(fig_name=name, fig_size=fig_size,)
        plt.title(title, fontdict={"fontsize": 13}, pad=10,)
        t: NDArray = tracking[choices[0]][i_offset:]
        statistic: NDArray = tracking[choices[1]][i_offset:]
        statistic_fn = lambda t: scale*t**exponent
        plt.plot(
            t, statistic, lw=0.4, color="k",
        )
        if do_ref_curve:
            plt.plot(
                t, statistic_fn(t), color="blue", alpha=0.5, 
                label = rf"{labels[1]}" 
                        + rf"$\quad$" 
                        + rf"{labels[2]}" 
                        + rf"$={exponent:0.4f}$",
            )
        plt.legend()
        axes = plt.gca()
        axes.autoscale(enable=True, axis="both", tight=True)
        plt.loglog()
        # plt.ylim(0,)
        # plt.xlim(0,)
        plt.ylabel(rf"{labels[0]}")
        plt.xlabel(r"Time  $t$")
        plt.grid(ls=":")
        # plt.close()

    def phase_diagram(
            self,
            name: str,
            title: str,
            expts: dict,
            i_equal: int,
            fig_size: tuple[float,float]=(4,4,),
        ) -> None:
        """
        Plot p_1-p_2 phase diagram for the DP bedload model.
        """
        _ = self.create_figure(fig_name=name, fig_size=fig_size,)
        plt.title(title, fontdict={"fontsize": 11.5})

        p1_p2 = np.array([ (expt["p_1"], expt["p_2"]) for (key, expt,) in expts.items()]).T
        p1 = np.concat([p1_p2[0, :], [p1_p2[0, -1]], [0, 0, p1_p2[0, 0], p1_p2[0, 0]]])
        p2 = np.concat([p1_p2[1, :], [0], [0, 1, 1, p1_p2[1, 0]]])
        p1c = np.concat([p1_p2[0, :], [p1_p2[0, -1]], [1, 1, p1_p2[0, 0], p1_p2[0, 0]]])
        p2c = np.concat([p1_p2[1, :], [0], [0, 1, 1, p1_p2[1, 0]]])

        # plt.plot(*p1_p2, "o", ms=3, color="DarkBlue",)
        plt.plot(*p1_p2, "-", color="DarkBlue",)
        plt.fill(p1, p2, color="DarkBlue", alpha=0.1,)
        plt.fill(p1c, p2c, color="DarkRed", alpha=0.1,)
        plt.plot((0,1), (0,1), ":", color="DarkBlue", alpha=0.3,)
        sym_expt = expts[i_equal]
        plt.plot(sym_expt["p_1"], sym_expt["p_2"], "o", ms=5, color="DarkBlue",)
        plt.xlim(0, 1,)
        plt.ylim(0, 1,)
        plt.xlabel(r"Collective entrainment - solo detrainment  $p_1$")
        plt.ylabel(r"Collective detrainment  $p_2$")
        axes = plt.gca()
        axes.set_aspect(1)
        plt.text(x=0.15, y=0.47, s="static bed", color="DarkBlue", font={"size": 14},)
        plt.text(
            x=0.85, y=0.55, s="mobile\nbed", color="DarkRed", 
            horizontalalignment="center", font={"size": 14},
        )
        plt.grid(ls=":")
        # plt.close()

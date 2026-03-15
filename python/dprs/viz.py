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

__all__ = [
    "Viz"
]

class Viz:
    """
    Provide a visualization class.

    Args:
        dpi:
            resolution for rasterized images
        font_size:
            general font size

    Attributes:
        dpi (int):
            resolution for rasterized images
        font_size (int):
            general font size
        fdict  (dict):
            dictionary to which each figure is appended as it is generated
    """

    dpi: int
    font_size: int
    fdict: dict[Any, Any]
    font_family: str

    def __init__(self, dpi: int = 150, font_size: int = 11) -> None:
        """
        Instantiate the visualization class and set some figure parameters.
        
        Set a standard font size and font family, choosing Arial if possible.
        """
        self.dpi = dpi
        self.font_size = font_size
        self.fdict = {}
        self.font_family = "Arial" #if "Arial" in self.get_fonts() else "Helvetica"
        try:
            mpl.rc("font", size=self.font_size, family=self.font_family)
        except:
            mpl.rc("font", size=self.font_size, family="")


    def create_figure(
        self,
        fig_name: str,
        fig_size: tuple[float, float] | None = None,
        dpi: int | None = None,
    ) -> Figure:
        """
        Initialize a Pyplot figure.

        Set its size and dpi. Append it to the figures dictionary.


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
            (8, 8) if fig_size is None else fig_size
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

    def image_lattice(
            self,
            name: str,
            title: str,
            lattices: NDArray, 
            p: sim.Parameters,
            i_lattice: int=0, 
            x: int | None=None, 
            y: int | None=None,
            fig_size: tuple[float,float]=(6,4,),
        ) -> tuple[Figure, Any]:
        """
        Plot colorized image of lattice.
        """
        _ = self.create_figure(fig_name=name, fig_size=fig_size,)
        plt.title(title)
        color_map = ListedColormap(((0.9, 0.9, 0.9,), (0.65, 0, 0.65),))
        x = (lattices.shape[0] if x is None else x)
        y = (lattices.shape[1] if y is None else y)
        plt.imshow(
            lattices[0:x, 0:y, i_lattice,].T, 
            vmin=0, vmax=1,
            cmap=color_map, 
            origin="lower",
        )
        color_bar = plt.colorbar(
            ticks=(0.25, 0.75,), 
            shrink=0.5*(p.n_y/p.n_x)**0.25, 
            aspect=15,
            label="cell state",
        )
        color_bar.set_ticklabels((0, 1,),)
        plt.xlabel(r"$x$")
        plt.ylabel(r"$y$")
        plt.grid(ls=":")
        # plt.close()

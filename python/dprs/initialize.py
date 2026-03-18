"""
Configure to run in `IPython`_.

---------------------------------------------------------------------

Sets up `IPython`_ environment if e.g. we're running
in a `Jupyter notebook`_ or `Jupyter QtConsole`_.

 - prepares Matplotlib to display inline and (for Macs)
   at a 'retina' resolution -- if this
   is not available, a benign error report (currently disabled)
   is made and progress continues
 - enables automatic reloading (in case the code has been modded) when
   a notebook is re-run in-situ
"""

# import logging
# import matplotlib as mpl

# Jupyter `%magic` commands `%load_ext`, `%aimport`, and `%autoreload`
#  are needed here to force the notebook to reload the `streamline` module,
#  and its constituent modules, as changes are made to it.
# Force module to reload

# https://ipython.readthedocs.io/en/stable/api/generated/IPython.core.getipython.html
# <ipython-input-2-5aa624c5c899>:1:   
#    DeprecationWarning: `magic(...)` is deprecated since IPython 0.13 
#    (warning added in 8.1), use run_line_magic(magic_name, parameter_s).

try:
    from IPython import get_ipython #type: ignore
        
    def check_is_ipython() -> bool:
        """Check if we are running an IPython kernel from Jupyter etc."""
        try:
            if "IPKernelApp" not in get_ipython().config:  # pragma: no cover
                return False
        except ImportError:
            return False
        except AttributeError:
            return False
        return True


    is_python: bool = check_is_ipython()

    if is_python:
        try:
            # get_ipython().magic("config InlineBackend.figure_format = 'retina'")
            get_ipython().run_line_magic(
                "config", 
                "InlineBackend.figure_format = 'retina'",
            )
        except NameError:
            pass

        try:
            # get_ipython().magic("matplotlib inline")
            get_ipython().run_line_magic("matplotlib", "inline",)
        except NameError:
            pass


        try:
            # get_ipython().magic("load_ext autoreload")
            # get_ipython().magic("autoreload 2")
            get_ipython().run_line_magic("load_ext", "autoreload",)
            get_ipython().run_line_magic("autoreload", "2",)
            # get_ipython().magic('aimport '+package_name)
        except NameError as error:
            print(
                "Error trying to invoke get_ipython(), "
                + "possibly because not running IPython:",
                error,
            )
        # except:
        #     print('Possibly benign error trying to config autoreload')
except:
    pass
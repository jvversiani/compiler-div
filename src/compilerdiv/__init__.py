"""compilerdiv: behavioral equivalence analysis of diverse compilers.

Compares the binaries emitted by a reference compiler against those emitted by
diverse variants, across a corpus of self-contained programs, using four
departure detectors calibrated against empirical noise floors.

The compilers under test and the corpus language are declared in the config;
nothing here is tied to a particular toolchain or language.
"""

__version__ = "0.1.0"

from .config import Settings, load_settings
from .store.raw import RawStore

__all__ = ["Settings", "load_settings", "RawStore", "__version__"]

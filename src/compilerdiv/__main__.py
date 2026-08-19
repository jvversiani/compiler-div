"""Enables ``python -m compilerdiv`` without a runpy double-import warning."""

import sys

from .cli.main import main

if __name__ == "__main__":
    sys.exit(main())

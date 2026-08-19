"""Acquisition: compile, run, trace. Writes only to the raw store."""

from .build import compile_one, elf_facts, run_binary
from .preflight import preflight
from .sweep import Sweeper
from .trace import ArgNormalizer, parse_trace, trace_once

__all__ = [
    "compile_one",
    "elf_facts",
    "run_binary",
    "preflight",
    "Sweeper",
    "ArgNormalizer",
    "parse_trace",
    "trace_once",
]

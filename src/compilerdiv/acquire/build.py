"""Compilation and benchmark measurement.

Two properties worth stating explicitly:

* :func:`run_binary` captures stdout and the exit code, and the caller verifies
  both against the program's embedded expected output. Without that check a
  binary that segfaults at startup still contributes timing, size, and (fewer)
  syscalls -- and fewer syscalls reads as "closer to baseline" under a
  divergence metric.
* ELF layout facts (segment count, size) are collected once per binary. They are
  the evidence for classifying a departure as layout-conditional rather than
  semantic, which is the open question behind any ``read(+1)``-shaped finding.
"""

from __future__ import annotations

import re
import shutil
import subprocess
import time
from dataclasses import dataclass
from pathlib import Path

from ..config import CompilerSpec, Settings
from ..corpus import Program, check_output


@dataclass
class CompileResult:
    ok: bool
    wall_s: float
    size_b: int
    error: str = ""


@dataclass
class ExecResult:
    ok: bool
    wall_s: float
    returncode: int
    verified: bool
    detail: str = ""


@dataclass
class ElfFacts:
    """Static layout facts used by the departure taxonomy.

    ``probe_ok`` distinguishes "readelf ran and the binary genuinely has no
    segments" from "readelf never ran". Only ``size_b`` is populated in the
    latter case, and the resume state must not treat such a row as collected --
    otherwise installing binutils later cannot repair the store.
    """

    size_b: int = 0
    n_segments: int = 0
    n_sections: int = 0
    interp: str = ""
    n_dynamic_needed: int = 0
    probe_ok: bool = False


def compile_one(
    settings: Settings,
    spec: CompilerSpec,
    config_name: str,
    program: Program,
    out_path: Path,
) -> CompileResult:
    """Compile one source. Timing is wall-clock around the compiler process."""
    flags = settings.config(config_name).flags
    argv = spec.build_argv(program.path, out_path, flags)
    cwd = str((settings.root / spec.cwd).resolve()) if spec.cwd else None

    out_path.parent.mkdir(parents=True, exist_ok=True)
    if out_path.exists():
        out_path.unlink()

    start = time.perf_counter()
    try:
        res = subprocess.run(
            argv,
            cwd=cwd,
            stdout=subprocess.DEVNULL,
            stderr=subprocess.PIPE,
            timeout=settings.trace.timeout_s,
        )
    except subprocess.TimeoutExpired:
        return CompileResult(False, time.perf_counter() - start, 0, "compile timeout")
    except OSError as e:
        return CompileResult(
            False, time.perf_counter() - start, 0, f"could not run: {e}"
        )
    wall = time.perf_counter() - start

    ok = res.returncode == 0 and out_path.is_file() and out_path.stat().st_size > 0
    if not ok:
        tail = res.stderr.decode(errors="replace").strip()
        msg = tail.splitlines()[-1] if tail else "no output produced"
        return CompileResult(False, wall, 0, msg)

    return CompileResult(True, wall, out_path.stat().st_size)


def run_binary(settings: Settings, binary: Path, program: Program) -> ExecResult:
    """Execute a binary untraced, timing it and verifying its output.

    Verification uses the program's own embedded expected-output block, so the
    integration test and the behavioral measurement now agree by construction
    rather than living in separate scripts.
    """
    start = time.perf_counter()
    try:
        res = subprocess.run(
            [str(binary)],
            cwd=str(binary.parent),
            capture_output=True,
            text=True,
            timeout=settings.bench.exec_timeout_s,
        )
    except subprocess.TimeoutExpired:
        return ExecResult(False, time.perf_counter() - start, -1, False, "exec timeout")
    except OSError as e:
        return ExecResult(
            False, time.perf_counter() - start, -1, False, f"exec failed: {e}"
        )
    wall = time.perf_counter() - start

    if res.returncode != 0:
        return ExecResult(
            False, wall, res.returncode, False, f"nonzero exit {res.returncode}"
        )

    if not settings.bench.verify_output:
        return ExecResult(True, wall, 0, True, "")

    passed, detail = check_output(res.stdout, program)
    return ExecResult(True, wall, 0, passed, detail or "")


# re.M is load-bearing: readelf's segment table starts several lines into the
# output, so without it ``^`` anchors to the start of the whole capture and the
# pattern never matches -- n_segments silently stays 0 and the layout probe
# drops to two variables while still reporting "ok".
_SEG_RE = re.compile(
    r"^\s+(LOAD|DYNAMIC|INTERP|GNU_RELRO|GNU_STACK|NOTE|PHDR|TLS)\b", re.M
)


def elf_facts(binary: Path) -> ElfFacts:
    """Collect static layout facts via readelf, degrading gracefully.

    These feed the layout-correlation probe: if membership in a conditional
    departure's affected set tracks segment count or binary size, the departure
    is a linking artifact and not program behavior.
    """
    facts = ElfFacts(size_b=binary.stat().st_size if binary.is_file() else 0)
    if shutil.which("readelf") is None:
        return facts

    try:
        res = subprocess.run(
            ["readelf", "-lWSd", str(binary)],
            capture_output=True,
            text=True,
            timeout=60,
        )
    except (OSError, subprocess.TimeoutExpired):
        return facts

    if res.returncode != 0:
        return facts

    text = res.stdout
    facts.n_segments = len(_SEG_RE.findall(text))
    facts.n_sections = len(re.findall(r"^\s*\[\s*\d+\]\s+\S", text, re.M))
    m = re.search(r"\[Requesting program interpreter:\s*([^\]]+)\]", text)
    if m:
        facts.interp = m.group(1).strip()
    facts.n_dynamic_needed = len(re.findall(r"\(NEEDED\)", text))
    facts.probe_ok = True
    return facts

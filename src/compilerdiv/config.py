"""Configuration model.

A validated dataclass tree loaded from YAML. The important property is
:meth:`Settings.fingerprint`: acquisition parameters that would invalidate an
existing raw log are hashed, and the hash is stored alongside the raw data. A
mismatch on resume is a hard error, not something the operator is trusted to
remember -- silently resuming under changed settings would mix incomparable
measurements into one store, undetectably.
"""

from __future__ import annotations

import hashlib
import json
from dataclasses import dataclass, field, asdict
from pathlib import Path
from typing import Any

import yaml

from .corpus import BUILTIN_LANGUAGES, LanguageSpec, language_registry

# ---------------------------------------------------------------------------
# Reserved compiler tags. These are not compilers the user configures; they are
# control passes derived from the baseline, and they are *generated* rather
# than named, because there are ``builds x passes`` of them.
#
# The controls are nested, and the nesting is the point:
#
#   build b  -- an independent compilation of the same source. Differences
#               between builds are build nondeterminism: link layout, embedded
#               paths, timestamps.
#   pass p   -- an independent trace session of one build's binary.
#               Differences between passes of the SAME build are run-to-run
#               nondeterminism: ASLR, scheduler, futex ordering, DNS retries.
#
# With passes nested inside builds the two are measured directly. Flat controls
# can only infer the build class by subtracting the run class from a single
# rebuild, which is an estimate from n=2 and cannot distinguish a reproducible
# property of the compiler from a one-off linker hiccup.
#
# ``(0, 0)`` is the baseline itself and keeps the user's own key (``A``), so
# every detector, figure and workbook that refers to the baseline is unaffected.
# ---------------------------------------------------------------------------

#: Separates the baseline key from the ``build.pass`` coordinate.
TAG_SEP = "@"


def control_tag(baseline: str, build: int, pass_: int) -> str:
    """Tag for one (build, pass) control group. ``(0, 0)`` is the baseline."""
    if build == 0 and pass_ == 0:
        return baseline
    return f"{baseline}{TAG_SEP}{build}.{pass_}"


def parse_control_tag(tag: str) -> tuple[int, int] | None:
    """``(build, pass)`` for a generated tag, or ``None`` if it is not one.

    A bare baseline key returns ``None`` rather than ``(0, 0)``: callers want to
    know "is this a control tag", and the baseline is not one.
    """
    if TAG_SEP not in tag:
        return None
    _, _, coord = tag.partition(TAG_SEP)
    build, _, pass_ = coord.partition(".")
    try:
        return int(build), int(pass_)
    except ValueError:
        return None


class ConfigError(ValueError):
    """Raised when the YAML configuration is malformed or inconsistent."""


# ---------------------------------------------------------------------------
# Leaf specs
# ---------------------------------------------------------------------------


@dataclass(frozen=True)
class CompilerSpec:
    """A single compiler under test.

    Attributes
    ----------
    key:
        Short identifier used in the raw logs (``A``, ``B``, ``C``). Must not
        contain :data:`TAG_SEP`, which is reserved for the generated control
        tags.
    label:
        Human-readable name used in plots and reports.
    cmd:
        Argument vector prefix, e.g. ``["cc"]`` or ``["./build.sh", "compile"]``.
    cwd:
        Working directory for the invocation, or ``None`` for the project root.
    src_first:
        If true, the source path is placed immediately after ``cmd`` rather
        than at the end. Needed for drivers that dispatch on argument position:
        with the source last they fall through to a different mode entirely,
        typically a project-build path that then fails looking for a manifest.
    output_flag:
        How this compiler names its output file. The default ``-o`` emits two
        tokens (``-o`` ``<out>``), which covers most compilers. A flag ending
        in ``=`` or ``:`` is *joined* to the
        path as a single token instead, for compilers like Nim (``-o:``), D's
        dmd (``-of=``) or Zig (``-femit-bin=``). An empty string emits just the
        path, for compilers that take the output positionally.
    color:
        Matplotlib colour for this compiler across all plots.
    """

    key: str
    label: str
    cmd: tuple[str, ...]
    cwd: str | None = None
    src_first: bool = False
    output_flag: str = "-o"
    color: str = "#4C72B0"

    def _output_args(self, out: Path) -> list[str]:
        flag = self.output_flag
        if not flag:
            return [str(out)]
        if flag.endswith(("=", ":")):
            return [f"{flag}{out}"]
        return [flag, str(out)]

    def build_argv(self, src: Path, out: Path, flags: tuple[str, ...]) -> list[str]:
        """Assemble the full compile command for one source file."""
        out_args = self._output_args(out)
        if self.src_first:
            return [*self.cmd, str(src), *flags, *out_args]
        return [*self.cmd, *flags, *out_args, str(src)]


@dataclass(frozen=True)
class BuildConfig:
    """A named set of compiler flags (an "optimisation configuration")."""

    name: str
    flags: tuple[str, ...] = ()
    #: ``(compiler_key)`` values that cannot build this config -- e.g. a
    #: compiler that cannot link statically, for a ``static`` config.
    unsupported: tuple[str, ...] = ()


@dataclass(frozen=True)
class TraceSettings:
    """How binaries are traced.

    Attributes
    ----------
    reps:
        Number of independent trace runs per (binary, tag).
    wrapper:
        Command prefix placed before ``strace``. On servers where ptrace is
        confined, ``["unshare", "-Ur"]`` is required. Empty list to disable.
    strace_bin:
        Path or name of the strace executable.
    capture_args:
        Record syscall arguments (enables the argument-divergence detector).
        Costs disk and time: the trace is a full log rather than a summary.
    arg_syscalls:
        Syscalls whose arguments are recorded. Keep this narrow; recording
        arguments for ``mmap`` produces noise-dominated garbage.
    capture_sequence:
        Record per-thread syscall ordering (enables the n-gram detector).
    ngram_sizes:
        n-gram widths extracted from the per-thread sequences.
    timeout_s:
        Per-run wall-clock ceiling.
    """

    reps: int = 3
    wrapper: tuple[str, ...] = ("unshare", "-Ur")
    strace_bin: str = "strace"
    capture_args: bool = True
    arg_syscalls: tuple[str, ...] = (
        "openat",
        "open",
        "stat",
        "newfstatat",
        "lstat",
        "access",
        "readlink",
        "execve",
        "connect",
        "sendto",
        "bind",
        "unlink",
        "unlinkat",
        "rename",
        "renameat",
        "mkdir",
        "mkdirat",
        "chmod",
        "fchmodat",
        "socket",
    )
    capture_sequence: bool = True
    ngram_sizes: tuple[int, ...] = (3, 4, 5)
    timeout_s: int = 600


@dataclass(frozen=True)
class BenchSettings:
    """Compile-time / exec-time / size measurement."""

    reps: int = 3
    exec_timeout_s: int = 300
    #: Verify the binary exits 0 and reproduces its embedded expected output
    #: before its timings are recorded. A crashing binary makes *fewer*
    #: syscalls and would otherwise look "close to baseline".
    verify_output: bool = True
    #: Record timings even when verification fails (the row is marked
    #: ``verified=False``). Useful for diagnosing a broken variant.
    keep_unverified: bool = False


@dataclass(frozen=True)
class ControlSettings:
    """The nested baseline control design: ``builds`` x ``passes``.

    Attributes
    ----------
    builds:
        Independent compilations of the same source with the baseline compiler.
        ``1`` disables the rebuild control entirely. ``2`` is the minimum that
        detects build nondeterminism at all; ``3`` or more is the minimum that
        says anything about how *reproducible* it is, which is what the per-file
        build floor in :mod:`..stats.noise` actually needs.
    passes:
        Independent trace sessions per build. ``1`` disables the rerun control.
        ``2`` is the minimum that separates run noise from build noise.

    Together these produce ``builds * passes`` baseline sample groups, each of
    ``trace.reps`` traces. The cost is linear in the product, so raising both is
    the expensive axis -- see the budget printed by ``preflight``.
    """

    builds: int = 2
    passes: int = 2

    def __post_init__(self) -> None:
        if self.builds < 1 or self.passes < 1:
            raise ConfigError(
                f"controls.builds and controls.passes must be >= 1 "
                f"(got builds={self.builds}, passes={self.passes})"
            )

    @property
    def rerun_enabled(self) -> bool:
        """Whether any same-binary re-trace happens."""
        return self.passes > 1

    @property
    def rebuild_enabled(self) -> bool:
        """Whether any recompilation of the same source happens."""
        return self.builds > 1

    @property
    def n_groups(self) -> int:
        """Baseline sample groups, including the baseline itself."""
        return self.builds * self.passes


@dataclass(frozen=True)
class DetectSettings:
    """Thresholds for the departure detectors and the taxonomy.

    Attributes
    ----------
    count_tol:
        A per-syscall mean-count delta above this (and above the file's own
        noise floor) counts as a count departure.
    auto_noisy_min_files:
        A syscall differing between control groups in at least this many files
        is classed as
        noise and excluded from significance testing.
    uniform_threshold:
        Fraction of *affected* files that must share an identical delta for the
        departure to be classed ``uniform`` rather than ``conditional``.
    program_dependent_rho:
        |Spearman rho| between per-file delta magnitude and program size above
        which a departure is classed ``program_dependent``
    excluded_syscalls:
        Dropped from *count* statistics entirely. Kept empty by default: a
        syscall that differs on every file looks like a nuisance but *is* the
        finding, and excluding it destroys the evidence for it. The taxonomy
        files such a departure under ``uniform``, where it belongs.
    departure_ignore:
        Syscalls excluded from the *departure detector* only (ASLR / allocator
        / scheduler churn). Distinct from the noise classification above.
    """

    count_tol: float = 0.5
    auto_noisy_min_files: int = 3
    uniform_threshold: float = 0.95
    program_dependent_rho: float = 0.5
    excluded_syscalls: tuple[str, ...] = ()
    departure_ignore: tuple[str, ...] = (
        "mmap",
        "munmap",
        "mprotect",
        "brk",
        "mremap",
        "futex",
        "getrandom",
        "clone",
        "clone3",
        "set_robust_list",
        "rt_sigprocmask",
        "sched_getaffinity",
    )
    #: Regexes applied in order to trace arguments before comparison, to strip
    #: run-varying content (pids, tmp paths, addresses).
    arg_normalizers: tuple[tuple[str, str], ...] = (
        (r"/proc/\d+/", "/proc/<PID>/"),
        (r"/tmp/[A-Za-z0-9._-]{6,}", "/tmp/<RANDOM>"),
        (r"0x[0-9a-f]{6,}", "<ADDR>"),
        (r"\b\d{4,}\b", "<NUM>"),
    )


@dataclass(frozen=True)
class PathSettings:
    """Filesystem layout. All paths are resolved against ``project_root``."""

    project_root: str = "."
    corpus_dir: str = "src/codebase"
    work_dir: str = "build/compilerdiv_work"
    raw_dir: str = "results/raw"
    out_dir: str = "results"
    keep_binaries: bool = False


@dataclass(frozen=True)
class Settings:
    """Top-level settings object."""

    baseline: str
    compilers: tuple[CompilerSpec, ...]
    configs: tuple[BuildConfig, ...]
    paths: PathSettings = field(default_factory=PathSettings)
    trace: TraceSettings = field(default_factory=TraceSettings)
    bench: BenchSettings = field(default_factory=BenchSettings)
    controls: ControlSettings = field(default_factory=ControlSettings)
    detect: DetectSettings = field(default_factory=DetectSettings)
    #: Recognised source languages. Defaults to the built-in compiled-language
    #: set; the YAML ``languages:`` section overrides/extends it by extension.
    languages: tuple[LanguageSpec, ...] = BUILTIN_LANGUAGES
    flush_every: int = 20

    # -- languages ----------------------------------------------------------

    def language_for(self, path_or_ext: str | Path) -> LanguageSpec | None:
        """The language whose extension matches, or ``None`` if unrecognised."""
        s = str(path_or_ext)
        ext = s if s.startswith(".") and "/" not in s else Path(s).suffix
        return language_registry(self.languages).get(ext)

    def corpus_languages(self) -> list[LanguageSpec]:
        """Languages actually present in ``corpus_dir``, detected by extension.

        This is the "detect the targeted language" step: the harness looks at
        which recognised extensions exist on disk rather than being told.
        """
        by_ext = language_registry(self.languages)
        exts: set[str] = set()
        if self.corpus_path.is_dir():
            for p in self.corpus_path.iterdir():
                if p.is_file() and p.suffix in by_ext:
                    exts.add(p.suffix)
        # One entry per *language*, even when several of its extensions are
        # present (C++ spans .cpp/.cc/.cxx). The first extension in sorted order
        # supplies the representative spec; for a built-in they are identical.
        seen: dict[str, LanguageSpec] = {}
        for e in sorted(exts):
            spec = by_ext[e]
            seen.setdefault(spec.name, spec)
        return list(seen.values())

    def n_corpus_files(self) -> int:
        """Recognised source files on disk. Counted, not parsed -- this only
        feeds the preflight budget estimate."""
        by_ext = language_registry(self.languages)
        if not self.corpus_path.is_dir():
            return 0
        return sum(
            1 for p in self.corpus_path.iterdir() if p.is_file() and p.suffix in by_ext
        )

    # -- derived accessors --------------------------------------------------

    @property
    def root(self) -> Path:
        return Path(self.paths.project_root).resolve()

    @property
    def corpus_path(self) -> Path:
        return self.root / self.paths.corpus_dir

    @property
    def work_path(self) -> Path:
        return self.root / self.paths.work_dir

    @property
    def raw_path(self) -> Path:
        return self.root / self.paths.raw_dir

    @property
    def out_path(self) -> Path:
        return self.root / self.paths.out_dir

    def compiler(self, key: str) -> CompilerSpec:
        for c in self.compilers:
            if c.key == key:
                return c
        raise KeyError(f"unknown compiler key: {key!r}")

    def label(self, key: str) -> str:
        """Human label for a compiler key *or* a generated control tag."""
        coord = parse_control_tag(key)
        if coord is None:
            return self.compiler(key).label
        build, pass_ = coord
        base = self.compiler(self.baseline).label
        if build == 0:
            return f"{base} (rerun {pass_})"
        if pass_ == 0:
            return f"{base} (build {build})"
        return f"{base} (build {build}, rerun {pass_})"

    def color(self, key: str) -> str:
        coord = parse_control_tag(key)
        if coord is None:
            return self.compiler(key).color
        # Reruns of the primary build are the lightest -- they are the least
        # interesting control. Later builds darken so they stay separable.
        build, _ = coord
        return "#8C8C8C" if build == 0 else "#B0B0B0"

    @property
    def compiler_keys(self) -> list[str]:
        return [c.key for c in self.compilers]

    @property
    def variant_keys(self) -> list[str]:
        """Non-baseline compilers -- the ones compared against A."""
        return [c.key for c in self.compilers if c.key != self.baseline]

    def config(self, name: str) -> BuildConfig:
        for c in self.configs:
            if c.name == name:
                return c
        raise KeyError(f"unknown config: {name!r}")

    @property
    def config_names(self) -> list[str]:
        return [c.name for c in self.configs]

    def compilers_for(self, config_name: str) -> list[str]:
        """Compiler keys that can build this config."""
        unsupported = set(self.config(config_name).unsupported)
        return [c.key for c in self.compilers if c.key not in unsupported]

    # -- control tags -------------------------------------------------------
    #
    # Every site that needs the control structure asks here. Nothing outside
    # this class parses a tag string.

    def tag(self, build: int, pass_: int) -> str:
        """Tag for one (build, pass) group of the baseline."""
        return control_tag(self.baseline, build, pass_)

    def tag_coord(self, tag: str) -> tuple[int, int]:
        """``(build, pass)`` for any baseline tag, including the baseline."""
        return parse_control_tag(tag) or (0, 0)

    def tag_build(self, tag: str) -> int:
        return self.tag_coord(tag)[0]

    def tag_pass(self, tag: str) -> int:
        return self.tag_coord(tag)[1]

    def baseline_tags(self) -> list[str]:
        """Every baseline sample group, ``A`` first. The full baseline sample.

        This -- not ``baseline`` alone -- is what a variant should be compared
        against. Comparing against ``A`` only would use one group out of
        ``builds * passes``, and report the baseline's own build quirks as
        departures of the variant.
        """
        return [
            self.tag(b, p)
            for b in range(self.controls.builds)
            for p in range(self.controls.passes)
        ]

    def control_tags(self) -> list[str]:
        """Baseline sample groups other than the baseline itself."""
        return [t for t in self.baseline_tags() if t != self.baseline]

    def pass_tags(self, build: int) -> list[str]:
        """Every trace session of one build. Differences here are run noise."""
        return [self.tag(build, p) for p in range(self.controls.passes)]

    def rerun_tags(self, build: int = 0) -> list[str]:
        """The non-primary trace sessions of one build."""
        return [self.tag(build, p) for p in range(1, self.controls.passes)]

    def build_tags(self) -> list[str]:
        """One representative tag per build. Differences here are build noise."""
        return [self.tag(b, 0) for b in range(self.controls.builds)]

    def trace_budget(self) -> dict[str, int]:
        """Per-program measurement counts, for the preflight report."""
        n_variants = len(self.variant_keys)
        baseline_traces = self.controls.n_groups * self.trace.reps
        return {
            "baseline_groups": self.controls.n_groups,
            "baseline_compiles": self.controls.builds,
            "baseline_traces": baseline_traces,
            "variant_traces": n_variants * self.trace.reps,
            "traces_per_program": baseline_traces + n_variants * self.trace.reps,
        }

    # -- fingerprint --------------------------------------------------------

    def fingerprint(self) -> str:
        """Hash of every setting that affects the *contents* of the raw log.

        Presentation-only settings (colours, labels, plot thresholds) are
        excluded, so re-running the analysis with a different
        ``program_dependent_rho`` does not invalidate a multi-hour sweep.
        """
        payload = {
            "baseline": self.baseline,
            "compilers": [
                {
                    "key": c.key,
                    "cmd": list(c.cmd),
                    "cwd": c.cwd,
                    "src_first": c.src_first,
                }
                for c in sorted(self.compilers, key=lambda x: x.key)
            ],
            "configs": [
                {
                    "name": c.name,
                    "flags": list(c.flags),
                    "unsupported": sorted(c.unsupported),
                }
                for c in sorted(self.configs, key=lambda x: x.name)
            ],
            "trace": {
                "reps": self.trace.reps,
                "wrapper": list(self.trace.wrapper),
                "capture_args": self.trace.capture_args,
                "arg_syscalls": sorted(self.trace.arg_syscalls),
                "capture_sequence": self.trace.capture_sequence,
                "ngram_sizes": sorted(self.trace.ngram_sizes),
            },
            "bench": {
                "reps": self.bench.reps,
                "verify_output": self.bench.verify_output,
            },
            "controls": {
                "builds": self.controls.builds,
                "passes": self.controls.passes,
            },
        }
        blob = json.dumps(payload, sort_keys=True, separators=(",", ":"))
        return hashlib.sha256(blob.encode()).hexdigest()[:16]

    def to_dict(self) -> dict[str, Any]:
        return asdict(self)


# ---------------------------------------------------------------------------
# Loading
# ---------------------------------------------------------------------------


def _as_tuple(v, default=()):
    if v is None:
        return tuple(default)
    if isinstance(v, (list, tuple)):
        return tuple(v)
    return (v,)


def _load_compilers(raw: list[dict]) -> tuple[CompilerSpec, ...]:
    out = []
    seen = set()
    for entry in raw:
        for req in ("key", "label", "cmd"):
            if req not in entry:
                raise ConfigError(f"compiler entry missing {req!r}: {entry}")
        key = str(entry["key"])
        if TAG_SEP in key:
            raise ConfigError(
                f"compiler key {key!r} may not contain {TAG_SEP!r}: that "
                "separator is reserved for the generated control tags "
                "(e.g. 'A@1.2' = build 1, rerun 2 of the baseline)"
            )
        if key in seen:
            raise ConfigError(f"duplicate compiler key: {key!r}")
        seen.add(key)
        out.append(
            CompilerSpec(
                key=key,
                label=str(entry["label"]),
                cmd=_as_tuple(entry["cmd"]),
                cwd=entry.get("cwd"),
                src_first=bool(entry.get("src_first", False)),
                output_flag=str(entry.get("output_flag", "-o")),
                color=str(entry.get("color", "#4C72B0")),
            )
        )
    if not out:
        raise ConfigError("no compilers configured")
    return tuple(out)


def _load_languages(raw: list[dict]) -> tuple[LanguageSpec, ...]:
    """Merge configured languages over the built-ins, keyed by extension.

    An entry sharing a built-in's extension overrides it; a new extension
    extends the set. Absent or empty config leaves the built-ins untouched.
    """
    by_ext: dict[str, LanguageSpec] = {s.extension: s for s in BUILTIN_LANGUAGES}
    for entry in raw or []:
        for req in ("name", "extension"):
            if req not in entry:
                raise ConfigError(f"language entry missing {req!r}: {entry}")
        ext = str(entry["extension"])
        if not ext.startswith("."):
            ext = "." + ext
        block = entry.get("block_comment")
        if block is not None:
            block_t = tuple(block)
            if len(block_t) != 2:
                raise ConfigError(
                    f"language {entry['name']!r}: block_comment must be a "
                    f"[open, close] pair, got {block!r}"
                )
        else:
            block_t = None
        by_ext[ext] = LanguageSpec(
            name=str(entry["name"]),
            extension=ext,
            line_comment=str(entry.get("line_comment", "//")),
            block_comment=block_t,
            hello_world=str(entry.get("hello_world", "")),
        )
    return tuple(by_ext.values())


def _load_configs(raw: list[dict]) -> tuple[BuildConfig, ...]:
    out = []
    for entry in raw:
        if "name" not in entry:
            raise ConfigError(f"config entry missing 'name': {entry}")
        out.append(
            BuildConfig(
                name=str(entry["name"]),
                flags=_as_tuple(entry.get("flags")),
                unsupported=_as_tuple(entry.get("unsupported")),
            )
        )
    if not out:
        raise ConfigError("no build configs configured")
    return tuple(out)


def _legacy_flag(raw, default: bool) -> bool:
    """Read a control that used to be a bool or a ``{enabled: bool}`` mapping."""
    if isinstance(raw, dict):
        return bool(raw.get("enabled", default))
    return bool(raw) if raw is not None else default


def _parse_controls(c: dict) -> ControlSettings:
    """Parse the ``controls`` block, accepting the pre-nesting spelling.

    The old form named the two controls and toggled them:
    ``rerun: true`` / ``rebuild: false``. The new form counts them. A bool maps
    onto the count that reproduces its meaning -- one extra pass, or one extra
    build -- so an untouched config keeps working and means the same thing.
    Explicit counts win where both are given.
    """
    cdef = ControlSettings()
    if "builds" in c or "passes" in c:
        builds = int(c.get("builds", cdef.builds))
        passes = int(c.get("passes", cdef.passes))
    else:
        builds = 2 if _legacy_flag(c.get("rebuild"), True) else 1
        passes = 2 if _legacy_flag(c.get("rerun"), True) else 1
    return ControlSettings(builds=builds, passes=passes)


def load_settings(path: str | Path) -> Settings:
    """Load and validate a YAML configuration file."""
    p = Path(path)
    if not p.is_file():
        raise ConfigError(f"config file not found: {p}")
    with p.open(encoding="utf-8") as fh:
        doc = yaml.safe_load(fh) or {}

    compilers = _load_compilers(doc.get("compilers", []))
    configs = _load_configs(doc.get("configs", []))
    languages = _load_languages(doc.get("languages", []))
    baseline = doc.get("baseline")
    if baseline is None:
        raise ConfigError("'baseline' must be set")
    if baseline not in {c.key for c in compilers}:
        raise ConfigError(f"baseline {baseline!r} is not a configured compiler")

    paths_raw = doc.get("paths", {}) or {}
    # Relative project_root is resolved against the config file's directory,
    # so a config can sit anywhere and still point at the repo.
    proot = paths_raw.get("project_root", ".")
    if not Path(proot).is_absolute():
        proot = str((p.parent / proot).resolve())
    paths = PathSettings(
        project_root=proot,
        corpus_dir=paths_raw.get("corpus_dir", "src/codebase"),
        work_dir=paths_raw.get("work_dir", "build/compilerdiv_work"),
        raw_dir=paths_raw.get("raw_dir", "results/raw"),
        out_dir=paths_raw.get("out_dir", "results"),
        keep_binaries=bool(paths_raw.get("keep_binaries", False)),
    )

    t = doc.get("trace", {}) or {}
    defaults = TraceSettings()
    trace = TraceSettings(
        reps=int(t.get("reps", defaults.reps)),
        wrapper=(
            _as_tuple(t.get("wrapper"), defaults.wrapper)
            if "wrapper" in t
            else defaults.wrapper
        ),
        strace_bin=str(t.get("strace_bin", defaults.strace_bin)),
        capture_args=bool(t.get("capture_args", defaults.capture_args)),
        arg_syscalls=(
            _as_tuple(t.get("arg_syscalls"), defaults.arg_syscalls)
            if "arg_syscalls" in t
            else defaults.arg_syscalls
        ),
        capture_sequence=bool(t.get("capture_sequence", defaults.capture_sequence)),
        ngram_sizes=tuple(int(x) for x in t.get("ngram_sizes", defaults.ngram_sizes)),
        timeout_s=int(t.get("timeout_s", defaults.timeout_s)),
    )

    b = doc.get("bench", {}) or {}
    bdef = BenchSettings()
    bench = BenchSettings(
        reps=int(b.get("reps", bdef.reps)),
        exec_timeout_s=int(b.get("exec_timeout_s", bdef.exec_timeout_s)),
        verify_output=bool(b.get("verify_output", bdef.verify_output)),
        keep_unverified=bool(b.get("keep_unverified", bdef.keep_unverified)),
    )

    controls = _parse_controls(doc.get("controls", {}) or {})

    d = doc.get("detect", {}) or {}
    ddef = DetectSettings()
    norms = d.get("arg_normalizers")
    if norms is not None:
        arg_normalizers = tuple((str(x["pattern"]), str(x["replace"])) for x in norms)
    else:
        arg_normalizers = ddef.arg_normalizers
    detect = DetectSettings(
        count_tol=float(d.get("count_tol", ddef.count_tol)),
        auto_noisy_min_files=int(
            d.get("auto_noisy_min_files", ddef.auto_noisy_min_files)
        ),
        uniform_threshold=float(d.get("uniform_threshold", ddef.uniform_threshold)),
        program_dependent_rho=float(
            d.get("program_dependent_rho", ddef.program_dependent_rho)
        ),
        excluded_syscalls=(
            _as_tuple(d.get("excluded_syscalls"), ddef.excluded_syscalls)
            if "excluded_syscalls" in d
            else ddef.excluded_syscalls
        ),
        departure_ignore=(
            _as_tuple(d.get("departure_ignore"), ddef.departure_ignore)
            if "departure_ignore" in d
            else ddef.departure_ignore
        ),
        arg_normalizers=arg_normalizers,
    )

    return Settings(
        baseline=str(baseline),
        compilers=compilers,
        configs=configs,
        paths=paths,
        trace=trace,
        bench=bench,
        controls=controls,
        detect=detect,
        languages=languages,
        flush_every=int(doc.get("flush_every", 20)),
    )

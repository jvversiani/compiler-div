"""Command-line interface.

Acquisition and analysis are separate commands on purpose. A sweep takes hours;
tuning a threshold or a plot takes seconds. The raw store is the boundary.

    compilerdiv preflight              # check strace and the compilers, then stop
    compilerdiv acquire                # sweep (resumable)
    compilerdiv acquire --start Foo    # resume the corpus from a given program
    compilerdiv acquire --reset        # discard the raw log and start over
    compilerdiv analyze                # reports + figures from the raw log
    compilerdiv all                    # preflight, acquire, analyze
    compilerdiv corpus                 # corpus stats + SLOC figure
    compilerdiv doctor                 # what is in the raw store
"""

from __future__ import annotations

import argparse
import sys
from pathlib import Path

from ..analyze import analyze
from ..config import ConfigError, Settings, load_settings
from ..corpus import MixedCorpusError, load_corpus
from ..store.raw import FingerprintMismatch, RawStore

DEFAULT_CONFIG = "compilerdiv.yaml"


def _load(args) -> Settings:
    try:
        return load_settings(args.config)
    except ConfigError as e:
        sys.exit(f"config error: {e}")


def _store(settings: Settings) -> RawStore:
    return RawStore(settings.raw_path, settings.fingerprint())


def cmd_preflight(args) -> int:
    from ..acquire.preflight import preflight

    settings = _load(args)
    rep = preflight(settings)
    print(rep.render())
    return 0 if rep.ok else 1


def cmd_acquire(args) -> int:
    from ..acquire.preflight import preflight
    from ..acquire.sweep import Sweeper

    settings = _load(args)

    if not args.skip_preflight:
        rep = preflight(settings)
        print(rep.render())
        if not rep.ok:
            return 1

    store = _store(settings)
    try:
        store.init(settings, reset=args.reset)
    except FingerprintMismatch as e:
        sys.exit(f"\n{e}\n")

    sweeper = Sweeper(settings, store, verbose=not args.quiet)
    try:
        stats = sweeper.run(start=args.start)
    except KeyboardInterrupt:
        store.flush()
        print(
            "\n[interrupted] buffered rows flushed; rerun `compilerdiv acquire` to resume."
        )
        return 130

    print(
        f"\nAcquisition done: compiled={stats.compiled} traced={stats.traced} "
        f"compile_failed={stats.compile_failed} unverified={stats.unverified} "
        f"trace_failed={stats.trace_failed}"
    )
    print(f"Raw store: {store.root}")
    return 0


def cmd_analyze(args) -> int:
    settings = _load(args)
    store = _store(settings)
    if not store.manifest_path().is_file():
        sys.exit(f"no raw store at {store.root}. Run `compilerdiv acquire` first.")
    try:
        store.init(settings, reset=False)
    except FingerprintMismatch as e:
        sys.exit(f"\n{e}\n")

    res = analyze(settings, store, verbose=not args.quiet)
    print(f"\nWrote {len(res.figures)} figures under {settings.out_path / 'figures'}")
    for wb in res.workbooks:
        print(f"  {wb}")
    return 0


def cmd_all(args) -> int:
    rc = cmd_acquire(args)
    if rc != 0:
        return rc
    return cmd_analyze(args)


def cmd_corpus(args) -> int:
    from ..plots.bench import plot_sloc_density

    settings = _load(args)
    programs = load_corpus(settings.corpus_path, settings.languages)
    total = sum(p.sloc for p in programs)
    checkable = sum(1 for p in programs if p.checkable)
    langs = settings.corpus_languages()
    if langs:
        print(f"Languages       : {', '.join(sorted(l.name for l in langs))}")
    print(f"Programs        : {len(programs)}")
    print(f"Total SLOC      : {total}")
    print(f"Mean SLOC       : {total / len(programs):.1f}")
    print(
        f"Min / Max SLOC  : {min(p.sloc for p in programs)} / {max(p.sloc for p in programs)}"
    )
    print(f"With expected   : {checkable}  (usable as integration tests)")
    missing = [p.stem for p in programs if not p.checkable]
    if missing:
        print(f"Without expected: {len(missing)}")
        for m in missing[:20]:
            print(f"  - {m}")
        if len(missing) > 20:
            print(f"  ... and {len(missing) - 20} more")

    p = plot_sloc_density(programs, settings.out_path / "figures" / "corpus")
    if p:
        print(f"\nWrote {p}")
    return 0


def cmd_doctor(args) -> int:
    from ..store.raw import storage_backend

    settings = _load(args)
    store = _store(settings)
    backend = storage_backend()
    print(f"Config       : {Path(args.config).resolve()}")
    print(f"Fingerprint  : {settings.fingerprint()}")
    print(f"Raw store    : {store.root}")
    print(f"Storage      : {backend}")
    if backend == "csv":
        print(
            "  [note] pyarrow is not installed, so the raw store falls back to CSV.\n"
            "         A full sweep produces tens of millions of rows; install the\n"
            "         extra for much smaller files and faster resume:\n"
            "           pip install 'compilerdiv[fast]'"
        )
    if not store.manifest_path().is_file():
        print("  (no manifest -- nothing acquired yet)")
        return 0

    import json

    m = json.loads(store.manifest_path().read_text())
    match = m.get("fingerprint") == settings.fingerprint()
    print(
        f"  stored fingerprint: {m.get('fingerprint')} "
        f"{'(matches)' if match else '(MISMATCH -- resume would be refused)'}"
    )
    print(f"  created           : {m.get('created')}")

    from ..store.raw import ARGS, BENCH, COUNTS, ELF, ERRORS, SEQUENCE

    for stream in (COUNTS, SEQUENCE, ARGS, BENCH, ELF, ERRORS):
        parts = store.parts(stream)
        if not parts:
            print(f"  {stream:9s}: empty")
            continue
        df = store.read(stream)
        size_mb = sum(p.stat().st_size for p in parts) / 1e6
        print(
            f"  {stream:9s}: {len(df):>9,} rows  {len(parts):>3} parts  {size_mb:6.1f} MB"
        )

    state = store.resume_state()
    print(f"\n  bench rows done  : {len(state.bench_done)}")
    print(f"  traced keys      : {len(state.trace_reps)}")
    return 0


def build_parser() -> argparse.ArgumentParser:
    p = argparse.ArgumentParser(
        prog="compilerdiv",
        description="Behavioral equivalence analysis of diverse compilers.",
    )
    p.add_argument(
        "-c",
        "--config",
        default=DEFAULT_CONFIG,
        help=f"YAML config (default: {DEFAULT_CONFIG})",
    )
    p.add_argument("-q", "--quiet", action="store_true", help="less terminal output")
    sub = p.add_subparsers(dest="cmd", required=True)

    sp = sub.add_parser("preflight", help="check strace and the compilers")
    sp.set_defaults(func=cmd_preflight)

    sa = sub.add_parser("acquire", help="run the sweep (resumable)")
    sa.add_argument("--start", default=None, help="resume the corpus from this program")
    sa.add_argument(
        "--reset", action="store_true", help="delete the raw log and start over"
    )
    sa.add_argument("--skip-preflight", action="store_true")
    sa.set_defaults(func=cmd_acquire)

    sn = sub.add_parser("analyze", help="reports and figures from the raw log")
    sn.set_defaults(func=cmd_analyze)

    sl = sub.add_parser("all", help="acquire then analyze")
    sl.add_argument("--start", default=None)
    sl.add_argument("--reset", action="store_true")
    sl.add_argument("--skip-preflight", action="store_true")
    sl.set_defaults(func=cmd_all)

    sc = sub.add_parser("corpus", help="corpus stats and the SLOC figure")
    sc.set_defaults(func=cmd_corpus)

    sd = sub.add_parser("doctor", help="inspect the raw store")
    sd.set_defaults(func=cmd_doctor)

    return p


def main(argv: list[str] | None = None) -> int:
    args = build_parser().parse_args(argv)
    try:
        return args.func(args)
    except MixedCorpusError as e:
        # A configuration error, not a crash: print it plainly, no traceback.
        print(f"\nerror: {e}\n", file=sys.stderr)
        return 2


if __name__ == "__main__":
    sys.exit(main())

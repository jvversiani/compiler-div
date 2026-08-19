"""CLI argument parsing and command wiring.

Only the parser is exercised here -- it is pure and fast. The commands
themselves shell out to compilers/strace and are covered by the acquisition
integration path, not unit tests.
"""

from __future__ import annotations

import importlib

import pytest

from compilerdiv.cli.main import (
    DEFAULT_CONFIG,
    build_parser,
    cmd_acquire,
    cmd_analyze,
    cmd_corpus,
    cmd_doctor,
    cmd_preflight,
    main,
)

pytestmark = pytest.mark.cli

cli = importlib.import_module("compilerdiv.cli.main")  # module, not re-exported main()
preflight_mod = importlib.import_module("compilerdiv.acquire.preflight")
sweep_mod = importlib.import_module("compilerdiv.acquire.sweep")


class TestParser:
    def test_default_config(self):
        args = build_parser().parse_args(["preflight"])
        assert args.config == DEFAULT_CONFIG
        assert args.func is cmd_preflight

    def test_config_flag(self):
        args = build_parser().parse_args(["-c", "my.yaml", "doctor"])
        assert args.config == "my.yaml"
        assert args.func is cmd_doctor

    def test_quiet_flag(self):
        args = build_parser().parse_args(["-q", "analyze"])
        assert args.quiet is True
        assert args.func is cmd_analyze

    def test_subcommand_required(self):
        with pytest.raises(SystemExit):
            build_parser().parse_args([])

    def test_unknown_subcommand_rejected(self):
        with pytest.raises(SystemExit):
            build_parser().parse_args(["frobnicate"])

    @pytest.mark.parametrize(
        "cmd,func",
        [
            ("preflight", cmd_preflight),
            ("acquire", cmd_acquire),
            ("analyze", cmd_analyze),
            ("corpus", cmd_corpus),
            ("doctor", cmd_doctor),
        ],
    )
    def test_every_subcommand_wires_a_func(self, cmd, func):
        args = build_parser().parse_args([cmd])
        assert args.func is func


class TestAcquireArgs:
    def test_start_and_reset(self):
        args = build_parser().parse_args(["acquire", "--start", "Foo", "--reset"])
        assert args.start == "Foo"
        assert args.reset is True
        assert args.skip_preflight is False

    def test_skip_preflight(self):
        args = build_parser().parse_args(["acquire", "--skip-preflight"])
        assert args.skip_preflight is True

    def test_acquire_defaults(self):
        args = build_parser().parse_args(["acquire"])
        assert args.start is None
        assert args.reset is False


# ---------------------------------------------------------------------------
# The commands themselves, with heavy callees stubbed
# ---------------------------------------------------------------------------


CONFIG_TEMPLATE = """\
baseline: A
paths:
  project_root: {root}
  corpus_dir: corpus
  raw_dir: raw
  out_dir: out
compilers:
  - {{key: A, label: gcc, cmd: [gcc]}}
  - {{key: B, label: clang, cmd: [clang]}}
configs:
  - {{name: basic}}
trace:
  wrapper: []
  strace_bin: "true"
"""


@pytest.fixture
def cli_config(tmp_path):
    """Write a runnable config plus a tiny corpus; return the config path."""
    corpus = tmp_path / "corpus"
    corpus.mkdir()
    for i in range(2):
        (corpus / f"p{i}.rs").write_text(
            "// =======================\n// Expected output:\n// ok\n"
            "// =======================\nfn main() {}\n"
        )
    cfg = tmp_path / "cfg.yaml"
    cfg.write_text(CONFIG_TEMPLATE.format(root=str(tmp_path)))
    return cfg


def _run(cfg, *argv):
    args = build_parser().parse_args(["-c", str(cfg), *argv])
    return args.func(args)


class TestCorpusCommand:
    def test_corpus_runs(self, cli_config, capsys):
        rc = _run(cli_config, "corpus")
        assert rc == 0
        out = capsys.readouterr().out
        assert "Programs" in out
        assert "rust" in out  # language detection line


class TestDoctorCommand:
    def test_doctor_no_manifest(self, cli_config, capsys):
        rc = _run(cli_config, "doctor")
        assert rc == 0
        assert "no manifest" in capsys.readouterr().out

    def test_doctor_with_data(self, cli_config, capsys):
        from compilerdiv.config import load_settings
        from compilerdiv.store.raw import BENCH, COUNTS, RawStore

        s = load_settings(cli_config)
        store = RawStore(s.raw_path, s.fingerprint())
        store.init(s)
        store.add(COUNTS, ["basic", "A", "p0", 1, "read", 4])
        store.add(BENCH, ["basic", "A", "p0", 1, 0.5, 0.01, 100, True, 0])
        store.flush()

        rc = _run(cli_config, "doctor")
        assert rc == 0
        out = capsys.readouterr().out
        assert "Fingerprint" in out
        assert "matches" in out


class TestPreflightCommand:
    def test_preflight_ok(self, cli_config, monkeypatch):
        rep = preflight_mod.PreflightReport(ok=True, lines=["=== ok ==="], failures=[])
        monkeypatch.setattr(preflight_mod, "preflight", lambda settings: rep)
        assert _run(cli_config, "preflight") == 0

    def test_preflight_failure_returns_1(self, cli_config, monkeypatch):
        rep = preflight_mod.PreflightReport(
            ok=False, lines=[], failures=[("x", "boom")]
        )
        monkeypatch.setattr(preflight_mod, "preflight", lambda settings: rep)
        assert _run(cli_config, "preflight") == 1


class TestAcquireCommand:
    def _stub(self, monkeypatch):
        rep = preflight_mod.PreflightReport(ok=True, lines=["ok"], failures=[])
        monkeypatch.setattr(preflight_mod, "preflight", lambda settings: rep)

        class FakeSweeper:
            def __init__(self, settings, store, *, verbose=True):
                self.store = store

            def run(self, start=None):
                return sweep_mod.SweepStats(compiled=2, traced=6)

        monkeypatch.setattr(sweep_mod, "Sweeper", FakeSweeper)

    def test_acquire_runs(self, cli_config, monkeypatch, capsys):
        self._stub(monkeypatch)
        rc = _run(cli_config, "acquire")
        assert rc == 0
        assert "Acquisition done" in capsys.readouterr().out

    def test_acquire_stops_on_preflight_failure(self, cli_config, monkeypatch):
        rep = preflight_mod.PreflightReport(ok=False, lines=[], failures=[("x", "y")])
        monkeypatch.setattr(preflight_mod, "preflight", lambda settings: rep)
        assert _run(cli_config, "acquire") == 1

    def test_acquire_skip_preflight(self, cli_config, monkeypatch):
        self._stub(monkeypatch)
        assert _run(cli_config, "acquire", "--skip-preflight") == 0


class TestAnalyzeCommand:
    def test_analyze_without_store_exits(self, cli_config):
        with pytest.raises(SystemExit):
            _run(cli_config, "analyze")

    def test_analyze_runs_with_stub(self, cli_config, monkeypatch, capsys):
        from compilerdiv.analyze import AnalysisResult
        from compilerdiv.config import load_settings
        from compilerdiv.store.raw import RawStore

        s = load_settings(cli_config)
        RawStore(s.raw_path, s.fingerprint()).init(s)  # creates the manifest

        monkeypatch.setattr(
            cli, "analyze", lambda settings, store, verbose=True: AnalysisResult()
        )
        rc = _run(cli_config, "analyze")
        assert rc == 0


class TestMainDispatch:
    def test_main_returns_command_rc(self, cli_config, monkeypatch):
        monkeypatch.setattr(
            preflight_mod,
            "preflight",
            lambda settings: preflight_mod.PreflightReport(True, ["ok"], []),
        )
        rc = main(["-c", str(cli_config), "preflight"])
        assert rc == 0

    def test_module_entrypoint(self, monkeypatch):
        """`python -m compilerdiv` with no subcommand exits via argparse."""
        import runpy
        import sys

        monkeypatch.setattr(sys, "argv", ["compilerdiv"])
        with pytest.raises(SystemExit):
            runpy.run_module("compilerdiv", run_name="__main__")

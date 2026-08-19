"""Corpus parsing, statistics, config validation, noise floors.

Organised OOP: one ``Test*`` class per concern, each tagged with a marker so it
can be selected from the command line (``pytest -m corpus``, ``-m config`` ...).
"""

from __future__ import annotations

import numpy as np
import pandas as pd
import pytest

from compilerdiv.config import (
    BuildConfig,
    CompilerSpec,
    ConfigError,
    ControlSettings,
    DetectSettings,
    Settings,
    TraceSettings,
    load_settings,
)
from compilerdiv.corpus import (
    Program,
    check_output,
    count_sloc,
    display_name,
    normalize_ws,
    parse_expected,
    truncate,
)
from compilerdiv.stats.equivalence import benjamini_hochberg, js_divergence
from compilerdiv.stats.noise import derive_noise

from conftest import FakeFrames, _counts_frame

HEADER = """\
// Rosetta Code task: Abbreviations, easy
// Source: rosettacode.org/wiki/Abbreviations,_easy
// Content licensed under GFDL 1.2 (Rosetta Code).
// =======================
// Expected output:
// RIGHT REPEAT *error* PUT
// MOVE RESTORE
// =======================
fn main() { println!("RIGHT REPEAT *error* PUT"); }
"""


def _prog(expected):
    return Program(
        stem="p", path=None, expected=expected, has_block=True, sentinel=False, sloc=1
    )


# ---------------------------------------------------------------------------
# Corpus
# ---------------------------------------------------------------------------


@pytest.mark.corpus
class TestExpectedOutputParsing:
    def test_extracts_block(self):
        lines, has_block, sentinel = parse_expected(HEADER)
        assert has_block
        assert not sentinel
        assert lines == ["RIGHT REPEAT *error* PUT", "MOVE RESTORE"]

    def test_sentinel_detected(self):
        text = (
            "// =======================\n"
            "// Expected output:\n"
            "// (no expected output provided on Rosetta Code)\n"
            "// =======================\n"
        )
        lines, has_block, sentinel = parse_expected(text)
        assert has_block and sentinel

    def test_no_block(self):
        lines, has_block, sentinel = parse_expected("fn main() {}\n")
        assert not has_block
        assert lines == []

    def test_stops_at_closing_delimiter(self):
        text = HEADER + "// Expected output:\n// SHOULD NOT APPEAR\n"
        lines, _, _ = parse_expected(text)
        assert "SHOULD NOT APPEAR" not in lines


@pytest.mark.corpus
class TestCountSloc:
    def test_excludes_blanks_and_comments(self):
        src = """\
// a comment
fn main() {

    let x = 1; // trailing comment counts as code
    /* block
       comment */
    println!("{}", x);
}
"""
        assert count_sloc(src) == 4  # fn, let, println, }

    def test_nested_block_comments(self):
        src = "/* outer /* inner */ still outer */\nfn main() {}\n"
        assert count_sloc(src) == 1

    def test_code_before_block_comment(self):
        assert count_sloc("let x = 1; /* c */\n") == 1

    def test_empty_source_is_zero(self):
        assert count_sloc("\n\n   \n") == 0

    def test_only_comments_is_zero(self):
        assert count_sloc("// a\n/* b */\n") == 0


@pytest.mark.corpus
class TestNameHelpers:
    def test_normalize_ws(self):
        assert normalize_ws("  a   b \t c ") == "a b c"

    def test_display_name_strips_rosetta_index(self):
        assert display_name("Dining_philosophers_269") == "Dining_philosophers"
        assert display_name("Rot-13") == "Rot-13"

    def test_truncate_short_unchanged(self):
        assert truncate("short", 25) == "short"

    def test_truncate_long_gets_ellipsis(self):
        out = truncate("x" * 40, 10)
        assert len(out) == 10 and out.endswith("...")


@pytest.mark.corpus
class TestCheckOutput:
    def test_pass(self):
        ok, detail = check_output("a\nb\n", _prog(("a", "b")))
        assert ok and detail is None

    def test_whitespace_normalized(self):
        ok, _ = check_output("a    b\n", _prog(("a b",)))
        assert ok

    def test_mismatch_reports_line(self):
        ok, detail = check_output("a\nX\n", _prog(("a", "b")))
        assert not ok
        assert "line 1" in detail

    def test_missing_line(self):
        ok, detail = check_output("a\n", _prog(("a", "b")))
        assert not ok
        assert "no line" in detail

    def test_no_oracle_passes(self):
        ok, detail = check_output("anything", _prog(None))
        assert ok

    def test_extra_output_lines_are_ignored(self):
        # Only the expected lines are compared; trailing extra output is fine.
        ok, _ = check_output("a\nb\nc\n", _prog(("a", "b")))
        assert ok


# ---------------------------------------------------------------------------
# Statistics
# ---------------------------------------------------------------------------


@pytest.mark.stats
class TestJsDivergence:
    def test_identical_is_zero(self):
        a = np.array([1.0, 2.0, 3.0])
        assert js_divergence(a, a) == pytest.approx(0.0, abs=1e-12)

    def test_disjoint_is_one_bit(self):
        a = np.array([1.0, 0.0])
        b = np.array([0.0, 1.0])
        assert js_divergence(a, b) == pytest.approx(1.0)

    def test_empty_is_nan(self):
        assert np.isnan(js_divergence(np.array([0.0]), np.array([1.0])))

    def test_size_bias_is_real(self):
        """The documented failure mode: same +3 delta, different totals.

        This is why JS is descriptive-only in this package.
        """
        small = js_divergence(np.array([60.0, 10.0]), np.array([63.0, 10.0]))
        big = js_divergence(np.array([5000.0, 10.0]), np.array([5003.0, 10.0]))
        assert small > big * 10

    def test_symmetric(self):
        a = np.array([3.0, 1.0, 2.0])
        b = np.array([1.0, 2.0, 4.0])
        assert js_divergence(a, b) == pytest.approx(js_divergence(b, a))


@pytest.mark.stats
class TestBenjaminiHochberg:
    def test_monotone_and_bounded(self):
        p = np.array([0.001, 0.008, 0.039, 0.041, 0.042, 0.06, 0.074, 0.205])
        adj = benjamini_hochberg(p)
        assert np.all(adj >= p - 1e-12)
        assert np.all(adj <= 1.0)
        assert np.all(np.diff(adj[np.argsort(p)]) >= -1e-12)

    def test_known_values(self):
        p = np.array([0.01, 0.02, 0.03, 0.04, 0.05])
        adj = benjamini_hochberg(p)
        assert adj[0] == pytest.approx(0.05)
        assert adj[4] == pytest.approx(0.05)

    def test_empty(self):
        assert len(benjamini_hochberg([])) == 0


# ---------------------------------------------------------------------------
# Noise floors
# ---------------------------------------------------------------------------


@pytest.mark.noise
class TestNoiseFloor:
    def test_run_noise_derived_from_a_rerun_pass(self, settings):
        rows = []
        for i in range(5):
            f = f"p{i}"
            rows += [("A", f, "futex", 3.0), (settings.tag(0, 1), f, "futex", 9.0)]
        floor = derive_noise(FakeFrames(_counts_frame(rows)), settings, "basic")
        assert "futex" in floor.run_noisy
        assert floor.n_files_rerun == 5

    def test_below_threshold_not_noisy(self, settings):
        rows = []
        for i in range(5):
            f = f"p{i}"
            differ = 5.0 if i < 2 else 3.0  # only 2 files differ, threshold is 3
            rows += [("A", f, "futex", 3.0), (settings.tag(0, 1), f, "futex", differ)]
        floor = derive_noise(FakeFrames(_counts_frame(rows)), settings, "basic")
        assert "futex" not in floor.run_noisy

    def test_read_invisible_to_a_rerun_but_caught_by_a_rebuild(self, settings):
        """The central claim about the control design.

        ``read`` is byte-identical across passes of one build, because they
        trace the *same binary*. Only another build can reveal build-to-build
        variation.
        """
        rows = []
        for i in range(5):
            f = f"p{i}"
            rows += [
                ("A", f, "read", 4.0),
                (settings.tag(0, 1), f, "read", 4.0),  # same binary -> identical
                (settings.tag(1, 0), f, "read", 5.0),  # different binary -> differs
            ]
        floor = derive_noise(FakeFrames(_counts_frame(rows)), settings, "basic")
        assert "read" not in floor.run_noisy  # a rerun cannot see it
        assert "read" in floor.build_noisy  # another build can
        assert "read" in floor.all_noisy

    def test_build_noise_excludes_already_run_noisy(self, settings):
        """A syscall that is run-noisy must not also be blamed on the build."""
        rows = []
        for i in range(5):
            f = f"p{i}"
            rows += [
                ("A", f, "futex", 3.0),
                (settings.tag(0, 1), f, "futex", 9.0),
                (settings.tag(1, 0), f, "futex", 11.0),
            ]
        floor = derive_noise(FakeFrames(_counts_frame(rows)), settings, "basic")
        assert "futex" in floor.run_noisy
        assert "futex" not in floor.build_noisy

    def test_unstable_sequence_files_detected(self, settings):
        ngrams = pd.DataFrame(
            [
                {
                    "config": "basic",
                    "compiler": "A",
                    "file": "p0",
                    "n": 3,
                    "ngram": "a→b→c",
                    "stable": True,
                    "n_threads": 4,
                },
                {
                    "config": "basic",
                    "compiler": settings.tag(0, 1),
                    "file": "p0",
                    "n": 3,
                    "ngram": "x→y→z",
                    "stable": True,
                    "n_threads": 4,
                },
                {
                    "config": "basic",
                    "compiler": "A",
                    "file": "p1",
                    "n": 3,
                    "ngram": "a→b→c",
                    "stable": True,
                    "n_threads": 1,
                },
                {
                    "config": "basic",
                    "compiler": settings.tag(0, 1),
                    "file": "p1",
                    "n": 3,
                    "ngram": "a→b→c",
                    "stable": True,
                    "n_threads": 1,
                },
            ]
        )
        floor = derive_noise(FakeFrames(_counts_frame([]), ngrams), settings, "basic")
        assert "p0" in floor.unstable_seq_files
        assert "p1" not in floor.unstable_seq_files


# ---------------------------------------------------------------------------
# Config
# ---------------------------------------------------------------------------


@pytest.mark.config
class TestFingerprint:
    def test_stable_across_identical_settings(self, settings):
        other = Settings(
            baseline="A",
            compilers=settings.compilers,
            configs=settings.configs,
            detect=DetectSettings(auto_noisy_min_files=3),
        )
        assert settings.fingerprint() == other.fingerprint()

    def test_changes_with_trace_reps(self, settings):
        other = Settings(
            baseline="A",
            compilers=settings.compilers,
            configs=settings.configs,
            trace=TraceSettings(reps=5),
        )
        assert settings.fingerprint() != other.fingerprint()

    def test_ignores_presentation_settings(self, settings):
        """Retuning a threshold must not invalidate a multi-hour sweep."""
        other = Settings(
            baseline="A",
            compilers=settings.compilers,
            configs=settings.configs,
            detect=DetectSettings(program_dependent_rho=0.9, uniform_threshold=0.5),
        )
        assert settings.fingerprint() == other.fingerprint()

    def test_changes_with_controls(self, settings):
        other = Settings(
            baseline="A",
            compilers=settings.compilers,
            configs=settings.configs,
            controls=ControlSettings(builds=1),
        )
        assert settings.fingerprint() != other.fingerprint()

    def test_ignores_languages(self, make_settings):
        """Languages define the corpus, not the trace data; not fingerprinted."""
        from compilerdiv.corpus import LanguageSpec

        a = make_settings()
        b = make_settings(languages=(LanguageSpec("only", ".only", "#", None),))
        assert a.fingerprint() == b.fingerprint()


@pytest.mark.config
class TestBuildArgvOrder:
    def test_src_first_argument_order(self):
        from pathlib import Path

        c = CompilerSpec(
            key="C", label="variant_gcc", cmd=("./build.sh", "compile"), src_first=True
        )
        argv = c.build_argv(Path("/src/a.rs"), Path("/out/a"), ("-C", "opt-level=2"))
        assert argv[:3] == ["./build.sh", "compile", "/src/a.rs"]
        assert argv[-2:] == ["-o", "/out/a"]

    def test_normal_argument_order(self):
        from pathlib import Path

        c = CompilerSpec(key="A", label="reference", cmd=("cc",))
        argv = c.build_argv(Path("/src/a.rs"), Path("/out/a"), ())
        assert argv[-1] == "/src/a.rs"


@pytest.mark.config
class TestConfigValidation:
    def test_key_colliding_with_the_tag_separator_rejected(self, write_config):
        """Control tags are generated as `A@<build>.<pass>`, so a compiler key
        containing the separator could shadow one."""
        cfg = write_config(
            "baseline: A\n"
            "compilers:\n"
            "  - {key: A, label: reference, cmd: [cc]}\n"
            "  - {key: 'A@1.0', label: bad, cmd: [x]}\n"
            "configs:\n"
            "  - {name: basic}\n"
        )
        with pytest.raises(ConfigError, match="reserved"):
            load_settings(cfg)

    def test_baseline_must_exist(self, write_config):
        cfg = write_config(
            "baseline: Z\n"
            "compilers:\n"
            "  - {key: A, label: reference, cmd: [cc]}\n"
            "configs:\n"
            "  - {name: basic}\n"
        )
        with pytest.raises(ConfigError, match="not a configured compiler"):
            load_settings(cfg)

    def test_duplicate_compiler_key_rejected(self, write_config):
        cfg = write_config(
            "baseline: A\n"
            "compilers:\n"
            "  - {key: A, label: one, cmd: [a]}\n"
            "  - {key: A, label: two, cmd: [b]}\n"
            "configs:\n"
            "  - {name: basic}\n"
        )
        with pytest.raises(ConfigError, match="duplicate"):
            load_settings(cfg)

    def test_missing_config_file(self, tmp_path):
        with pytest.raises(ConfigError, match="not found"):
            load_settings(tmp_path / "nope.yaml")

    def test_no_compilers_rejected(self, write_config):
        with pytest.raises(ConfigError, match="no compilers"):
            load_settings(write_config("baseline: A\nconfigs:\n  - {name: basic}\n"))

    def test_unsupported_pairs_excluded(self, write_config):
        cfg = write_config(
            "baseline: A\n"
            "compilers:\n"
            "  - {key: A, label: reference, cmd: [cc]}\n"
            "  - {key: C, label: gcc, cmd: [gcc]}\n"
            "configs:\n"
            "  - {name: basic}\n"
            "  - {name: static, flags: ['-C', 'target-feature=+crt-static'], unsupported: [C]}\n"
        )
        s = load_settings(cfg)
        assert s.compilers_for("basic") == ["A", "C"]
        assert s.compilers_for("static") == ["A"]

    def test_wrapper_configurable(self, write_config):
        cfg = write_config(
            "baseline: A\n"
            "compilers:\n"
            "  - {key: A, label: reference, cmd: [cc]}\n"
            "configs:\n"
            "  - {name: basic}\n"
            "trace:\n"
            "  wrapper: []\n"
        )
        s = load_settings(cfg)
        assert s.trace.wrapper == ()


@pytest.mark.config
class TestSettingsAccessors:
    def test_variant_keys_exclude_baseline(self, settings):
        assert settings.variant_keys == ["B"]

    def test_control_tags_follow_the_grid(self, make_settings):
        both = make_settings()
        assert set(both.control_tags()) == {
            both.tag(0, 1),
            both.tag(1, 0),
            both.tag(1, 1),
        }
        neither = make_settings(controls=ControlSettings(builds=1, passes=1))
        assert neither.control_tags() == []

    def test_label_for_control_tags(self, settings):
        assert "rerun" in settings.label(settings.tag(0, 1))
        assert "build" in settings.label(settings.tag(1, 0))

    def test_unknown_compiler_key_raises(self, settings):
        with pytest.raises(KeyError):
            settings.compiler("Z")

"""Multi-language support: detection, per-language parsing, compiler invocation.

The harness runs against any compiled language configured in the YAML. The
tests cover the whole chain:

* the built-in language registry and its invariants;
* comment-aware SLOC and expected-output parsing for ``//``, ``#``, ``--``,
  ``!`` line comments and ``/* */``, ``#[ ]#``, ``{- -}``, ``{ }`` blocks;
* extension-driven corpus discovery, and the rejection of mixed-language
  corpora;
* language *detection* from what is actually on disk;
* configuring/overriding/extending languages from YAML;
* the ``output_flag`` that lets non-``-o`` compilers (Nim, Zig, dmd) work.
"""

from __future__ import annotations

from pathlib import Path

import pytest

from compilerdiv.config import (
    BuildConfig,
    CompilerSpec,
    ConfigError,
    PathSettings,
    Settings,
    load_settings,
)
from compilerdiv.corpus import (
    BUILTIN_LANGUAGES,
    LanguageSpec,
    MixedCorpusError,
    count_sloc,
    language_registry,
    load_corpus,
    load_program,
    parse_expected,
    validate_corpus,
)

from conftest import make_header

pytestmark = pytest.mark.languages


# ---------------------------------------------------------------------------
# The built-in registry
# ---------------------------------------------------------------------------


class TestBuiltinRegistry:
    def test_registry_is_nonempty(self):
        assert len(BUILTIN_LANGUAGES) >= 10

    def test_extensions_start_with_dot(self):
        assert all(l.extension.startswith(".") for l in BUILTIN_LANGUAGES)

    def test_extensions_are_unique(self):
        exts = [l.extension for l in BUILTIN_LANGUAGES]
        assert len(exts) == len(set(exts)), "two languages share an extension"

    def test_every_language_has_a_line_comment(self):
        # The expected-output header is line-comment based, so a language with
        # no line comment could not carry an embedded integration test.
        assert all(l.line_comment for l in BUILTIN_LANGUAGES)

    def test_every_language_has_a_hello_world(self):
        # preflight needs a compilable probe for each built-in language.
        assert all(l.hello_world.strip() for l in BUILTIN_LANGUAGES)

    def test_hello_worlds_print_ok(self):
        assert all("ok" in l.hello_world for l in BUILTIN_LANGUAGES)

    @pytest.mark.parametrize("name", ["rust", "c", "cpp", "go", "haskell", "fortran"])
    def test_expected_languages_present(self, name):
        assert name in {l.name for l in BUILTIN_LANGUAGES}

    def test_no_interpreted_languages(self):
        """The pipeline traces a binary; interpreted languages have none."""
        names = {l.name for l in BUILTIN_LANGUAGES}
        assert names.isdisjoint({"python", "ruby", "perl", "javascript", "bash"})


class TestLanguageRegistry:
    def test_none_falls_back_to_builtins(self):
        reg = language_registry(None)
        assert reg[".rs"].name == "rust"
        assert len(reg) == len({l.extension for l in BUILTIN_LANGUAGES})

    def test_empty_sequence_falls_back_to_builtins(self):
        assert language_registry(())[".c"].name == "c"

    def test_custom_specs_win(self):
        custom = LanguageSpec("mylang", ".rs", "#", None)
        reg = language_registry([custom])
        assert reg[".rs"].line_comment == "#"
        assert set(reg) == {".rs"}


# ---------------------------------------------------------------------------
# SLOC across comment styles
# ---------------------------------------------------------------------------


class TestSlocCommentStyles:
    def test_default_c_style(self):
        assert count_sloc("fn main() {}\n// c\n") == 1

    def test_hash_line_comment(self):
        src = "# a comment\nx = 1\ny = 2  # trailing\n"
        assert count_sloc(src, line_comment="#", block_comment=None) == 2

    def test_double_dash_line_comment(self):
        src = '-- a comment\nmain = putStrLn "ok"\n'
        assert count_sloc(src, line_comment="--", block_comment=("{-", "-}")) == 1

    def test_bang_line_comment_fortran(self):
        src = "! a comment\nprogram p\n  print *, 1  ! inline\nend program p\n"
        assert count_sloc(src, line_comment="!", block_comment=None) == 3

    def test_haskell_block_comment(self):
        src = "{- block\n   comment -}\nmain = return ()\n"
        assert count_sloc(src, line_comment="--", block_comment=("{-", "-}")) == 1

    def test_nim_nested_block_comment(self):
        src = "#[ outer #[ inner ]# still ]#\necho 1\n"
        assert count_sloc(src, line_comment="#", block_comment=("#[", "]#")) == 1

    def test_pascal_brace_block_comment(self):
        src = "{ a comment }\nbegin\n  writeln('x');\nend.\n"
        assert count_sloc(src, line_comment="//", block_comment=("{", "}")) == 3

    def test_code_before_hash_counts(self):
        assert count_sloc("x = 1 # note\n", line_comment="#", block_comment=None) == 1

    def test_no_block_comment_markers_are_literal(self):
        # With block_comment=None, a stray '/*' is code, not a comment opener.
        assert (
            count_sloc("a = /*not a comment*/\n", line_comment="#", block_comment=None)
            == 1
        )

    def test_multi_char_open_multi_char_close(self):
        # #[ and ]# have length 2; boundaries must be respected exactly.
        src = "code1\n#[\ninside\n]#\ncode2\n"
        assert count_sloc(src, line_comment="#", block_comment=("#[", "]#")) == 2


# ---------------------------------------------------------------------------
# Expected-output header across comment styles
# ---------------------------------------------------------------------------


class TestExpectedHeaderCommentStyles:
    def test_slash_header(self):
        text = make_header(["line one", "line two"], "//")
        lines, has_block, sentinel = parse_expected(text, "//")
        assert has_block and not sentinel
        assert lines == ["line one", "line two"]

    def test_hash_header(self):
        text = make_header(["hello", "world"], "#")
        lines, has_block, sentinel = parse_expected(text, "#")
        assert has_block and not sentinel
        assert lines == ["hello", "world"]

    def test_double_dash_header(self):
        text = make_header(["42"], "--")
        lines, has_block, sentinel = parse_expected(text, "--")
        assert lines == ["42"]

    def test_wrong_marker_leaves_prefix(self):
        """Parsing a #-file with the // marker fails to strip the prefix."""
        text = make_header(["value"], "#")
        lines, _, _ = parse_expected(text, "//")
        # The '#' marker is not stripped, so the content still carries it.
        assert lines and lines[0].startswith("#")

    def test_sentinel_detected_with_hash(self):
        text = make_header(None, "#")
        lines, has_block, sentinel = parse_expected(text, "#")
        assert has_block and sentinel


# ---------------------------------------------------------------------------
# Corpus discovery by extension
# ---------------------------------------------------------------------------


class TestCorpusDiscovery:
    def test_discovers_only_recognised_extensions(self, make_corpus):
        d = make_corpus(
            {
                "a.rs": "fn main() {}\n",
                "b.rs": "fn main() {}\n",
                "notes.txt": "ignore me\n",
                "Makefile": "all:\n",
            }
        )
        progs = load_corpus(d)
        stems = {p.stem for p in progs}
        assert stems == {"a", "b"}

    def test_each_program_gets_its_language(self, make_corpus):
        rust = make_corpus({"a.rs": "fn main() {}\n"}, subdir="rust")
        hask = make_corpus({"b.hs": "main = return ()\n"}, subdir="hask")
        assert load_corpus(rust)[0].lang.name == "rust"
        assert load_corpus(hask)[0].lang.name == "haskell"

    def test_each_language_parses_with_its_own_markers(self, make_corpus):
        rs = make_header(["from rust"], "//") + "fn main(){}\n"
        hs = make_header(["from haskell"], "--") + "main = return ()\n"
        rust = make_corpus({"a.rs": rs}, subdir="rust")
        hask = make_corpus({"b.hs": hs}, subdir="hask")
        assert load_corpus(rust)[0].expected == ("from rust",)
        assert load_corpus(hask)[0].expected == ("from haskell",)

    def test_empty_dir_raises(self, tmp_path):
        (tmp_path / "empty").mkdir()
        with pytest.raises(FileNotFoundError, match="recognised extension"):
            load_corpus(tmp_path / "empty")

    def test_missing_dir_raises(self, tmp_path):
        with pytest.raises(FileNotFoundError):
            load_corpus(tmp_path / "does_not_exist")

    def test_unconfigured_extension_ignored(self, make_corpus):
        d = make_corpus({"a.rs": "fn main(){}\n", "b.zzz": "whatever\n"})
        assert {p.stem for p in load_corpus(d)} == {"a"}

    def test_custom_language_list_restricts_discovery(self, make_corpus):
        d = make_corpus({"a.rs": "fn main(){}\n", "b.c": "int main(){}\n"})
        only_c = [LanguageSpec("c", ".c", "//", ("/*", "*/"))]
        assert {p.stem for p in load_corpus(d, only_c)} == {"b"}


class TestMixedCorpusRejected:
    """One corpus, one extension -- otherwise one argv would build two languages."""

    def test_two_languages_raise(self, make_corpus):
        d = make_corpus({"a.rs": "fn main(){}\n", "b.c": "int main(){return 0;}\n"})
        with pytest.raises(MixedCorpusError, match="mixes 2 source extensions"):
            load_corpus(d)

    def test_error_names_both_extensions_and_an_example(self, make_corpus):
        d = make_corpus({"a.rs": "fn main(){}\n", "b.c": "int main(){return 0;}\n"})
        with pytest.raises(MixedCorpusError) as ei:
            load_corpus(d)
        msg = str(ei.value)
        assert ".rs" in msg and ".c" in msg
        assert "a.rs" in msg and "b.c" in msg

    def test_same_language_different_extensions_still_rejected(self, make_corpus):
        # .cpp and .cc are both C++, but the compiler argv is fixed and drivers
        # do dispatch on suffix, so the rule is by extension.
        d = make_corpus({"a.cpp": "int main(){}\n", "b.cc": "int main(){}\n"})
        with pytest.raises(MixedCorpusError):
            load_corpus(d)

    def test_unrecognised_extensions_do_not_trigger_it(self, make_corpus):
        d = make_corpus({"a.rs": "fn main(){}\n", "README.md": "hi\n"})
        assert {p.stem for p in load_corpus(d)} == {"a"}

    def test_narrowed_languages_block_resolves_it(self, make_corpus):
        d = make_corpus({"a.rs": "fn main(){}\n", "b.c": "int main(){}\n"})
        only_c = [LanguageSpec("c", ".c", "//", ("/*", "*/"))]
        assert {p.stem for p in load_corpus(d, only_c)} == {"b"}

    def test_start_anchor_cannot_hide_a_second_language(self, make_corpus):
        # Slicing the corpus must not slice the check out of view.
        d = make_corpus({"a.c": "int main(){}\n", "b.rs": "fn main(){}\n"})
        with pytest.raises(MixedCorpusError):
            load_corpus(d, start="b")

    def test_validate_corpus_matches_load_corpus(self, make_corpus):
        d = make_corpus({"a.rs": "fn main(){}\n", "b.c": "int main(){}\n"})
        with pytest.raises(MixedCorpusError):
            validate_corpus(d)

    def test_validate_corpus_accepts_single_language(self, make_corpus):
        d = make_corpus({"a.rs": "fn main(){}\n", "b.rs": "fn main(){}\n"})
        validate_corpus(d)  # does not raise

    def test_validate_corpus_tolerates_missing_dir(self, tmp_path):
        # analyze() calls this before it knows whether a corpus is even present.
        validate_corpus(tmp_path / "nope")

    def test_not_a_value_error(self, make_corpus):
        # analyze() swallows ValueError around the optional corpus figure; this
        # must abort the run instead of being skipped past.
        d = make_corpus({"a.rs": "fn main(){}\n", "b.c": "int main(){}\n"})
        with pytest.raises(MixedCorpusError):
            load_corpus(d)
        assert not issubclass(MixedCorpusError, ValueError)


class TestCorpusStartAnchor:
    def _corpus(self, make_corpus):
        return make_corpus(
            {"a.rs": "fn main(){}\n", "b.rs": "fn main(){}\n", "c.rs": "fn main(){}\n"}
        )

    def test_start_bare_stem(self, make_corpus):
        d = self._corpus(make_corpus)
        assert [p.stem for p in load_corpus(d, start="b")] == ["b", "c"]

    def test_start_with_extension(self, make_corpus):
        d = self._corpus(make_corpus)
        assert [p.stem for p in load_corpus(d, start="b.rs")] == ["b", "c"]

    def test_start_missing_raises(self, make_corpus):
        d = self._corpus(make_corpus)
        with pytest.raises(ValueError, match="not found"):
            load_corpus(d, start="zzz")


class TestLoadProgram:
    def test_default_markers_when_no_lang(self, tmp_path):
        p = tmp_path / "x.rs"
        p.write_text(make_header(["out"], "//") + "fn main(){}\n")
        prog = load_program(p)
        assert prog.expected == ("out",)
        assert prog.checkable

    def test_lang_markers_used(self, tmp_path):
        lang = LanguageSpec("hs", ".hs", "--", ("{-", "-}"))
        p = tmp_path / "x.hs"
        p.write_text(make_header(["out"], "--") + "main = return ()\n")
        prog = load_program(p, lang)
        assert prog.expected == ("out",)
        assert prog.lang is lang

    def test_sentinel_makes_unusable(self, tmp_path):
        p = tmp_path / "x.rs"
        p.write_text(make_header(None, "//"))
        prog = load_program(p)
        assert prog.sentinel
        assert not prog.checkable


# ---------------------------------------------------------------------------
# Detection on a Settings object
# ---------------------------------------------------------------------------


class TestLanguageDetection:
    def _settings(self, corpus_dir: Path) -> Settings:
        return Settings(
            baseline="A",
            compilers=(CompilerSpec(key="A", label="cc", cmd=("gcc",)),),
            configs=(BuildConfig(name="basic"),),
            paths=PathSettings(
                project_root=str(corpus_dir.parent), corpus_dir=corpus_dir.name
            ),
        )

    def test_language_for_by_extension(self, make_settings):
        s = make_settings()
        assert s.language_for(".rs").name == "rust"
        assert s.language_for("foo/bar.c").name == "c"

    def test_language_for_unknown_is_none(self, make_settings):
        assert make_settings().language_for(".zzz") is None

    def test_corpus_languages_single(self, make_corpus):
        d = make_corpus({"a.c": "int main(){}\n", "b.c": "int main(){}\n"})
        s = self._settings(d)
        langs = s.corpus_languages()
        assert [l.name for l in langs] == ["c"]

    def test_corpus_languages_mixed(self, make_corpus):
        d = make_corpus(
            {"a.rs": "fn main(){}\n", "b.go": "package main\nfunc main(){}\n"}
        )
        s = self._settings(d)
        assert sorted(l.name for l in s.corpus_languages()) == ["go", "rust"]

    def test_corpus_languages_dedup_shared_name(self, make_corpus):
        # .cpp and .cc are the same language 'cpp'; detection must not list it
        # twice as one language even though two extensions matched.
        d = make_corpus({"a.cpp": "int main(){}\n", "b.cc": "int main(){}\n"})
        s = self._settings(d)
        names = [l.name for l in s.corpus_languages()]
        assert names == ["cpp"]

    def test_corpus_languages_empty_when_none_recognised(self, make_corpus):
        d = make_corpus({"a.txt": "x\n"})
        s = self._settings(d)
        assert s.corpus_languages() == []


# ---------------------------------------------------------------------------
# Configuring languages from YAML
# ---------------------------------------------------------------------------


class TestLanguageConfig:
    def test_absent_section_keeps_builtins(self, write_config):
        s = load_settings(
            write_config(
                "baseline: A\n"
                "compilers:\n  - {key: A, label: cc, cmd: [gcc]}\n"
                "configs:\n  - {name: basic}\n"
            )
        )
        assert s.language_for(".rs").name == "rust"
        assert s.language_for(".go").name == "go"

    def test_new_language_extends_registry(self, write_config):
        s = load_settings(
            write_config(
                "baseline: A\n"
                "compilers:\n  - {key: A, label: cc, cmd: [gcc]}\n"
                "configs:\n  - {name: basic}\n"
                "languages:\n"
                "  - name: mylang\n"
                "    extension: .ml2\n"
                "    line_comment: '##'\n"
                "    block_comment: ['<<', '>>']\n"
                "    hello_world: 'say ok'\n"
            )
        )
        spec = s.language_for(".ml2")
        assert spec.name == "mylang"
        assert spec.line_comment == "##"
        assert spec.block_comment == ("<<", ">>")
        # built-ins survive alongside the new one.
        assert s.language_for(".rs").name == "rust"

    def test_override_builtin_by_extension(self, write_config):
        s = load_settings(
            write_config(
                "baseline: A\n"
                "compilers:\n  - {key: A, label: cc, cmd: [gcc]}\n"
                "configs:\n  - {name: basic}\n"
                "languages:\n"
                "  - {name: rustlike, extension: .rs, line_comment: '#'}\n"
            )
        )
        assert s.language_for(".rs").name == "rustlike"
        assert s.language_for(".rs").line_comment == "#"

    def test_extension_dot_is_optional(self, write_config):
        s = load_settings(
            write_config(
                "baseline: A\n"
                "compilers:\n  - {key: A, label: cc, cmd: [gcc]}\n"
                "configs:\n  - {name: basic}\n"
                "languages:\n  - {name: x, extension: foo}\n"
            )
        )
        assert s.language_for(".foo").name == "x"

    def test_null_block_comment(self, write_config):
        s = load_settings(
            write_config(
                "baseline: A\n"
                "compilers:\n  - {key: A, label: cc, cmd: [gcc]}\n"
                "configs:\n  - {name: basic}\n"
                "languages:\n  - {name: x, extension: .x, block_comment: null}\n"
            )
        )
        assert s.language_for(".x").block_comment is None

    def test_missing_name_rejected(self, write_config):
        with pytest.raises(ConfigError, match="name"):
            load_settings(
                write_config(
                    "baseline: A\n"
                    "compilers:\n  - {key: A, label: cc, cmd: [gcc]}\n"
                    "configs:\n  - {name: basic}\n"
                    "languages:\n  - {extension: .x}\n"
                )
            )

    def test_missing_extension_rejected(self, write_config):
        with pytest.raises(ConfigError, match="extension"):
            load_settings(
                write_config(
                    "baseline: A\n"
                    "compilers:\n  - {key: A, label: cc, cmd: [gcc]}\n"
                    "configs:\n  - {name: basic}\n"
                    "languages:\n  - {name: x}\n"
                )
            )

    def test_bad_block_comment_pair_rejected(self, write_config):
        with pytest.raises(ConfigError, match="block_comment"):
            load_settings(
                write_config(
                    "baseline: A\n"
                    "compilers:\n  - {key: A, label: cc, cmd: [gcc]}\n"
                    "configs:\n  - {name: basic}\n"
                    "languages:\n  - {name: x, extension: .x, block_comment: ['only-one']}\n"
                )
            )


# ---------------------------------------------------------------------------
# Compiler output flag (invocation is per-compiler, not per-language)
# ---------------------------------------------------------------------------


class TestOutputFlag:
    SRC = Path("/src/a.c")
    OUT = Path("/out/a")

    def test_default_dash_o_two_tokens(self):
        c = CompilerSpec(key="A", label="gcc", cmd=("gcc",))
        argv = c.build_argv(self.SRC, self.OUT, ())
        assert argv == ["gcc", "-o", "/out/a", "/src/a.c"]

    def test_flags_precede_output(self):
        c = CompilerSpec(key="A", label="gcc", cmd=("gcc",))
        argv = c.build_argv(self.SRC, self.OUT, ("-O2",))
        assert argv == ["gcc", "-O2", "-o", "/out/a", "/src/a.c"]

    def test_equals_flag_is_joined(self):
        # dmd -of=out, zig -femit-bin=out: single joined token.
        c = CompilerSpec(
            key="Z", label="zig", cmd=("zig", "build-exe"), output_flag="-femit-bin="
        )
        argv = c.build_argv(Path("/s/a.zig"), self.OUT, ())
        assert "-femit-bin=/out/a" in argv
        assert "-o" not in argv

    def test_colon_flag_is_joined(self):
        # nim c -o:out
        c = CompilerSpec(key="N", label="nim", cmd=("nim", "c"), output_flag="-o:")
        argv = c.build_argv(Path("/s/a.nim"), self.OUT, ())
        assert "-o:/out/a" in argv

    def test_empty_flag_is_positional(self):
        c = CompilerSpec(key="A", label="cc", cmd=("cc",), output_flag="")
        argv = c.build_argv(self.SRC, self.OUT, ())
        assert argv == ["cc", "/out/a", "/src/a.c"]

    def test_output_flag_with_src_first(self):
        c = CompilerSpec(
            key="C",
            label="wrapped",
            cmd=("./build.sh", "compile"),
            src_first=True,
            output_flag="-o",
        )
        argv = c.build_argv(Path("/s/a.rs"), self.OUT, ("-C", "opt-level=2"))
        assert argv[:3] == ["./build.sh", "compile", "/s/a.rs"]
        assert argv[-2:] == ["-o", "/out/a"]

    def test_output_flag_loaded_from_yaml(self, write_config):
        s = load_settings(
            write_config(
                "baseline: A\n"
                "compilers:\n"
                "  - {key: A, label: nim, cmd: [nim, c], output_flag: '-o:'}\n"
                "configs:\n  - {name: basic}\n"
            )
        )
        assert s.compiler("A").output_flag == "-o:"

    def test_output_flag_not_in_fingerprint(self, make_settings):
        """It only names the output file; it does not change the binary."""
        a = make_settings(
            compilers=(
                CompilerSpec(key="A", label="cc", cmd=("cc",), output_flag="-o"),
            )
        )
        b = make_settings(
            compilers=(
                CompilerSpec(key="A", label="cc", cmd=("cc",), output_flag="-o:"),
            )
        )
        assert a.fingerprint() == b.fingerprint()


class TestMixedCorpusStopsEveryCommand:
    """The guard is enforced at each entry point, not only in load_corpus."""

    def _mixed(self, make_corpus):
        return make_corpus({"a.rs": "fn main(){}\n", "b.c": "int main(){}\n"})

    def test_preflight_fails_before_probing_compilers(self, make_corpus, tmp_path):
        from compilerdiv.acquire.preflight import preflight

        settings = Settings(
            paths=PathSettings(project_root=tmp_path, corpus_dir=Path("corpus")),
            compilers=[CompilerSpec(key="A", label="ref", cmd=["cc"])],
            configs=[BuildConfig(name="basic", flags=[])],
            baseline="A",
        )
        self._mixed(make_corpus)
        rep = preflight(settings)
        assert not rep.ok
        assert any(name == "corpus" for name, _ in rep.failures)
        assert "mixes 2 source extensions" in rep.render()

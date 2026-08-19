"""The corpus: source discovery, embedded integration tests, SLOC.

Expected-output parsing and comment-aware line counting both live here as pure
functions, and both are unit-tested.

The corpus is language-agnostic. Which files are discovered, how their comment
headers are stripped, and how SLOC is counted are all driven by a
:class:`LanguageSpec`, selected per file from its extension. A broad set of
*compiled* languages is recognised out of the box (:data:`BUILTIN_LANGUAGES`);
the configuration can override any of them or add new ones.

One corpus holds **one** extension. The compilers in a run are a fixed argv
prefix applied to every program, so a corpus mixing ``.rs`` and ``.c`` would ask
one toolchain to build both and silently record the failures of whichever
language it cannot compile as if they were properties of the *variant*. A mixed
corpus is therefore rejected outright (:class:`MixedCorpusError`) rather than
half-measured. Point ``corpus_dir`` at one language per run, or narrow the
``languages:`` block so only one extension is recognised.

Only *compiled* languages belong here: the pipeline is compile -> binary -> run
-> strace, so an interpreted language has no binary to trace. Every language
therefore also carries a ``hello_world`` probe that ``preflight`` compiles to
prove each compiler works before a multi-hour sweep.

The expected-output block format is the one described in the paper (Fig. 4).
For a ``//``-comment language it looks like::

    // Rosetta Code task: Abbreviations, easy
    // Source: rosettacode.org/wiki/Abbreviations,_easy
    // Content licensed under GFDL 1.2 (Rosetta Code).
    // =======================
    // Expected output:
    // RIGHT REPEAT *error* PUT MOVE RESTORE ...
    // =======================

Only the *line-comment marker* is language-specific; the ``Expected output:``
and ``=======================`` sentinels are literal and shared across
languages.
"""

from __future__ import annotations

import re
from dataclasses import dataclass
from pathlib import Path
from typing import Sequence

NO_OUTPUT_SENTINEL = "(no expected output provided on Rosetta Code)"

_DELIM = "======================="
_WS = re.compile(r"\s+")
_FILE_NUM_RE = re.compile(r"_\d+$")


# ---------------------------------------------------------------------------
# Languages
# ---------------------------------------------------------------------------


@dataclass(frozen=True)
class LanguageSpec:
    """One *compiled* source language the harness knows how to read.

    A language describes only how to *read* a source file, never how to compile
    it -- compiler invocation (the argv, the output flag) lives entirely in the
    ``compilers:`` configuration, so the same language can be driven by several
    different toolchains.

    Attributes
    ----------
    name:
        Human label (``rust``, ``c``, ``go``). Presentation only.
    extension:
        File suffix *including the dot* (``.rs``). Selects which corpus files
        belong to this language. Detection is purely by extension, so two
        languages must not share one.
    line_comment:
        Marker that begins a to-end-of-line comment (``//``, ``#``, ``--``,
        ``!``). Drives both expected-output header stripping and SLOC counting.
    block_comment:
        ``(open, close)`` delimiters for block comments, or ``None`` for
        languages that have none. Nesting is tolerated.
    hello_world:
        A minimal program that prints ``ok`` and exits 0, compiled by
        ``preflight`` to confirm each compiler works. Empty disables the compile
        probe for this language (preflight then only checks that strace works).
    """

    name: str
    extension: str
    line_comment: str = "//"
    block_comment: tuple[str, str] | None = ("/*", "*/")
    hello_world: str = ""


#: Compiled languages recognised without any configuration. A ``languages:``
#: entry whose extension matches one of these overrides it; a new extension
#: extends the set. Interpreted languages are deliberately absent: there is no
#: binary to trace. The ``hello_world`` probes are filename-agnostic so they
#: compile regardless of the temp path ``preflight`` writes them to.
BUILTIN_LANGUAGES: tuple[LanguageSpec, ...] = (
    LanguageSpec(
        "rust",
        ".rs",
        "//",
        ("/*", "*/"),
        'fn main() { println!("ok"); }\n',
    ),
    LanguageSpec(
        "c",
        ".c",
        "//",
        ("/*", "*/"),
        '#include <stdio.h>\nint main(void) { printf("ok\\n"); return 0; }\n',
    ),
    LanguageSpec(
        "cpp",
        ".cpp",
        "//",
        ("/*", "*/"),
        '#include <iostream>\nint main() { std::cout << "ok\\n"; }\n',
    ),
    LanguageSpec(
        "cpp",
        ".cc",
        "//",
        ("/*", "*/"),
        '#include <iostream>\nint main() { std::cout << "ok\\n"; }\n',
    ),
    LanguageSpec(
        "cpp",
        ".cxx",
        "//",
        ("/*", "*/"),
        '#include <iostream>\nint main() { std::cout << "ok\\n"; }\n',
    ),
    LanguageSpec(
        "go",
        ".go",
        "//",
        ("/*", "*/"),
        'package main\nimport "fmt"\nfunc main() { fmt.Println("ok") }\n',
    ),
    LanguageSpec(
        "zig",
        ".zig",
        "//",
        None,
        'const std = @import("std");\n'
        'pub fn main() void { std.debug.print("ok\\n", .{}); }\n',
    ),
    LanguageSpec(
        "d",
        ".d",
        "//",
        ("/*", "*/"),
        'import std.stdio;\nvoid main() { writeln("ok"); }\n',
    ),
    LanguageSpec(
        "nim",
        ".nim",
        "#",
        ("#[", "]#"),
        'echo "ok"\n',
    ),
    LanguageSpec(
        "swift",
        ".swift",
        "//",
        ("/*", "*/"),
        'print("ok")\n',
    ),
    LanguageSpec(
        "crystal",
        ".cr",
        "#",
        None,
        'puts "ok"\n',
    ),
    LanguageSpec(
        "haskell",
        ".hs",
        "--",
        ("{-", "-}"),
        'main :: IO ()\nmain = putStrLn "ok"\n',
    ),
    LanguageSpec(
        "fortran",
        ".f90",
        "!",
        None,
        'program main\n  print *, "ok"\nend program main\n',
    ),
    LanguageSpec(
        "fortran",
        ".f95",
        "!",
        None,
        'program main\n  print *, "ok"\nend program main\n',
    ),
    LanguageSpec(
        "pascal",
        ".pas",
        "//",
        ("{", "}"),
        "program probe;\nbegin\n  writeln('ok');\nend.\n",
    ),
)


def language_registry(specs: Sequence[LanguageSpec] | None) -> dict[str, LanguageSpec]:
    """Map ``extension -> spec``; falls back to the built-ins when ``specs`` is empty."""
    chosen = tuple(specs) if specs else BUILTIN_LANGUAGES
    return {s.extension: s for s in chosen}


class MixedCorpusError(Exception):
    """The corpus contains more than one recognised source extension.

    Deliberately **not** a :class:`ValueError`: ``analyze`` swallows
    ``FileNotFoundError``/``ValueError`` around the optional corpus figure, and
    this condition must abort the run rather than be skipped past.
    """


def _reject_mixed_extensions(paths: Sequence[Path], corpus_dir: Path) -> None:
    """Raise :class:`MixedCorpusError` unless every path shares one extension.

    Checked on the *discovered* set, before any ``start`` anchor is applied --
    resuming mid-corpus must not be able to hide a second language by slicing it
    out of view.

    The rule is by extension, not by language name, so a corpus mixing ``.cpp``
    and ``.cc`` is rejected too even though both are C++. That is intentional:
    each file is handed to the same compiler argv, and drivers do dispatch on
    suffix.
    """
    groups: dict[str, list[str]] = {}
    for p in paths:
        groups.setdefault(p.suffix, []).append(p.name)
    if len(groups) <= 1:
        return

    width = max(len(e) for e in groups)
    detail = "\n".join(
        f"  {ext:<{width}}  {len(names):>4} file(s)   e.g. {sorted(names)[0]}"
        for ext, names in sorted(groups.items())
    )
    raise MixedCorpusError(
        f"corpus at {corpus_dir} mixes {len(groups)} source extensions:\n"
        f"{detail}\n"
        "A run applies one fixed compiler argv to every program, so a mixed "
        "corpus would record one language's compile failures as findings about "
        "the variant. Split the corpus by language and point `paths.corpus_dir` "
        "at one of them, or restrict the `languages:` block so only one "
        "extension is recognised."
    )


def validate_corpus(
    corpus_dir: Path, languages: Sequence[LanguageSpec] | None = None
) -> None:
    """Fail fast on a mixed-language corpus, without parsing any file.

    A missing or empty directory is *not* an error here -- that is
    :func:`load_corpus`'s call to make, and ``analyze`` tolerates it.
    """
    if not corpus_dir.is_dir():
        return
    by_ext = language_registry(languages)
    paths = [p for p in corpus_dir.iterdir() if p.is_file() and p.suffix in by_ext]
    _reject_mixed_extensions(paths, corpus_dir)


def _line_comment_re(marker: str) -> re.Pattern:
    return re.compile(r"^" + re.escape(marker) + r"\s?")


def normalize_ws(s: str) -> str:
    """Collapse runs of whitespace and strip. Used for output comparison."""
    return _WS.sub(" ", s).strip()


def display_name(stem: str) -> str:
    """Strip the trailing Rosetta index: ``Dining_philosophers_269`` -> ``Dining_philosophers``."""
    return _FILE_NUM_RE.sub("", str(stem))


def truncate(name: str, maxlen: int = 25) -> str:
    s = str(name)
    return s if len(s) <= maxlen else s[: maxlen - 3] + "..."


@dataclass(frozen=True)
class Program:
    """One corpus program.

    Attributes
    ----------
    stem:
        Filename without extension; the primary key throughout the pipeline.
    path:
        Absolute path to the source file.
    expected:
        Expected stdout lines from the embedded header, or ``None`` if the
        program carries no usable block.
    has_block:
        Whether an "Expected output:" marker was found at all.
    sentinel:
        Whether the block is the explicit "no expected output" sentinel.
    sloc:
        Comment- and blank-excluded source line count.
    """

    stem: str
    path: Path
    expected: tuple[str, ...] | None
    has_block: bool
    sentinel: bool
    sloc: int
    lang: LanguageSpec | None = None

    @property
    def checkable(self) -> bool:
        """True when this program can serve as its own integration test."""
        return self.expected is not None and len(self.expected) > 0


def parse_expected(text: str, line_comment: str = "//") -> tuple[list[str], bool, bool]:
    """Extract the expected-output block from a source file's text.

    Parameters
    ----------
    line_comment:
        The language's line-comment marker, stripped from each header line
        before it is treated as expected output. Defaults to ``//`` so callers
        using C-style comment markers are unaffected.

    Returns
    -------
    (lines, has_block, sentinel)
        ``lines`` are comment-stripped expected stdout lines. ``has_block`` is
        whether the marker was present. ``sentinel`` is whether the block is
        the explicit "no expected output provided" placeholder.
    """
    marker = _line_comment_re(line_comment)
    lines: list[str] = []
    has_block = False
    in_block = False

    for raw in text.splitlines():
        stripped = raw.rstrip("\n")
        content = marker.sub("", stripped)

        if not in_block:
            if "Expected output:" in stripped:
                in_block = True
                has_block = True
            continue
        if _DELIM in stripped:
            break
        lines.append(content)

    joined = " ".join(l.strip() for l in lines).strip()
    sentinel = NO_OUTPUT_SENTINEL in joined
    return lines, has_block, sentinel


def count_sloc(
    text: str,
    line_comment: str = "//",
    block_comment: tuple[str, str] | None = ("/*", "*/"),
) -> int:
    """Count non-blank, non-comment lines, handling nested block comments.

    A line with code before a trailing line comment still counts as code. The
    comment markers are language-specific: ``//`` + ``/* */`` for C-family
    languages, ``#`` + ``#[ ]#`` for Nim, ``--`` + ``{- -}`` for Haskell, ``!``
    with no block form for Fortran, and so on.

    Nested block comments (``#[ #[ ]# ]#`` in Nim, and any language whose block
    comments nest) require tracking depth on the way *in* as well as out.
    Looking only for the close once inside a block lets the first close
    terminate the whole comment, so the remainder is miscounted as code and
    SLOC is inflated for any file that nests. SLOC feeds Figure 5 and the
    taxonomy's program-size correlation, so the error would propagate.
    """
    b_open, b_close = block_comment if block_comment else (None, None)
    count = 0
    depth = 0

    for raw in text.splitlines():
        line = raw.strip()
        if not line:
            continue

        has_code = False
        i = 0
        n = len(line)
        while i < n:
            if depth:
                if b_close and line.startswith(b_close, i):
                    depth -= 1
                    i += len(b_close)
                elif b_open and line.startswith(b_open, i):
                    depth += 1
                    i += len(b_open)
                else:
                    i += 1
            else:
                if b_open and line.startswith(b_open, i):
                    depth += 1
                    i += len(b_open)
                elif line_comment and line.startswith(line_comment, i):
                    break
                else:
                    if not line[i].isspace():
                        has_code = True
                    i += 1

        if has_code:
            count += 1

    return count


def load_program(path: Path, lang: LanguageSpec | None = None) -> Program:
    """Parse one source file into a :class:`Program` using its language spec.

    When ``lang`` is ``None`` the file is read with the default ``//`` / ``/*
    */`` C-style markers.
    """
    text = path.read_text(encoding="utf-8", errors="replace")
    lc = lang.line_comment if lang else "//"
    bc = lang.block_comment if lang else ("/*", "*/")
    lines, has_block, sentinel = parse_expected(text, lc)
    expected: tuple[str, ...] | None
    if not has_block or sentinel or not lines:
        expected = None
    else:
        expected = tuple(lines)
    return Program(
        stem=path.stem,
        path=path.resolve(),
        expected=expected,
        has_block=has_block,
        sentinel=sentinel,
        sloc=count_sloc(text, lc, bc),
        lang=lang,
    )


def load_corpus(
    corpus_dir: Path,
    languages: Sequence[LanguageSpec] | None = None,
    start: str | None = None,
) -> list[Program]:
    """Discover and parse every recognised source file under ``corpus_dir``.

    Files are matched by extension against ``languages`` (the built-in set when
    ``None``), and each is parsed with the spec its suffix selects. Exactly one
    extension may be present; see :func:`validate_corpus`.

    Parameters
    ----------
    languages:
        The languages to recognise. ``None`` uses :data:`BUILTIN_LANGUAGES`.
    start:
        Optional resume anchor (bare stem, a filename, or a path). Programs
        sorted before it are skipped.

    Raises
    ------
    MixedCorpusError
        More than one recognised extension is present.
    """
    by_ext = language_registry(languages)
    paths = (
        sorted(p for p in corpus_dir.iterdir() if p.is_file() and p.suffix in by_ext)
        if corpus_dir.is_dir()
        else []
    )
    if not paths:
        exts = ", ".join(sorted(by_ext)) or "(none configured)"
        raise FileNotFoundError(
            f"no source files with a recognised extension ({exts}) in {corpus_dir}"
        )

    _reject_mixed_extensions(paths, corpus_dir)

    if start:
        anchor = Path(start).name
        # Strip a trailing recognised extension, if the anchor carries one.
        stem_anchor = Path(anchor).stem if Path(anchor).suffix in by_ext else anchor
        stems = [p.stem for p in paths]
        if stem_anchor in stems:
            paths = paths[stems.index(stem_anchor) :]
        else:
            raise ValueError(f"start anchor {stem_anchor!r} not found in {corpus_dir}")

    return [load_program(p, by_ext[p.suffix]) for p in paths]


def check_output(stdout: str, program: Program) -> tuple[bool, str | None]:
    """Compare a binary's stdout against the program's embedded expectation.

    Returns ``(passed, detail)``. ``detail`` is ``None`` on success, otherwise a
    human-readable first-mismatch description.

    A program with no expected block is treated as *passing* -- absence of an
    oracle is not evidence of failure -- but callers should surface it as a
    warning via :attr:`Program.checkable`.
    """
    if not program.checkable:
        return True, None

    got_lines = stdout.splitlines()
    assert program.expected is not None
    for i, exp in enumerate(program.expected):
        got = got_lines[i] if i < len(got_lines) else "<no line>"
        if normalize_ws(exp) != normalize_ws(got):
            return False, f"line {i}: expected {exp!r} || got {got!r}"

    if len(got_lines) < len(program.expected):
        return False, f"line {len(got_lines)}: expected <more lines> || got <no line>"

    return True, None

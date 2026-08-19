# compilerdiv -- convenience wrapper around the CLI.
#
# Can be used along `compilerdiv <command>`

PYTHON  ?= python3
CONFIG  ?= myconfig.yaml
START   ?=
compilerdiv := $(PYTHON) -m compilerdiv -c $(CONFIG)

START_ARG := $(if $(START),--start $(START),)

.PHONY: help install install-dev preflight acquire analyze run corpus doctor \
        test test-cov format lint typecheck check clean distclean

# Marker to select a single test area, e.g. `make test M=languages`.
M ?=
M_ARG := $(if $(M),-m $(M),)

help:
	@echo "compilerdiv targets:"
	@echo "  make install       - install the package"
	@echo "  make install-dev   - install with dev + parquet extras"
	@echo "  make preflight     - check strace and the compilers, then stop"
	@echo "  make acquire       - run the sweep (resumable; START=Foo to resume)"
	@echo "  make analyze       - reports and figures from the raw log"
	@echo "  make run           - acquire then analyze"
	@echo "  make corpus        - corpus stats and the SLOC figure"
	@echo "  make doctor        - inspect the raw store"
	@echo "  make test          - run the test suite (make test M=languages for one area)"
	@echo "  make test-cov      - run the test suite with a coverage report"
	@echo "  make format        - reformat the code with black"
	@echo "  make lint          - black --check + pylint + mypy (what CI runs)"
	@echo "  make typecheck     - mypy only"
	@echo "  make check         - lint then test-cov (full local gate)"
	@echo "  make clean         - remove figures and workbooks (KEEPS the raw log)"
	@echo "  make distclean     - remove everything including the raw log"
	@echo ""
	@echo "Variables: CONFIG=$(CONFIG)  START=$(START)  M=<marker>"
	@echo "Test markers: corpus languages config stats noise trace store detectors taxonomy cli"

install:
	$(PYTHON) -m pip install -e .

install-dev:
	$(PYTHON) -m pip install -e ".[dev]"

preflight:
	$(compilerdiv) preflight

acquire:
	$(compilerdiv) acquire $(START_ARG)

analyze:
	$(compilerdiv) analyze

run:
	$(compilerdiv) all $(START_ARG)

corpus:
	$(compilerdiv) corpus

doctor:
	$(compilerdiv) doctor

test:
	$(PYTHON) -m pytest -q $(M_ARG)

test-cov:
	$(PYTHON) -m pytest --cov=compilerdiv --cov-report=term-missing --cov-fail-under=85 $(M_ARG)

# `src` covers both the package and the suite: tests live at src/tests.
format:
	$(PYTHON) -m black src

lint:
	$(PYTHON) -m black --check --diff src
	$(PYTHON) -m pylint src/compilerdiv
	$(PYTHON) -m mypy

typecheck:
	$(PYTHON) -m mypy

check: lint test-cov

# Deliberately does NOT touch results/raw: a sweep costs hours, and figures
# cost seconds. Use distclean to discard acquired data.
clean:
	rm -rf results/figures results/*.xlsx build/compilerdiv_work
	@echo "Removed figures and workbooks. Raw log kept (use 'make distclean' to drop it)."

distclean: clean
	rm -rf results build
	@echo "Removed the raw log too."

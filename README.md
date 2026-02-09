# that's ruffly-all.

Answering *what's necessary to migrate to ruff right now*.

### Motivation

Starting with  `ruff` is simple. But enabling `"ALL"` rules in an
existing application? That’s a challenge.

This project helps identify the minimal set of rules to disable
temporarily, ensuring the CI remains green while allowing maintainers
to iteratively fix their codebase at their own pace.

### Installation

```bash
pip install ruffly-all
```

or

```bash
cargo install ruffly-all
```

### Usage

Find a project you'd like to use `ruff` in, we'll be using `pylint` as
an example[^1].

```bash
cd pylint/
ruffly-all
```

```console
[tool.ruff]
lint.select = [ "ALL" ]
lint.ignore = [
  "D203",    # incompatible to D211
  "D212",    # incompatible to D213
  "ERA",     # https://docs.astral.sh/ruff/rules/#eradicate-era
  "YTT",     # https://docs.astral.sh/ruff/rules/#flake8-2020-ytt
  "ANN",     # https://docs.astral.sh/ruff/rules/#flake8-annotations-ann
  "S",       # https://docs.astral.sh/ruff/rules/#flake8-bandit-s
  "BLE",     # https://docs.astral.sh/ruff/rules/#flake8-blind-except-ble
  "FBT",     # https://docs.astral.sh/ruff/rules/#flake8-boolean-trap-fbt
  "B",       # https://docs.astral.sh/ruff/rules/#flake8-bugbear-b
  "A",       # https://docs.astral.sh/ruff/rules/#flake8-builtins-a
  "COM",     # https://docs.astral.sh/ruff/rules/#flake8-commas-com
  "C4",      # https://docs.astral.sh/ruff/rules/#flake8-comprehensions-c4
  "DTZ",     # https://docs.astral.sh/ruff/rules/#flake8-datetimez-dtz
  "T10",     # https://docs.astral.sh/ruff/rules/#flake8-debugger-t10
  "EM",      # https://docs.astral.sh/ruff/rules/#flake8-errmsg-em
  "EXE",     # https://docs.astral.sh/ruff/rules/#flake8-executable-exe
  "FIX",     # https://docs.astral.sh/ruff/rules/#flake8-fixme-fix
  "ISC",     # https://docs.astral.sh/ruff/rules/#flake8-implicit-str-concat-isc
  "ICN",     # https://docs.astral.sh/ruff/rules/#flake8-import-conventions-icn
  "LOG",     # https://docs.astral.sh/ruff/rules/#flake8-logging-log
  "G",       # https://docs.astral.sh/ruff/rules/#flake8-logging-format-g
  "INP",     # https://docs.astral.sh/ruff/rules/#flake8-no-pep420-inp
  "PIE",     # https://docs.astral.sh/ruff/rules/#flake8-pie-pie
  "T20",     # https://docs.astral.sh/ruff/rules/#flake8-print-t20
  "PYI",     # https://docs.astral.sh/ruff/rules/#flake8-pyi-pyi
  "PT",      # https://docs.astral.sh/ruff/rules/#flake8-pytest-style-pt
  "Q",       # https://docs.astral.sh/ruff/rules/#flake8-quotes-q
  "RSE",     # https://docs.astral.sh/ruff/rules/#flake8-raise-rse
  "RET",     # https://docs.astral.sh/ruff/rules/#flake8-return-ret
  "SLF",     # https://docs.astral.sh/ruff/rules/#flake8-self-slf
  "SIM",     # https://docs.astral.sh/ruff/rules/#flake8-simplify-sim
  "TID",     # https://docs.astral.sh/ruff/rules/#flake8-tidy-imports-tid
  "TD",      # https://docs.astral.sh/ruff/rules/#flake8-todos-td
  "TC",      # https://docs.astral.sh/ruff/rules/#flake8-type-checking-tc
  "ARG",     # https://docs.astral.sh/ruff/rules/#flake8-unused-arguments-arg
  "PTH",     # https://docs.astral.sh/ruff/rules/#flake8-use-pathlib-pth
  "FLY",     # https://docs.astral.sh/ruff/rules/#flynt-fly
  "C90",     # https://docs.astral.sh/ruff/rules/#mccabe-c90
  "NPY",     # https://docs.astral.sh/ruff/rules/#numpy-specific-rules-npy
  "N",       # https://docs.astral.sh/ruff/rules/#pep8-naming-n
  "PERF",    # https://docs.astral.sh/ruff/rules/#perflint-perf
  "E",       # https://docs.astral.sh/ruff/rules/#error-e
  "W",       # https://docs.astral.sh/ruff/rules/#warning-w
  "D",       # https://docs.astral.sh/ruff/rules/#pydocstyle-d
  "F",       # https://docs.astral.sh/ruff/rules/#pyflakes-f
  "PGH",     # https://docs.astral.sh/ruff/rules/#pygrep-hooks-pgh
  "PLC",     # https://docs.astral.sh/ruff/rules/#convention-plc
  "PLE",     # https://docs.astral.sh/ruff/rules/#error-ple
  "PLR",     # https://docs.astral.sh/ruff/rules/#refactor-plr
  "PLW",     # https://docs.astral.sh/ruff/rules/#warning-plw
  "UP",      # https://docs.astral.sh/ruff/rules/#pyupgrade-up
  "FURB",    # https://docs.astral.sh/ruff/rules/#refurb-furb
  "RUF",     # https://docs.astral.sh/ruff/rules/#ruff-specific-rules-ruf
  "TRY",     # https://docs.astral.sh/ruff/rules/#tryceratops-try
]
```

[^1]: While this does look like the `pylint` project had a lot going on,
keep in mind extensive parts of the repo are faulty python snippets
necessary as test resources for `pylint`s unittests.

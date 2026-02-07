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
  "A",
  "ANN",
  "ARG",
  "B",
  "BLE",
  "C",
  "COM",
  "D",
  "DTZ",
  "E",
  "EM",
  "ERA",
  "EXE",
  "F",
  "FBT",
  "FIX",
  "FLY",
  "FURB",
  "G",
  "ICN",
  "INP",
  "ISC",
  "LOG",
  "N",
  "NPY",
  "PERF",
  "PGH",
  "PIE",
  "PLC",
  "PLE",
  "PLR",
  "PLW",
  "PT",
  "PTH",
  "PYI",
  "Q",
  "RET",
  "RSE",
  "RUF",
  "S",
  "SIM",
  "SLF",
  "T",
  "TC",
  "TD",
  "TID",
  "TRY",
  "UP",
  "W",
  "YTT",
]
```

[^1]: While this does look like the `pylint` project had a lot going on,
keep in mind extensive parts of the repo are faulty python snippets
necessary as test resources for `pylint`s unittests.

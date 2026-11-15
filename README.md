# Bayesian Statistics (bayesian-statistics-ai999x)

[![tests](https://github.com/AI999X/bayesian-statistics-ai999x/actions/workflows/ci.yml/badge.svg)](https://github.com/AI999X/bayesian-statistics-ai999x/actions/workflows/ci.yml) [![PyPI](https://img.shields.io/pypi/v/bayesian-statistics-ai999x)](https://pypi.org/project/bayesian-statistics-ai999x/) [![Hugging Face](https://img.shields.io/badge/%F0%9F%A4%97-Hugging%20Face-yellow)](https://huggingface.co/AI999X/bayesian-statistics-ai999x)

Bayesian Statistics covers Bayes' theorem, conjugate priors and posteriors, credible intervals, Bayes factors, Monte Carlo estimation, uncertainty propagation, sensitivity analysis and decision analysis: 78 calculations in 12 topics, each with its formula, its steps and its units, and a plain reason instead of a number when an input is missing or out of range. One compiled engine, four ways to use it: the desktop app, the command line, the browser and Python. Everything runs on your own computer, offline.

[![The Bayesian Statistics desktop app: Probability that one Poisson rate exceeds another (gamma posteriors), with its worked example and live scene](https://www.ai999x.com/?ax_shot=bayesian-statistics-ai999x/hero.webp)](https://www.ai999x.com/models/bayesian-statistics-ai999x/)

## Four ways to use it

| | |
|---|---|
| **Desktop app** (Windows, Linux) | [Windows](https://downloads.ai999x.com/desktop/latest/ai999x-windows-x86_64.zip) · [Linux](https://downloads.ai999x.com/desktop/latest/ai999x-linux-x86_64.tar.gz) |
| **Command line** `bayesian-statistics` (Windows, Linux) | [Windows](https://downloads.ai999x.com/desktop/latest/ai999x-cli-windows-x86_64.zip) · [Linux](https://downloads.ai999x.com/desktop/latest/ai999x-cli-linux-x86_64.tar.gz) |
| **Browser** (any device) | [Try it on the model page](https://www.ai999x.com/models/bayesian-statistics-ai999x/#try) |
| **Python** 3.10+ | `pip install bayesian-statistics-ai999x`: on Windows and Linux (x86-64) `pip` installs the same compiled engine; on other systems (macOS, ARM) it installs a pure-Python build that gives the same answers |

## Example

```python
import bayesian_statistics_ai999x as bayesian_statistics
a = bayesian_statistics.ask("A test has sensitivity 0.9 and specificity 0.95; the prevalence is 0.01. What is the positive predictive value?")
print(a.outcome, a.value)
```

Output:

```text
SOLVED 0.153846153846
```

Every calculation, with its formula, a worked example and its figures: **[Bayesian Statistics on ai999x.com](https://www.ai999x.com/models/bayesian-statistics-ai999x/)**. Install guides for the app and the command line: [https://www.ai999x.com/desktop-cli/](https://www.ai999x.com/desktop-cli/).

Version 1.6.3 · License: Apache-2.0 · Author: Ibnul Jaif Farabi · Organization: AI999X · Contact: support@ai999x.com · [Support the project](https://buymeacoffee.com/ibnuljaiffarabi)

*The Future Is Self-Improving.*

## Source and releases

This repository holds the installable source of each release, exactly as published on [PyPI](https://pypi.org/project/bayesian-statistics-ai999x/) (`pip install .` builds it). Releases are tagged `v1.6.3`; the [GitHub release](https://github.com/AI999X/bayesian-statistics-ai999x/releases/tag/v1.6.3) carries the wheel, the desktop app and the command-line program for Windows and Linux, and their checksums.

## Build the program from source

`rust/` holds the Rust source of the `bayesian-statistics` command-line program and browser module: the AI999X engine (`rust/ax-engine`), this model (`rust/bayesian-statistics`) and its data (`rust/models/bayesian_statistics/pack.json`). With Rust 1.80 or later:

```bash
cd rust
cargo build --release -p bayesian-statistics-ai999x
./target/release/bayesian-statistics --help
cargo test -p bayesian-statistics-ai999x
```

# bayesian-statistics-ai999x — offline, verified bayesian statistics solver with step-by-step answers

[![tests](https://github.com/AI999X/bayesian-statistics-ai999x/actions/workflows/ci.yml/badge.svg)](https://github.com/AI999X/bayesian-statistics-ai999x/actions/workflows/ci.yml) [![PyPI](https://img.shields.io/pypi/v/bayesian-statistics-ai999x)](https://pypi.org/project/bayesian-statistics-ai999x/) [![Hugging Face](https://img.shields.io/badge/%F0%9F%A4%97-Hugging%20Face-yellow)](https://huggingface.co/AI999X/bayesian-statistics-ai999x)

**[Install](https://pypi.org/project/bayesian-statistics-ai999x/)** · **[Capability guide](https://huggingface.co/AI999X/bayesian-statistics-ai999x/tree/main/docs)** · **[Evaluation](https://huggingface.co/AI999X/bayesian-statistics-ai999x/tree/main/evaluation)** · **[AI999X on Hugging Face](https://huggingface.co/AI999X)** · **[Website](https://ai999x.com)** · **[Support: Buy Me a Coffee](https://buymeacoffee.com/ibnuljaiffarabi)**

License: Apache-2.0 · Author: Ibnul Jaif Farabi (creator and engineer) · Organization: AI999X · Version 1.0.0

bayesian-statistics-ai999x is an offline, deterministic Python solver that answers bayesian statistics questions with verified results from 78 certified capabilities, gives 378 documented answers identically on Windows and Linux, and refuses rather than guesses when an answer cannot be verified.

**bayesian-statistics-ai999x** is a compact, deterministic bayesian statistics engine for Python. It reads a question in plain English (or a typed call), picks a certified bayesian statistics capability, computes the answer exactly, checks it, and returns the result with its steps, units, LaTeX and assumptions — or an honest refusal that says what is missing. It runs fully offline on any CPU, needs no GPU, no API key and no internet, and is built to give **zero wrong answers**: when it is not sure, it refuses instead of guessing.

[![Buy me a coffee](https://img.shields.io/badge/Buy%20me%20a%20coffee-support%20development-FFDD00?logo=buymeacoffee&logoColor=black)](https://buymeacoffee.com/ibnuljaiffarabi) ![License](https://img.shields.io/badge/license-Apache--2.0-blue) ![Python](https://img.shields.io/badge/python-3.10%20%7C%203.11%20%7C%203.12%20%7C%203.13%20%7C%203.14-blue) ![Offline](https://img.shields.io/badge/runs-offline%20on%20CPU-green)

## Highlights

- **Verified, not generated.** Every answer is computed by a certified bayesian statistics capability and checked (substituted back, dimension-checked or compared with an independent computation) before it is returned.
- **Zero wrong answers by design.** Seven explicit outcomes — SOLVED, NEEDS_INPUT, AMBIGUOUS, INVALID_INPUT, OUT_OF_SCOPE, UNRECOGNIZED_FORMAT, INTERNAL_ERROR — never a silent guess. Every one of its 378 documented answers is reproduced exactly on every tested platform.
- **Plain-English questions and a guaranteed typed interface.** `ask()` reads documented question forms; `solve(capability_id, **inputs)` always solves a supported task with valid inputs.
- **Explains itself.** Steps, formulas in LaTeX, units, assumptions and the model used, ready for notebooks, tutoring apps and homework checkers.
- **Small and portable.** Pure Python (sympy, mpmath); identical answers on Windows and Linux (Python 3.10, 3.11, 3.12, 3.13, 3.14).
- **Coverage:** Bayes' theorem, conjugate posteriors, credible intervals, Bayes factors, decision analysis, uncertainty propagation, Monte Carlo and sensitivity analysis.

## Model overview

| Property | Value |
|---|---|
| Package | `bayesian-statistics-ai999x` 1.0.0 (import `bayesian_statistics_ai999x`) |
| Type | Deterministic neuro-symbolic bayesian statistics engine (rule-based reading + certified symbolic capabilities) |
| Capabilities | 78 certified procedures |
| Knowledge pack | version 1.0.1 |
| Golden answers | 378 documented answers, identical on every tested platform |
| Platforms tested | Windows (Python 3.13), Linux x86-64 (Python 3.10, 3.11, 3.12, 3.13, 3.14) |
| Runtime | CPU only, offline; no GPU, no network, no API key |
| License | Apache-2.0 |
| Creator / engineer | Ibnul Jaif Farabi |
| Organization | AI999X |

## Quickstart

```bash
pip install bayesian-statistics-ai999x
```

```python
import bayesian_statistics_ai999x as bayesian_statistics

bayesian_statistics.ask("A test has sensitivity 0.9 and specificity 0.95; the prevalence is 0.01. What is the positive predictive value?").show()
# -> SOLVED — ppv = 0.153846, npv = 0.998938, p_positive = 0.0585, false_discovery_rate = 0.846154  (diagnostic_test_predictive_values)
bayesian_statistics.ask("Beta-binomial posterior: prior Beta(2, 2), 7 successes in 10 trials. Find the posterior mean.").show()
# -> SOLVED — post_alpha = 9, post_beta = 5, posterior_mean = 0.642857, posterior_variance = 0.0153061  (beta_binomial_posterior)
bayesian_statistics.ask("95% highest-density interval of a Beta(3, 9) posterior.").show()
# -> SOLVED — lower = 0.0405552, upper = 0.483724, width = 0.443168  (beta_hdi)
bayesian_statistics.ask("Expected value of perfect information: payoffs = [[100, -20], [40, 30]], probabilities = [0.4, 0.6].").show()
# -> SOLVED — evpi = 24, ev_with_perfect_information = 58, best_expected_value = 34  (expected_value_of_perfect_information)
bayesian_statistics.solve("uncertainty_product_quotient", a=12.0, ua=0.3, b=4.0, ub=0.2).show()
# -> SOLVED — product = 48, u_product = 2.68328, quotient = 3, u_quotient = 0.167705  (uncertainty_product_quotient)
bayesian_statistics.ask("Who created you?").message
# -> bayesian-statistics-ai999x was created and engineered by Ibnul Jaif Farabi at AI999X.
```

Command line: `bayesian-statistics-ai999x "<question>"`, `bayesian-statistics-ai999x --capabilities`. The outputs above were produced by running this exact code on bayesian-statistics-ai999x 1.0.0.

## Find the right capability, and draw it

`bayesian_statistics.search()` finds which certified capability a question is about — across **every** AI999X model, so a question for another model points you to it. It runs **[ai999x-encoder](https://huggingface.co/AI999X/ai999x-encoder)**, AI999X's own capability encoder (4.7M parameters, a 4.9 MB ONNX model, offline; the root `model.safetensors` of this repository): `pip install "bayesian-statistics-ai999x[search]"`. `ask(question, suggest=True)` adds the same pointers to a question it does not answer; an answer itself never changes.

```python
import bayesian_statistics_ai999x as bayesian_statistics

bayesian_statistics.search("Solve x^2 - 5x + 6 = 0", k=2)
# -> Roots of a quadratic equation (algebra-ai999x, 0.67); Solution of two linear equations in two unknowns (algebra-ai999x, 0.57)
```

## How every answer is verified

1. **Reading** — the question is turned into typed quantities (value, unit, kind, role) and one asked quantity; every stated number must be used or the question is refused.
2. **Planning** — only certified capabilities may answer; a capability applies only under the conditions the wording states (for example a pendulum, a liquid at rest, boiling).
3. **Computing** — exact symbolic or high-precision numeric evaluation.
4. **Checking** — dimensions, substitution back into the defining equation, and agreement between independent readings; any disagreement becomes a refusal, never a number.

Capabilities are certified before they ship: each must reproduce hand-worked reference cases, reject a deliberately wrong variant (a control), and leave every benchmark answer unchanged.

## Capabilities

78 certified procedures. A few of them:

- **Procedures:** Bayes action: minimum posterior expected loss; Bayes factor for a normal mean (known variance); Bayes factor for a point null on a proportion (beta prior under H1); Bayes factor for two simple binomial hypotheses; Bayes' theorem for an event and its complement; Bayesian A/B test: probability that A beats B; Best action by expected monetary value; Beta prior from a mean and standard deviation; Beta-binomial conjugate posterior; Break-even (indifference) probability of two actions; Certainty equivalent with logarithmic utility; Control-variate Monte Carlo estimate; Covariance propagation through a linear map y = A x; Credible interval for a Poisson rate; Credible interval for a proportion (beta-binomial); Decision threshold probability (treat / do not treat); Decision under uncertainty: maximin, maximax, Laplace, minimax regret, Hurwicz; Dirichlet-multinomial conjugate posterior; Dirichlet-multinomial probability of a count vector (evidence); Effective degrees of freedom and expanded uncertainty (Welch-Satterthwaite); Equal-tailed credible interval of a beta distribution; Equal-tailed credible interval of a gamma distribution; Exact mean and standard deviation of a product of independent normals; Exact mean and variance of a polynomial of a normal variable …

`bayesian_statistics_ai999x.templates()` lists every question form `ask()` reads; the capability guide (PDF, every example executed) is included in this repository.

## Out of scope

- Open-ended explanation, essay writing or tutoring dialogue: it is a solver, not a chatbot.
- Questions outside its certified bayesian statistics capabilities, and any question whose stated numbers it cannot all use — it refuses with the reason instead.
- High-stakes engineering, medical or legal decisions without independent review.

## Limitations

- It answers what its certified capabilities cover (Bayes' theorem, conjugate posteriors, credible intervals, Bayes factors, decision analysis, uncertainty propagation, Monte Carlo and sensitivity analysis); anything else is refused with a reason.
- `ask()` reads documented question forms and common phrasings; unusual wording may be refused — use `solve()` for guaranteed results.
- Results are as correct as the certified relations and their stated conditions; a certified relation is a checked model, not a claim about every physical situation.

## Citation

```bibtex
@software{bayesian_statistics_ai999x_1_0_0,
  title  = {bayesian-statistics-ai999x: an offline, verified bayesian statistics solver},
  author = {Farabi, Ibnul Jaif}, organization = {AI999X},
  year   = {2026}, version = {1.0.0},
  url    = {https://huggingface.co/AI999X/bayesian-statistics-ai999x}
}
```

## Contact

Questions, bug reports and requests: open a discussion on https://huggingface.co/AI999X/bayesian-statistics-ai999x/discussions or visit https://ai999x.com.

## Support the project

bayesian-statistics-ai999x is free and open source. If it helps you, you can support further development on [Buy Me a Coffee](https://buymeacoffee.com/ibnuljaiffarabi).

Created and engineered by **Ibnul Jaif Farabi** · **[AI999X](https://huggingface.co/AI999X)** · [ai999x.com](https://ai999x.com)

## Source and releases

This repository holds the installable source of each release, exactly as published on [PyPI](https://pypi.org/project/bayesian-statistics-ai999x/) (`pip install .` builds it). Releases are tagged `v<version>`; the wheel is attached to each [GitHub release](https://github.com/AI999X/bayesian-statistics-ai999x/releases). The optional ONNX models (formula graphs and, where measured, a decision model) are on [Hugging Face](https://huggingface.co/AI999X/bayesian-statistics-ai999x/tree/main/onnx).

# bayesian-statistics-ai999x: offline, verified bayesian statistics solver with step-by-step answers

[![tests](https://github.com/AI999X/bayesian-statistics-ai999x/actions/workflows/ci.yml/badge.svg)](https://github.com/AI999X/bayesian-statistics-ai999x/actions/workflows/ci.yml) [![PyPI](https://img.shields.io/pypi/v/bayesian-statistics-ai999x)](https://pypi.org/project/bayesian-statistics-ai999x/) [![Hugging Face](https://img.shields.io/badge/%F0%9F%A4%97-Hugging%20Face-yellow)](https://huggingface.co/AI999X/bayesian-statistics-ai999x)

**[Desktop app & command line](https://www.ai999x.com/desktop-cli/)** · **[Compute in your browser](https://www.ai999x.com/models/bayesian-statistics-ai999x/#try)** · **[Install (Python)](https://pypi.org/project/bayesian-statistics-ai999x/)** · **[Guides](https://huggingface.co/AI999X/bayesian-statistics-ai999x/tree/main/docs)** · **[Evaluation](https://huggingface.co/AI999X/bayesian-statistics-ai999x/tree/main/evaluation)** · **[AI999X on Hugging Face](https://huggingface.co/AI999X)** · **[Website](https://ai999x.com)** · **[Support: Buy Me a Coffee](https://buymeacoffee.com/ibnuljaiffarabi)**

License: Apache-2.0 · Author: Ibnul Jaif Farabi (creator and engineer) · Organization: AI999X · Version 1.5.0

Field: **Mathematics & Statistics** · Tasks: Question Answering

bayesian-statistics-ai999x is an offline, deterministic bayesian statistics solver written in Rust: a desktop app, a command-line program, a browser module and a Python package. It answers bayesian statistics questions with verified results from 78 certified capabilities, gives 378 documented answers identically on Windows and Linux, and refuses rather than guesses when an answer cannot be verified.

**bayesian-statistics-ai999x** reads a question in plain English (or a typed call), picks a certified bayesian statistics capability, computes the answer exactly, checks it, and returns the result with its steps, units, LaTeX and assumptions, or a refusal that says what is missing. It runs fully offline on any CPU, needs no GPU, no API key and no internet, and is built to give **zero wrong answers**: when it is not sure, it refuses instead of guessing.

[![Buy me a coffee](https://img.shields.io/badge/Buy%20me%20a%20coffee-support%20development-FFDD00?logo=buymeacoffee&logoColor=black)](https://buymeacoffee.com/ibnuljaiffarabi) ![License](https://img.shields.io/badge/license-Apache--2.0-blue) ![Python](https://img.shields.io/badge/python-3.10%20%7C%203.11%20%7C%203.12%20%7C%203.13%20%7C%203.14-blue) ![Offline](https://img.shields.io/badge/runs-offline%20on%20CPU-green) ![Rust](https://img.shields.io/badge/Rust-Windows%20%7C%20Linux%20%7C%20WebAssembly-orange)

**Run it your way.** bayesian-statistics-ai999x comes as the AI999X desktop app and as the command-line program `bayesian-statistics` for Windows and Linux, runs in your browser as WebAssembly, and installs as a Python package. The desktop app, the command line and the browser run the same compiled Rust program on your own device: offline, no Python needed.

## Get it

| Way to run it | For | Get it |
|---|---|---|
| **AI999X Bayesian Statistics desktop app** 1.5.0 | Windows, Linux. This model in its own app: search, ask in words, forms, live figures, explanations, offline | [Windows (x86-64)](https://downloads.ai999x.com/desktop/1.5.0/ai999x-bayesian-statistics-1.5.0-windows-x86_64.zip) · [Linux (x86-64)](https://downloads.ai999x.com/desktop/1.5.0/ai999x-bayesian-statistics-1.5.0-linux-x86_64.tar.gz) |
| **AI999X desktop app** 1.5.0 | Windows, Linux. Every AI999X model in one app: search, ask in words, forms, live figures, explanations, offline | [Windows (x86-64)](https://downloads.ai999x.com/desktop/1.5.0/ai999x-1.5.0-windows-x86_64.zip) · [Linux (x86-64)](https://downloads.ai999x.com/desktop/1.5.0/ai999x-1.5.0-linux-x86_64.tar.gz) |
| **Command line** (`bayesian-statistics`) | Windows, Linux. One program per model, offline, no Python | [Windows (x86-64)](https://downloads.ai999x.com/desktop/1.5.0/ai999x-cli-1.5.0-windows-x86_64.zip) · [Linux (x86-64)](https://downloads.ai999x.com/desktop/1.5.0/ai999x-cli-1.5.0-linux-x86_64.tar.gz) |
| **In your browser** | any device. Nothing to install | [Compute on the model page](https://www.ai999x.com/models/bayesian-statistics-ai999x/#try) |
| **Python** | Python 3.10+ | `pip install bayesian-statistics-ai999x` |

Checksums: [SHA256SUMS](https://downloads.ai999x.com/desktop/1.5.0/SHA256SUMS) · every version: [https://downloads.ai999x.com/desktop/latest/](https://downloads.ai999x.com/desktop/latest/) · install guide: [desktop & command line](https://www.ai999x.com/desktop-cli/)

## Quick start

### Desktop app

Download **AI999X Bayesian Statistics** for your system (above), unpack it and start it; type a question in words or pick a capability, fill in the form and press Compute. Figures and an explanation of the answer appear beside it. The AI999X desktop app has the same, with Bayesian Statistics among all AI999X models.

Linux, from a terminal:

```bash
curl -LO https://downloads.ai999x.com/desktop/1.5.0/ai999x-bayesian-statistics-1.5.0-linux-x86_64.tar.gz
curl -LO https://downloads.ai999x.com/desktop/1.5.0/SHA256SUMS
sha256sum -c --ignore-missing SHA256SUMS
tar xzf ai999x-bayesian-statistics-1.5.0-linux-x86_64.tar.gz
./ai999x-bayesian-statistics-1.5.0-linux-x86_64/ai999x-bayesian-statistics
```

### Command line

Linux:

```bash
curl -LO https://downloads.ai999x.com/desktop/1.5.0/ai999x-cli-1.5.0-linux-x86_64.tar.gz
curl -LO https://downloads.ai999x.com/desktop/1.5.0/SHA256SUMS
sha256sum -c --ignore-missing SHA256SUMS
mkdir -p ~/.local/share && tar xzf ai999x-cli-1.5.0-linux-x86_64.tar.gz -C ~/.local/share
echo 'export PATH="$HOME/.local/share/ai999x-cli-1.5.0-linux-x86_64:$PATH"' >> ~/.bashrc && source ~/.bashrc
bayesian-statistics --version
```

Windows (PowerShell):

```powershell
$F = "ai999x-cli-1.5.0-windows-x86_64.zip"
Invoke-WebRequest "https://downloads.ai999x.com/desktop/1.5.0/$F" -OutFile $F
Invoke-WebRequest "https://downloads.ai999x.com/desktop/1.5.0/SHA256SUMS" -OutFile SHA256SUMS
(Get-FileHash $F -Algorithm SHA256).Hash.ToLower() -eq ((Select-String -SimpleMatch $F SHA256SUMS).Line -split ' ')[0]
Expand-Archive $F -DestinationPath "$env:LOCALAPPDATA\AI999X" -Force
$D = "$env:LOCALAPPDATA\AI999X\ai999x-cli-1.5.0-windows-x86_64"; $env:Path += ";$D"
[Environment]::SetEnvironmentVariable("Path", [Environment]::GetEnvironmentVariable("Path", "User") + ";$D", "User")
bayesian-statistics --version
```

Every command below was run as shown; the text under it is what the program printed.

**Ask in words**: Write the question as you would say it: the program reads the quantities and their units, picks the capability and answers.

```console
$ bayesian-statistics ask "A test has sensitivity 0.9 and specificity 0.95; the prevalence is 0.01. What is the positive predictive value?"
Read as: Positive and negative predictive values of a test
ppv = 0.1538461538
npv = 0.9989378651
p_positive = 0.0585
false_discovery_rate = 0.8461538462
false_omission_rate = 0.001062134891
  by: Positive and negative predictive values of a test  (Bayes' theorem with the total probability of a positive result)
  prevalence = 0.01
  sensitivity = 0.9
  specificity = 0.95
  assumes: probabilities between 0 and 1; both results possible
```

**What a capability needs**: The equation or method, every input and output with its SI unit. Read this before solve: it tells you the input names.

```console
$ bayesian-statistics info diagnostic_test_predictive_values
Positive and negative predictive values of a test
  Bayes' theorem with the total probability of a positive result
  in   prevalence prevalence (pre-test probability of the condition) [1]
  in   sensitivity sensitivity P(positive | condition) [1]
  in   specificity specificity P(negative | no condition) [1]
  out  ppv      positive predictive value P(condition | positive) [1]
  out  npv      negative predictive value P(no condition | negative) [1]
  out  p_positive probability of a positive result [1]
  out  false_discovery_rate 1 - PPV [1]
  out  false_omission_rate 1 - NPV [1]
```

**Solve**: Inputs as name=value, each with any unit of the right kind. The answer lists every output, the method used, each input converted to SI, and the conditions the answer assumes.

```console
$ bayesian-statistics solve diagnostic_test_predictive_values prevalence=0.01 sensitivity=0.9 specificity=0.95
ppv = 0.1538461538
npv = 0.9989378651
p_positive = 0.0585
false_discovery_rate = 0.8461538462
false_omission_rate = 0.001062134891
  by: Positive and negative predictive values of a test  (Bayes' theorem with the total probability of a positive result)
  prevalence = 0.01
  sensitivity = 0.9
  specificity = 0.95
  assumes: probabilities between 0 and 1; both results possible
```

**Many questions at once**: One JSON request per line in, one JSON answer per line out, in order: for scripts, notebooks and agents (the program starts once). In PowerShell: Get-Content cases.jsonl | bayesian-statistics batch

`cases.jsonl`:

```json
{"key": "diagnostic_test_predictive_values", "inputs": {"prevalence": 0.01, "sensitivity": 0.9, "specificity": 0.95}}
{"key": "beta_binomial_posterior", "inputs": {"alpha": 2.0, "beta": 2.0, "successes": "7", "trials": "10"}}
```

```console
$ bayesian-statistics batch < cases.jsonl
{"outcome":"SOLVED","message":"","value":0.153846153846,"unit":"","solved_for":"ppv","si":[["prevalence",0.01],["sensitivity",0.9],["specificity",0.95]],"values":[["ppv",0.153846153846,""],["npv",0.998937865109,""],["p_positive",0.0585,""],["false_discovery_rate",0.846153846154,""],["false_omission_rate",0.00106213489113,""]],"target":"ppv","phrase":"positive predictive value P(condition | positive)","assumptions":["probabilities between 0 and 1","both results possible"],"name":"Positive and negative predictive values of a test","equation":"Bayes' theorem with the total probability of a positive result"}
{"outcome":"SOLVED","message":"","value":0.642857142857,"unit":"","solved_for":"posterior_mean","si":[["alpha",2.0],["beta",2.0]],"values":[["post_alpha",9.0,""],["post_beta",5.0,""],["posterior_mean",0.642857142857,""],["posterior_variance",0.015306122449,""],["posterior_sd",0.123717914826,""]],"target":"posterior_mean","phrase":"posterior alpha = alpha + k","assumptions":["alpha, beta > 0","0 <= k <= n"],"name":"Beta-binomial conjugate posterior","equation":"conjugate update of the beta prior with the binomial likelihood"}
```

### In your browser

Open [Bayesian Statistics on ai999x.com](https://www.ai999x.com/models/bayesian-statistics-ai999x/#try), choose a capability and press **Compute**: the model runs on your own device as WebAssembly; nothing is sent anywhere. Change any input (any unit of the right kind) and the answer, its explanation and the figures follow.

In your own page: `bayesian_statistics_ai999x.wasm` and the loader `ax-wasm.js` (https://www.ai999x.com/compute/rust/ax-wasm.js).

### Python

```bash
pip install bayesian-statistics-ai999x
```

## Worked examples in every form

The same worked example from the command line, from Python and in the browser. Each command-line output is what the program printed.

### Bayes factor for a normal mean (known variance)

BF10 for H1: mu ~ N(mu0, tau^2) against H0: mu = mu0, from the sample mean of n observations with known sigma; BF01 and the posterior probability of H1 at equal prior odds.

Command line:

```console
$ bayesian-statistics solve bayes_factor_normal_mean sample_mean=0 n=1 sigma=1 mu0=0 prior_sd=1
bf10 = 0.7071067812
bf01 = 1.414213562
log10_bf10 = -0.1505149978
posterior_h1 = 0.4142135624
  by: Bayes factor for a normal mean (known variance)  (Savage-Dickey density ratio of the posterior and prior at mu0)
  sample_mean = 0
  sigma = 1
  mu0 = 0
  prior_sd = 1
  assumes: sigma, tau > 0; n >= 1
```

Python:

```python
import bayesian_statistics_ai999x as m

a = m.solve("bayes_factor_normal_mean", sample_mean="0", n="1", sigma="1", mu0="0", prior_sd="1")
print(a.values or (a.value, a.unit))
```

Browser: [open this capability on the model page](https://www.ai999x.com/models/bayesian-statistics-ai999x/#cap-bayes_factor_normal_mean) and press **Compute in your browser**.

_What this means:_ This calculation determines the strength of evidence for a normal mean differing from a specified value using a known variance. By comparing the posterior and prior densities at the null value, it yields a Bayes factor of 0.707107 for the alternative model against the null. The corresponding inverse Bayes factor is 1.41421, and the logarithmic scale value is -0.150515. With equal prior odds, the posterior probability of the alternative model is estimated as 0.414214. These metrics rely on a sample size of 1, a sample mean of 0, and a prior standard deviation of 1, assuming a known population standard deviation of 1.

### Bayes factor for a point null on a proportion (beta prior under H1)

BF10 for H1: p ~ Beta(alpha, beta) against H0: p = p0 after k successes in n trials (default uniform prior), with BF01 and the posterior probability of H1 at equal prior odds.

Command line:

```console
$ bayesian-statistics solve bayes_factor_point_null_binomial p0=0.5 successes=2 trials=2 alpha=1 beta=1
bf10 = 1.333333333
bf01 = 0.75
log10_bf10 = 0.1249387366
posterior_h1 = 0.5714285714
  by: Bayes factor for a point null on a proportion (beta prior under H1)  (ratio of the beta-binomial marginal likelihood to the binomial likelihood at p0)
  p0 = 0.5
  alpha = 1
  beta = 1
  assumes: 0 < p0 < 1; alpha, beta > 0; 0 <= k <= n
```

Python:

```python
import bayesian_statistics_ai999x as m

a = m.solve("bayes_factor_point_null_binomial", p0="0.5", successes="2", trials="2", alpha="1", beta="1")
print(a.values or (a.value, a.unit))
```

Browser: [open this capability on the model page](https://www.ai999x.com/models/bayesian-statistics-ai999x/#cap-bayes_factor_point_null_binomial) and press **Compute in your browser**.

_What this means:_ This calculation determines the strength of evidence for a beta-distributed proportion against a fixed null value. Given 2 successes in 2 trials, with a null hypothesis of 0.5 and prior parameters alpha of 1 and beta of 1, the resulting Bayes factor is 1.33333. The reciprocal, 0.75, represents evidence for the null. Under equal prior odds, the posterior probability that the alternative model is true is 0.571429.

### Bayes factor for two simple binomial hypotheses

Bayes factor BF10 for H1: p = p1 against H0: p = p0 after k successes in n trials, its log10 and the posterior probability of H1 with equal prior odds.

Command line:

```console
$ bayesian-statistics solve bayes_factor_binomial_simple p0=0.5 p1=0.7 successes=14 trials=20
bf10 = 5.184417905
log10_bf10 = 0.7147000018
posterior_h1 = 0.8383032946
  by: Bayes factor for two simple binomial hypotheses  (likelihood ratio of the two simple hypotheses)
  p0 = 0.5
  p1 = 0.7
  assumes: 0 < p0, p1 < 1; 0 <= k <= n
```

Python:

```python
import bayesian_statistics_ai999x as m

a = m.solve("bayes_factor_binomial_simple", p0="0.5", p1="0.7", successes="14", trials="20")
print(a.values or (a.value, a.unit))
```

Browser: [open this capability on the model page](https://www.ai999x.com/models/bayesian-statistics-ai999x/#cap-bayes_factor_binomial_simple) and press **Compute in your browser**.

_What this means:_ This calculation compares two simple hypotheses about a success probability using observed data consisting of 14 successes in 20 trials. The likelihood ratio yields a Bayes factor of 5.18442 for the hypothesis with probability 0.7 against the hypothesis with probability 0.5. This evidence translates to a log base ten scale value of 0.7147. Assuming equal prior odds between the two hypotheses, the resulting posterior probability that the first hypothesis is true given the data is 0.838303.

### Marginal likelihood (evidence) of binomial data under a beta prior

Prior predictive probability of k successes in n trials when p ~ Beta(alpha, beta), and its logarithm.

Command line:

```console
$ bayesian-statistics solve beta_binomial_evidence alpha=1 beta=1 successes=7 trials=10
marginal_likelihood = 0.09090909091
log_marginal_likelihood = -2.397895273
  by: Marginal likelihood (evidence) of binomial data under a beta prior  (closed form with log-beta functions)
  alpha = 1
  beta = 1
  assumes: alpha, beta > 0; 0 <= k <= n
```

Python:

```python
import bayesian_statistics_ai999x as m

a = m.solve("beta_binomial_evidence", alpha="1", beta="1", successes="7", trials="10")
print(a.values or (a.value, a.unit))
```

Browser: [open this capability on the model page](https://www.ai999x.com/models/bayesian-statistics-ai999x/#cap-beta_binomial_evidence) and press **Compute in your browser**.

_What this means:_ This calculation determines the probability of observing exactly 7 successes out of 10 independent Bernoulli trials, assuming the underlying success rate follows a Beta distribution with parameters alpha equal to 1 and beta equal to 1. The resulting value, 0.0909091, represents the evidence for the data given the prior model. Its natural logarithm is computed as -2.3979, which is often more numerically stable for large numbers of trials.

## Documentation

- **Rust user guide**: the command line, every capability with a command and what it printed: [PDF](https://huggingface.co/AI999X/bayesian-statistics-ai999x/blob/main/docs/bayesian_statistics_ai999x_rust_guide.pdf) · [Markdown](https://huggingface.co/AI999X/bayesian-statistics-ai999x/blob/main/docs/bayesian_statistics_ai999x_rust_guide.md)
- **Python guide**: every capability with a Python example: [PDF](https://huggingface.co/AI999X/bayesian-statistics-ai999x/blob/main/docs/bayesian_statistics_ai999x_capabilities.pdf) · [Markdown](https://huggingface.co/AI999X/bayesian-statistics-ai999x/blob/main/docs/bayesian_statistics_ai999x_capabilities.md)
- **Desktop app & command line**: downloads, install, ask and search, figures, offline use, checksums: https://www.ai999x.com/desktop-cli/
- **Model page**: compute in your browser, every capability explained: https://www.ai999x.com/models/bayesian-statistics-ai999x/

## Highlights

- **Verified, not generated.** Every answer is computed by a certified bayesian statistics capability and checked (substituted back, dimension-checked or compared with an independent computation) before it is returned.
- **Zero wrong answers by design.** Seven explicit outcomes (SOLVED, NEEDS_INPUT, AMBIGUOUS, INVALID_INPUT, OUT_OF_SCOPE, UNRECOGNIZED_FORMAT, INTERNAL_ERROR), never a silent guess. Every one of its 378 documented answers is reproduced exactly on every tested platform.
- **Plain-English questions and a guaranteed typed interface.** `ask()` reads documented question forms; `solve(capability_id, **inputs)` always solves a supported task with valid inputs.
- **Explains itself.** Steps, formulas in LaTeX, units, assumptions and the model used, ready for notebooks, tutoring apps and homework checkers.
- **Native and in the browser.** Every capability also computes natively in Rust: the desktop app and the `bayesian-statistics` command line for Windows and Linux, and WebAssembly on the model page, no Python, no network.
- **Small and portable.** Pure Python (sympy, mpmath); identical answers on Windows and Linux (Python 3.10, 3.11, 3.12, 3.13, 3.14).
- **Coverage:** Bayes' theorem, conjugate posteriors, credible intervals, Bayes factors, decision analysis, uncertainty propagation, Monte Carlo and sensitivity analysis.

## Model overview

| Property | Value |
|---|---|
| Package | `bayesian-statistics-ai999x` 1.5.0 (import `bayesian_statistics_ai999x`) |
| Type | Deterministic neuro-symbolic bayesian statistics engine (rule-based reading + certified symbolic capabilities) |
| Capabilities | 78 certified procedures |
| Knowledge pack | version 1.0.1 |
| Golden answers | 378 documented answers, identical on every tested platform |
| Platforms tested | Windows (Python 3.13), Linux x86-64 (Python 3.10, 3.11, 3.12, 3.13, 3.14) |
| Runtime | CPU only, offline; no GPU, no network, no API key |
| License | Apache-2.0 |
| Creator / engineer | Ibnul Jaif Farabi |
| Organization | AI999X |

## Python quick start

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

The Python package also installs a Python command, `bayesian-statistics-ai999x "<question>"`; for the native program use `bayesian-statistics` (above). The outputs above were produced by running this exact code on bayesian-statistics-ai999x 1.5.0.

## How every answer is verified

1. **Reading**: the question is turned into typed quantities (value, unit, kind, role) and one asked quantity; every stated number must be used or the question is refused.
2. **Planning**: only certified capabilities may answer; a capability applies only under the conditions the wording states (for example a pendulum, a liquid at rest, boiling).
3. **Computing**: exact symbolic or high-precision numeric evaluation.
4. **Checking**: dimensions, substitution back into the defining equation, and agreement between independent readings; any disagreement becomes a refusal, never a number.

Capabilities are certified before they ship: each must reproduce hand-worked reference cases, reject a deliberately wrong variant (a control), and leave every benchmark answer unchanged.

## Python worked examples, with their steps

Each question was put to `bayesian-statistics-ai999x`; the answer is the one it returned, with the steps it took.

**Positive and negative predictive values of a test** · _Bayes' theorem and conditional probability_

> A test has sensitivity 0.9 and specificity 0.95; the prevalence is 0.01. What is the positive predictive value?

→ **ppv = 0.1538461538, npv = 0.9989378651, p_positive = 0.0585, false_discovery_rate = 0.8461538462, false_omission_rate = 0.001062134891**

<details><summary>Steps</summary>

- Positive and negative predictive values of a test: prevalence = 0.01, sensitivity = 0.9, specificity = 0.95 → ppv = 0.1538461538, npv = 0.9989378651, p_positive = 0.0585, false_discovery_rate = 0.8461538462, false_omission_rate = 0.001062134891

</details>

**Beta-binomial conjugate posterior** · _Conjugate priors_

> Beta-binomial posterior: prior Beta(2, 2), 7 successes in 10 trials. Find the posterior mean.

→ **post_alpha = 9, post_beta = 5, posterior_mean = 0.6428571429, posterior_variance = 0.01530612245, posterior_sd = 0.1237179148**

<details><summary>Steps</summary>

- Beta-binomial conjugate posterior: alpha = 2, beta = 2, successes = 7, trials = 10 → post_alpha = 9, post_beta = 5, posterior_mean = 0.6428571429, posterior_variance = 0.01530612245, posterior_sd = 0.1237179148

</details>

**Highest-density interval of a beta posterior** · _Credible intervals_

> 95% highest-density interval of a Beta(3, 9) posterior.

→ **lower = 0.04055516641, upper = 0.4837236593, width = 0.4431684929**

<details><summary>Steps</summary>

- Highest-density interval of a beta posterior: alpha = 3, beta = 9, level = 0.95 → lower = 0.04055516641, upper = 0.4837236593, width = 0.4431684929

</details>

**Expected value of perfect information (EVPI)** · _Decision analysis_

> Expected value of perfect information: payoffs = [[100, -20], [40, 30]], probabilities = [0.4, 0.6].

→ **evpi = 24, ev_with_perfect_information = 58, best_expected_value = 34**

<details><summary>Steps</summary>

- Expected value of perfect information (EVPI): payoffs = [100 -20; 40 30], probabilities = [0.4, 0.6] → evpi = 24, ev_with_perfect_information = 58, best_expected_value = 34

</details>

**Uncertainty of a product and a quotient** · _Uncertainty propagation_

`m.solve("uncertainty_product_quotient", a=12.0, ua=0.3, b=4.0, ub=0.2)`

→ **product = 48, u_product = 2.683281573, quotient = 3, u_quotient = 0.1677050983**

<details><summary>Steps</summary>

- Uncertainty of a product and a quotient: a = 12, ua = 0.3, b = 4, ub = 0.2 → product = 48, u_product = 2.683281573, quotient = 3, u_quotient = 0.1677050983

</details>

**Bayes factor for two simple binomial hypotheses** · _Bayes factors and model comparison_

> Bayes factor for p1 = 0.7 against p0 = 0.5 after 14 successes in 20 trials.

→ **bf10 = 5.184417905, log10_bf10 = 0.7147000018, posterior_h1 = 0.8383032946**

<details><summary>Steps</summary>

- Bayes factor for two simple binomial hypotheses: p0 = 0.5, p1 = 0.7, successes = 14, trials = 20 → bf10 = 5.184417905, log10_bf10 = 0.7147000018, posterior_h1 = 0.8383032946

</details>

## Figures

Every point was computed by `bayesian-statistics-ai999x`.

**Beta prior and posterior after 7 successes in 10 trials**

![Beta prior and posterior after 7 successes in 10 trials, computed by bayesian-statistics-ai999x](https://huggingface.co/AI999X/bayesian-statistics-ai999x/resolve/main/figures/beta-binomial-posterior-density.svg)

_A Beta(2, 2) prior updated with 7 successes in 10 trials: the posterior Beta(9, 5) with mean 0.642857 and standard deviation 0.123718; shaded: the equal-tailed 95 % credible interval [0.385738, 0.861421]. Both densities are drawn from the package's cumulative probabilities at 199 values of θ (each interval's probability divided by its width)._

**Posterior mean for every count of successes and failures**

![Posterior mean for every count of successes and failures, computed by bayesian-statistics-ai999x](https://huggingface.co/AI999X/bayesian-statistics-ai999x/resolve/main/figures/beta-binomial-posterior-heatmap.svg)

_The posterior mean of a success probability under a Beta(2, 2) prior after s successes (rows, 0 to 5) and f failures (columns, 0 to 5), each cell from the package (rounded to 3 decimals): from 0.222 after 5 failures to 0.778 after 5 successes, 0.5 with no data._

**Probability of the condition after repeated tests**

![Probability of the condition after repeated tests, computed by bayesian-statistics-ai999x](https://huggingface.co/AI999X/bayesian-statistics-ai999x/resolve/main/figures/sequential-test-posterior-stems.svg)

_Repeated diagnostic tests with prevalence 0.01, sensitivity 0.9 and specificity 0.95: the package's posterior probability of the condition after k = 1 … 5 positive results (0.153846, 0.765957, 0.983308, 0.999058, 0.999948) and after 1 … 5 negative results (down to 1.3054e-07)._

**Prior and posterior over three hypotheses**

![Prior and posterior over three hypotheses, computed by bayesian-statistics-ai999x](https://huggingface.co/AI999X/bayesian-statistics-ai999x/resolve/main/figures/discrete-bayes-posterior-bars.svg)

_Bayes' rule over 3 hypotheses with priors 0.5, 0.3, 0.2 and likelihoods 0.1, 0.4, 0.7: the package's posterior probabilities 0.16129, 0.387097, 0.451613 (evidence 0.31); the most probable hypothesis is 3 with 0.451613._

**Category probabilities after multinomial counts**

![Category probabilities after multinomial counts, computed by bayesian-statistics-ai999x](https://huggingface.co/AI999X/bayesian-statistics-ai999x/resolve/main/figures/dirichlet-multinomial-posterior-bars.svg)

_A Dirichlet(1, 1, 1) prior over 3 categories updated with the counts 10, 5, 3: the package's posterior Dirichlet(11, 6, 4), posterior means 0.52381, 0.285714, 0.190476; whiskers ±2 posterior standard deviations (0.106479, 0.0963143, 0.083719)._

**Normal posterior shrinking towards the data as n grows**

![Normal posterior shrinking towards the data as n grows, computed by bayesian-statistics-ai999x](https://huggingface.co/AI999X/bayesian-statistics-ai999x/resolve/main/figures/normal-normal-summary-band.svg)

_A normal prior (mean 0, sd 1) and data with σ = 2 and sample mean 1: the package's posterior mean and 95 % credible interval for 17 sample sizes from 1 to 100, moving from the prior towards the data; marked: n = 10, posterior mean = 0.714286, interval [-0.333359, 1.76193]._

## Capabilities

78 certified procedures. A few of them:

- **Procedures:** Bayes action: minimum posterior expected loss; Bayes factor for a normal mean (known variance); Bayes factor for a point null on a proportion (beta prior under H1); Bayes factor for two simple binomial hypotheses; Bayes' theorem for an event and its complement; Bayesian A/B test: probability that A beats B; Best action by expected monetary value; Beta prior from a mean and standard deviation; Beta-binomial conjugate posterior; Break-even (indifference) probability of two actions; Certainty equivalent with logarithmic utility; Control-variate Monte Carlo estimate; Covariance propagation through a linear map y = A x; Credible interval for a Poisson rate; Credible interval for a proportion (beta-binomial); Decision threshold probability (treat / do not treat); Decision under uncertainty: maximin, maximax, Laplace, minimax regret, Hurwicz; Dirichlet-multinomial conjugate posterior; Dirichlet-multinomial probability of a count vector (evidence); Effective degrees of freedom and expanded uncertainty (Welch-Satterthwaite); Equal-tailed credible interval of a beta distribution; Equal-tailed credible interval of a gamma distribution; Exact mean and standard deviation of a product of independent normals; Exact mean and variance of a polynomial of a normal variable …

`bayesian_statistics_ai999x.templates()` lists every question form `ask()` reads; the capability guide (PDF, every example executed) is included in this repository.

## Out of scope

- Open-ended explanation, essay writing or tutoring dialogue: it is a solver, not a chatbot.
- Questions outside its certified bayesian statistics capabilities, and any question whose stated numbers it cannot all use: it refuses with the reason instead.
- High-stakes engineering, medical or legal decisions without independent review.

## Limitations

- It answers what its certified capabilities cover (Bayes' theorem, conjugate posteriors, credible intervals, Bayes factors, decision analysis, uncertainty propagation, Monte Carlo and sensitivity analysis); anything else is refused with a reason.
- `ask()` reads documented question forms and common phrasings; unusual wording may be refused; use `solve()` for guaranteed results.
- Results are as correct as the certified relations and their stated conditions; a certified relation is a checked model, not a claim about every physical situation.

## Citation

```bibtex
@software{bayesian_statistics_ai999x_1_5_0,
  title  = {bayesian-statistics-ai999x: an offline, verified bayesian statistics solver},
  author = {Farabi, Ibnul Jaif}, organization = {AI999X},
  year   = {2026}, version = {1.5.0},
  url    = {https://huggingface.co/AI999X/bayesian-statistics-ai999x}
}
```

## Contact

Questions, bug reports and requests: open a discussion on https://huggingface.co/AI999X/bayesian-statistics-ai999x/discussions or visit https://ai999x.com.

## Support the project

bayesian-statistics-ai999x is free and open source. If it helps you, you can support further development on [Buy Me a Coffee](https://buymeacoffee.com/ibnuljaiffarabi).

Created and engineered by **Ibnul Jaif Farabi** · **[AI999X](https://huggingface.co/AI999X)** · [ai999x.com](https://ai999x.com)

## Source and releases

This repository holds the installable source of each release, exactly as published on [PyPI](https://pypi.org/project/bayesian-statistics-ai999x/) (`pip install .` builds it). Releases are tagged `v1.5.0`; the [GitHub release](https://github.com/AI999X/bayesian-statistics-ai999x/releases/tag/v1.5.0) carries the wheel, the desktop app and the command-line program for Windows and Linux, and their checksums.

## Build the program from source

`rust/` holds the Rust source of the `bayesian-statistics` command-line program and browser module: the AI999X engine (`rust/ax-engine`), this model (`rust/bayesian-statistics`) and its data (`rust/models/bayesian_statistics/pack.json`). With Rust 1.80 or later:

```bash
cd rust
cargo build --release -p bayesian-statistics-ai999x
./target/release/bayesian-statistics --help
cargo test -p bayesian-statistics-ai999x
```

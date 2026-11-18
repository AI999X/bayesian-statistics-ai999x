# Bayesian Statistics: the desktop app, user guide

**Bayesian Statistics**, desktop app 1.6.5 for Windows and Linux. **Creator / engineer:** Ibnul Jaif Farabi · **Company:** AI999X · support@ai999x.com

Bayesian Statistics covers Bayes' theorem, conjugate priors and posteriors, credible intervals, Bayes factors, Monte Carlo estimation, uncertainty propagation, sensitivity analysis and decision analysis. Its desktop app holds all 78 calculations in 12 topics: pick one, change its inputs and the answer, a live scene and charts follow at once. It runs on your own computer with no network connection and nothing else to install. The pictures in this guide were taken in this app.

## Contents

- 1. Get it and start it
- 2. The window
- 3. Compute a calculation
- 4. Ask in words
- 5. The live scene and the charts
- 6. Light and dark
- 7. Every calculation
- 8. Other ways to use it

## 1. Get it and start it

| system | download |
|---|---|
| Windows x86-64 | https://github.com/AI999X/ai999x-desktop/releases/download/v1.6.5/ai999x-bayesian-statistics-1.6.5-windows-x86_64.zip |
| Linux x86-64 | https://github.com/AI999X/ai999x-desktop/releases/download/v1.6.5/ai999x-bayesian-statistics-1.6.5-linux-x86_64.tar.gz |

Windows: Download ai999x-bayesian-statistics-1.6.5-windows-x86_64.zip, right-click it → Extract All, then open the folder and start ai999x-bayesian-statistics.exe. If Windows SmartScreen asks, choose More info → Run anyway.

Linux:

```shell
curl -LO https://github.com/AI999X/ai999x-desktop/releases/download/v1.6.5/ai999x-bayesian-statistics-1.6.5-linux-x86_64.tar.gz
curl -LO https://github.com/AI999X/ai999x-desktop/releases/download/v1.6.5/SHA256SUMS
sha256sum -c --ignore-missing SHA256SUMS
tar xzf ai999x-bayesian-statistics-1.6.5-linux-x86_64.tar.gz
./ai999x-bayesian-statistics-1.6.5-linux-x86_64/ai999x-bayesian-statistics
```

Checksums of every download: https://github.com/AI999X/ai999x-desktop/releases/download/v1.6.5/SHA256SUMS. Install steps for every app: https://www.ai999x.com/desktop-cli/

## 2. The window

![The Bayesian Statistics app, opened on Positive and negative predictive values of a test with its worked example.](https://www.ai999x.com/?ax_shot=bayesian-statistics-ai999x/hero.webp)

*The Bayesian Statistics app, opened on Positive and negative predictive values of a test with its worked example.*

On the left, a search box and the topics with their calculations. In the middle, the chosen calculation: its answer at the top, then the live scene (or its charts), what the answer means and the formula. On the right, its inputs, each with a slider where that makes sense, and the Compute and Reset buttons. At the bottom, a box for a question in words.

## 3. Compute a calculation

Open Bayes' theorem and conditional probability › Positive and negative predictive values of a test (or type part of its name in the search box). It opens with a worked example filled in: “A test has sensitivity 0.9 and specificity 0.95; the prevalence is 0.01. What is the positive predictive value?” The answer: ppv = 0.1538461538, npv = 0.9989378651, p_positive = 0.0585, false_discovery_rate = 0.8461538462.

Change any input (type a number with its unit, or drag its slider) and the answer, the scene and the charts follow; Compute recomputes after typing, Reset brings back the worked example. An input outside the range the formula holds for is refused with the reason, never answered with a guessed number.

![Positive and negative predictive values of a test: the live scene of the worked example.](https://www.ai999x.com/?ax_shot=bayesian-statistics-ai999x/scene-1.webp)

*Positive and negative predictive values of a test: the live scene of the worked example.*

## 4. Ask in words

Type a question as you would say it, for example “A test has sensitivity 0.9 and specificity 0.95; the prevalence is 0.01. What is the positive predictive value?” The app reads the quantities and their units, picks the calculation and answers; the command-line program reads it the same way and printed:

```text
Read as: Positive and negative predictive values of a test
ppv = 0.1538461538
npv = 0.9989378651
```

When a question names a quantity the app cannot place, it says what is missing instead of guessing.

## 5. The live scene and the charts

Every calculation draws a scene from its answer: it moves as an input changes. The Charts button shows charts that vary one input around your values; Enlarge opens one in a large view. Scenes from other topics:

![Beta-binomial conjugate posterior](https://www.ai999x.com/?ax_shot=bayesian-statistics-ai999x/scene-2.webp)

*Beta-binomial conjugate posterior*

![Highest-density interval of a beta posterior](https://www.ai999x.com/?ax_shot=bayesian-statistics-ai999x/scene-3.webp)

*Highest-density interval of a beta posterior*

![Expected value of perfect information (EVPI)](https://www.ai999x.com/?ax_shot=bayesian-statistics-ai999x/scene-4.webp)

*Expected value of perfect information (EVPI)*

![Uncertainty of a product and a quotient](https://www.ai999x.com/?ax_shot=bayesian-statistics-ai999x/scene-5.webp)

*Uncertainty of a product and a quotient*

![Bayes factor for two simple binomial hypotheses](https://www.ai999x.com/?ax_shot=bayesian-statistics-ai999x/scene-6.webp)

*Bayes factor for two simple binomial hypotheses*

![Gauss-Hermite rule for expectations under a normal distribution](https://www.ai999x.com/?ax_shot=bayesian-statistics-ai999x/scene-7.webp)

*Gauss-Hermite rule for expectations under a normal distribution*

![Morris elementary-effects screening](https://www.ai999x.com/?ax_shot=bayesian-statistics-ai999x/scene-8.webp)

*Morris elementary-effects screening*

![Positive and negative predictive values of a test: a chart, enlarged.](https://www.ai999x.com/?ax_shot=bayesian-statistics-ai999x/chart-1.webp)

*Positive and negative predictive values of a test: a chart, enlarged.*

![Beta-binomial conjugate posterior: a chart, enlarged.](https://www.ai999x.com/?ax_shot=bayesian-statistics-ai999x/chart-2.webp)

*Beta-binomial conjugate posterior: a chart, enlarged.*

## 6. Light and dark

The app follows a light or a dark theme (the switch is in its top bar).

![The Bayesian Statistics app in the dark theme.](https://www.ai999x.com/?ax_shot=bayesian-statistics-ai999x/hero-dark.webp)

*The Bayesian Statistics app in the dark theme.*

## 7. Every calculation

78 calculations in 12 topics; each has its own entry on the model's page (https://www.ai999x.com/models/bayesian-statistics-ai999x/) with its formula, worked example and charts.

| topic | calculations |
|---|---|
| Bayes factors and model comparison | Bayes factor for a normal mean (known variance), Bayes factor for a point null on a proportion (beta prior under H1), Bayes factor for two simple binomial hypotheses, Marginal likelihood (evidence) of binomial data under a beta prior, Posterior model probabilities from log evidences, Posterior probability of a hypothesis from a Bayes factor |
| Bayes' theorem and conditional probability | Bayes' theorem for an event and its complement, Law of total probability, Positive and negative predictive values of a test, Posterior after repeated independent tests, Posterior over several discrete hypotheses |
| Conjugate priors | Bayesian A/B test: probability that A beats B, Beta-binomial conjugate posterior, Dirichlet-multinomial conjugate posterior, Dirichlet-multinomial probability of a count vector (evidence), Gamma-exponential conjugate posterior of a rate, Gamma-Poisson conjugate posterior of a rate, Laplace's rule of succession, MAP estimate of a Poisson rate (gamma prior), MAP estimate of a proportion (beta-binomial), Normal mean with known variance: posterior and credible interval from a sample mean, Normal mean with known variance: posterior from data, Normal model with unknown mean and variance (normal-inverse-gamma), Posterior predictive count (negative binomial), Posterior predictive distribution of a new observation, Posterior predictive probability (beta-binomial), Posterior probability that p exceeds a value, Probability that one Poisson rate exceeds another (gamma posteriors) |
| Credible intervals | Credible interval for a Poisson rate, Credible interval for a proportion (beta-binomial), Equal-tailed credible interval of a beta distribution, Equal-tailed credible interval of a gamma distribution, Highest-density interval of a beta posterior, Highest-density interval of a gamma posterior, Jeffreys interval for a binomial proportion |
| Decision analysis | Bayes action: minimum posterior expected loss, Best action by expected monetary value, Break-even (indifference) probability of two actions, Certainty equivalent with logarithmic utility, Decision threshold probability (treat / do not treat), Decision under uncertainty: maximin, maximax, Laplace, minimax regret, Hurwicz, Expected utility and certainty equivalent (exponential utility), Expected value of perfect information (EVPI), Expected value of sample information (EVSI) |
| Information and quadrature for Bayesian computation | Gauss-Hermite rule for expectations under a normal distribution, Kullback-Leibler divergence between two beta distributions, Kullback-Leibler divergence between two normal distributions |
| Monte Carlo estimation | Control-variate Monte Carlo estimate, Importance sampling estimate and effective sample size, Monte Carlo estimate of an integral from uniform samples, Number of Monte Carlo draws for a target accuracy, Point of the Halton low-discrepancy sequence, Stratified sampling estimate and standard error |
| Odds and likelihood ratios | Likelihood ratios of a diagnostic test, Odds and log-odds of a probability, Post-test probability from a likelihood ratio, Probability from odds |
| Posterior approximation | Laplace (normal) approximation of a beta posterior |
| Prior elicitation | Beta prior from a mean and standard deviation |
| Sensitivity analysis | Morris elementary-effects screening, Sobol indices of a linear (additive) model, Sobol indices of a model with an interaction term, Sobol indices of Sobol's G-function, Sobol sensitivity indices of the Ishigami function |
| Uncertainty propagation | Covariance propagation through a linear map y = A x, Effective degrees of freedom and expanded uncertainty (Welch-Satterthwaite), Exact mean and standard deviation of a product of independent normals, Exact mean and variance of a polynomial of a normal variable, Exact versus first-order propagation through exp(X), Inverse-variance weighted mean of measurements, Type B standard uncertainty from a stated half-width, Uncertainty of a function of one variable, Uncertainty of a linear combination of correlated inputs, Uncertainty of a power x^n, Uncertainty of a power-law product y = k x1^p1 x2^p2 ..., Uncertainty of a product and a quotient, Uncertainty of a product and quotient with correlation, Uncertainty of a sum or linear combination (independent inputs) |

## 8. Other ways to use it

The same engine runs on the command line (`bayesian-statistics`, its own guide), in Python (`pip install bayesian-statistics-ai999x`, `import bayesian_statistics_ai999x`, its own guide) and in the browser on https://www.ai999x.com/models/bayesian-statistics-ai999x/.

Licence: Apache-2.0 · AI999X · https://www.ai999x.com/models/bayesian-statistics-ai999x/

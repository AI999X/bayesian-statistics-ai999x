"""bayesian-statistics-ai999x — offline Bayesian statistics answers with their formula, steps and verification.

    import bayesian_statistics_ai999x as m
    m.ask("A test has sensitivity 0.9 and specificity 0.95; the prevalence is 0.01. "
          "What is the positive predictive value?").show()          # PPV = 0.1538...
    m.ask("Beta-binomial posterior: prior Beta(2, 2), 7 successes in 10 trials. Find the posterior mean.").show()
    m.solve("uncertainty_product_quotient", {"a": 12.0, "ua": 0.3, "b": 4.0, "ub": 0.2})   # u(ab) = 2.683...
    m.capabilities()                   # every certified capability

Covers Bayes' theorem and diagnostic reasoning (predictive values, odds, likelihood ratios, repeated tests);
conjugate updating (beta-binomial, gamma-Poisson, gamma-exponential, normal-normal, normal-inverse-gamma,
Dirichlet-multinomial); equal-tailed and highest-density credible intervals; posterior predictive probabilities; MAP
and Laplace approximations; Bayes factors and posterior model probabilities; decision analysis (expected value,
EVPI, EVSI, Bayes actions, utility and certainty equivalents, decision criteria, thresholds); first-order and exact
uncertainty propagation (GUM, Welch-Satterthwaite, type B); deterministic Monte Carlo tools (sample size, importance
sampling, control variates, stratification, Halton points, Gauss-Hermite rules); variance-based sensitivity analysis
(Sobol indices of analytic models, Morris elementary effects); KL divergences. Every capability is certified against
independent library computations; a question that is not determined is refused rather than guessed.

Creator / engineer: Ibnul Jaif Farabi. Company: AI999X.
"""
from __future__ import annotations

from pathlib import Path

try:                                       # the installed package carries its own copy of the engine
    from ._engine import Answer
    from ._engine.product import Product
except ImportError:                        # the factory's source tree, during development
    from ai999x_factory.runtime import Answer
    from ai999x_factory.runtime.product import Product

__version__ = "1.0.3"
_P = Product("bayesian_statistics", [Path(__file__).resolve().parent / "pack", Path(__file__).resolve().parents[3]])
pack, ask, solve, compile_problem = _P.pack, _P.ask, _P.solve, _P.compile_problem
formula, formulas, quantities, conditions, capabilities = _P.formula, _P.formulas, _P.quantities, _P.conditions, _P.capabilities
examples, family_examples, structured_example, to_si = _P.examples, _P.family_examples, _P.structured_example, _P.to_si
templates, procedure_examples = _P.templates, _P.procedure_examples
prove, derivations, about = _P.prove, _P.derivations, (lambda: {**_P.about(), "version": __version__})
knowledge, lookup, related = _P.knowledge, _P.lookup, _P.related
plot, plot_relation, search = _P.plot, _P.plot_relation, _P.search   # figures (SVG) and capability search ([search])
__all__ = ["ask", "solve", "prove", "derivations", "about", "formula", "formulas", "quantities", "conditions", "capabilities",
           "compile_problem", "pack", "Answer", "plot", "plot_relation", "search", "__version__"]

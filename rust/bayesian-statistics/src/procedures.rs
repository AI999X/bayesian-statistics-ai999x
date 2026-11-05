//! bayesian_statistics: procedures translated from the package's Python by tools/py2rust.py (do not edit; regenerate).
//! Source: models/bayesian_statistics/capabilities/procedures.py. Each keeps Python's number rules through
//! ax_engine::py and is checked against the package by tools/rust_accuracy.py before the build is listed.
#![allow(clippy::all, non_snake_case, unused_parens, unused_braces, unused_mut, unused_variables, unreachable_code)]

use ax_engine::py::{self, Py, PyErr, R};
use ax_engine::Val;
use serde_json::Value;
use std::collections::HashMap;

/// The procedures this crate implements (tools/export_rust_pack.py reads this list).
pub const NATIVE: &[&str] = &[
    "ab_test_probability_beta",
    "bayes_action_minimum_expected_loss",
    "bayes_factor_binomial_simple",
    "bayes_factor_normal_mean",
    "bayes_factor_point_null_binomial",
    "bayes_theorem",
    "beta_binomial_credible_interval",
    "beta_binomial_evidence",
    "beta_binomial_posterior",
    "beta_binomial_predictive",
    "beta_credible_interval",
    "beta_hdi",
    "beta_posterior_tail_probability",
    "beta_prior_from_mean_sd",
    "break_even_probability",
    "certainty_equivalent_exponential_utility",
    "certainty_equivalent_log_utility",
    "control_variate_estimate",
    "covariance_propagation_linear",
    "decision_criteria_under_uncertainty",
    "diagnostic_test_predictive_values",
    "dirichlet_multinomial_evidence",
    "dirichlet_multinomial_posterior",
    "discrete_bayes_posterior",
    "expected_value_decision",
    "expected_value_of_perfect_information",
    "expected_value_of_sample_information",
    "gamma_credible_interval",
    "gamma_exponential_posterior",
    "gamma_hdi",
    "gamma_poisson_credible_interval",
    "gamma_poisson_posterior",
    "gamma_poisson_predictive",
    "gauss_hermite_rule",
    "halton_point",
    "importance_sampling_estimate",
    "inverse_variance_weighted_mean",
    "jeffreys_interval",
    "kl_divergence_beta",
    "kl_divergence_normal",
    "laplace_approximation_beta",
    "likelihood_ratios",
    "lognormal_propagation",
    "map_beta_binomial",
    "map_gamma_poisson",
    "monte_carlo_integration",
    "monte_carlo_sample_size",
    "morris_elementary_effects",
    "normal_inverse_gamma_posterior",
    "normal_normal_posterior",
    "normal_normal_summary",
    "normal_posterior_predictive",
    "odds_to_probability",
    "polynomial_of_normal_moments",
    "posterior_from_bayes_factor",
    "posterior_from_likelihood_ratio",
    "posterior_model_probabilities",
    "probability_rate_greater_gamma",
    "probability_to_odds",
    "product_of_normals_exact_moments",
    "rule_of_succession",
    "sequential_test_posterior",
    "sobol_indices_g_function",
    "sobol_indices_interaction_model",
    "sobol_indices_ishigami",
    "sobol_indices_linear_model",
    "stratified_sampling_estimate",
    "total_probability",
    "treatment_threshold",
    "type_b_standard_uncertainty",
    "uncertainty_function",
    "uncertainty_linear_correlated",
    "uncertainty_power",
    "uncertainty_power_law",
    "uncertainty_product_correlated",
    "uncertainty_product_quotient",
    "uncertainty_sum",
    "welch_satterthwaite_expanded_uncertainty",
];

type Out = Result<(Vec<(String, Value)>, f64), String>;

/// The run's result as the engine's answer: a dict of outputs and the residual of the package's own check; a
/// ValueError is the refusal's reason; any other error is an internal error (None).
fn finish(r: R, residual: impl Fn(&Py, &Py) -> f64, v: &HashMap<String, Val>) -> Option<Out> {
    let o = match r {
        Ok(o) => o,
        Err(PyErr::Value(m)) => return Some(Err(m)),
        Err(e) => {
            if std::env::var_os("AX_DEBUG").is_some() {        // the error behind an internal error, for diagnosis
                eprintln!("internal error: {e:?}");
            }
            return None;
        }
    };
    let Py::Dict(d) = &o else { return None };
    let outs: Vec<(String, Value)> = d.borrow().iter().map(|(k, x)| (py::to_str(k), py::to_json(x))).collect();
    let i = Py::dict(v.iter().map(|(k, x)| (Py::str(k), py::from_val(x))).collect());
    let res = residual(&i, &o);
    if std::env::var_os("AX_DEBUG").is_some() && !(res <= 1e-9) {
        eprintln!("check failed: residual {res}");
    }
    Some(Ok((outs, res)))
}

/// The engine's entry point (ax_engine::pack::NativeFn): None for a key that is not one of these procedures.
pub fn native(key: &str, v: &HashMap<String, Val>) -> Option<Out> {
    match key {
        "ab_test_probability_beta" => {
            let r = s_1({ let mut d = vec![]; for k in ["alpha_a", "beta_a", "alpha_b", "beta_b"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_21(i.clone(), o.clone(), Py::None).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "bayes_action_minimum_expected_loss" => {
            let r = s_51({ let mut d = vec![]; for k in ["losses", "probabilities"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_98(i.clone(), o.clone(), Py::None).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "bayes_factor_binomial_simple" => {
            let r = s_128({ let mut d = vec![]; for k in ["successes", "trials", "p0", "p1"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_155(i.clone(), o.clone(), Py::None).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "bayes_factor_normal_mean" => {
            let r = s_185({ let mut d = vec![]; for k in ["sample_mean", "n", "sigma", "mu0", "prior_sd"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_205(i.clone(), o.clone(), Py::None).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "bayes_factor_point_null_binomial" => {
            let r = s_235({ let mut d = vec![]; for k in ["successes", "trials", "p0", "alpha", "beta"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_268(i.clone(), o.clone(), Py::None).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "bayes_theorem" => {
            let r = s_298({ let mut d = vec![]; for k in ["p_a", "p_b_given_a", "p_b_given_not_a"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_319(i.clone(), o.clone(), Py::None).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "beta_binomial_credible_interval" => {
            let r = s_349({ let mut d = vec![]; for k in ["alpha", "beta", "successes", "trials", "level"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_457(i.clone(), o.clone(), Py::None).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "beta_binomial_evidence" => {
            let r = s_487({ let mut d = vec![]; for k in ["alpha", "beta", "successes", "trials"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_510(i.clone(), o.clone(), Py::None).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "beta_binomial_posterior" => {
            let r = s_540({ let mut d = vec![]; for k in ["alpha", "beta", "successes", "trials"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_564(i.clone(), o.clone(), Py::None).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "beta_binomial_predictive" => {
            let r = s_594({ let mut d = vec![]; for k in ["alpha", "beta", "future_trials", "future_successes"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_618(i.clone(), o.clone(), Py::None).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "beta_credible_interval" => {
            let r = s_648({ let mut d = vec![]; for k in ["alpha", "beta", "level"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_746(i.clone(), o.clone(), Py::None).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "beta_hdi" => {
            let r = s_776({ let mut d = vec![]; for k in ["alpha", "beta", "level"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_889(i.clone(), o.clone(), Py::None).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "beta_posterior_tail_probability" => {
            let r = s_919({ let mut d = vec![]; for k in ["alpha", "beta", "threshold", "successes", "trials"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_975(i.clone(), o.clone(), Py::None).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "beta_prior_from_mean_sd" => {
            let r = s_1005({ let mut d = vec![]; for k in ["mean", "sd"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1028(i.clone(), o.clone(), Py::None).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "break_even_probability" => {
            let r = s_1058({ let mut d = vec![]; for k in ["payoffs"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1098(i.clone(), o.clone(), Py::None).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "certainty_equivalent_exponential_utility" => {
            let r = s_1128({ let mut d = vec![]; for k in ["outcomes", "probabilities", "risk_tolerance"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1166(i.clone(), o.clone(), Py::None).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "certainty_equivalent_log_utility" => {
            let r = s_1196({ let mut d = vec![]; for k in ["outcomes", "probabilities"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1227(i.clone(), o.clone(), Py::None).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "control_variate_estimate" => {
            let r = s_1257({ let mut d = vec![]; for k in ["y", "c", "control_mean"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1286(i.clone(), o.clone(), Py::None).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "covariance_propagation_linear" => {
            let r = s_1316({ let mut d = vec![]; for k in ["matrix", "covariance"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1375(i.clone(), o.clone(), Py::None).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "decision_criteria_under_uncertainty" => {
            let r = s_1405({ let mut d = vec![]; for k in ["payoffs", "criterion", "optimism"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1467(i.clone(), o.clone(), Py::None).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "diagnostic_test_predictive_values" => {
            let r = s_1497({ let mut d = vec![]; for k in ["prevalence", "sensitivity", "specificity"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1522(i.clone(), o.clone(), Py::None).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "dirichlet_multinomial_evidence" => {
            let r = s_1552({ let mut d = vec![]; for k in ["alpha", "counts"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1585(i.clone(), o.clone(), Py::None).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "dirichlet_multinomial_posterior" => {
            let r = s_1615({ let mut d = vec![]; for k in ["alpha", "counts"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1648(i.clone(), o.clone(), Py::None).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "discrete_bayes_posterior" => {
            let r = s_1678({ let mut d = vec![]; for k in ["priors", "likelihoods"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1710(i.clone(), o.clone(), Py::None).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "expected_value_decision" => {
            let r = s_1740({ let mut d = vec![]; for k in ["payoffs", "probabilities"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1787(i.clone(), o.clone(), Py::None).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "expected_value_of_perfect_information" => {
            let r = s_1817({ let mut d = vec![]; for k in ["payoffs", "probabilities"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1868(i.clone(), o.clone(), Py::None).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "expected_value_of_sample_information" => {
            let r = s_1898({ let mut d = vec![]; for k in ["payoffs", "priors", "likelihoods"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1959(i.clone(), o.clone(), Py::None).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "gamma_credible_interval" => {
            let r = s_1989({ let mut d = vec![]; for k in ["shape", "rate", "level"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2058(i.clone(), o.clone(), Py::None).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "gamma_exponential_posterior" => {
            let r = s_2088({ let mut d = vec![]; for k in ["shape", "rate", "times"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2114(i.clone(), o.clone(), Py::None).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "gamma_hdi" => {
            let r = s_2144({ let mut d = vec![]; for k in ["shape", "rate", "level"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2223(i.clone(), o.clone(), Py::None).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "gamma_poisson_credible_interval" => {
            let r = s_2253({ let mut d = vec![]; for k in ["shape", "rate", "counts", "level"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2338(i.clone(), o.clone(), Py::None).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "gamma_poisson_posterior" => {
            let r = s_2368({ let mut d = vec![]; for k in ["shape", "rate", "counts"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2398(i.clone(), o.clone(), Py::None).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "gamma_poisson_predictive" => {
            let r = s_2428({ let mut d = vec![]; for k in ["shape", "rate", "events"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2448(i.clone(), o.clone(), Py::None).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "gauss_hermite_rule" => {
            let r = s_2478({ let mut d = vec![]; for k in ["n"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2534(i.clone(), o.clone(), Py::None).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "halton_point" => {
            let r = s_2564({ let mut d = vec![]; for k in ["index", "dimension"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2592(i.clone(), o.clone(), Py::None).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "importance_sampling_estimate" => {
            let r = s_2622({ let mut d = vec![]; for k in ["values", "target_density", "proposal_density"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2659(i.clone(), o.clone(), Py::None).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "inverse_variance_weighted_mean" => {
            let r = s_2689({ let mut d = vec![]; for k in ["values", "uncertainties"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2716(i.clone(), o.clone(), Py::None).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "jeffreys_interval" => {
            let r = s_2746({ let mut d = vec![]; for k in ["successes", "trials", "level"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2853(i.clone(), o.clone(), Py::None).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "kl_divergence_beta" => {
            let r = s_2883({ let mut d = vec![]; for k in ["alpha1", "beta1", "alpha2", "beta2"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2901(i.clone(), o.clone(), Py::None).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "kl_divergence_normal" => {
            let r = s_2931({ let mut d = vec![]; for k in ["mean1", "sd1", "mean2", "sd2"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2946(i.clone(), o.clone(), Py::None).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "laplace_approximation_beta" => {
            let r = s_2976({ let mut d = vec![]; for k in ["alpha", "beta", "level"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3028(i.clone(), o.clone(), Py::None).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "likelihood_ratios" => {
            let r = s_3058({ let mut d = vec![]; for k in ["sensitivity", "specificity"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3088(i.clone(), o.clone(), Py::None).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "lognormal_propagation" => {
            let r = s_3118({ let mut d = vec![]; for k in ["mu", "sigma"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3134(i.clone(), o.clone(), Py::None).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "map_beta_binomial" => {
            let r = s_3164({ let mut d = vec![]; for k in ["alpha", "beta", "successes", "trials"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3193(i.clone(), o.clone(), Py::None).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "map_gamma_poisson" => {
            let r = s_3223({ let mut d = vec![]; for k in ["shape", "rate", "counts"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3255(i.clone(), o.clone(), Py::None).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "monte_carlo_integration" => {
            let r = s_3285({ let mut d = vec![]; for k in ["values", "lower", "upper"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3308(i.clone(), o.clone(), Py::None).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "monte_carlo_sample_size" => {
            let r = s_3338({ let mut d = vec![]; for k in ["sigma", "half_width", "level"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3385(i.clone(), o.clone(), Py::None).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "morris_elementary_effects" => {
            let r = s_3415({ let mut d = vec![]; for k in ["outputs", "delta"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3461(i.clone(), o.clone(), Py::None).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "normal_inverse_gamma_posterior" => {
            let r = s_3491({ let mut d = vec![]; for k in ["mu0", "kappa0", "alpha0", "beta0", "data", "level"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3579(i.clone(), o.clone(), Py::None).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "normal_normal_posterior" => {
            let r = s_3609({ let mut d = vec![]; for k in ["prior_mean", "prior_sd", "sigma", "data"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3632(i.clone(), o.clone(), Py::None).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "normal_normal_summary" => {
            let r = s_3662({ let mut d = vec![]; for k in ["prior_mean", "prior_sd", "sigma", "sample_mean", "n", "level"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3716(i.clone(), o.clone(), Py::None).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "normal_posterior_predictive" => {
            let r = s_3746({ let mut d = vec![]; for k in ["prior_mean", "prior_sd", "sigma", "sample_mean", "n", "threshold"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3767(i.clone(), o.clone(), Py::None).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "odds_to_probability" => {
            let r = s_3797({ let mut d = vec![]; for k in ["odds"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3810(i.clone(), o.clone(), Py::None).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "polynomial_of_normal_moments" => {
            let r = s_3840({ let mut d = vec![]; for k in ["coefficients", "mean", "sd"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3869(i.clone(), o.clone(), Py::None).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "posterior_from_bayes_factor" => {
            let r = s_3899({ let mut d = vec![]; for k in ["bayes_factor", "prior"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3921(i.clone(), o.clone(), Py::None).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "posterior_from_likelihood_ratio" => {
            let r = s_3951({ let mut d = vec![]; for k in ["prior", "likelihood_ratio"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3971(i.clone(), o.clone(), Py::None).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "posterior_model_probabilities" => {
            let r = s_4001({ let mut d = vec![]; for k in ["log_evidences", "priors"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_4049(i.clone(), o.clone(), Py::None).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "probability_rate_greater_gamma" => {
            let r = s_4079({ let mut d = vec![]; for k in ["shape1", "rate1", "shape2", "rate2"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_4119(i.clone(), o.clone(), Py::None).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "probability_to_odds" => {
            let r = s_4149({ let mut d = vec![]; for k in ["probability"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_4167(i.clone(), o.clone(), Py::None).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "product_of_normals_exact_moments" => {
            let r = s_4197({ let mut d = vec![]; for k in ["mean_x", "sd_x", "mean_y", "sd_y"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_4212(i.clone(), o.clone(), Py::None).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "rule_of_succession" => {
            let r = s_4242({ let mut d = vec![]; for k in ["successes", "trials"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_4260(i.clone(), o.clone(), Py::None).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "sequential_test_posterior" => {
            let r = s_4290({ let mut d = vec![]; for k in ["prevalence", "sensitivity", "specificity", "positives", "negatives"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_4332(i.clone(), o.clone(), Py::None).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "sobol_indices_g_function" => {
            let r = s_4362({ let mut d = vec![]; for k in ["a"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_4388(i.clone(), o.clone(), Py::None).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "sobol_indices_interaction_model" => {
            let r = s_4418({ let mut d = vec![]; for k in ["a1", "a2", "a12", "mean1", "sd1", "mean2", "sd2"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_4437(i.clone(), o.clone(), Py::None).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "sobol_indices_ishigami" => {
            let r = s_4467({ let mut d = vec![]; for k in ["a", "b"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_4482(i.clone(), o.clone(), Py::None).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "sobol_indices_linear_model" => {
            let r = s_4512({ let mut d = vec![]; for k in ["coefficients", "sds"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_4539(i.clone(), o.clone(), Py::None).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "stratified_sampling_estimate" => {
            let r = s_4569({ let mut d = vec![]; for k in ["weights", "means", "sds", "sizes"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_4607(i.clone(), o.clone(), Py::None).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "total_probability" => {
            let r = s_4637({ let mut d = vec![]; for k in ["priors", "conditionals"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_4667(i.clone(), o.clone(), Py::None).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "treatment_threshold" => {
            let r = s_4697({ let mut d = vec![]; for k in ["loss_fp", "loss_fn", "probability"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_4724(i.clone(), o.clone(), Py::None).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "type_b_standard_uncertainty" => {
            let r = s_4754({ let mut d = vec![]; for k in ["half_width", "distribution"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_4769(i.clone(), o.clone(), Py::None).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "uncertainty_function" => {
            let r = s_4799({ let mut d = vec![]; for k in ["x", "ux", "function"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_4831(i.clone(), o.clone(), Py::None).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "uncertainty_linear_correlated" => {
            let r = s_4861({ let mut d = vec![]; for k in ["values", "uncertainties", "correlation", "coefficients"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_4935(i.clone(), o.clone(), Py::None).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "uncertainty_power" => {
            let r = s_4965({ let mut d = vec![]; for k in ["x", "ux", "exponent"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_4991(i.clone(), o.clone(), Py::None).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "uncertainty_power_law" => {
            let r = s_5021({ let mut d = vec![]; for k in ["coefficient", "values", "exponents", "uncertainties"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_5050(i.clone(), o.clone(), Py::None).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "uncertainty_product_correlated" => {
            let r = s_5080({ let mut d = vec![]; for k in ["a", "ua", "b", "ub", "rho"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_5102(i.clone(), o.clone(), Py::None).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "uncertainty_product_quotient" => {
            let r = s_5132({ let mut d = vec![]; for k in ["a", "ua", "b", "ub"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_5155(i.clone(), o.clone(), Py::None).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "uncertainty_sum" => {
            let r = s_5185({ let mut d = vec![]; for k in ["values", "uncertainties", "coefficients"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_5213(i.clone(), o.clone(), Py::None).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "welch_satterthwaite_expanded_uncertainty" => {
            let r = s_5243({ let mut d = vec![]; for k in ["uncertainties", "dof", "level"] { if let Some(x) = v.get(k) { d.push((Py::str(k), py::from_val(x))); } } Py::dict(d) });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_5332(i.clone(), o.clone(), Py::None).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        _ => None,
    }
}

#[allow(non_snake_case)]
fn c__CRIT() -> R {
    thread_local!(static V: R = (|| -> R { Ok(Py::list(vec![Py::str("maximin"), Py::str("maximax"), Py::str("laplace"), Py::str("minimax regret"), Py::str("hurwicz")])) })());
    V.with(|v| v.clone())
}

#[allow(non_snake_case)]
fn c__FUNS() -> R {
    thread_local!(static V: R = (|| -> R { Ok(Py::list(vec![Py::str("exp"), Py::str("ln"), Py::str("log10"), Py::str("sqrt"), Py::str("sin"), Py::str("cos"), Py::str("tan"), Py::str("reciprocal")])) })());
    V.with(|v| v.clone())
}

#[allow(non_snake_case)]
fn c__LN_SQRT_2PI() -> R {
    thread_local!(static V: R = (|| -> R { Ok(Py::Float(0.9189385332046727)) })());
    V.with(|v| v.clone())
}

#[allow(non_snake_case)]
fn c__PRIMES() -> R {
    thread_local!(static V: R = (|| -> R { Ok(Py::list(vec![Py::Int(2i64), Py::Int(3i64), Py::Int(5i64), Py::Int(7i64), Py::Int(11i64), Py::Int(13i64), Py::Int(17i64), Py::Int(19i64), Py::Int(23i64), Py::Int(29i64), Py::Int(31i64), Py::Int(37i64), Py::Int(41i64), Py::Int(43i64), Py::Int(47i64), Py::Int(53i64), Py::Int(59i64), Py::Int(61i64), Py::Int(67i64), Py::Int(71i64)])) })());
    V.with(|v| v.clone())
}

#[allow(non_snake_case)]
fn c__SQRT2() -> R {
    thread_local!(static V: R = (|| -> R { Ok(Py::Float(1.4142135623730951)) })());
    V.with(|v| v.clone())
}

#[allow(non_snake_case)]
fn c__TINY() -> R {
    thread_local!(static V: R = (|| -> R { Ok(Py::Float(1e-300)) })());
    V.with(|v| v.clone())
}

#[allow(non_snake_case)]
fn c__TYPEB() -> R {
    thread_local!(static V: R = (|| -> R { Ok(Py::list(vec![Py::str("rectangular"), Py::str("triangular"), Py::str("arcsine")])) })());
    V.with(|v| v.clone())
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__argbest(mut v_v: Py, mut v_sign: Py) -> R {
    let mut v_best: Py = Py::None;
    v_best = py::minmax_key(&[Py::list((py::range_iter(&[(py::len(&v_v.clone())?).i()?])).into_iter().collect::<Vec<Py>>())], &{ let v_sign = v_sign.clone(); let v_v = v_v.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_i: Py = a.get(0).cloned().unwrap_or(Py::None);
            return Ok(py::mul(&v_sign.clone(), &py::getitem(&v_v.clone(), &v_i.clone())?)?);
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) }, true)?;
    return Ok(v_best.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__bb_check(mut v_alpha: Py, mut v_beta: Py, mut v_successes: Py, mut v_trials: Py) -> R {
    let mut v_a: Py = Py::None;
    let mut v_b: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_n: Py = Py::None;
    let t3169 = (Py::tuple(vec![f__pos(v_alpha.clone(), Py::str("alpha"))?, f__pos(v_beta.clone(), Py::str("beta"))?])).items()?;
    if t3169.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_a = t3169[0].clone();
    v_b = t3169[1].clone();
    let t3175 = (Py::tuple(vec![f__whole(v_trials.clone(), Py::str("the number of trials"), Py::Int(0i64))?, f__whole(v_successes.clone(), Py::str("the number of successes"), Py::Int(0i64))?])).items()?;
    if t3175.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_n = t3175[0].clone();
    v_k = t3175[1].clone();
    if ({ let t3176 = v_k.clone(); Py::Bool({ let t3177 = v_n.clone(); if py::gt(&t3176, &t3177)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("successes cannot exceed trials"))));
    }
    return Ok(Py::tuple(vec![v_a.clone(), v_b.clone(), v_k.clone(), v_n.clone()]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__bcf(mut v_a: Py, mut v_b: Py, mut v_x: Py) -> R {
    let mut v_aa: Py = Py::None;
    let mut v_c: Py = Py::None;
    let mut v_d: Py = Py::None;
    let mut v_de: Py = Py::None;
    let mut v_h: Py = Py::None;
    let mut v_m: Py = Py::None;
    let mut v_m2: Py = Py::None;
    let mut v_qab: Py = Py::None;
    let mut v_qam: Py = Py::None;
    let mut v_qap: Py = Py::None;
    let t5306 = (Py::tuple(vec![py::add(&v_a.clone(), &v_b.clone())?, py::add(&v_a.clone(), &Py::Float(1.0))?, py::sub(&v_a.clone(), &Py::Float(1.0))?])).items()?;
    if t5306.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_qab = t5306[0].clone();
    v_qap = t5306[1].clone();
    v_qam = t5306[2].clone();
    let t5307 = (Py::tuple(vec![Py::Float(1.0), py::sub(&Py::Float(1.0), &py::truediv(&py::mul(&v_qab.clone(), &v_x.clone())?, &v_qap.clone())?)?])).items()?;
    if t5307.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_c = t5307[0].clone();
    v_d = t5307[1].clone();
    if ({ let t5308 = py::abs(&v_d.clone())?; Py::Bool({ let t5309 = c__TINY()?; if py::lt(&t5308, &t5309)? { true } else { false } }) }).truthy() {
        v_d = c__TINY()?;
    }
    v_d = py::truediv(&Py::Float(1.0), &v_d.clone())?;
    v_h = v_d.clone();
    for t5310 in py::range_iter(&[(Py::Int(1i64)).i()?, (Py::Int(100000i64)).i()?]) {
        v_m = t5310;
        v_m2 = py::mul(&Py::Int(2i64), &v_m.clone())?;
        v_aa = py::truediv(&py::mul(&py::mul(&v_m.clone(), &py::sub(&v_b.clone(), &v_m.clone())?)?, &v_x.clone())?, &py::mul(&py::add(&v_qam.clone(), &v_m2.clone())?, &py::add(&v_a.clone(), &v_m2.clone())?)?)?;
        v_d = py::add(&Py::Float(1.0), &py::mul(&v_aa.clone(), &v_d.clone())?)?;
        v_d = (if ({ let t5311 = py::abs(&v_d.clone())?; Py::Bool({ let t5312 = c__TINY()?; if py::lt(&t5311, &t5312)? { true } else { false } }) }).truthy() { c__TINY()? } else { v_d.clone() });
        v_c = py::add(&Py::Float(1.0), &py::truediv(&v_aa.clone(), &v_c.clone())?)?;
        v_c = (if ({ let t5313 = py::abs(&v_c.clone())?; Py::Bool({ let t5314 = c__TINY()?; if py::lt(&t5313, &t5314)? { true } else { false } }) }).truthy() { c__TINY()? } else { v_c.clone() });
        v_d = py::truediv(&Py::Float(1.0), &v_d.clone())?;
        v_h = py::mul(&v_h, &py::mul(&v_d.clone(), &v_c.clone())?)?;
        v_aa = py::truediv(&py::mul(&py::mul(&py::neg(&py::add(&v_a.clone(), &v_m.clone())?)?, &py::add(&v_qab.clone(), &v_m.clone())?)?, &v_x.clone())?, &py::mul(&py::add(&v_a.clone(), &v_m2.clone())?, &py::add(&v_qap.clone(), &v_m2.clone())?)?)?;
        v_d = py::add(&Py::Float(1.0), &py::mul(&v_aa.clone(), &v_d.clone())?)?;
        v_d = (if ({ let t5315 = py::abs(&v_d.clone())?; Py::Bool({ let t5316 = c__TINY()?; if py::lt(&t5315, &t5316)? { true } else { false } }) }).truthy() { c__TINY()? } else { v_d.clone() });
        v_c = py::add(&Py::Float(1.0), &py::truediv(&v_aa.clone(), &v_c.clone())?)?;
        v_c = (if ({ let t5317 = py::abs(&v_c.clone())?; Py::Bool({ let t5318 = c__TINY()?; if py::lt(&t5317, &t5318)? { true } else { false } }) }).truthy() { c__TINY()? } else { v_c.clone() });
        v_d = py::truediv(&Py::Float(1.0), &v_d.clone())?;
        v_de = py::mul(&v_d.clone(), &v_c.clone())?;
        v_h = py::mul(&v_h, &v_de.clone())?;
        if ({ let t5319 = py::abs(&py::sub(&v_de.clone(), &Py::Float(1.0))?)?; Py::Bool({ let t5320 = Py::Float(3e-16); if py::lt(&t5319, &t5320)? { true } else { false } }) }).truthy() {
            return Ok(v_h.clone());
        }
    }
    return Err(PyErr::Other("ArithmeticError".into()));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__beta_logpdf(mut v_x: Py, mut v_a: Py, mut v_b: Py) -> R {
    return Ok(py::sub(&py::add(&py::mul(&py::sub(&v_a.clone(), &Py::Float(1.0))?, &py::math1("log", &v_x.clone())?)?, &py::mul(&py::sub(&v_b.clone(), &Py::Float(1.0))?, &py::math1("log1p", &py::neg(&v_x.clone())?)?)?)?, &f__lbeta(v_a.clone(), v_b.clone())?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__beta_pdf(mut v_x: Py, mut v_a: Py, mut v_b: Py) -> R {
    if ({ let t2820 = { let t2816 = v_x.clone(); Py::Bool({ let t2817 = Py::Float(0.0); if py::le(&t2816, &t2817)? { true } else { false } }) }; if t2820.truthy() { t2820 } else { { let t2818 = v_x.clone(); Py::Bool({ let t2819 = Py::Float(1.0); if py::ge(&t2818, &t2819)? { true } else { false } }) } } }).truthy() {
        if ({ let t2831 = { let t2825 = { let t2821 = v_x.clone(); Py::Bool({ let t2822 = Py::Float(0.0); if py::le(&t2821, &t2822)? { true } else { false } }) }; if !t2825.truthy() { t2825 } else { { let t2823 = v_a.clone(); Py::Bool({ let t2824 = Py::Int(1i64); if py::lt(&t2823, &t2824)? { true } else { false } }) } } }; if t2831.truthy() { t2831 } else { { let t2830 = { let t2826 = v_x.clone(); Py::Bool({ let t2827 = Py::Float(1.0); if py::ge(&t2826, &t2827)? { true } else { false } }) }; if !t2830.truthy() { t2830 } else { { let t2828 = v_b.clone(); Py::Bool({ let t2829 = Py::Int(1i64); if py::lt(&t2828, &t2829)? { true } else { false } }) } } } } }).truthy() {
            return Ok(py::float(&Py::str("inf"))?);
        }
        if ({ let t2842 = { let t2836 = { let t2832 = v_x.clone(); Py::Bool({ let t2833 = Py::Float(0.0); if py::le(&t2832, &t2833)? { true } else { false } }) }; if !t2836.truthy() { t2836 } else { { let t2834 = v_a.clone(); Py::Bool({ let t2835 = Py::Int(1i64); if py::eq(&t2834, &t2835) { true } else { false } }) } } }; if t2842.truthy() { t2842 } else { { let t2841 = { let t2837 = v_x.clone(); Py::Bool({ let t2838 = Py::Float(1.0); if py::ge(&t2837, &t2838)? { true } else { false } }) }; if !t2841.truthy() { t2841 } else { { let t2839 = v_b.clone(); Py::Bool({ let t2840 = Py::Int(1i64); if py::eq(&t2839, &t2840) { true } else { false } }) } } } } }).truthy() {
            return Ok(py::math1("exp", &py::neg(&f__lbeta(v_a.clone(), v_b.clone())?)?)?);
        }
        return Ok(Py::Float(0.0));
    }
    return Ok(py::math1("exp", &py::sub(&py::add(&py::mul(&py::sub(&v_a.clone(), &Py::Float(1.0))?, &py::math1("log", &v_x.clone())?)?, &py::mul(&py::sub(&v_b.clone(), &Py::Float(1.0))?, &py::math1("log1p", &py::neg(&v_x.clone())?)?)?)?, &f__lbeta(v_a.clone(), v_b.clone())?)?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__beta_ppf(mut v_p: Py, mut v_a: Py, mut v_b: Py) -> R {
    let mut v_m: Py = Py::None;
    if ({ let t2764 = v_p.clone(); Py::Bool({ let t2765 = Py::Float(0.0); if py::le(&t2764, &t2765)? { true } else { false } }) }).truthy() {
        return Ok(Py::Float(0.0));
    }
    if ({ let t2766 = v_p.clone(); Py::Bool({ let t2767 = Py::Float(1.0); if py::ge(&t2766, &t2767)? { true } else { false } }) }).truthy() {
        return Ok(Py::Float(1.0));
    }
    v_m = py::truediv(&v_a.clone(), &py::add(&v_a.clone(), &v_b.clone())?)?;
    return Ok(f__invert({ let v_a = v_a.clone(); let v_b = v_b.clone(); let v_m = v_m.clone(); let v_p = v_p.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_x: Py = a.get(0).cloned().unwrap_or(Py::None);
            return Ok(f__ibeta(v_a.clone(), v_b.clone(), v_x.clone(), Py::None)?);
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) }, { let v_a = v_a.clone(); let v_b = v_b.clone(); let v_m = v_m.clone(); let v_p = v_p.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_x: Py = a.get(0).cloned().unwrap_or(Py::None);
            return Ok(f__beta_pdf(v_x.clone(), v_a.clone(), v_b.clone())?);
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) }, v_p.clone(), Py::Float(0.0), Py::Float(1.0), v_m.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__cholesky_ok(mut v_s: Py, mut v_name: Py) -> R {
    let mut v_L: Py = Py::None;
    let mut v_d: Py = Py::None;
    let mut v_i: Py = Py::None;
    let mut v_j: Py = Py::None;
    let mut v_n: Py = Py::None;
    let mut v_scale: Py = Py::None;
    let mut v_v: Py = Py::None;
    v_n = py::len(&v_s.clone())?;
    if (Py::Bool(({ let mut out = vec![]; for t4904 in py::iter(&(v_s.clone()))? { let mut v_r: Py; 
v_r = t4904;
out.push({ let t4905 = py::len(&v_r.clone())?; Py::Bool({ let t4906 = v_n.clone(); if py::ne(&t4905, &t4906) { true } else { false } }) }); } Py::list(out) }).items()?.iter().any(|x| x.truthy()))).truthy() {
        return Err(py::value_error(py::to_str(&Py::str(&[py::format(&v_name.clone(), "")?, " must be square".to_string()].concat()))));
    }
    v_scale = py::minmax(&[Py::Float(1.0), py::minmax(&[{ let mut out = vec![]; for t4907 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v_i: Py; 
v_i = t4907;
out.push(py::abs(&py::getitem(&py::getitem(&v_s.clone(), &v_i.clone())?, &v_i.clone())?)?); } Py::list(out) }], true)?], true)?;
    for t4908 in py::range_iter(&[(v_n.clone()).i()?]) {
        v_i = t4908;
        for t4909 in py::range_iter(&[(v_i.clone()).i()?]) {
            v_j = t4909;
            if ({ let t4910 = py::abs(&py::sub(&py::getitem(&py::getitem(&v_s.clone(), &v_i.clone())?, &v_j.clone())?, &py::getitem(&py::getitem(&v_s.clone(), &v_j.clone())?, &v_i.clone())?)?)?; Py::Bool({ let t4911 = py::mul(&Py::Float(1e-12), &v_scale.clone())?; if py::gt(&t4910, &t4911)? { true } else { false } }) }).truthy() {
                return Err(py::value_error(py::to_str(&Py::str(&[py::format(&v_name.clone(), "")?, " must be symmetric".to_string()].concat()))));
            }
        }
    }
    v_L = { let mut out = vec![]; for t4912 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v__: Py; 
v__ = t4912;
out.push(py::mul(&Py::list(vec![Py::Float(0.0)]), &v_n.clone())?); } Py::list(out) };
    for t4913 in py::range_iter(&[(v_n.clone()).i()?]) {
        v_j = t4913;
        v_d = py::sub(&py::getitem(&py::getitem(&v_s.clone(), &v_j.clone())?, &v_j.clone())?, &py::fsum(&{ let mut out = vec![]; for t4914 in py::range_iter(&[(v_j.clone()).i()?]) { let mut v_k: Py; 
v_k = t4914;
out.push(py::pow(&py::getitem(&py::getitem(&v_L.clone(), &v_j.clone())?, &v_k.clone())?, &Py::Int(2i64))?); } Py::list(out) })?)?;
        if ({ let t4915 = v_d.clone(); Py::Bool({ let t4916 = py::mul(&py::neg(&Py::Float(1e-12))?, &v_scale.clone())?; if py::lt(&t4915, &t4916)? { true } else { false } }) }).truthy() {
            return Err(py::value_error(py::to_str(&Py::str(&[py::format(&v_name.clone(), "")?, " must be positive semi-definite".to_string()].concat()))));
        }
        py::setitem(&py::getitem(&v_L.clone(), &v_j.clone())?, &v_j.clone(), py::math1("sqrt", &py::minmax(&[v_d.clone(), Py::Float(0.0)], true)?)?)?;
        for t4917 in py::range_iter(&[(py::add(&v_j.clone(), &Py::Int(1i64))?).i()?, (v_n.clone()).i()?]) {
            v_i = t4917;
            v_v = py::sub(&py::getitem(&py::getitem(&v_s.clone(), &v_i.clone())?, &v_j.clone())?, &py::fsum(&{ let mut out = vec![]; for t4918 in py::range_iter(&[(v_j.clone()).i()?]) { let mut v_k: Py; 
v_k = t4918;
out.push(py::mul(&py::getitem(&py::getitem(&v_L.clone(), &v_i.clone())?, &v_k.clone())?, &py::getitem(&py::getitem(&v_L.clone(), &v_j.clone())?, &v_k.clone())?)?); } Py::list(out) })?)?;
            if ({ let t4919 = py::getitem(&py::getitem(&v_L.clone(), &v_j.clone())?, &v_j.clone())?; Py::Bool({ let t4920 = py::mul(&Py::Float(1e-15), &v_scale.clone())?; if py::gt(&t4919, &t4920)? { true } else { false } }) }).truthy() {
                py::setitem(&py::getitem(&v_L.clone(), &v_i.clone())?, &v_j.clone(), py::truediv(&v_v.clone(), &py::getitem(&py::getitem(&v_L.clone(), &v_j.clone())?, &v_j.clone())?)?)?;
            } else {
                if ({ let t4921 = py::abs(&v_v.clone())?; Py::Bool({ let t4922 = py::mul(&Py::Float(1e-09), &v_scale.clone())?; if py::gt(&t4921, &t4922)? { true } else { false } }) }).truthy() {
                    return Err(py::value_error(py::to_str(&Py::str(&[py::format(&v_name.clone(), "")?, " must be positive semi-definite".to_string()].concat()))));
                }
            }
        }
    }
    return Ok(v_s.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__diff(mut v_a: Py, mut v_b: Py) -> R {
    if ({ let t5333 = Py::Bool(matches!(v_a.clone(), Py::Dict(_))); if t5333.truthy() { t5333 } else { Py::Bool(matches!(v_b.clone(), Py::Dict(_))) } }).truthy() {
        if ({ let t5337 = py::not(&{ let t5334 = Py::Bool(matches!(v_a.clone(), Py::Dict(_))); if !t5334.truthy() { t5334 } else { Py::Bool(matches!(v_b.clone(), Py::Dict(_))) } }); if t5337.truthy() { t5337 } else { { let t5335 = py::method(&v_a.clone(), "keys", &[])?; Py::Bool({ let t5336 = py::method(&v_b.clone(), "keys", &[])?; if py::ne(&t5335, &t5336) { true } else { false } }) } } }).truthy() {
            return Ok(Py::Float(1.0));
        }
        return Ok({ let t = { let mut out = vec![]; for t5338 in py::iter(&(v_a.clone()))? { let mut v_k: Py; 
v_k = t5338;
out.push(f__diff(py::getitem(&v_a.clone(), &v_k.clone())?, py::getitem(&v_b.clone(), &v_k.clone())?)?); } Py::list(out) }; if t.items()?.is_empty() { Py::Float(0.0) } else { py::minmax(&[t], true)? } });
    }
    if ({ let t5339 = Py::Bool(matches!(v_a.clone(), Py::List(_) | Py::Tuple(_))); if t5339.truthy() { t5339 } else { Py::Bool(matches!(v_b.clone(), Py::List(_) | Py::Tuple(_))) } }).truthy() {
        if ({ let t5343 = py::not(&{ let t5340 = Py::Bool(matches!(v_a.clone(), Py::List(_) | Py::Tuple(_))); if !t5340.truthy() { t5340 } else { Py::Bool(matches!(v_b.clone(), Py::List(_) | Py::Tuple(_))) } }); if t5343.truthy() { t5343 } else { { let t5341 = py::len(&v_a.clone())?; Py::Bool({ let t5342 = py::len(&v_b.clone())?; if py::ne(&t5341, &t5342) { true } else { false } }) } } }).truthy() {
            return Ok(Py::Float(1.0));
        }
        return Ok({ let t = { let mut out = vec![]; for t5344 in py::iter(&(py::zip(&[v_a.clone(), v_b.clone()])?))? { let mut v_u: Py; let mut v_v: Py; 
let t5345 = (t5344).items()?;
if t5345.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_u = t5345[0].clone();
v_v = t5345[1].clone();
out.push(f__diff(v_u.clone(), v_v.clone())?); } Py::list(out) }; if t.items()?.is_empty() { Py::Float(0.0) } else { py::minmax(&[t], true)? } });
    }
    if ({ let t5350 = { let t5346 = v_a.clone(); Py::Bool({ let t5347 = Py::None; if py::is(&t5346, &t5347) { true } else { false } }) }; if t5350.truthy() { t5350 } else { { let t5348 = v_b.clone(); Py::Bool({ let t5349 = Py::None; if py::is(&t5348, &t5349) { true } else { false } }) } } }).truthy() {
        return Ok((if ({ let t5351 = v_a.clone(); Py::Bool({ let t5352 = v_b.clone(); if py::is(&t5351, &t5352) { true } else { false } }) }).truthy() { Py::Float(0.0) } else { Py::Float(1.0) }));
    }
    if ({ let t5353 = Py::Bool(matches!(v_a.clone(), Py::Str(_))); if t5353.truthy() { t5353 } else { Py::Bool(matches!(v_b.clone(), Py::Str(_))) } }).truthy() {
        return Ok((if ({ let t5354 = v_a.clone(); Py::Bool({ let t5355 = v_b.clone(); if py::eq(&t5354, &t5355) { true } else { false } }) }).truthy() { Py::Float(0.0) } else { Py::Float(1.0) }));
    }
    let t5357: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            let t5356 = (Py::tuple(vec![py::float(&v_a.clone())?, py::float(&v_b.clone())?])).items()?;
            if t5356.len() != 2 { return Err(PyErr::Value("unpack".into())); }
            v_a = t5356[0].clone();
            v_b = t5356[1].clone();
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t5357 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if ((matches!(e, PyErr::Type(_)) || matches!(&e, PyErr::Other(c) if c.as_str() == "TypeError"))) || (matches!(e, PyErr::Value(_))) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    if (py::not(&{ let t5358 = py::math1("isfinite", &v_a.clone())?; if !t5358.truthy() { t5358 } else { py::math1("isfinite", &v_b.clone())? } })).truthy() {
        return Ok((if ({ let t5359 = v_a.clone(); Py::Bool({ let t5360 = v_b.clone(); if py::eq(&t5359, &t5360) { true } else { false } }) }).truthy() { Py::Float(0.0) } else { Py::Float(1.0) }));
    }
    return Ok(py::truediv(&py::abs(&py::sub(&v_a.clone(), &v_b.clone())?)?, &py::minmax(&[Py::Float(1e-15), py::abs(&v_a.clone())?, py::abs(&v_b.clone())?], true)?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__digamma(mut v_x: Py) -> R {
    let mut v_f: Py = Py::None;
    let mut v_r: Py = Py::None;
    v_r = Py::Float(0.0);
    while ({ let t2891 = v_x.clone(); Py::Bool({ let t2892 = Py::Float(10.0); if py::lt(&t2891, &t2892)? { true } else { false } }) }).truthy() {
        v_r = py::sub(&v_r, &py::truediv(&Py::Float(1.0), &v_x.clone())?)?;
        v_x = py::iadd(&v_x, &Py::Float(1.0))?;
    }
    v_f = py::truediv(&Py::Float(1.0), &py::mul(&v_x.clone(), &v_x.clone())?)?;
    return Ok(py::sub(&py::sub(&py::add(&v_r.clone(), &py::math1("log", &v_x.clone())?)?, &py::truediv(&Py::Float(0.5), &v_x.clone())?)?, &py::mul(&v_f.clone(), &py::sub(&py::truediv(&Py::Int(1i64), &Py::Int(12i64))?, &py::mul(&v_f.clone(), &py::sub(&py::truediv(&Py::Int(1i64), &Py::Int(120i64))?, &py::mul(&v_f.clone(), &py::sub(&py::truediv(&Py::Int(1i64), &Py::Int(252i64))?, &py::mul(&v_f.clone(), &py::sub(&py::truediv(&Py::Int(1i64), &Py::Int(240i64))?, &py::mul(&v_f.clone(), &py::truediv(&Py::Int(1i64), &Py::Int(132i64))?)?)?)?)?)?)?)?)?)?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__dm_check(mut v_alpha: Py, mut v_counts: Py) -> R {
    let mut v_a: Py = Py::None;
    let mut v_x: Py = Py::None;
    v_a = f__vec(v_alpha.clone(), Py::str("the prior concentrations"), Py::None, Py::Int(2i64))?;
    v_x = f__vec(v_counts.clone(), Py::str("the counts"), py::len(&v_a.clone())?, Py::Int(1i64))?;
    if (Py::Bool(({ let mut out = vec![]; for t1626 in py::iter(&(v_a.clone()))? { let mut v_v: Py; 
v_v = t1626;
out.push({ let t1627 = v_v.clone(); Py::Bool({ let t1628 = Py::Int(0i64); if py::le(&t1627, &t1628)? { true } else { false } }) }); } Py::list(out) }).items()?.iter().any(|x| x.truthy()))).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("Dirichlet concentrations must be positive"))));
    }
    if (Py::Bool(({ let mut out = vec![]; for t1629 in py::iter(&(v_x.clone()))? { let mut v_v: Py; 
v_v = t1629;
out.push({ let t1634 = { let t1630 = v_v.clone(); Py::Bool({ let t1631 = Py::Int(0i64); if py::lt(&t1630, &t1631)? { true } else { false } }) }; if t1634.truthy() { t1634 } else { { let t1632 = v_v.clone(); Py::Bool({ let t1633 = py::int(&v_v.clone())?; if py::ne(&t1632, &t1633) { true } else { false } }) } } }); } Py::list(out) }).items()?.iter().any(|x| x.truthy()))).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("counts must be whole numbers >= 0"))));
    }
    return Ok(Py::tuple(vec![v_a.clone(), v_x.clone()]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__dx(mut v_prevalence: Py, mut v_sensitivity: Py, mut v_specificity: Py) -> R {
    let mut v_neg: Py = Py::None;
    let mut v_pos: Py = Py::None;
    let mut v_pv: Py = Py::None;
    let mut v_se: Py = Py::None;
    let mut v_sp: Py = Py::None;
    let t1507 = (Py::tuple(vec![f__prob(v_prevalence.clone(), Py::str("prevalence"), Py::Bool(false))?, f__prob(v_sensitivity.clone(), Py::str("sensitivity"), Py::Bool(false))?, f__prob(v_specificity.clone(), Py::str("specificity"), Py::Bool(false))?])).items()?;
    if t1507.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_pv = t1507[0].clone();
    v_se = t1507[1].clone();
    v_sp = t1507[2].clone();
    v_pos = py::add(&py::mul(&v_pv.clone(), &v_se.clone())?, &py::mul(&py::sub(&Py::Float(1.0), &v_pv.clone())?, &py::sub(&Py::Float(1.0), &v_sp.clone())?)?)?;
    v_neg = py::sub(&Py::Float(1.0), &v_pos.clone())?;
    if ({ let t1512 = { let t1508 = v_pos.clone(); Py::Bool({ let t1509 = Py::Float(0.0); if py::le(&t1508, &t1509)? { true } else { false } }) }; if t1512.truthy() { t1512 } else { { let t1510 = v_neg.clone(); Py::Bool({ let t1511 = Py::Float(0.0); if py::le(&t1510, &t1511)? { true } else { false } }) } } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("a positive (or negative) result is impossible with these values: a predictive value is undefined"))));
    }
    return Ok(Py::tuple(vec![v_pv.clone(), v_se.clone(), v_sp.clone(), v_pos.clone(), v_neg.clone()]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__ev(mut v_m: Py, mut v_p: Py) -> R {
    return Ok({ let mut out = vec![]; for t1942 in py::iter(&(v_m.clone()))? { let mut v_r: Py; 
v_r = t1942;
out.push(py::fsum(&{ let mut out = vec![]; for t1943 in py::iter(&(py::zip(&[v_r.clone(), v_p.clone()])?))? { let mut v_a: Py; let mut v_b: Py; 
let t1944 = (t1943).items()?;
if t1944.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_a = t1944[0].clone();
v_b = t1944[1].clone();
out.push(py::mul(&v_a.clone(), &v_b.clone())?); } Py::list(out) })?); } Py::list(out) });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__fin(mut v_v: Py, mut v_name: Py) -> R {
    if ({ let t5246 = Py::Bool(matches!(v_v.clone(), Py::Bool(_))); if t5246.truthy() { t5246 } else { { let t5245 = py::not(&Py::Bool(matches!(v_v.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_) | Py::Float(_)))); if t5245.truthy() { t5245 } else { py::not(&py::math1("isfinite", &v_v.clone())?) } } } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str(&[py::format(&v_name.clone(), "")?, " must be a finite number".to_string()].concat()))));
    }
    return Ok(py::float(&v_v.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__gamma_logpdf(mut v_x: Py, mut v_k: Py) -> R {
    return Ok(py::sub(&py::sub(&py::mul(&py::sub(&v_k.clone(), &Py::Float(1.0))?, &py::math1("log", &v_x.clone())?)?, &v_x.clone())?, &py::math1("lgamma", &v_k.clone())?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__gamma_ppf(mut v_p: Py, mut v_k: Py) -> R {
    let mut v_hi: Py = Py::None;
    if ({ let t2280 = v_p.clone(); Py::Bool({ let t2281 = Py::Float(0.0); if py::le(&t2280, &t2281)? { true } else { false } }) }).truthy() {
        return Ok(Py::Float(0.0));
    }
    v_hi = py::minmax(&[Py::Float(1.0), v_k.clone()], true)?;
    while ({ let t2302 = { let t2282 = py::getitem(&f__igamma(v_k.clone(), v_hi.clone())?, &Py::Int(0i64))?; Py::Bool({ let t2283 = v_p.clone(); if py::lt(&t2282, &t2283)? { true } else { false } }) }; if !t2302.truthy() { t2302 } else { { let t2300 = v_hi.clone(); Py::Bool({ let t2301 = Py::Float(1e+300); if py::lt(&t2300, &t2301)? { true } else { false } }) } } }).truthy() {
        v_hi = py::mul(&v_hi, &Py::Float(2.0))?;
    }
    return Ok(f__invert({ let v_hi = v_hi.clone(); let v_k = v_k.clone(); let v_p = v_p.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_x: Py = a.get(0).cloned().unwrap_or(Py::None);
            return Ok(f__igamma(v_k.clone(), v_x.clone())?);
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) }, { let v_hi = v_hi.clone(); let v_k = v_k.clone(); let v_p = v_p.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_x: Py = a.get(0).cloned().unwrap_or(Py::None);
            return Ok((if ({ let t2328 = v_x.clone(); Py::Bool({ let t2329 = Py::Int(0i64); if py::gt(&t2328, &t2329)? { true } else { false } }) }).truthy() { py::math1("exp", &f__gamma_logpdf(v_x.clone(), v_k.clone())?)? } else { Py::Float(0.0) }));
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) }, v_p.clone(), Py::Float(0.0), v_hi.clone(), v_k.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__gp_check(mut v_shape: Py, mut v_rate: Py, mut v_counts: Py) -> R {
    let mut v_a: Py = Py::None;
    let mut v_b: Py = Py::None;
    let mut v_x: Py = Py::None;
    let t3228 = (Py::tuple(vec![f__pos(v_shape.clone(), Py::str("the prior shape"))?, f__pos(v_rate.clone(), Py::str("the prior rate"))?])).items()?;
    if t3228.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_a = t3228[0].clone();
    v_b = t3228[1].clone();
    v_x = f__vec(v_counts.clone(), Py::str("the counts"), Py::None, Py::Int(1i64))?;
    if (Py::Bool(({ let mut out = vec![]; for t3237 in py::iter(&(v_x.clone()))? { let mut v_v: Py; 
v_v = t3237;
out.push({ let t3242 = { let t3238 = v_v.clone(); Py::Bool({ let t3239 = Py::Int(0i64); if py::lt(&t3238, &t3239)? { true } else { false } }) }; if t3242.truthy() { t3242 } else { { let t3240 = v_v.clone(); Py::Bool({ let t3241 = py::int(&v_v.clone())?; if py::ne(&t3240, &t3241) { true } else { false } }) } } }); } Py::list(out) }).items()?.iter().any(|x| x.truthy()))).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("Poisson counts must be whole numbers >= 0"))));
    }
    return Ok(Py::tuple(vec![v_a.clone(), v_b.clone(), v_x.clone()]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__hermite_e(mut v_n: Py, mut v_x: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_p0: Py = Py::None;
    let mut v_p1: Py = Py::None;
    let t2513 = (Py::tuple(vec![Py::Float(1.0), v_x.clone()])).items()?;
    if t2513.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_p0 = t2513[0].clone();
    v_p1 = t2513[1].clone();
    if ({ let t2514 = v_n.clone(); Py::Bool({ let t2515 = Py::Int(0i64); if py::eq(&t2514, &t2515) { true } else { false } }) }).truthy() {
        return Ok(Py::tuple(vec![Py::Float(1.0), Py::Float(0.0)]));
    }
    for t2516 in py::range_iter(&[(Py::Int(1i64)).i()?, (v_n.clone()).i()?]) {
        v_k = t2516;
        let t2517 = (Py::tuple(vec![v_p1.clone(), py::sub(&py::mul(&v_x.clone(), &v_p1.clone())?, &py::mul(&v_k.clone(), &v_p0.clone())?)?])).items()?;
        if t2517.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_p0 = t2517[0].clone();
        v_p1 = t2517[1].clone();
    }
    return Ok(Py::tuple(vec![v_p1.clone(), v_p0.clone()]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__ibeta(mut v_a: Py, mut v_b: Py, mut v_x: Py, mut v_y: Py) -> R {
    let mut v_lf: Py = Py::None;
    let mut v_v: Py = Py::None;
    if ({ let t5298 = v_y.clone(); Py::Bool({ let t5299 = Py::None; if py::is(&t5298, &t5299) { true } else { false } }) }).truthy() {
        v_y = py::sub(&Py::Float(1.0), &v_x.clone())?;
    }
    if ({ let t5300 = v_x.clone(); Py::Bool({ let t5301 = Py::Float(0.0); if py::le(&t5300, &t5301)? { true } else { false } }) }).truthy() {
        return Ok(Py::tuple(vec![Py::Float(0.0), Py::Float(1.0)]));
    }
    if ({ let t5302 = v_y.clone(); Py::Bool({ let t5303 = Py::Float(0.0); if py::le(&t5302, &t5303)? { true } else { false } }) }).truthy() {
        return Ok(Py::tuple(vec![Py::Float(1.0), Py::Float(0.0)]));
    }
    v_lf = py::sub(&py::add(&py::mul(&v_a.clone(), &py::math1("log", &v_x.clone())?)?, &py::mul(&v_b.clone(), &py::math1("log", &v_y.clone())?)?)?, &f__lbeta(v_a.clone(), v_b.clone())?)?;
    if ({ let t5304 = v_x.clone(); Py::Bool({ let t5305 = py::truediv(&py::add(&v_a.clone(), &Py::Float(1.0))?, &py::add(&py::add(&v_a.clone(), &v_b.clone())?, &Py::Float(2.0))?)?; if py::lt(&t5304, &t5305)? { true } else { false } }) }).truthy() {
        v_v = py::truediv(&py::mul(&py::math1("exp", &v_lf.clone())?, &f__bcf(v_a.clone(), v_b.clone(), v_x.clone())?)?, &v_a.clone())?;
        return Ok(Py::tuple(vec![v_v.clone(), py::sub(&Py::Float(1.0), &v_v.clone())?]));
    }
    v_v = py::truediv(&py::mul(&py::math1("exp", &v_lf.clone())?, &f__bcf(v_b.clone(), v_a.clone(), v_y.clone())?)?, &v_b.clone())?;
    return Ok(Py::tuple(vec![py::sub(&Py::Float(1.0), &v_v.clone())?, v_v.clone()]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__igamma(mut v_a: Py, mut v_x: Py) -> R {
    let mut v__: Py = Py::None;
    let mut v_an: Py = Py::None;
    let mut v_ap: Py = Py::None;
    let mut v_b: Py = Py::None;
    let mut v_c: Py = Py::None;
    let mut v_d: Py = Py::None;
    let mut v_de: Py = Py::None;
    let mut v_h: Py = Py::None;
    let mut v_i: Py = Py::None;
    let mut v_lf: Py = Py::None;
    let mut v_p: Py = Py::None;
    let mut v_q: Py = Py::None;
    let mut v_s: Py = Py::None;
    if ({ let t2284 = v_x.clone(); Py::Bool({ let t2285 = Py::Float(0.0); if py::le(&t2284, &t2285)? { true } else { false } }) }).truthy() {
        return Ok(Py::tuple(vec![Py::Float(0.0), Py::Float(1.0)]));
    }
    v_lf = py::sub(&py::sub(&py::mul(&v_a.clone(), &py::math1("log", &v_x.clone())?)?, &v_x.clone())?, &py::math1("lgamma", &v_a.clone())?)?;
    if ({ let t2286 = v_x.clone(); Py::Bool({ let t2287 = py::add(&v_a.clone(), &Py::Float(1.0))?; if py::lt(&t2286, &t2287)? { true } else { false } }) }).truthy() {
        let t2288 = (Py::tuple(vec![v_a.clone(), py::truediv(&Py::Float(1.0), &v_a.clone())?])).items()?;
        if t2288.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_ap = t2288[0].clone();
        v_s = t2288[1].clone();
        v_d = v_s.clone();
        for t2289 in py::range_iter(&[(Py::Int(100000i64)).i()?]) {
            v__ = t2289;
            v_ap = py::iadd(&v_ap, &Py::Float(1.0))?;
            v_d = py::mul(&v_d, &py::truediv(&v_x.clone(), &v_ap.clone())?)?;
            v_s = py::iadd(&v_s, &v_d.clone())?;
            if ({ let t2290 = py::abs(&v_d.clone())?; Py::Bool({ let t2291 = py::mul(&py::abs(&v_s.clone())?, &Py::Float(1e-17))?; if py::lt(&t2290, &t2291)? { true } else { false } }) }).truthy() {
                break;
            }
        }
        v_p = py::mul(&v_s.clone(), &py::math1("exp", &v_lf.clone())?)?;
        return Ok(Py::tuple(vec![v_p.clone(), py::sub(&Py::Float(1.0), &v_p.clone())?]));
    }
    v_b = py::sub(&py::add(&v_x.clone(), &Py::Float(1.0))?, &v_a.clone())?;
    let t2292 = (Py::tuple(vec![py::truediv(&Py::Float(1.0), &c__TINY()?)?, py::truediv(&Py::Float(1.0), &v_b.clone())?])).items()?;
    if t2292.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_c = t2292[0].clone();
    v_d = t2292[1].clone();
    v_h = v_d.clone();
    for t2293 in py::range_iter(&[(Py::Int(1i64)).i()?, (Py::Int(100000i64)).i()?]) {
        v_i = t2293;
        v_an = py::mul(&py::neg(&v_i.clone())?, &py::sub(&v_i.clone(), &v_a.clone())?)?;
        v_b = py::iadd(&v_b, &Py::Float(2.0))?;
        v_d = py::add(&py::mul(&v_an.clone(), &v_d.clone())?, &v_b.clone())?;
        v_d = (if ({ let t2294 = py::abs(&v_d.clone())?; Py::Bool({ let t2295 = c__TINY()?; if py::lt(&t2294, &t2295)? { true } else { false } }) }).truthy() { c__TINY()? } else { v_d.clone() });
        v_c = py::add(&v_b.clone(), &py::truediv(&v_an.clone(), &v_c.clone())?)?;
        v_c = (if ({ let t2296 = py::abs(&v_c.clone())?; Py::Bool({ let t2297 = c__TINY()?; if py::lt(&t2296, &t2297)? { true } else { false } }) }).truthy() { c__TINY()? } else { v_c.clone() });
        v_d = py::truediv(&Py::Float(1.0), &v_d.clone())?;
        v_de = py::mul(&v_d.clone(), &v_c.clone())?;
        v_h = py::mul(&v_h, &v_de.clone())?;
        if ({ let t2298 = py::abs(&py::sub(&v_de.clone(), &Py::Float(1.0))?)?; Py::Bool({ let t2299 = Py::Float(3e-16); if py::lt(&t2298, &t2299)? { true } else { false } }) }).truthy() {
            break;
        }
    }
    v_q = py::mul(&py::math1("exp", &v_lf.clone())?, &v_h.clone())?;
    return Ok(Py::tuple(vec![py::sub(&Py::Float(1.0), &v_q.clone())?, v_q.clone()]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__invert(mut v_tails: Py, mut v_pdf: Py, mut v_p: Py, mut v_lo: Py, mut v_hi: Py, mut v_x0: Py) -> R {
    let mut v__: Py = Py::None;
    let mut v_d: Py = Py::None;
    let mut v_g: Py = Py::None;
    let mut v_gx: Py = Py::None;
    let mut v_lower: Py = Py::None;
    let mut v_target: Py = Py::None;
    let mut v_x: Py = Py::None;
    let mut v_xn: Py = Py::None;
    v_lower = { let t5272 = v_p.clone(); Py::Bool({ let t5273 = Py::Float(0.5); if py::le(&t5272, &t5273)? { true } else { false } }) };
    v_target = (if (v_lower.clone()).truthy() { v_p.clone() } else { py::sub(&Py::Float(1.0), &v_p.clone())? });
    v_g = { let v_g = v_g.clone(); let v_hi = v_hi.clone(); let v_lo = v_lo.clone(); let v_lower = v_lower.clone(); let v_p = v_p.clone(); let v_pdf = v_pdf.clone(); let v_tails = v_tails.clone(); let v_target = v_target.clone(); let v_x0 = v_x0.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_x: Py = a.get(0).cloned().unwrap_or(Py::None);
            let mut v_f: Py = Py::None;
            let mut v_s: Py = Py::None;
            let t5274 = (py::call(&v_tails.clone(), &[v_x.clone()])?).items()?;
            if t5274.len() != 2 { return Err(PyErr::Value("unpack".into())); }
            v_f = t5274[0].clone();
            v_s = t5274[1].clone();
            return Ok((if (v_lower.clone()).truthy() { py::sub(&v_f.clone(), &v_target.clone())? } else { py::sub(&v_target.clone(), &v_s.clone())? }));
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) };
    v_x = (if ({ let t5275 = v_lo.clone(); Py::Bool({ let t5276 = v_x0.clone(); if py::lt(&t5275, &t5276)? { { let t5277 = v_hi.clone(); if py::lt(&t5276, &t5277)? { true } else { false } } } else { false } }) }).truthy() { v_x0.clone() } else { (if (py::math1("isfinite", &v_hi.clone())?).truthy() { py::mul(&Py::Float(0.5), &py::add(&v_lo.clone(), &v_hi.clone())?)? } else { py::add(&v_lo.clone(), &Py::Float(1.0))? }) });
    for t5278 in py::range_iter(&[(Py::Int(400i64)).i()?]) {
        v__ = t5278;
        v_gx = py::call(&v_g.clone(), &[v_x.clone()])?;
        if ({ let t5279 = v_gx.clone(); Py::Bool({ let t5280 = Py::Float(0.0); if py::eq(&t5279, &t5280) { true } else { false } }) }).truthy() {
            return Ok(v_x.clone());
        }
        if ({ let t5281 = v_gx.clone(); Py::Bool({ let t5282 = Py::Float(0.0); if py::gt(&t5281, &t5282)? { true } else { false } }) }).truthy() {
            v_hi = v_x.clone();
        } else {
            v_lo = v_x.clone();
        }
        v_d = py::call(&v_pdf.clone(), &[v_x.clone()])?;
        v_xn = (if ({ let t5285 = { let t5283 = v_d.clone(); Py::Bool({ let t5284 = Py::Float(0.0); if py::gt(&t5283, &t5284)? { true } else { false } }) }; if !t5285.truthy() { t5285 } else { py::math1("isfinite", &v_d.clone())? } }).truthy() { py::sub(&v_x.clone(), &py::truediv(&v_gx.clone(), &v_d.clone())?)? } else { py::float(&Py::str("nan"))? });
        if (py::not(&{ let t5286 = v_lo.clone(); Py::Bool({ let t5287 = v_xn.clone(); if py::lt(&t5286, &t5287)? { { let t5288 = v_hi.clone(); if py::lt(&t5287, &t5288)? { true } else { false } } } else { false } }) })).truthy() {
            v_xn = (if (py::math1("isfinite", &v_hi.clone())?).truthy() { py::mul(&Py::Float(0.5), &py::add(&v_lo.clone(), &v_hi.clone())?)? } else { py::minmax(&[py::mul(&Py::Float(2.0), &v_x.clone())?, py::add(&v_x.clone(), &Py::Float(1.0))?], true)? });
        }
        if ({ let t5296 = { let t5289 = py::abs(&py::sub(&v_xn.clone(), &v_x.clone())?)?; Py::Bool({ let t5290 = py::mul(&Py::Float(2e-16), &py::abs(&v_x.clone())?)?; if py::le(&t5289, &t5290)? { true } else { false } }) }; if t5296.truthy() { t5296 } else { { let t5295 = { let t5291 = v_xn.clone(); Py::Bool({ let t5292 = v_lo.clone(); if py::eq(&t5291, &t5292) { true } else { false } }) }; if t5295.truthy() { t5295 } else { { let t5293 = v_xn.clone(); Py::Bool({ let t5294 = v_hi.clone(); if py::eq(&t5293, &t5294) { true } else { false } }) } } } } }).truthy() {
            return Ok(v_xn.clone());
        }
        v_x = v_xn.clone();
    }
    return Ok(v_x.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__lbeta(mut v_a: Py, mut v_b: Py) -> R {
    return Ok(py::sub(&py::add(&py::math1("lgamma", &v_a.clone())?, &py::math1("lgamma", &v_b.clone())?)?, &py::math1("lgamma", &py::add(&v_a.clone(), &v_b.clone())?)?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__level(mut v_v: Py) -> R {
    v_v = (if ({ let t5254 = v_v.clone(); Py::Bool({ let t5255 = Py::None; if py::is(&t5254, &t5255) { true } else { false } }) }).truthy() { Py::Float(0.95) } else { f__fin(v_v.clone(), Py::str("level"))? });
    if (py::not(&{ let t5256 = Py::Float(0.0); Py::Bool({ let t5257 = v_v.clone(); if py::lt(&t5256, &t5257)? { { let t5258 = Py::Float(1.0); if py::lt(&t5257, &t5258)? { true } else { false } } } else { false } }) })).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the credibility level must be strictly between 0 and 1 (0.95 for 95%)"))));
    }
    return Ok(v_v.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__logsumexp(mut v_vals: Py, mut v_weights: Py) -> R {
    let mut v_m: Py = Py::None;
    v_weights = { let t4026 = v_weights.clone(); if t4026.truthy() { t4026 } else { py::mul(&Py::list(vec![Py::Float(1.0)]), &py::len(&v_vals.clone())?)? } };
    v_m = py::minmax(&[{ let mut out = vec![]; for t4027 in py::iter(&(py::zip(&[v_vals.clone(), v_weights.clone()])?))? { let mut v_v: Py; let mut v_w: Py; 
let t4028 = (t4027).items()?;
if t4028.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_v = t4028[0].clone();
v_w = t4028[1].clone();
if ({ let t4029 = v_w.clone(); Py::Bool({ let t4030 = Py::Int(0i64); if py::gt(&t4029, &t4030)? { true } else { false } }) }).truthy() { out.push(v_v.clone()); }} Py::list(out) }], true)?;
    return Ok(py::add(&v_m.clone(), &py::math1("log", &py::fsum(&{ let mut out = vec![]; for t4031 in py::iter(&(py::zip(&[v_vals.clone(), v_weights.clone()])?))? { let mut v_v: Py; let mut v_w: Py; 
let t4032 = (t4031).items()?;
if t4032.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_v = t4032[0].clone();
v_w = t4032[1].clone();
if ({ let t4033 = v_w.clone(); Py::Bool({ let t4034 = Py::Int(0i64); if py::gt(&t4033, &t4034)? { true } else { false } }) }).truthy() { out.push(py::mul(&v_w.clone(), &py::math1("exp", &py::sub(&v_v.clone(), &v_m.clone())?)?)?); }} Py::list(out) })?)?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__mat(mut v_m: Py, mut v_name: Py, mut v_rows: Py, mut v_cols: Py) -> R {
    if ({ let t4876 = py::not(&Py::Bool(matches!(v_m.clone(), Py::List(_) | Py::Tuple(_)))); if t4876.truthy() { t4876 } else { { let t4875 = py::not(&v_m.clone()); if t4875.truthy() { t4875 } else { py::not(&Py::Bool(({ let mut out = vec![]; for t4874 in py::iter(&(v_m.clone()))? { let mut v_r: Py; 
v_r = t4874;
out.push(Py::Bool(matches!(v_r.clone(), Py::List(_) | Py::Tuple(_)))); } Py::list(out) }).items()?.iter().all(|x| x.truthy()))) } } } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str(&[py::format(&v_name.clone(), "")?, " must be a matrix (rows of numbers)".to_string()].concat()))));
    }
    v_m = { let mut out = vec![]; for t4877 in py::iter(&(v_m.clone()))? { let mut v_r: Py; 
v_r = t4877;
out.push({ let mut out = vec![]; for t4878 in py::iter(&(v_r.clone()))? { let mut v_v: Py; 
v_v = t4878;
out.push(f__fin(v_v.clone(), v_name.clone())?); } Py::list(out) }); } Py::list(out) };
    if ({ let t4882 = Py::Bool(({ let mut out = vec![]; for t4879 in py::iter(&(v_m.clone()))? { let mut v_r: Py; 
v_r = t4879;
out.push({ let t4880 = py::len(&v_r.clone())?; Py::Bool({ let t4881 = py::len(&py::getitem(&v_m.clone(), &Py::Int(0i64))?)?; if py::ne(&t4880, &t4881) { true } else { false } }) }); } Py::list(out) }).items()?.iter().any(|x| x.truthy())); if t4882.truthy() { t4882 } else { py::not(&py::getitem(&v_m.clone(), &Py::Int(0i64))?) } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str(&[py::format(&v_name.clone(), "")?, " must be rectangular".to_string()].concat()))));
    }
    if ({ let t4887 = { let t4883 = v_rows.clone(); Py::Bool({ let t4884 = Py::None; if !py::is(&t4883, &t4884) { true } else { false } }) }; if !t4887.truthy() { t4887 } else { { let t4885 = py::len(&v_m.clone())?; Py::Bool({ let t4886 = v_rows.clone(); if py::ne(&t4885, &t4886) { true } else { false } }) } } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str(&[py::format(&v_name.clone(), "")?, " must have ".to_string(), py::format(&v_rows.clone(), "")?, " rows".to_string()].concat()))));
    }
    if ({ let t4892 = { let t4888 = v_cols.clone(); Py::Bool({ let t4889 = Py::None; if !py::is(&t4888, &t4889) { true } else { false } }) }; if !t4892.truthy() { t4892 } else { { let t4890 = py::len(&py::getitem(&v_m.clone(), &Py::Int(0i64))?)?; Py::Bool({ let t4891 = v_cols.clone(); if py::ne(&t4890, &t4891) { true } else { false } }) } } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str(&[py::format(&v_name.clone(), "")?, " must have ".to_string(), py::format(&v_cols.clone(), "")?, " columns".to_string()].concat()))));
    }
    return Ok(v_m.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__mean(mut v_x: Py) -> R {
    return Ok(py::truediv(&py::fsum(&v_x.clone())?, &py::len(&v_x.clone())?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__nn_check(mut v_prior_mean: Py, mut v_prior_sd: Py, mut v_sigma: Py) -> R {
    return Ok(Py::tuple(vec![f__fin(v_prior_mean.clone(), Py::str("the prior mean"))?, f__pos(v_prior_sd.clone(), Py::str("the prior standard deviation"))?, f__pos(v_sigma.clone(), Py::str("sigma"))?]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__nn_post(mut v_m0: Py, mut v_t0: Py, mut v_s: Py, mut v_xbar: Py, mut v_n: Py) -> R {
    let mut v_prec: Py = Py::None;
    let mut v_v: Py = Py::None;
    v_prec = py::add(&py::truediv(&Py::Int(1i64), &py::pow(&v_t0.clone(), &Py::Int(2i64))?)?, &py::truediv(&v_n.clone(), &py::pow(&v_s.clone(), &Py::Int(2i64))?)?)?;
    v_v = py::truediv(&Py::Int(1i64), &v_prec.clone())?;
    return Ok(Py::tuple(vec![py::mul(&v_v.clone(), &py::add(&py::truediv(&v_m0.clone(), &py::pow(&v_t0.clone(), &Py::Int(2i64))?)?, &py::truediv(&py::mul(&v_n.clone(), &v_xbar.clone())?, &py::pow(&v_s.clone(), &Py::Int(2i64))?)?)?)?, v_v.clone()]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__nonneg(mut v_v: Py, mut v_name: Py) -> R {
    v_v = f__fin(v_v.clone(), v_name.clone())?;
    if ({ let t5137 = v_v.clone(); Py::Bool({ let t5138 = Py::Int(0i64); if py::lt(&t5137, &t5138)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str(&[py::format(&v_name.clone(), "")?, " must not be negative".to_string()].concat()))));
    }
    return Ok(v_v.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__norm_pdf(mut v_z: Py) -> R {
    return Ok(py::math1("exp", &py::sub(&py::mul(&py::mul(&py::neg(&Py::Float(0.5))?, &v_z.clone())?, &v_z.clone())?, &c__LN_SQRT_2PI()?)?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__payoff_check(mut v_payoffs: Py, mut v_probabilities: Py, mut v_name: Py) -> R {
    let mut v_m: Py = Py::None;
    let mut v_p: Py = Py::None;
    v_m = f__mat(v_payoffs.clone(), v_name.clone(), Py::None, Py::None)?;
    v_p = f__simplex(v_probabilities.clone(), Py::str("the state probabilities"), py::len(&py::getitem(&v_m.clone(), &Py::Int(0i64))?)?)?;
    return Ok(Py::tuple(vec![v_m.clone(), v_p.clone()]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__pos(mut v_v: Py, mut v_name: Py) -> R {
    v_v = f__fin(v_v.clone(), v_name.clone())?;
    if ({ let t4757 = v_v.clone(); Py::Bool({ let t4758 = Py::Int(0i64); if py::le(&t4757, &t4758)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str(&[py::format(&v_name.clone(), "")?, " must be positive".to_string()].concat()))));
    }
    return Ok(v_v.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__prob(mut v_v: Py, mut v_name: Py, mut v_open_: Py) -> R {
    v_v = f__fin(v_v.clone(), v_name.clone())?;
    if ({ let t4708 = v_open_.clone(); if !t4708.truthy() { t4708 } else { py::not(&{ let t4705 = Py::Float(0.0); Py::Bool({ let t4706 = v_v.clone(); if py::lt(&t4705, &t4706)? { { let t4707 = Py::Float(1.0); if py::lt(&t4706, &t4707)? { true } else { false } } } else { false } }) }) } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str(&[py::format(&v_name.clone(), "")?, " must be strictly between 0 and 1".to_string()].concat()))));
    }
    if (py::not(&{ let t4709 = Py::Float(0.0); Py::Bool({ let t4710 = v_v.clone(); if py::le(&t4709, &t4710)? { { let t4711 = Py::Float(1.0); if py::le(&t4710, &t4711)? { true } else { false } } } else { false } }) })).truthy() {
        return Err(py::value_error(py::to_str(&Py::str(&[py::format(&v_name.clone(), "")?, " must be a probability between 0 and 1".to_string()].concat()))));
    }
    return Ok(v_v.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__r12(mut v_v: Py) -> R {
    if (Py::Bool(matches!(v_v.clone(), Py::Dict(_)))).truthy() {
        return Ok({ let mut out = vec![]; for t5324 in py::iter(&(py::method(&v_v.clone(), "items", &[])?))? { let mut v_k: Py; let mut v_x: Py; 
let t5325 = (t5324).items()?;
if t5325.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_k = t5325[0].clone();
v_x = t5325[1].clone();
out.push((v_k.clone(), f__r12(v_x.clone())?)); } Py::dict(out) });
    }
    if (Py::Bool(matches!(v_v.clone(), Py::List(_) | Py::Tuple(_)))).truthy() {
        return Ok({ let mut out = vec![]; for t5326 in py::iter(&(v_v.clone()))? { let mut v_x: Py; 
v_x = t5326;
out.push(f__r12(v_x.clone())?); } Py::list(out) });
    }
    if ({ let t5330 = Py::Bool(matches!(v_v.clone(), Py::Float(_))); if !t5330.truthy() { t5330 } else { { let t5329 = py::math1("isfinite", &v_v.clone())?; if !t5329.truthy() { t5329 } else { { let t5327 = v_v.clone(); Py::Bool({ let t5328 = Py::Float(0.0); if py::ne(&t5327, &t5328) { true } else { false } }) } } } } }).truthy() {
        return Ok(py::float(&Py::str(&[py::format(&v_v.clone(), ".12g")?].concat()))?);
    }
    return Ok(v_v.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__radical_inverse(mut v_i: Py, mut v_b: Py) -> R {
    let mut v_d: Py = Py::None;
    let mut v_den: Py = Py::None;
    let mut v_num: Py = Py::None;
    let t2578 = (Py::tuple(vec![Py::Int(0i64), Py::Int(1i64)])).items()?;
    if t2578.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_num = t2578[0].clone();
    v_den = t2578[1].clone();
    while ({ let t2581 = v_i.clone(); Py::Bool({ let t2582 = Py::Int(0i64); if py::gt(&t2581, &t2582)? { true } else { false } }) }).truthy() {
        let t2579 = (Py::tuple(vec![py::floordiv(&v_i.clone(), &v_b.clone())?, py::modulo(&v_i.clone(), &v_b.clone())?])).items()?;
        if t2579.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_i = t2579[0].clone();
        v_d = t2579[1].clone();
        let t2580 = (Py::tuple(vec![py::add(&py::mul(&v_num.clone(), &v_b.clone())?, &v_d.clone())?, py::mul(&v_den.clone(), &v_b.clone())?])).items()?;
        if t2580.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_num = t2580[0].clone();
        v_den = t2580[1].clone();
    }
    return Ok(py::truediv(&v_num.clone(), &v_den.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__simplex(mut v_p: Py, mut v_name: Py, mut v_n: Py) -> R {
    v_p = f__vec(v_p.clone(), v_name.clone(), v_n.clone(), Py::Int(1i64))?;
    if (Py::Bool(({ let mut out = vec![]; for t4648 in py::iter(&(v_p.clone()))? { let mut v_v: Py; 
v_v = t4648;
out.push({ let t4649 = v_v.clone(); Py::Bool({ let t4650 = Py::Int(0i64); if py::lt(&t4649, &t4650)? { true } else { false } }) }); } Py::list(out) }).items()?.iter().any(|x| x.truthy()))).truthy() {
        return Err(py::value_error(py::to_str(&Py::str(&[py::format(&v_name.clone(), "")?, " must not be negative".to_string()].concat()))));
    }
    if ({ let t4651 = py::abs(&py::sub(&py::fsum(&v_p.clone())?, &Py::Float(1.0))?)?; Py::Bool({ let t4652 = Py::Float(1e-09); if py::gt(&t4651, &t4652)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str(&[py::format(&v_name.clone(), "")?, " must sum to 1 (they sum to ".to_string(), py::format(&py::fsum(&v_p.clone())?, ".6g")?, ")".to_string()].concat()))));
    }
    return Ok(v_p.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__t_pdf(mut v_t: Py, mut v_v: Py) -> R {
    return Ok(py::math1("exp", &py::sub(&py::sub(&py::sub(&py::math1("lgamma", &py::mul(&Py::Float(0.5), &py::add(&v_v.clone(), &Py::Int(1i64))?)?)?, &py::math1("lgamma", &py::mul(&Py::Float(0.5), &v_v.clone())?)?)?, &py::mul(&Py::Float(0.5), &py::math1("log", &py::mul(&v_v.clone(), &Py::Float(std::f64::consts::PI))?)?)?)?, &py::mul(&py::mul(&Py::Float(0.5), &py::add(&v_v.clone(), &Py::Int(1i64))?)?, &py::math1("log1p", &py::truediv(&py::mul(&v_t.clone(), &v_t.clone())?, &v_v.clone())?)?)?)?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__t_ppf(mut v_p: Py, mut v_v: Py) -> R {
    if (py::not(&{ let t5269 = Py::Float(0.0); Py::Bool({ let t5270 = v_p.clone(); if py::lt(&t5269, &t5270)? { { let t5271 = Py::Float(1.0); if py::lt(&t5270, &t5271)? { true } else { false } } } else { false } }) })).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("a probability strictly between 0 and 1 is required"))));
    }
    return Ok(f__invert({ let v_p = v_p.clone(); let v_v = v_v.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_t: Py = a.get(0).cloned().unwrap_or(Py::None);
            return Ok(f__t_tails(v_t.clone(), v_v.clone())?);
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) }, { let v_p = v_p.clone(); let v_v = v_v.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_t: Py = a.get(0).cloned().unwrap_or(Py::None);
            return Ok(f__t_pdf(v_t.clone(), v_v.clone())?);
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) }, v_p.clone(), py::neg(&Py::Float(1000000000000.0))?, Py::Float(1000000000000.0), Py::Float(0.0))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__t_tails(mut v_t: Py, mut v_v: Py) -> R {
    let mut v__c: Py = Py::None;
    let mut v_i: Py = Py::None;
    let mut v_x: Py = Py::None;
    let mut v_y: Py = Py::None;
    let t5297 = (Py::tuple(vec![py::truediv(&v_v.clone(), &py::add(&v_v.clone(), &py::mul(&v_t.clone(), &v_t.clone())?)?)?, py::truediv(&py::mul(&v_t.clone(), &v_t.clone())?, &py::add(&v_v.clone(), &py::mul(&v_t.clone(), &v_t.clone())?)?)?])).items()?;
    if t5297.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_x = t5297[0].clone();
    v_y = t5297[1].clone();
    let t5321 = (f__ibeta(py::mul(&Py::Float(0.5), &v_v.clone())?, Py::Float(0.5), v_x.clone(), v_y.clone())?).items()?;
    if t5321.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_i = t5321[0].clone();
    v__c = t5321[1].clone();
    return Ok((if ({ let t5322 = v_t.clone(); Py::Bool({ let t5323 = Py::Int(0i64); if py::gt(&t5322, &t5323)? { true } else { false } }) }).truthy() { Py::tuple(vec![py::sub(&Py::Float(1.0), &py::mul(&Py::Float(0.5), &v_i.clone())?)?, py::mul(&Py::Float(0.5), &v_i.clone())?]) } else { Py::tuple(vec![py::mul(&Py::Float(0.5), &v_i.clone())?, py::sub(&Py::Float(1.0), &py::mul(&Py::Float(0.5), &v_i.clone())?)?]) }));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__tqli_first(mut v_d: Py, mut v_e: Py) -> R {
    let mut v_b: Py = Py::None;
    let mut v_c: Py = Py::None;
    let mut v_dd: Py = Py::None;
    let mut v_early: Py = Py::None;
    let mut v_f: Py = Py::None;
    let mut v_g: Py = Py::None;
    let mut v_i: Py = Py::None;
    let mut v_it: Py = Py::None;
    let mut v_l: Py = Py::None;
    let mut v_m: Py = Py::None;
    let mut v_n: Py = Py::None;
    let mut v_p: Py = Py::None;
    let mut v_r: Py = Py::None;
    let mut v_s: Py = Py::None;
    v_n = py::len(&v_d.clone())?;
    let t2488 = (Py::tuple(vec![Py::list((v_d.clone()).items()?), py::add(&Py::list((v_e.clone()).items()?), &Py::list(vec![Py::Float(0.0)]))?])).items()?;
    if t2488.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_d = t2488[0].clone();
    v_e = t2488[1].clone();
    for t2489 in py::range_iter(&[(v_n.clone()).i()?]) {
        v_l = t2489;
        v_it = Py::Int(0i64);
        while (Py::Bool(true)).truthy() {
            v_m = v_l.clone();
            while ({ let t2492 = v_m.clone(); Py::Bool({ let t2493 = py::sub(&v_n.clone(), &Py::Int(1i64))?; if py::lt(&t2492, &t2493)? { true } else { false } }) }).truthy() {
                v_dd = py::add(&py::abs(&py::getitem(&v_d.clone(), &v_m.clone())?)?, &py::abs(&py::getitem(&v_d.clone(), &py::add(&v_m.clone(), &Py::Int(1i64))?)?)?)?;
                if ({ let t2490 = py::add(&py::abs(&py::getitem(&v_e.clone(), &v_m.clone())?)?, &v_dd.clone())?; Py::Bool({ let t2491 = v_dd.clone(); if py::eq(&t2490, &t2491) { true } else { false } }) }).truthy() {
                    break;
                }
                v_m = py::iadd(&v_m, &Py::Int(1i64))?;
            }
            if ({ let t2494 = v_m.clone(); Py::Bool({ let t2495 = v_l.clone(); if py::eq(&t2494, &t2495) { true } else { false } }) }).truthy() {
                break;
            }
            v_it = py::iadd(&v_it, &Py::Int(1i64))?;
            if ({ let t2496 = v_it.clone(); Py::Bool({ let t2497 = Py::Int(100i64); if py::gt(&t2496, &t2497)? { true } else { false } }) }).truthy() {
                return Err(PyErr::Other("ArithmeticError".into()));
            }
            v_g = py::truediv(&py::sub(&py::getitem(&v_d.clone(), &py::add(&v_l.clone(), &Py::Int(1i64))?)?, &py::getitem(&v_d.clone(), &v_l.clone())?)?, &py::mul(&Py::Float(2.0), &py::getitem(&v_e.clone(), &v_l.clone())?)?)?;
            v_r = py::hypot(&[v_g.clone(), Py::Float(1.0)])?;
            v_g = py::add(&py::sub(&py::getitem(&v_d.clone(), &v_m.clone())?, &py::getitem(&v_d.clone(), &v_l.clone())?)?, &py::truediv(&py::getitem(&v_e.clone(), &v_l.clone())?, &py::add(&v_g.clone(), &py::copysign(&v_r.clone(), &v_g.clone())?)?)?)?;
            let t2498 = Py::Float(1.0);
            v_s = t2498.clone();
            v_c = t2498.clone();
            v_p = Py::Float(0.0);
            v_i = py::sub(&v_m.clone(), &Py::Int(1i64))?;
            v_early = Py::Bool(false);
            while ({ let t2505 = v_i.clone(); Py::Bool({ let t2506 = v_l.clone(); if py::ge(&t2505, &t2506)? { true } else { false } }) }).truthy() {
                let t2499 = (Py::tuple(vec![py::mul(&v_s.clone(), &py::getitem(&v_e.clone(), &v_i.clone())?)?, py::mul(&v_c.clone(), &py::getitem(&v_e.clone(), &v_i.clone())?)?])).items()?;
                if t2499.len() != 2 { return Err(PyErr::Value("unpack".into())); }
                v_f = t2499[0].clone();
                v_b = t2499[1].clone();
                v_r = py::hypot(&[v_f.clone(), v_g.clone()])?;
                py::setitem(&v_e.clone(), &py::add(&v_i.clone(), &Py::Int(1i64))?, v_r.clone())?;
                if ({ let t2500 = v_r.clone(); Py::Bool({ let t2501 = Py::Float(0.0); if py::eq(&t2500, &t2501) { true } else { false } }) }).truthy() {
                    let t2502 = v_d.clone(); let t2503 = py::add(&v_i.clone(), &Py::Int(1i64))?;
                    py::setitem(&t2502, &t2503, py::sub(&py::getitem(&t2502, &t2503)?, &v_p.clone())?)?;
                    py::setitem(&v_e.clone(), &v_m.clone(), Py::Float(0.0))?;
                    v_early = Py::Bool(true);
                    break;
                }
                let t2504 = (Py::tuple(vec![py::truediv(&v_f.clone(), &v_r.clone())?, py::truediv(&v_g.clone(), &v_r.clone())?])).items()?;
                if t2504.len() != 2 { return Err(PyErr::Value("unpack".into())); }
                v_s = t2504[0].clone();
                v_c = t2504[1].clone();
                v_g = py::sub(&py::getitem(&v_d.clone(), &py::add(&v_i.clone(), &Py::Int(1i64))?)?, &v_p.clone())?;
                v_r = py::add(&py::mul(&py::sub(&py::getitem(&v_d.clone(), &v_i.clone())?, &v_g.clone())?, &v_s.clone())?, &py::mul(&py::mul(&Py::Float(2.0), &v_c.clone())?, &v_b.clone())?)?;
                v_p = py::mul(&v_s.clone(), &v_r.clone())?;
                py::setitem(&v_d.clone(), &py::add(&v_i.clone(), &Py::Int(1i64))?, py::add(&v_g.clone(), &v_p.clone())?)?;
                v_g = py::sub(&py::mul(&v_c.clone(), &v_r.clone())?, &v_b.clone())?;
                v_i = py::sub(&v_i, &Py::Int(1i64))?;
            }
            if (v_early.clone()).truthy() {
                continue;
            }
            let t2507 = v_d.clone(); let t2508 = v_l.clone();
            py::setitem(&t2507, &t2508, py::sub(&py::getitem(&t2507, &t2508)?, &v_p.clone())?)?;
            py::setitem(&v_e.clone(), &v_l.clone(), v_g.clone())?;
            py::setitem(&v_e.clone(), &v_m.clone(), Py::Float(0.0))?;
        }
    }
    return Ok(py::sorted(&v_d.clone(), (Py::Bool(false)).truthy())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__vec(mut v_x: Py, mut v_name: Py, mut v_n: Py, mut v_min_len: Py) -> R {
    if (py::not(&Py::Bool(matches!(v_x.clone(), Py::List(_) | Py::Tuple(_))))).truthy() {
        return Err(py::value_error(py::to_str(&Py::str(&[py::format(&v_name.clone(), "")?, " must be a list of numbers".to_string()].concat()))));
    }
    v_x = { let mut out = vec![]; for t5244 in py::iter(&(v_x.clone()))? { let mut v_v: Py; 
v_v = t5244;
out.push(f__fin(v_v.clone(), v_name.clone())?); } Py::list(out) };
    if ({ let t5247 = py::len(&v_x.clone())?; Py::Bool({ let t5248 = v_min_len.clone(); if py::lt(&t5247, &t5248)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str(&[py::format(&v_name.clone(), "")?, " needs at least ".to_string(), py::format(&v_min_len.clone(), "")?, " values".to_string()].concat()))));
    }
    if ({ let t5253 = { let t5249 = v_n.clone(); Py::Bool({ let t5250 = Py::None; if !py::is(&t5249, &t5250) { true } else { false } }) }; if !t5253.truthy() { t5253 } else { { let t5251 = py::len(&v_x.clone())?; Py::Bool({ let t5252 = v_n.clone(); if py::ne(&t5251, &t5252) { true } else { false } }) } } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str(&[py::format(&v_name.clone(), "")?, " must have ".to_string(), py::format(&v_n.clone(), "")?, " values (one per entry of the other list)".to_string()].concat()))));
    }
    return Ok(v_x.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__whole(mut v_v: Py, mut v_name: Py, mut v_lo: Py) -> R {
    v_v = f__fin(v_v.clone(), v_name.clone())?;
    if ({ let t4305 = { let t4301 = v_v.clone(); Py::Bool({ let t4302 = py::int(&v_v.clone())?; if py::ne(&t4301, &t4302) { true } else { false } }) }; if t4305.truthy() { t4305 } else { { let t4303 = v_v.clone(); Py::Bool({ let t4304 = v_lo.clone(); if py::lt(&t4303, &t4304)? { true } else { false } }) } } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str(&[py::format(&v_name.clone(), "")?, " must be a whole number >= ".to_string(), py::format(&v_lo.clone(), "")?].concat()))));
    }
    return Ok(py::int(&v_v.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__z(mut v_p: Py) -> R {
    if (py::not(&{ let t3680 = Py::Float(0.0); Py::Bool({ let t3681 = v_p.clone(); if py::lt(&t3680, &t3681)? { { let t3682 = Py::Float(1.0); if py::lt(&t3681, &t3682)? { true } else { false } } } else { false } }) })).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("a probability strictly between 0 and 1 is required"))));
    }
    return Ok(f__invert({ let v_p = v_p.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_z: Py = a.get(0).cloned().unwrap_or(Py::None);
            return Ok(Py::tuple(vec![py::mul(&Py::Float(0.5), &py::math1("erfc", &py::truediv(&py::neg(&v_z.clone())?, &c__SQRT2()?)?)?)?, py::mul(&Py::Float(0.5), &py::math1("erfc", &py::truediv(&v_z.clone(), &c__SQRT2()?)?)?)?]));
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) }, py::func(|a: &[Py]| -> R { f__norm_pdf(a.get(0).cloned().unwrap_or(Py::None)) }), v_p.clone(), py::neg(&Py::Float(40.0))?, Py::Float(40.0), Py::Float(0.0))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__z_moment(mut v_k: Py) -> R {
    return Ok((if (py::modulo(&v_k.clone(), &Py::Int(2i64))?).truthy() { Py::Float(0.0) } else { (if (v_k.clone()).truthy() { py::float(&py::prod(&Py::list((py::range_iter(&[(py::sub(&v_k.clone(), &Py::Int(1i64))?).i()?, (Py::Int(0i64)).i()?, (py::neg(&Py::Int(2i64))?).i()?])).into_iter().collect::<Vec<Py>>()))?)? } else { Py::Float(1.0) }) }));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_run_bayes(mut v_p_a: Py, mut v_p_b_given_a: Py, mut v_p_b_given_not_a: Py) -> R {
    let mut v_l0: Py = Py::None;
    let mut v_l1: Py = Py::None;
    let mut v_pa: Py = Py::None;
    let mut v_pb: Py = Py::None;
    let t308 = (Py::tuple(vec![f__prob(v_p_a.clone(), Py::str("P(A)"), Py::Bool(false))?, f__prob(v_p_b_given_a.clone(), Py::str("P(B|A)"), Py::Bool(false))?, f__prob(v_p_b_given_not_a.clone(), Py::str("P(B|not A)"), Py::Bool(false))?])).items()?;
    if t308.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_pa = t308[0].clone();
    v_l1 = t308[1].clone();
    v_l0 = t308[2].clone();
    v_pb = py::add(&py::mul(&v_pa.clone(), &v_l1.clone())?, &py::mul(&py::sub(&Py::Float(1.0), &v_pa.clone())?, &v_l0.clone())?)?;
    if ({ let t309 = v_pb.clone(); Py::Bool({ let t310 = Py::Float(0.0); if py::le(&t309, &t310)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("P(B) is 0: B cannot occur, so P(A|B) is undefined"))));
    }
    return Ok(Py::dict(vec![(Py::str("p_a_given_b"), py::truediv(&py::mul(&v_pa.clone(), &v_l1.clone())?, &v_pb.clone())?), (Py::str("p_b"), v_pb.clone()), (Py::str("p_not_a_given_b"), py::truediv(&py::mul(&py::sub(&Py::Float(1.0), &v_pa.clone())?, &v_l0.clone())?, &v_pb.clone())?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_run_bayes_action(mut v_losses: Py, mut v_probabilities: Py) -> R {
    let mut v_b: Py = Py::None;
    let mut v_el: Py = Py::None;
    let mut v_m: Py = Py::None;
    let mut v_p: Py = Py::None;
    let t86 = (f__payoff_check(v_losses.clone(), v_probabilities.clone(), Py::str("the loss matrix"))?).items()?;
    if t86.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_m = t86[0].clone();
    v_p = t86[1].clone();
    v_el = f__ev(v_m.clone(), v_p.clone())?;
    v_b = f__argbest(v_el.clone(), py::neg(&Py::Int(1i64))?)?;
    return Ok(Py::dict(vec![(Py::str("expected_losses"), v_el.clone()), (Py::str("bayes_action"), py::add(&v_b.clone(), &Py::Int(1i64))?), (Py::str("bayes_risk"), py::getitem(&v_el.clone(), &v_b.clone())?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_run_bb_ci(mut v_alpha: Py, mut v_beta: Py, mut v_successes: Py, mut v_trials: Py, mut v_level: Py) -> R {
    let mut v_a: Py = Py::None;
    let mut v_b: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_n: Py = Py::None;
    let mut v_r: Py = Py::None;
    let t363 = (f__bb_check(v_alpha.clone(), v_beta.clone(), v_successes.clone(), v_trials.clone())?).items()?;
    if t363.len() != 4 { return Err(PyErr::Value("unpack".into())); }
    v_a = t363[0].clone();
    v_b = t363[1].clone();
    v_k = t363[2].clone();
    v_n = t363[3].clone();
    v_r = f_run_beta_ci(py::add(&v_a.clone(), &v_k.clone())?, py::sub(&py::add(&v_b.clone(), &v_n.clone())?, &v_k.clone())?, v_level.clone())?;
    return Ok(Py::dict(vec![(Py::str("post_alpha"), py::add(&v_a.clone(), &v_k.clone())?), (Py::str("post_beta"), py::sub(&py::add(&v_b.clone(), &v_n.clone())?, &v_k.clone())?), (Py::str("lower"), py::getitem(&v_r.clone(), &Py::str("lower"))?), (Py::str("upper"), py::getitem(&v_r.clone(), &Py::str("upper"))?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_run_bb_evidence(mut v_alpha: Py, mut v_beta: Py, mut v_successes: Py, mut v_trials: Py) -> R {
    let mut v_a: Py = Py::None;
    let mut v_b: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_lml: Py = Py::None;
    let mut v_n: Py = Py::None;
    let t501 = (f__bb_check(v_alpha.clone(), v_beta.clone(), v_successes.clone(), v_trials.clone())?).items()?;
    if t501.len() != 4 { return Err(PyErr::Value("unpack".into())); }
    v_a = t501[0].clone();
    v_b = t501[1].clone();
    v_k = t501[2].clone();
    v_n = t501[3].clone();
    v_lml = py::sub(&py::add(&py::sub(&py::sub(&py::math1("lgamma", &py::add(&v_n.clone(), &Py::Int(1i64))?)?, &py::math1("lgamma", &py::add(&v_k.clone(), &Py::Int(1i64))?)?)?, &py::math1("lgamma", &py::add(&py::sub(&v_n.clone(), &v_k.clone())?, &Py::Int(1i64))?)?)?, &f__lbeta(py::add(&v_a.clone(), &v_k.clone())?, py::sub(&py::add(&v_b.clone(), &v_n.clone())?, &v_k.clone())?)?)?, &f__lbeta(v_a.clone(), v_b.clone())?)?;
    return Ok(Py::dict(vec![(Py::str("marginal_likelihood"), py::math1("exp", &v_lml.clone())?), (Py::str("log_marginal_likelihood"), v_lml.clone())]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_run_bb_post(mut v_alpha: Py, mut v_beta: Py, mut v_successes: Py, mut v_trials: Py) -> R {
    let mut v_a: Py = Py::None;
    let mut v_a2: Py = Py::None;
    let mut v_b: Py = Py::None;
    let mut v_b2: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_m: Py = Py::None;
    let mut v_n: Py = Py::None;
    let mut v_v: Py = Py::None;
    let t554 = (f__bb_check(v_alpha.clone(), v_beta.clone(), v_successes.clone(), v_trials.clone())?).items()?;
    if t554.len() != 4 { return Err(PyErr::Value("unpack".into())); }
    v_a = t554[0].clone();
    v_b = t554[1].clone();
    v_k = t554[2].clone();
    v_n = t554[3].clone();
    let t555 = (Py::tuple(vec![py::add(&v_a.clone(), &v_k.clone())?, py::sub(&py::add(&v_b.clone(), &v_n.clone())?, &v_k.clone())?])).items()?;
    if t555.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_a2 = t555[0].clone();
    v_b2 = t555[1].clone();
    v_m = py::truediv(&v_a2.clone(), &py::add(&v_a2.clone(), &v_b2.clone())?)?;
    v_v = py::truediv(&py::mul(&v_a2.clone(), &v_b2.clone())?, &py::mul(&py::pow(&py::add(&v_a2.clone(), &v_b2.clone())?, &Py::Int(2i64))?, &py::add(&py::add(&v_a2.clone(), &v_b2.clone())?, &Py::Int(1i64))?)?)?;
    return Ok(Py::dict(vec![(Py::str("post_alpha"), v_a2.clone()), (Py::str("post_beta"), v_b2.clone()), (Py::str("posterior_mean"), v_m.clone()), (Py::str("posterior_variance"), v_v.clone()), (Py::str("posterior_sd"), py::math1("sqrt", &v_v.clone())?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_run_bb_pred(mut v_alpha: Py, mut v_beta: Py, mut v_future_trials: Py, mut v_future_successes: Py) -> R {
    let mut v_a: Py = Py::None;
    let mut v_b: Py = Py::None;
    let mut v_j: Py = Py::None;
    let mut v_lc: Py = Py::None;
    let mut v_lower: Py = Py::None;
    let mut v_m: Py = Py::None;
    let mut v_pmf: Py = Py::None;
    let mut v_upper: Py = Py::None;
    let t599 = (Py::tuple(vec![f__pos(v_alpha.clone(), Py::str("alpha"))?, f__pos(v_beta.clone(), Py::str("beta"))?])).items()?;
    if t599.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_a = t599[0].clone();
    v_b = t599[1].clone();
    let t605 = (Py::tuple(vec![f__whole(v_future_trials.clone(), Py::str("the number of future trials"), Py::Int(1i64))?, f__whole(v_future_successes.clone(), Py::str("the number of future successes"), Py::Int(0i64))?])).items()?;
    if t605.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_m = t605[0].clone();
    v_j = t605[1].clone();
    if ({ let t606 = v_j.clone(); Py::Bool({ let t607 = v_m.clone(); if py::gt(&t606, &t607)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("future successes cannot exceed future trials"))));
    }
    v_lc = { let v_a = v_a.clone(); let v_alpha = v_alpha.clone(); let v_b = v_b.clone(); let v_beta = v_beta.clone(); let v_future_successes = v_future_successes.clone(); let v_future_trials = v_future_trials.clone(); let v_j = v_j.clone(); let v_m = v_m.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_i: Py = a.get(0).cloned().unwrap_or(Py::None);
            return Ok(py::sub(&py::sub(&py::math1("lgamma", &py::add(&v_m.clone(), &Py::Int(1i64))?)?, &py::math1("lgamma", &py::add(&v_i.clone(), &Py::Int(1i64))?)?)?, &py::math1("lgamma", &py::add(&py::sub(&v_m.clone(), &v_i.clone())?, &Py::Int(1i64))?)?)?);
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) };
    v_pmf = { let v_a = v_a.clone(); let v_alpha = v_alpha.clone(); let v_b = v_b.clone(); let v_beta = v_beta.clone(); let v_future_successes = v_future_successes.clone(); let v_future_trials = v_future_trials.clone(); let v_j = v_j.clone(); let v_lc = v_lc.clone(); let v_m = v_m.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_i: Py = a.get(0).cloned().unwrap_or(Py::None);
            return Ok(py::math1("exp", &py::sub(&py::add(&py::call(&v_lc.clone(), &[v_i.clone()])?, &f__lbeta(py::add(&v_a.clone(), &v_i.clone())?, py::sub(&py::add(&v_b.clone(), &v_m.clone())?, &v_i.clone())?)?)?, &f__lbeta(v_a.clone(), v_b.clone())?)?)?);
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) };
    v_lower = py::fsum(&{ let mut out = vec![]; for t608 in py::range_iter(&[(Py::Int(0i64)).i()?, (py::add(&v_j.clone(), &Py::Int(1i64))?).i()?]) { let mut v_i: Py; 
v_i = t608;
out.push(py::call(&v_pmf.clone(), &[v_i.clone()])?); } Py::list(out) })?;
    v_upper = py::fsum(&{ let mut out = vec![]; for t609 in py::range_iter(&[(py::add(&v_j.clone(), &Py::Int(1i64))?).i()?, (py::add(&v_m.clone(), &Py::Int(1i64))?).i()?]) { let mut v_i: Py; 
v_i = t609;
out.push(py::call(&v_pmf.clone(), &[v_i.clone()])?); } Py::list(out) })?;
    return Ok(Py::dict(vec![(Py::str("probability"), py::call(&v_pmf.clone(), &[v_j.clone()])?), (Py::str("prob_at_most"), py::truediv(&v_lower.clone(), &py::add(&v_lower.clone(), &v_upper.clone())?)?), (Py::str("prob_more"), py::truediv(&v_upper.clone(), &py::add(&v_lower.clone(), &v_upper.clone())?)?), (Py::str("predictive_mean"), py::truediv(&py::mul(&v_m.clone(), &v_a.clone())?, &py::add(&v_a.clone(), &v_b.clone())?)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_run_beta_ci(mut v_alpha: Py, mut v_beta: Py, mut v_level: Py) -> R {
    let mut v_a: Py = Py::None;
    let mut v_b: Py = Py::None;
    let mut v_c: Py = Py::None;
    let t658 = (Py::tuple(vec![f__pos(v_alpha.clone(), Py::str("alpha"))?, f__pos(v_beta.clone(), Py::str("beta"))?, f__level(v_level.clone())?])).items()?;
    if t658.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_a = t658[0].clone();
    v_b = t658[1].clone();
    v_c = t658[2].clone();
    return Ok(Py::dict(vec![(Py::str("lower"), f__beta_ppf(py::mul(&Py::Float(0.5), &py::sub(&Py::Int(1i64), &v_c.clone())?)?, v_a.clone(), v_b.clone())?), (Py::str("upper"), f__beta_ppf(py::mul(&Py::Float(0.5), &py::add(&Py::Int(1i64), &v_c.clone())?)?, v_a.clone(), v_b.clone())?), (Py::str("median"), f__beta_ppf(Py::Float(0.5), v_a.clone(), v_b.clone())?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_run_beta_from_moments(mut v_mean: Py, mut v_sd: Py) -> R {
    let mut v_m: Py = Py::None;
    let mut v_nu: Py = Py::None;
    let mut v_s: Py = Py::None;
    let t1017 = (Py::tuple(vec![f__prob(v_mean.clone(), Py::str("the prior mean"), Py::Bool(true))?, f__pos(v_sd.clone(), Py::str("the prior standard deviation"))?])).items()?;
    if t1017.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_m = t1017[0].clone();
    v_s = t1017[1].clone();
    if ({ let t1018 = py::mul(&v_s.clone(), &v_s.clone())?; Py::Bool({ let t1019 = py::mul(&v_m.clone(), &py::sub(&Py::Int(1i64), &v_m.clone())?)?; if py::ge(&t1018, &t1019)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the variance must be smaller than mean x (1 - mean) for a beta distribution"))));
    }
    v_nu = py::sub(&py::truediv(&py::mul(&v_m.clone(), &py::sub(&Py::Int(1i64), &v_m.clone())?)?, &py::mul(&v_s.clone(), &v_s.clone())?)?, &Py::Int(1i64))?;
    return Ok(Py::dict(vec![(Py::str("alpha"), py::mul(&v_m.clone(), &v_nu.clone())?), (Py::str("beta"), py::mul(&py::sub(&Py::Int(1i64), &v_m.clone())?, &v_nu.clone())?), (Py::str("prior_sample_size"), v_nu.clone())]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_run_beta_hdi(mut v_alpha: Py, mut v_beta: Py, mut v_level: Py) -> R {
    let mut v__: Py = Py::None;
    let mut v_a: Py = Py::None;
    let mut v_b: Py = Py::None;
    let mut v_c: Py = Py::None;
    let mut v_g: Py = Py::None;
    let mut v_gm: Py = Py::None;
    let mut v_hi: Py = Py::None;
    let mut v_l: Py = Py::None;
    let mut v_lo: Py = Py::None;
    let mut v_mid: Py = Py::None;
    let mut v_u: Py = Py::None;
    let mut v_um: Py = Py::None;
    let t786 = (Py::tuple(vec![f__pos(v_alpha.clone(), Py::str("alpha"))?, f__pos(v_beta.clone(), Py::str("beta"))?, f__level(v_level.clone())?])).items()?;
    if t786.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_a = t786[0].clone();
    v_b = t786[1].clone();
    v_c = t786[2].clone();
    if ({ let t791 = { let t787 = v_a.clone(); Py::Bool({ let t788 = Py::Int(1i64); if py::le(&t787, &t788)? { true } else { false } }) }; if t791.truthy() { t791 } else { { let t789 = v_b.clone(); Py::Bool({ let t790 = Py::Int(1i64); if py::le(&t789, &t790)? { true } else { false } }) } } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the highest-density interval is computed for unimodal Beta with alpha > 1 and beta > 1"))));
    }
    v_g = { let v_a = v_a.clone(); let v_alpha = v_alpha.clone(); let v_b = v_b.clone(); let v_beta = v_beta.clone(); let v_c = v_c.clone(); let v_g = v_g.clone(); let v_level = v_level.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_l: Py = a.get(0).cloned().unwrap_or(Py::None);
            let mut v_u: Py = Py::None;
            v_u = f__beta_ppf(py::add(&py::getitem(&f__ibeta(v_a.clone(), v_b.clone(), v_l.clone(), Py::None)?, &Py::Int(0i64))?, &v_c.clone())?, v_a.clone(), v_b.clone())?;
            return Ok(Py::tuple(vec![py::sub(&(if ({ let t871 = v_u.clone(); Py::Bool({ let t872 = Py::Int(1i64); if py::lt(&t871, &t872)? { true } else { false } }) }).truthy() { f__beta_logpdf(v_u.clone(), v_a.clone(), v_b.clone())? } else { py::neg(&Py::Float(f64::INFINITY))? }), &f__beta_logpdf(v_l.clone(), v_a.clone(), v_b.clone())?)?, v_u.clone()]));
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) };
    let t873 = (Py::tuple(vec![Py::Float(0.0), f__beta_ppf(py::sub(&Py::Int(1i64), &v_c.clone())?, v_a.clone(), v_b.clone())?])).items()?;
    if t873.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_lo = t873[0].clone();
    v_hi = t873[1].clone();
    v_u = Py::Float(1.0);
    for t874 in py::range_iter(&[(Py::Int(200i64)).i()?]) {
        v__ = t874;
        v_mid = py::mul(&Py::Float(0.5), &py::add(&v_lo.clone(), &v_hi.clone())?)?;
        if ({ let t875 = v_mid.clone(); Py::Bool({ let t876 = Py::tuple(vec![v_lo.clone(), v_hi.clone()]); if py::contains(&t876, &t875)? { true } else { false } }) }).truthy() {
            break;
        }
        let t877 = (py::call(&v_g.clone(), &[v_mid.clone()])?).items()?;
        if t877.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_gm = t877[0].clone();
        v_um = t877[1].clone();
        if ({ let t878 = v_gm.clone(); Py::Bool({ let t879 = Py::Int(0i64); if py::gt(&t878, &t879)? { true } else { false } }) }).truthy() {
            v_lo = v_mid.clone();
        } else {
            let t880 = (Py::tuple(vec![v_mid.clone(), v_um.clone()])).items()?;
            if t880.len() != 2 { return Err(PyErr::Value("unpack".into())); }
            v_hi = t880[0].clone();
            v_u = t880[1].clone();
        }
    }
    v_l = py::mul(&Py::Float(0.5), &py::add(&v_lo.clone(), &v_hi.clone())?)?;
    v_u = f__beta_ppf(py::add(&py::getitem(&f__ibeta(v_a.clone(), v_b.clone(), v_l.clone(), Py::None)?, &Py::Int(0i64))?, &v_c.clone())?, v_a.clone(), v_b.clone())?;
    return Ok(Py::dict(vec![(Py::str("lower"), v_l.clone()), (Py::str("upper"), v_u.clone()), (Py::str("width"), py::sub(&v_u.clone(), &v_l.clone())?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_run_beta_tail(mut v_alpha: Py, mut v_beta: Py, mut v_threshold: Py, mut v_successes: Py, mut v_trials: Py) -> R {
    let mut v_a: Py = Py::None;
    let mut v_b: Py = Py::None;
    let mut v_c: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_lo_: Py = Py::None;
    let mut v_n: Py = Py::None;
    let mut v_up_: Py = Py::None;
    let t935 = (f__bb_check(v_alpha.clone(), v_beta.clone(), { let t933 = v_successes.clone(); if t933.truthy() { t933 } else { Py::Int(0i64) } }, { let t934 = v_trials.clone(); if t934.truthy() { t934 } else { Py::Int(0i64) } })?).items()?;
    if t935.len() != 4 { return Err(PyErr::Value("unpack".into())); }
    v_a = t935[0].clone();
    v_b = t935[1].clone();
    v_k = t935[2].clone();
    v_n = t935[3].clone();
    v_c = f__prob(v_threshold.clone(), Py::str("the threshold"), Py::Bool(true))?;
    let t966 = (f__ibeta(py::add(&v_a.clone(), &v_k.clone())?, py::sub(&py::add(&v_b.clone(), &v_n.clone())?, &v_k.clone())?, v_c.clone(), Py::None)?).items()?;
    if t966.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_lo_ = t966[0].clone();
    v_up_ = t966[1].clone();
    return Ok(Py::dict(vec![(Py::str("prob_above"), v_up_.clone()), (Py::str("prob_below"), v_lo_.clone())]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_run_bf_normal(mut v_sample_mean: Py, mut v_n: Py, mut v_sigma: Py, mut v_mu0: Py, mut v_prior_sd: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_l01: Py = Py::None;
    let mut v_m: Py = Py::None;
    let mut v_m0: Py = Py::None;
    let mut v_s: Py = Py::None;
    let mut v_t: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_xb: Py = Py::None;
    let t195 = (Py::tuple(vec![f__fin(v_sample_mean.clone(), Py::str("the sample mean"))?, f__whole(v_n.clone(), Py::str("the sample size"), Py::Int(1i64))?, f__pos(v_sigma.clone(), Py::str("sigma"))?, f__fin(v_mu0.clone(), Py::str("mu0"))?, f__pos(v_prior_sd.clone(), Py::str("the prior standard deviation"))?])).items()?;
    if t195.len() != 5 { return Err(PyErr::Value("unpack".into())); }
    v_xb = t195[0].clone();
    v_k = t195[1].clone();
    v_s = t195[2].clone();
    v_m0 = t195[3].clone();
    v_t = t195[4].clone();
    let t196 = (f__nn_post(v_m0.clone(), v_t.clone(), v_s.clone(), v_xb.clone(), v_k.clone())?).items()?;
    if t196.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_m = t196[0].clone();
    v_v = t196[1].clone();
    v_l01 = py::sub(&py::sub(&py::truediv(&py::mul(&py::neg(&Py::Float(0.5))?, &py::pow(&py::sub(&v_m0.clone(), &v_m.clone())?, &Py::Int(2i64))?)?, &v_v.clone())?, &py::mul(&Py::Float(0.5), &py::math1("log", &v_v.clone())?)?)?, &py::mul(&py::neg(&Py::Float(0.5))?, &py::math1("log", &py::mul(&v_t.clone(), &v_t.clone())?)?)?)?;
    return Ok(Py::dict(vec![(Py::str("bf10"), py::math1("exp", &py::neg(&v_l01.clone())?)?), (Py::str("bf01"), py::math1("exp", &v_l01.clone())?), (Py::str("log10_bf10"), py::truediv(&py::neg(&v_l01.clone())?, &py::math1("log", &Py::Int(10i64))?)?), (Py::str("posterior_h1"), py::truediv(&Py::Int(1i64), &py::add(&Py::Int(1i64), &py::math1("exp", &v_l01.clone())?)?)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_run_bf_point_null(mut v_successes: Py, mut v_trials: Py, mut v_p0: Py, mut v_alpha: Py, mut v_beta: Py) -> R {
    let mut v_a: Py = Py::None;
    let mut v_b: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_lbf: Py = Py::None;
    let mut v_n: Py = Py::None;
    let mut v_q: Py = Py::None;
    let t243 = (Py::tuple(vec![f__whole(v_trials.clone(), Py::str("the number of trials"), Py::Int(1i64))?, f__whole(v_successes.clone(), Py::str("the number of successes"), Py::Int(0i64))?])).items()?;
    if t243.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_n = t243[0].clone();
    v_k = t243[1].clone();
    if ({ let t244 = v_k.clone(); Py::Bool({ let t245 = v_n.clone(); if py::gt(&t244, &t245)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("successes cannot exceed trials"))));
    }
    v_q = f__prob(v_p0.clone(), Py::str("p0"), Py::Bool(true))?;
    let t259 = (Py::tuple(vec![f__pos((if ({ let t255 = v_alpha.clone(); Py::Bool({ let t256 = Py::None; if py::is(&t255, &t256) { true } else { false } }) }).truthy() { Py::Float(1.0) } else { v_alpha.clone() }), Py::str("alpha"))?, f__pos((if ({ let t257 = v_beta.clone(); Py::Bool({ let t258 = Py::None; if py::is(&t257, &t258) { true } else { false } }) }).truthy() { Py::Float(1.0) } else { v_beta.clone() }), Py::str("beta"))?])).items()?;
    if t259.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_a = t259[0].clone();
    v_b = t259[1].clone();
    v_lbf = py::sub(&py::sub(&py::sub(&f__lbeta(py::add(&v_a.clone(), &v_k.clone())?, py::sub(&py::add(&v_b.clone(), &v_n.clone())?, &v_k.clone())?)?, &f__lbeta(v_a.clone(), v_b.clone())?)?, &py::mul(&v_k.clone(), &py::math1("log", &v_q.clone())?)?)?, &py::mul(&py::sub(&v_n.clone(), &v_k.clone())?, &py::math1("log1p", &py::neg(&v_q.clone())?)?)?)?;
    return Ok(Py::dict(vec![(Py::str("bf10"), py::math1("exp", &v_lbf.clone())?), (Py::str("bf01"), py::math1("exp", &py::neg(&v_lbf.clone())?)?), (Py::str("log10_bf10"), py::truediv(&v_lbf.clone(), &py::math1("log", &Py::Int(10i64))?)?), (Py::str("posterior_h1"), py::truediv(&Py::Int(1i64), &py::add(&Py::Int(1i64), &py::math1("exp", &py::neg(&v_lbf.clone())?)?)?)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_run_bf_posterior(mut v_bayes_factor: Py, mut v_prior: Py) -> R {
    let mut v_b: Py = Py::None;
    let mut v_o: Py = Py::None;
    let mut v_p: Py = Py::None;
    v_b = f__fin(v_bayes_factor.clone(), Py::str("the Bayes factor"))?;
    if ({ let t3902 = v_b.clone(); Py::Bool({ let t3903 = Py::Int(0i64); if py::le(&t3902, &t3903)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("a Bayes factor must be positive"))));
    }
    v_p = f__prob((if ({ let t3911 = v_prior.clone(); Py::Bool({ let t3912 = Py::None; if py::is(&t3911, &t3912) { true } else { false } }) }).truthy() { Py::Float(0.5) } else { v_prior.clone() }), Py::str("the prior probability of H1"), Py::Bool(true))?;
    v_o = py::mul(&py::truediv(&v_p.clone(), &py::sub(&Py::Float(1.0), &v_p.clone())?)?, &v_b.clone())?;
    return Ok(Py::dict(vec![(Py::str("posterior"), py::truediv(&v_o.clone(), &py::add(&Py::Float(1.0), &v_o.clone())?)?), (Py::str("posterior_odds"), v_o.clone()), (Py::str("posterior_h0"), py::truediv(&Py::Float(1.0), &py::add(&Py::Float(1.0), &v_o.clone())?)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_run_bf_simple(mut v_successes: Py, mut v_trials: Py, mut v_p0: Py, mut v_p1: Py) -> R {
    let mut v_a: Py = Py::None;
    let mut v_b: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_lbf: Py = Py::None;
    let mut v_n: Py = Py::None;
    let t136 = (Py::tuple(vec![f__whole(v_trials.clone(), Py::str("the number of trials"), Py::Int(1i64))?, f__whole(v_successes.clone(), Py::str("the number of successes"), Py::Int(0i64))?])).items()?;
    if t136.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_n = t136[0].clone();
    v_k = t136[1].clone();
    if ({ let t137 = v_k.clone(); Py::Bool({ let t138 = v_n.clone(); if py::gt(&t137, &t138)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("successes cannot exceed trials"))));
    }
    let t146 = (Py::tuple(vec![f__prob(v_p0.clone(), Py::str("p0"), Py::Bool(true))?, f__prob(v_p1.clone(), Py::str("p1"), Py::Bool(true))?])).items()?;
    if t146.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_a = t146[0].clone();
    v_b = t146[1].clone();
    v_lbf = py::add(&py::mul(&v_k.clone(), &py::sub(&py::math1("log", &v_b.clone())?, &py::math1("log", &v_a.clone())?)?)?, &py::mul(&py::sub(&v_n.clone(), &v_k.clone())?, &py::sub(&py::math1("log1p", &py::neg(&v_b.clone())?)?, &py::math1("log1p", &py::neg(&v_a.clone())?)?)?)?)?;
    return Ok(Py::dict(vec![(Py::str("bf10"), py::math1("exp", &v_lbf.clone())?), (Py::str("log10_bf10"), py::truediv(&v_lbf.clone(), &py::math1("log", &Py::Int(10i64))?)?), (Py::str("posterior_h1"), py::truediv(&Py::Int(1i64), &py::add(&Py::Int(1i64), &py::math1("exp", &py::neg(&v_lbf.clone())?)?)?)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_run_bilinear(mut v_a1: Py, mut v_a2: Py, mut v_a12: Py, mut v_mean1: Py, mut v_sd1: Py, mut v_mean2: Py, mut v_sd2: Py) -> R {
    let mut v_V: Py = Py::None;
    let mut v_m1: Py = Py::None;
    let mut v_m2: Py = Py::None;
    let mut v_s1: Py = Py::None;
    let mut v_s2: Py = Py::None;
    let mut v_v1: Py = Py::None;
    let mut v_v12: Py = Py::None;
    let mut v_v2: Py = Py::None;
    let t4423 = ({ let mut out = vec![]; for t4419 in py::iter(&(Py::tuple(vec![Py::tuple(vec![v_a1.clone(), Py::str("a1")]), Py::tuple(vec![v_a2.clone(), Py::str("a2")]), Py::tuple(vec![v_a12.clone(), Py::str("a12")]), Py::tuple(vec![v_mean1.clone(), Py::str("mean1")]), Py::tuple(vec![v_mean2.clone(), Py::str("mean2")])])))? { let mut v_n: Py; let mut v_v: Py; 
let t4420 = (t4419).items()?;
if t4420.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_v = t4420[0].clone();
v_n = t4420[1].clone();
out.push(f__fin(v_v.clone(), v_n.clone())?); } Py::list(out) }).items()?;
    if t4423.len() != 5 { return Err(PyErr::Value("unpack".into())); }
    v_a1 = t4423[0].clone();
    v_a2 = t4423[1].clone();
    v_a12 = t4423[2].clone();
    v_m1 = t4423[3].clone();
    v_m2 = t4423[4].clone();
    let t4426 = (Py::tuple(vec![f__pos(v_sd1.clone(), Py::str("sd1"))?, f__pos(v_sd2.clone(), Py::str("sd2"))?])).items()?;
    if t4426.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_s1 = t4426[0].clone();
    v_s2 = t4426[1].clone();
    v_v1 = py::pow(&py::mul(&py::add(&v_a1.clone(), &py::mul(&v_a12.clone(), &v_m2.clone())?)?, &v_s1.clone())?, &Py::Int(2i64))?;
    v_v2 = py::pow(&py::mul(&py::add(&v_a2.clone(), &py::mul(&v_a12.clone(), &v_m1.clone())?)?, &v_s2.clone())?, &Py::Int(2i64))?;
    v_v12 = py::pow(&py::mul(&py::mul(&v_a12.clone(), &v_s1.clone())?, &v_s2.clone())?, &Py::Int(2i64))?;
    v_V = py::add(&py::add(&v_v1.clone(), &v_v2.clone())?, &v_v12.clone())?;
    if ({ let t4427 = v_V.clone(); Py::Bool({ let t4428 = Py::Int(0i64); if py::eq(&t4427, &t4428) { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the model has zero variance"))));
    }
    return Ok(Py::dict(vec![(Py::str("S1"), py::truediv(&v_v1.clone(), &v_V.clone())?), (Py::str("S2"), py::truediv(&v_v2.clone(), &v_V.clone())?), (Py::str("S12"), py::truediv(&v_v12.clone(), &v_V.clone())?), (Py::str("ST1"), py::truediv(&py::add(&v_v1.clone(), &v_v12.clone())?, &v_V.clone())?), (Py::str("ST2"), py::truediv(&py::add(&v_v2.clone(), &v_v12.clone())?, &v_V.clone())?), (Py::str("variance"), v_V.clone())]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_run_break_even(mut v_payoffs: Py) -> R {
    let mut v_a: Py = Py::None;
    let mut v_b: Py = Py::None;
    let mut v_c: Py = Py::None;
    let mut v_d: Py = Py::None;
    let mut v_den: Py = Py::None;
    let mut v_m: Py = Py::None;
    let mut v_p: Py = Py::None;
    v_m = f__mat(v_payoffs.clone(), Py::str("the payoff matrix"), Py::Int(2i64), Py::Int(2i64))?;
    let t1080 = (v_m.clone()).items()?;
    if t1080.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    let t1081 = (t1080[0].clone()).items()?;
    if t1081.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_a = t1081[0].clone();
    v_b = t1081[1].clone();
    let t1082 = (t1080[1].clone()).items()?;
    if t1082.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_c = t1082[0].clone();
    v_d = t1082[1].clone();
    v_den = py::sub(&py::sub(&v_a.clone(), &v_b.clone())?, &py::sub(&v_c.clone(), &v_d.clone())?)?;
    if ({ let t1083 = v_den.clone(); Py::Bool({ let t1084 = Py::Int(0i64); if py::eq(&t1083, &t1084) { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the two actions' expected values never cross (parallel lines)"))));
    }
    v_p = py::truediv(&py::sub(&v_d.clone(), &v_b.clone())?, &v_den.clone())?;
    if (py::not(&{ let t1085 = Py::Int(0i64); Py::Bool({ let t1086 = v_p.clone(); if py::lt(&t1085, &t1086)? { { let t1087 = Py::Int(1i64); if py::lt(&t1086, &t1087)? { true } else { false } } } else { false } }) })).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("one action is at least as good for every probability: there is no break-even point in (0, 1)"))));
    }
    return Ok(Py::dict(vec![(Py::str("break_even"), v_p.clone()), (Py::str("preferred_above"), (if ({ let t1088 = py::sub(&v_a.clone(), &v_c.clone())?; Py::Bool({ let t1089 = Py::Int(0i64); if py::gt(&t1088, &t1089)? { true } else { false } }) }).truthy() { Py::Int(1i64) } else { Py::Int(2i64) })), (Py::str("expected_value_at_break_even"), py::add(&py::mul(&v_p.clone(), &v_a.clone())?, &py::mul(&py::sub(&Py::Int(1i64), &v_p.clone())?, &v_b.clone())?)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_run_cara(mut v_outcomes: Py, mut v_probabilities: Py, mut v_risk_tolerance: Py) -> R {
    let mut v_ce: Py = Py::None;
    let mut v_lse: Py = Py::None;
    let mut v_mean: Py = Py::None;
    let mut v_p: Py = Py::None;
    let mut v_r: Py = Py::None;
    let mut v_x: Py = Py::None;
    v_x = f__vec(v_outcomes.clone(), Py::str("the outcomes"), Py::None, Py::Int(2i64))?;
    v_p = f__simplex(v_probabilities.clone(), Py::str("the probabilities"), py::len(&v_x.clone())?)?;
    v_r = f__pos(v_risk_tolerance.clone(), Py::str("the risk tolerance"))?;
    v_lse = f__logsumexp({ let mut out = vec![]; for t1155 in py::iter(&(v_x.clone()))? { let mut v_v: Py; 
v_v = t1155;
out.push(py::truediv(&py::neg(&v_v.clone())?, &v_r.clone())?); } Py::list(out) }, v_p.clone())?;
    v_ce = py::mul(&py::neg(&v_r.clone())?, &v_lse.clone())?;
    v_mean = py::fsum(&{ let mut out = vec![]; for t1156 in py::iter(&(py::zip(&[v_x.clone(), v_p.clone()])?))? { let mut v_a: Py; let mut v_b: Py; 
let t1157 = (t1156).items()?;
if t1157.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_a = t1157[0].clone();
v_b = t1157[1].clone();
out.push(py::mul(&v_a.clone(), &v_b.clone())?); } Py::list(out) })?;
    return Ok(Py::dict(vec![(Py::str("expected_utility"), py::neg(&py::math1("expm1", &v_lse.clone())?)?), (Py::str("certainty_equivalent"), v_ce.clone()), (Py::str("risk_premium"), py::sub(&v_mean.clone(), &v_ce.clone())?), (Py::str("expected_value"), v_mean.clone())]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_run_cov_prop(mut v_matrix: Py, mut v_covariance: Py) -> R {
    let mut v_A: Py = Py::None;
    let mut v_AS: Py = Py::None;
    let mut v_C: Py = Py::None;
    let mut v_S: Py = Py::None;
    let mut v_i: Py = Py::None;
    let mut v_j: Py = Py::None;
    v_A = f__mat(v_matrix.clone(), Py::str("the sensitivity matrix"), Py::None, Py::None)?;
    v_S = f__mat(v_covariance.clone(), Py::str("the covariance matrix"), py::len(&py::getitem(&v_A.clone(), &Py::Int(0i64))?)?, py::len(&py::getitem(&v_A.clone(), &Py::Int(0i64))?)?)?;
    let _ = f__cholesky_ok(v_S.clone(), Py::str("the covariance matrix"))?;
    v_AS = { let mut out = vec![]; for t1357 in py::range_iter(&[(py::len(&v_A.clone())?).i()?]) { let mut v_i: Py; 
v_i = t1357;
out.push({ let mut out = vec![]; for t1358 in py::range_iter(&[(py::len(&v_S.clone())?).i()?]) { let mut v_j: Py; 
v_j = t1358;
out.push(py::fsum(&{ let mut out = vec![]; for t1359 in py::range_iter(&[(py::len(&v_S.clone())?).i()?]) { let mut v_k: Py; 
v_k = t1359;
out.push(py::mul(&py::getitem(&py::getitem(&v_A.clone(), &v_i.clone())?, &v_k.clone())?, &py::getitem(&py::getitem(&v_S.clone(), &v_k.clone())?, &v_j.clone())?)?); } Py::list(out) })?); } Py::list(out) }); } Py::list(out) };
    v_C = { let mut out = vec![]; for t1360 in py::range_iter(&[(py::len(&v_A.clone())?).i()?]) { let mut v_i: Py; 
v_i = t1360;
out.push({ let mut out = vec![]; for t1361 in py::range_iter(&[(py::len(&v_A.clone())?).i()?]) { let mut v_j: Py; 
v_j = t1361;
out.push(py::fsum(&{ let mut out = vec![]; for t1362 in py::range_iter(&[(py::len(&v_S.clone())?).i()?]) { let mut v_k: Py; 
v_k = t1362;
out.push(py::mul(&py::getitem(&py::getitem(&v_AS.clone(), &v_i.clone())?, &v_k.clone())?, &py::getitem(&py::getitem(&v_A.clone(), &v_j.clone())?, &v_k.clone())?)?); } Py::list(out) })?); } Py::list(out) }); } Py::list(out) };
    for t1363 in py::range_iter(&[(py::len(&v_A.clone())?).i()?]) {
        v_i = t1363;
        for t1364 in py::range_iter(&[(v_i.clone()).i()?]) {
            v_j = t1364;
            let t1365 = py::mul(&Py::Float(0.5), &py::add(&py::getitem(&py::getitem(&v_C.clone(), &v_i.clone())?, &v_j.clone())?, &py::getitem(&py::getitem(&v_C.clone(), &v_j.clone())?, &v_i.clone())?)?)?;
            py::setitem(&py::getitem(&v_C.clone(), &v_i.clone())?, &v_j.clone(), t1365.clone())?;
            py::setitem(&py::getitem(&v_C.clone(), &v_j.clone())?, &v_i.clone(), t1365.clone())?;
        }
    }
    return Ok(Py::dict(vec![(Py::str("output_covariance"), v_C.clone()), (Py::str("uncertainties"), { let mut out = vec![]; for t1366 in py::range_iter(&[(py::len(&v_A.clone())?).i()?]) { let mut v_i: Py; 
v_i = t1366;
out.push(py::math1("sqrt", &py::minmax(&[Py::Float(0.0), py::getitem(&py::getitem(&v_C.clone(), &v_i.clone())?, &v_i.clone())?], true)?)?); } Py::list(out) })]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_run_criteria(mut v_payoffs: Py, mut v_criterion: Py, mut v_optimism: Py) -> R {
    let mut v_a: Py = Py::None;
    let mut v_b: Py = Py::None;
    let mut v_col: Py = Py::None;
    let mut v_m: Py = Py::None;
    let mut v_s: Py = Py::None;
    let mut v_sign: Py = Py::None;
    v_m = f__mat(v_payoffs.clone(), Py::str("the payoff matrix"), Py::None, Py::None)?;
    if ({ let t1427 = v_criterion.clone(); Py::Bool({ let t1428 = c__CRIT()?; if !py::contains(&t1428, &t1427)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&py::add(&Py::str("criterion must be one of "), &py::method(&Py::str(", "), "join", &[c__CRIT()?])?)?)));
    }
    if ({ let t1429 = v_criterion.clone(); Py::Bool({ let t1430 = Py::str("maximin"); if py::eq(&t1429, &t1430) { true } else { false } }) }).truthy() {
        let t1432 = (Py::tuple(vec![{ let mut out = vec![]; for t1431 in py::iter(&(v_m.clone()))? { let mut v_r: Py; 
v_r = t1431;
out.push(py::minmax(&[v_r.clone()], false)?); } Py::list(out) }, Py::Int(1i64)])).items()?;
        if t1432.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_s = t1432[0].clone();
        v_sign = t1432[1].clone();
    } else {
        if ({ let t1433 = v_criterion.clone(); Py::Bool({ let t1434 = Py::str("maximax"); if py::eq(&t1433, &t1434) { true } else { false } }) }).truthy() {
            let t1436 = (Py::tuple(vec![{ let mut out = vec![]; for t1435 in py::iter(&(v_m.clone()))? { let mut v_r: Py; 
v_r = t1435;
out.push(py::minmax(&[v_r.clone()], true)?); } Py::list(out) }, Py::Int(1i64)])).items()?;
            if t1436.len() != 2 { return Err(PyErr::Value("unpack".into())); }
            v_s = t1436[0].clone();
            v_sign = t1436[1].clone();
        } else {
            if ({ let t1437 = v_criterion.clone(); Py::Bool({ let t1438 = Py::str("laplace"); if py::eq(&t1437, &t1438) { true } else { false } }) }).truthy() {
                let t1440 = (Py::tuple(vec![{ let mut out = vec![]; for t1439 in py::iter(&(v_m.clone()))? { let mut v_r: Py; 
v_r = t1439;
out.push(py::truediv(&py::fsum(&v_r.clone())?, &py::len(&v_r.clone())?)?); } Py::list(out) }, Py::Int(1i64)])).items()?;
                if t1440.len() != 2 { return Err(PyErr::Value("unpack".into())); }
                v_s = t1440[0].clone();
                v_sign = t1440[1].clone();
            } else {
                if ({ let t1441 = v_criterion.clone(); Py::Bool({ let t1442 = Py::str("hurwicz"); if py::eq(&t1441, &t1442) { true } else { false } }) }).truthy() {
                    if ({ let t1443 = v_optimism.clone(); Py::Bool({ let t1444 = Py::None; if py::is(&t1443, &t1444) { true } else { false } }) }).truthy() {
                        return Err(py::value_error(py::to_str(&Py::str("the Hurwicz criterion needs the optimism coefficient alpha (0 to 1)"))));
                    }
                    v_a = f__prob(v_optimism.clone(), Py::str("the optimism coefficient"), Py::Bool(false))?;
                    let t1453 = (Py::tuple(vec![{ let mut out = vec![]; for t1452 in py::iter(&(v_m.clone()))? { let mut v_r: Py; 
v_r = t1452;
out.push(py::add(&py::mul(&v_a.clone(), &py::minmax(&[v_r.clone()], true)?)?, &py::mul(&py::sub(&Py::Int(1i64), &v_a.clone())?, &py::minmax(&[v_r.clone()], false)?)?)?); } Py::list(out) }, Py::Int(1i64)])).items()?;
                    if t1453.len() != 2 { return Err(PyErr::Value("unpack".into())); }
                    v_s = t1453[0].clone();
                    v_sign = t1453[1].clone();
                } else {
                    v_col = { let mut out = vec![]; for t1454 in py::range_iter(&[(py::len(&py::getitem(&v_m.clone(), &Py::Int(0i64))?)?).i()?]) { let mut v_j: Py; 
v_j = t1454;
out.push(py::minmax(&[{ let mut out = vec![]; for t1455 in py::iter(&(v_m.clone()))? { let mut v_r: Py; 
v_r = t1455;
out.push(py::getitem(&v_r.clone(), &v_j.clone())?); } Py::list(out) }], true)?); } Py::list(out) };
                    let t1458 = (Py::tuple(vec![{ let mut out = vec![]; for t1456 in py::iter(&(v_m.clone()))? { let mut v_r: Py; 
v_r = t1456;
out.push(py::minmax(&[{ let mut out = vec![]; for t1457 in py::range_iter(&[(py::len(&v_r.clone())?).i()?]) { let mut v_j: Py; 
v_j = t1457;
out.push(py::sub(&py::getitem(&v_col.clone(), &v_j.clone())?, &py::getitem(&v_r.clone(), &v_j.clone())?)?); } Py::list(out) }], true)?); } Py::list(out) }, py::neg(&Py::Int(1i64))?])).items()?;
                    if t1458.len() != 2 { return Err(PyErr::Value("unpack".into())); }
                    v_s = t1458[0].clone();
                    v_sign = t1458[1].clone();
                }
            }
        }
    }
    v_b = f__argbest(v_s.clone(), v_sign.clone())?;
    return Ok(Py::dict(vec![(Py::str("scores"), v_s.clone()), (Py::str("best_action"), py::add(&v_b.clone(), &Py::Int(1i64))?), (Py::str("criterion_value"), py::getitem(&v_s.clone(), &v_b.clone())?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_run_cv(mut v_y: Py, mut v_c: Py, mut v_control_mean: Py) -> R {
    let mut v_b: Py = Py::None;
    let mut v_cc: Py = Py::None;
    let mut v_mc: Py = Py::None;
    let mut v_mu: Py = Py::None;
    let mut v_my: Py = Py::None;
    let mut v_scc: Py = Py::None;
    let mut v_syc: Py = Py::None;
    let mut v_syy: Py = Py::None;
    let mut v_yy: Py = Py::None;
    v_yy = f__vec(v_y.clone(), Py::str("the outputs"), Py::None, Py::Int(3i64))?;
    v_cc = f__vec(v_c.clone(), Py::str("the control values"), py::len(&v_yy.clone())?, Py::Int(1i64))?;
    v_mu = f__fin(v_control_mean.clone(), Py::str("the known control mean"))?;
    let t1268 = (Py::tuple(vec![f__mean(v_yy.clone())?, f__mean(v_cc.clone())?])).items()?;
    if t1268.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_my = t1268[0].clone();
    v_mc = t1268[1].clone();
    v_scc = py::fsum(&{ let mut out = vec![]; for t1269 in py::iter(&(v_cc.clone()))? { let mut v_v: Py; 
v_v = t1269;
out.push(py::pow(&py::sub(&v_v.clone(), &v_mc.clone())?, &Py::Int(2i64))?); } Py::list(out) })?;
    v_syy = py::fsum(&{ let mut out = vec![]; for t1270 in py::iter(&(v_yy.clone()))? { let mut v_v: Py; 
v_v = t1270;
out.push(py::pow(&py::sub(&v_v.clone(), &v_my.clone())?, &Py::Int(2i64))?); } Py::list(out) })?;
    if ({ let t1275 = { let t1271 = v_scc.clone(); Py::Bool({ let t1272 = Py::Int(0i64); if py::eq(&t1271, &t1272) { true } else { false } }) }; if t1275.truthy() { t1275 } else { { let t1273 = v_syy.clone(); Py::Bool({ let t1274 = Py::Int(0i64); if py::eq(&t1273, &t1274) { true } else { false } }) } } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the outputs and the control must both vary"))));
    }
    v_syc = py::fsum(&{ let mut out = vec![]; for t1276 in py::iter(&(py::zip(&[v_yy.clone(), v_cc.clone()])?))? { let mut v_a: Py; let mut v_b: Py; 
let t1277 = (t1276).items()?;
if t1277.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_a = t1277[0].clone();
v_b = t1277[1].clone();
out.push(py::mul(&py::sub(&v_a.clone(), &v_my.clone())?, &py::sub(&v_b.clone(), &v_mc.clone())?)?); } Py::list(out) })?;
    v_b = py::truediv(&v_syc.clone(), &v_scc.clone())?;
    return Ok(Py::dict(vec![(Py::str("estimate"), py::sub(&v_my.clone(), &py::mul(&v_b.clone(), &py::sub(&v_mc.clone(), &v_mu.clone())?)?)?), (Py::str("coefficient"), v_b.clone()), (Py::str("plain_estimate"), v_my.clone()), (Py::str("variance_reduction"), py::sub(&Py::Int(1i64), &py::truediv(&py::mul(&v_syc.clone(), &v_syc.clone())?, &py::mul(&v_scc.clone(), &v_syy.clone())?)?)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_run_diagnostic(mut v_prevalence: Py, mut v_sensitivity: Py, mut v_specificity: Py) -> R {
    let mut v_neg: Py = Py::None;
    let mut v_npv: Py = Py::None;
    let mut v_pos: Py = Py::None;
    let mut v_ppv: Py = Py::None;
    let mut v_pv: Py = Py::None;
    let mut v_se: Py = Py::None;
    let mut v_sp: Py = Py::None;
    let t1513 = (f__dx(v_prevalence.clone(), v_sensitivity.clone(), v_specificity.clone())?).items()?;
    if t1513.len() != 5 { return Err(PyErr::Value("unpack".into())); }
    v_pv = t1513[0].clone();
    v_se = t1513[1].clone();
    v_sp = t1513[2].clone();
    v_pos = t1513[3].clone();
    v_neg = t1513[4].clone();
    v_ppv = py::truediv(&py::mul(&v_pv.clone(), &v_se.clone())?, &v_pos.clone())?;
    v_npv = py::truediv(&py::mul(&py::sub(&Py::Float(1.0), &v_pv.clone())?, &v_sp.clone())?, &v_neg.clone())?;
    return Ok(Py::dict(vec![(Py::str("ppv"), v_ppv.clone()), (Py::str("npv"), v_npv.clone()), (Py::str("p_positive"), v_pos.clone()), (Py::str("false_discovery_rate"), py::sub(&Py::Float(1.0), &v_ppv.clone())?), (Py::str("false_omission_rate"), py::sub(&Py::Float(1.0), &v_npv.clone())?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_run_discrete_posterior(mut v_priors: Py, mut v_likelihoods: Py) -> R {
    let mut v_best: Py = Py::None;
    let mut v_ev: Py = Py::None;
    let mut v_joint: Py = Py::None;
    let mut v_lik: Py = Py::None;
    let mut v_p: Py = Py::None;
    let mut v_post: Py = Py::None;
    v_p = f__simplex(v_priors.clone(), Py::str("the prior probabilities"), Py::None)?;
    v_lik = f__vec(v_likelihoods.clone(), Py::str("the likelihoods"), py::len(&v_p.clone())?, Py::Int(1i64))?;
    if (Py::Bool(({ let mut out = vec![]; for t1694 in py::iter(&(v_lik.clone()))? { let mut v_v: Py; 
v_v = t1694;
out.push({ let t1695 = v_v.clone(); Py::Bool({ let t1696 = Py::Int(0i64); if py::lt(&t1695, &t1696)? { true } else { false } }) }); } Py::list(out) }).items()?.iter().any(|x| x.truthy()))).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the likelihoods must not be negative"))));
    }
    v_joint = { let mut out = vec![]; for t1697 in py::iter(&(py::zip(&[v_p.clone(), v_lik.clone()])?))? { let mut v_a: Py; let mut v_b: Py; 
let t1698 = (t1697).items()?;
if t1698.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_a = t1698[0].clone();
v_b = t1698[1].clone();
out.push(py::mul(&v_a.clone(), &v_b.clone())?); } Py::list(out) };
    v_ev = py::fsum(&v_joint.clone())?;
    if ({ let t1699 = v_ev.clone(); Py::Bool({ let t1700 = Py::Int(0i64); if py::le(&t1699, &t1700)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the evidence is 0: no hypothesis gives the observation a positive probability"))));
    }
    v_post = { let mut out = vec![]; for t1701 in py::iter(&(v_joint.clone()))? { let mut v_j: Py; 
v_j = t1701;
out.push(py::truediv(&v_j.clone(), &v_ev.clone())?); } Py::list(out) };
    v_best = py::minmax_key(&[Py::list((py::range_iter(&[(py::len(&v_post.clone())?).i()?])).into_iter().collect::<Vec<Py>>())], &{ let v_ev = v_ev.clone(); let v_joint = v_joint.clone(); let v_lik = v_lik.clone(); let v_likelihoods = v_likelihoods.clone(); let v_p = v_p.clone(); let v_post = v_post.clone(); let v_priors = v_priors.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_i: Py = a.get(0).cloned().unwrap_or(Py::None);
            return Ok(py::getitem(&v_post.clone(), &v_i.clone())?);
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) }, true)?;
    return Ok(Py::dict(vec![(Py::str("posterior"), v_post.clone()), (Py::str("evidence"), v_ev.clone()), (Py::str("map_hypothesis"), py::add(&v_best.clone(), &Py::Int(1i64))?), (Py::str("map_probability"), py::getitem(&v_post.clone(), &v_best.clone())?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_run_dm_evidence(mut v_alpha: Py, mut v_counts: Py) -> R {
    let mut v_a: Py = Py::None;
    let mut v_a0: Py = Py::None;
    let mut v_lp: Py = Py::None;
    let mut v_n: Py = Py::None;
    let mut v_x: Py = Py::None;
    let t1572 = (f__dm_check(v_alpha.clone(), v_counts.clone())?).items()?;
    if t1572.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_a = t1572[0].clone();
    v_x = t1572[1].clone();
    let t1573 = (Py::tuple(vec![py::fsum(&v_x.clone())?, py::fsum(&v_a.clone())?])).items()?;
    if t1573.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_n = t1573[0].clone();
    v_a0 = t1573[1].clone();
    v_lp = py::add(&py::sub(&py::add(&py::sub(&py::math1("lgamma", &py::add(&v_n.clone(), &Py::Int(1i64))?)?, &py::fsum(&{ let mut out = vec![]; for t1574 in py::iter(&(v_x.clone()))? { let mut v_v: Py; 
v_v = t1574;
out.push(py::math1("lgamma", &py::add(&v_v.clone(), &Py::Int(1i64))?)?); } Py::list(out) })?)?, &py::math1("lgamma", &v_a0.clone())?)?, &py::math1("lgamma", &py::add(&v_a0.clone(), &v_n.clone())?)?)?, &py::fsum(&{ let mut out = vec![]; for t1575 in py::iter(&(py::zip(&[v_a.clone(), v_x.clone()])?))? { let mut v_u: Py; let mut v_v: Py; 
let t1576 = (t1575).items()?;
if t1576.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_u = t1576[0].clone();
v_v = t1576[1].clone();
out.push(py::sub(&py::math1("lgamma", &py::add(&v_u.clone(), &v_v.clone())?)?, &py::math1("lgamma", &v_u.clone())?)?); } Py::list(out) })?)?;
    return Ok(Py::dict(vec![(Py::str("probability"), py::math1("exp", &v_lp.clone())?), (Py::str("log_probability"), v_lp.clone())]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_run_dm_post(mut v_alpha: Py, mut v_counts: Py) -> R {
    let mut v_a: Py = Py::None;
    let mut v_post: Py = Py::None;
    let mut v_s: Py = Py::None;
    let mut v_x: Py = Py::None;
    let t1635 = (f__dm_check(v_alpha.clone(), v_counts.clone())?).items()?;
    if t1635.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_a = t1635[0].clone();
    v_x = t1635[1].clone();
    v_post = { let mut out = vec![]; for t1636 in py::iter(&(py::zip(&[v_a.clone(), v_x.clone()])?))? { let mut v_u: Py; let mut v_v: Py; 
let t1637 = (t1636).items()?;
if t1637.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_u = t1637[0].clone();
v_v = t1637[1].clone();
out.push(py::add(&v_u.clone(), &v_v.clone())?); } Py::list(out) };
    v_s = py::fsum(&v_post.clone())?;
    return Ok(Py::dict(vec![(Py::str("post_alpha"), v_post.clone()), (Py::str("posterior_means"), { let mut out = vec![]; for t1638 in py::iter(&(v_post.clone()))? { let mut v_p: Py; 
v_p = t1638;
out.push(py::truediv(&v_p.clone(), &v_s.clone())?); } Py::list(out) }), (Py::str("posterior_variances"), { let mut out = vec![]; for t1639 in py::iter(&(v_post.clone()))? { let mut v_p: Py; 
v_p = t1639;
out.push(py::truediv(&py::mul(&v_p.clone(), &py::sub(&v_s.clone(), &v_p.clone())?)?, &py::mul(&py::mul(&v_s.clone(), &v_s.clone())?, &py::add(&v_s.clone(), &Py::Int(1i64))?)?)?); } Py::list(out) })]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_run_ev_decision(mut v_payoffs: Py, mut v_probabilities: Py) -> R {
    let mut v_b: Py = Py::None;
    let mut v_ev: Py = Py::None;
    let mut v_m: Py = Py::None;
    let mut v_p: Py = Py::None;
    let t1775 = (f__payoff_check(v_payoffs.clone(), v_probabilities.clone(), Py::str("the payoff matrix"))?).items()?;
    if t1775.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_m = t1775[0].clone();
    v_p = t1775[1].clone();
    v_ev = f__ev(v_m.clone(), v_p.clone())?;
    v_b = f__argbest(v_ev.clone(), Py::Int(1i64))?;
    return Ok(Py::dict(vec![(Py::str("expected_values"), v_ev.clone()), (Py::str("best_action"), py::add(&v_b.clone(), &Py::Int(1i64))?), (Py::str("best_expected_value"), py::getitem(&v_ev.clone(), &v_b.clone())?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_run_evpi(mut v_payoffs: Py, mut v_probabilities: Py) -> R {
    let mut v_b: Py = Py::None;
    let mut v_ev: Py = Py::None;
    let mut v_evpi: Py = Py::None;
    let mut v_m: Py = Py::None;
    let mut v_p: Py = Py::None;
    let mut v_wpi: Py = Py::None;
    let t1852 = (f__payoff_check(v_payoffs.clone(), v_probabilities.clone(), Py::str("the payoff matrix"))?).items()?;
    if t1852.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_m = t1852[0].clone();
    v_p = t1852[1].clone();
    v_ev = f__ev(v_m.clone(), v_p.clone())?;
    v_b = f__argbest(v_ev.clone(), Py::Int(1i64))?;
    v_wpi = py::fsum(&{ let mut out = vec![]; for t1856 in py::range_iter(&[(py::len(&v_p.clone())?).i()?]) { let mut v_j: Py; 
v_j = t1856;
out.push(py::mul(&py::minmax(&[{ let mut out = vec![]; for t1857 in py::iter(&(v_m.clone()))? { let mut v_r: Py; 
v_r = t1857;
out.push(py::getitem(&v_r.clone(), &v_j.clone())?); } Py::list(out) }], true)?, &py::getitem(&v_p.clone(), &v_j.clone())?)?); } Py::list(out) })?;
    v_evpi = py::fsum(&{ let mut out = vec![]; for t1858 in py::range_iter(&[(py::len(&v_p.clone())?).i()?]) { let mut v_j: Py; 
v_j = t1858;
out.push(py::mul(&py::getitem(&v_p.clone(), &v_j.clone())?, &py::sub(&py::minmax(&[{ let mut out = vec![]; for t1859 in py::iter(&(v_m.clone()))? { let mut v_r: Py; 
v_r = t1859;
out.push(py::getitem(&v_r.clone(), &v_j.clone())?); } Py::list(out) }], true)?, &py::getitem(&py::getitem(&v_m.clone(), &v_b.clone())?, &v_j.clone())?)?)?); } Py::list(out) })?;
    return Ok(Py::dict(vec![(Py::str("evpi"), v_evpi.clone()), (Py::str("ev_with_perfect_information"), v_wpi.clone()), (Py::str("best_expected_value"), py::getitem(&v_ev.clone(), &v_b.clone())?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_run_evsi(mut v_payoffs: Py, mut v_priors: Py, mut v_likelihoods: Py) -> R {
    let mut v_L: Py = Py::None;
    let mut v_a0: Py = Py::None;
    let mut v_acts: Py = Py::None;
    let mut v_b: Py = Py::None;
    let mut v_col: Py = Py::None;
    let mut v_ev: Py = Py::None;
    let mut v_ev0: Py = Py::None;
    let mut v_gain: Py = Py::None;
    let mut v_j: Py = Py::None;
    let mut v_m: Py = Py::None;
    let mut v_p: Py = Py::None;
    let mut v_post: Py = Py::None;
    let mut v_probs: Py = Py::None;
    let mut v_ps: Py = Py::None;
    let mut v_row: Py = Py::None;
    let mut v_total: Py = Py::None;
    let mut v_with_: Py = Py::None;
    let mut v_without: Py = Py::None;
    let t1933 = (f__payoff_check(v_payoffs.clone(), v_priors.clone(), Py::str("the payoff matrix"))?).items()?;
    if t1933.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_m = t1933[0].clone();
    v_p = t1933[1].clone();
    v_L = f__mat(v_likelihoods.clone(), Py::str("the likelihood matrix"), Py::None, py::len(&v_p.clone())?)?;
    for t1934 in py::range_iter(&[(py::len(&v_p.clone())?).i()?]) {
        v_j = t1934;
        v_col = { let mut out = vec![]; for t1935 in py::iter(&(v_L.clone()))? { let mut v_r: Py; 
v_r = t1935;
out.push(py::getitem(&v_r.clone(), &v_j.clone())?); } Py::list(out) };
        if ({ let t1941 = Py::Bool(({ let mut out = vec![]; for t1936 in py::iter(&(v_col.clone()))? { let mut v_v: Py; 
v_v = t1936;
out.push({ let t1937 = v_v.clone(); Py::Bool({ let t1938 = Py::Int(0i64); if py::lt(&t1937, &t1938)? { true } else { false } }) }); } Py::list(out) }).items()?.iter().any(|x| x.truthy())); if t1941.truthy() { t1941 } else { { let t1939 = py::abs(&py::sub(&py::fsum(&v_col.clone())?, &Py::Int(1i64))?)?; Py::Bool({ let t1940 = Py::Float(1e-09); if py::gt(&t1939, &t1940)? { true } else { false } }) } } }).truthy() {
            return Err(py::value_error(py::to_str(&Py::str("each column of the likelihood matrix (P(signal | state)) must sum to 1"))));
        }
    }
    v_ev0 = f__ev(v_m.clone(), v_p.clone())?;
    v_a0 = f__argbest(v_ev0.clone(), Py::Int(1i64))?;
    v_without = py::getitem(&v_ev0.clone(), &v_a0.clone())?;
    let t1945 = (Py::tuple(vec![Py::list(vec![]), Py::list(vec![]), Py::list(vec![]), Py::list(vec![])])).items()?;
    if t1945.len() != 4 { return Err(PyErr::Value("unpack".into())); }
    v_total = t1945[0].clone();
    v_acts = t1945[1].clone();
    v_probs = t1945[2].clone();
    v_gain = t1945[3].clone();
    for t1946 in py::iter(&(v_L.clone()))? {
        v_row = t1946;
        v_ps = py::fsum(&{ let mut out = vec![]; for t1947 in py::range_iter(&[(py::len(&v_p.clone())?).i()?]) { let mut v_j: Py; 
v_j = t1947;
out.push(py::mul(&py::getitem(&v_row.clone(), &v_j.clone())?, &py::getitem(&v_p.clone(), &v_j.clone())?)?); } Py::list(out) })?;
        let _ = py::method(&v_probs.clone(), "append", &[v_ps.clone()])?;
        if ({ let t1948 = v_ps.clone(); Py::Bool({ let t1949 = Py::Int(0i64); if py::le(&t1948, &t1949)? { true } else { false } }) }).truthy() {
            let _ = py::method(&v_acts.clone(), "append", &[Py::Int(0i64)])?;
            continue;
        }
        v_post = { let mut out = vec![]; for t1950 in py::range_iter(&[(py::len(&v_p.clone())?).i()?]) { let mut v_j: Py; 
v_j = t1950;
out.push(py::truediv(&py::mul(&py::getitem(&v_row.clone(), &v_j.clone())?, &py::getitem(&v_p.clone(), &v_j.clone())?)?, &v_ps.clone())?); } Py::list(out) };
        v_ev = f__ev(v_m.clone(), v_post.clone())?;
        v_b = f__argbest(v_ev.clone(), Py::Int(1i64))?;
        let _ = py::method(&v_acts.clone(), "append", &[py::add(&v_b.clone(), &Py::Int(1i64))?])?;
        let _ = py::method(&v_total.clone(), "append", &[py::mul(&v_ps.clone(), &py::getitem(&v_ev.clone(), &v_b.clone())?)?])?;
        let _ = py::method(&v_gain.clone(), "append", &[py::mul(&v_ps.clone(), &py::sub(&py::getitem(&v_ev.clone(), &v_b.clone())?, &py::getitem(&v_ev.clone(), &v_a0.clone())?)?)?])?;
    }
    v_with_ = py::fsum(&v_total.clone())?;
    return Ok(Py::dict(vec![(Py::str("evsi"), py::fsum(&v_gain.clone())?), (Py::str("ev_with_information"), v_with_.clone()), (Py::str("ev_without_information"), v_without.clone()), (Py::str("signal_probabilities"), v_probs.clone()), (Py::str("best_action_per_signal"), v_acts.clone())]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_run_gamma_ci(mut v_shape: Py, mut v_rate: Py, mut v_level: Py) -> R {
    let mut v_a: Py = Py::None;
    let mut v_b: Py = Py::None;
    let mut v_c: Py = Py::None;
    let t1999 = (Py::tuple(vec![f__pos(v_shape.clone(), Py::str("the shape"))?, f__pos(v_rate.clone(), Py::str("the rate"))?, f__level(v_level.clone())?])).items()?;
    if t1999.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_a = t1999[0].clone();
    v_b = t1999[1].clone();
    v_c = t1999[2].clone();
    return Ok(Py::dict(vec![(Py::str("lower"), py::truediv(&f__gamma_ppf(py::mul(&Py::Float(0.5), &py::sub(&Py::Int(1i64), &v_c.clone())?)?, v_a.clone())?, &v_b.clone())?), (Py::str("upper"), py::truediv(&f__gamma_ppf(py::mul(&Py::Float(0.5), &py::add(&Py::Int(1i64), &v_c.clone())?)?, v_a.clone())?, &v_b.clone())?), (Py::str("median"), py::truediv(&f__gamma_ppf(Py::Float(0.5), v_a.clone())?, &v_b.clone())?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_run_gamma_compare(mut v_shape1: Py, mut v_rate1: Py, mut v_shape2: Py, mut v_rate2: Py) -> R {
    let mut v_a1: Py = Py::None;
    let mut v_a2: Py = Py::None;
    let mut v_b1: Py = Py::None;
    let mut v_b2: Py = Py::None;
    let mut v_lo: Py = Py::None;
    let mut v_up: Py = Py::None;
    let t4086 = ({ let mut out = vec![]; for t4080 in py::iter(&(Py::tuple(vec![Py::tuple(vec![v_shape1.clone(), Py::str("shape1")]), Py::tuple(vec![v_rate1.clone(), Py::str("rate1")]), Py::tuple(vec![v_shape2.clone(), Py::str("shape2")]), Py::tuple(vec![v_rate2.clone(), Py::str("rate2")])])))? { let mut v_n: Py; let mut v_v: Py; 
let t4081 = (t4080).items()?;
if t4081.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_v = t4081[0].clone();
v_n = t4081[1].clone();
out.push(f__pos(v_v.clone(), v_n.clone())?); } Py::list(out) }).items()?;
    if t4086.len() != 4 { return Err(PyErr::Value("unpack".into())); }
    v_a1 = t4086[0].clone();
    v_b1 = t4086[1].clone();
    v_a2 = t4086[2].clone();
    v_b2 = t4086[3].clone();
    let t4110 = (f__ibeta(v_a1.clone(), v_a2.clone(), py::truediv(&v_b1.clone(), &py::add(&v_b1.clone(), &v_b2.clone())?)?, py::truediv(&v_b2.clone(), &py::add(&v_b1.clone(), &v_b2.clone())?)?)?).items()?;
    if t4110.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_lo = t4110[0].clone();
    v_up = t4110[1].clone();
    return Ok(Py::dict(vec![(Py::str("prob_1_greater"), v_up.clone()), (Py::str("prob_2_greater"), v_lo.clone()), (Py::str("rate_ratio_mean"), py::truediv(&py::truediv(&v_a1.clone(), &v_b1.clone())?, &py::truediv(&v_a2.clone(), &v_b2.clone())?)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_run_gamma_hdi(mut v_shape: Py, mut v_rate: Py, mut v_level: Py) -> R {
    let mut v__: Py = Py::None;
    let mut v_b: Py = Py::None;
    let mut v_c: Py = Py::None;
    let mut v_g: Py = Py::None;
    let mut v_hi: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_l: Py = Py::None;
    let mut v_lo: Py = Py::None;
    let mut v_mid: Py = Py::None;
    let mut v_u: Py = Py::None;
    let mut v_upper_of: Py = Py::None;
    let t2154 = (Py::tuple(vec![f__pos(v_shape.clone(), Py::str("the shape"))?, f__pos(v_rate.clone(), Py::str("the rate"))?, f__level(v_level.clone())?])).items()?;
    if t2154.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_k = t2154[0].clone();
    v_b = t2154[1].clone();
    v_c = t2154[2].clone();
    if ({ let t2155 = v_k.clone(); Py::Bool({ let t2156 = Py::Int(1i64); if py::le(&t2155, &t2156)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the highest-density interval is computed for a unimodal gamma with shape > 1"))));
    }
    v_upper_of = { let v_b = v_b.clone(); let v_c = v_c.clone(); let v_k = v_k.clone(); let v_level = v_level.clone(); let v_rate = v_rate.clone(); let v_shape = v_shape.clone(); let v_upper_of = v_upper_of.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_l: Py = a.get(0).cloned().unwrap_or(Py::None);
            let mut v_p: Py = Py::None;
            v_p = py::add(&py::getitem(&f__igamma(v_k.clone(), v_l.clone())?, &Py::Int(0i64))?, &v_c.clone())?;
            return Ok((if ({ let t2173 = v_p.clone(); Py::Bool({ let t2174 = Py::Float(1.0); if py::ge(&t2173, &t2174)? { true } else { false } }) }).truthy() { Py::Float(f64::INFINITY) } else { f__gamma_ppf(v_p.clone(), v_k.clone())? }));
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) };
    v_g = { let v_b = v_b.clone(); let v_c = v_c.clone(); let v_g = v_g.clone(); let v_k = v_k.clone(); let v_level = v_level.clone(); let v_rate = v_rate.clone(); let v_shape = v_shape.clone(); let v_upper_of = v_upper_of.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_l: Py = a.get(0).cloned().unwrap_or(Py::None);
            let mut v_u: Py = Py::None;
            v_u = py::call(&v_upper_of.clone(), &[v_l.clone()])?;
            return Ok(py::sub(&(if (py::math1("isfinite", &v_u.clone())?).truthy() { f__gamma_logpdf(v_u.clone(), v_k.clone())? } else { py::neg(&Py::Float(f64::INFINITY))? }), &f__gamma_logpdf(v_l.clone(), v_k.clone())?)?);
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) };
    let t2209 = (Py::tuple(vec![Py::Float(0.0), f__gamma_ppf(py::sub(&Py::Int(1i64), &v_c.clone())?, v_k.clone())?])).items()?;
    if t2209.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_lo = t2209[0].clone();
    v_hi = t2209[1].clone();
    for t2210 in py::range_iter(&[(Py::Int(300i64)).i()?]) {
        v__ = t2210;
        v_mid = py::mul(&Py::Float(0.5), &py::add(&v_lo.clone(), &v_hi.clone())?)?;
        if ({ let t2211 = v_mid.clone(); Py::Bool({ let t2212 = Py::tuple(vec![v_lo.clone(), v_hi.clone()]); if py::contains(&t2212, &t2211)? { true } else { false } }) }).truthy() {
            break;
        }
        if ({ let t2213 = py::call(&v_g.clone(), &[v_mid.clone()])?; Py::Bool({ let t2214 = Py::Int(0i64); if py::gt(&t2213, &t2214)? { true } else { false } }) }).truthy() {
            v_lo = v_mid.clone();
        } else {
            v_hi = v_mid.clone();
        }
    }
    v_l = py::mul(&Py::Float(0.5), &py::add(&v_lo.clone(), &v_hi.clone())?)?;
    v_u = py::call(&v_upper_of.clone(), &[v_l.clone()])?;
    return Ok(Py::dict(vec![(Py::str("lower"), py::truediv(&v_l.clone(), &v_b.clone())?), (Py::str("upper"), py::truediv(&v_u.clone(), &v_b.clone())?), (Py::str("width"), py::truediv(&py::sub(&v_u.clone(), &v_l.clone())?, &v_b.clone())?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_run_gauss_hermite(mut v_n: Py) -> R {
    let mut v__: Py = Py::None;
    let mut v_h: Py = Py::None;
    let mut v_h1: Py = Py::None;
    let mut v_i: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_nodes: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_weights: Py = Py::None;
    let mut v_x: Py = Py::None;
    let mut v_xs: Py = Py::None;
    v_k = f__whole(v_n.clone(), Py::str("the number of nodes"), Py::Int(1i64))?;
    if ({ let t2486 = v_k.clone(); Py::Bool({ let t2487 = Py::Int(40i64); if py::gt(&t2486, &t2487)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("at most 40 nodes"))));
    }
    v_xs = f__tqli_first(py::mul(&Py::list(vec![Py::Float(0.0)]), &v_k.clone())?, { let mut out = vec![]; for t2509 in py::range_iter(&[(Py::Int(1i64)).i()?, (v_k.clone()).i()?]) { let mut v_i: Py; 
v_i = t2509;
out.push(py::math1("sqrt", &v_i.clone())?); } Py::list(out) })?;
    let t2510 = (Py::tuple(vec![Py::list(vec![]), Py::list(vec![])])).items()?;
    if t2510.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_nodes = t2510[0].clone();
    v_weights = t2510[1].clone();
    for t2511 in py::iter(&(v_xs.clone()))? {
        v_x = t2511;
        for t2512 in py::range_iter(&[(Py::Int(3i64)).i()?]) {
            v__ = t2512;
            let t2518 = (f__hermite_e(v_k.clone(), v_x.clone())?).items()?;
            if t2518.len() != 2 { return Err(PyErr::Value("unpack".into())); }
            v_h = t2518[0].clone();
            v_h1 = t2518[1].clone();
            if ({ let t2519 = v_h1.clone(); Py::Bool({ let t2520 = Py::Int(0i64); if py::eq(&t2519, &t2520) { true } else { false } }) }).truthy() {
                break;
            }
            v_x = py::sub(&v_x, &py::truediv(&v_h.clone(), &py::mul(&v_k.clone(), &v_h1.clone())?)?)?;
        }
        let t2521 = (f__hermite_e(v_k.clone(), v_x.clone())?).items()?;
        if t2521.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_h = t2521[0].clone();
        v_h1 = t2521[1].clone();
        let _ = py::method(&v_nodes.clone(), "append", &[v_x.clone()])?;
        let _ = py::method(&v_weights.clone(), "append", &[(if ({ let t2522 = v_k.clone(); Py::Bool({ let t2523 = Py::Int(1i64); if py::gt(&t2522, &t2523)? { true } else { false } }) }).truthy() { py::math1("exp", &py::sub(&py::sub(&py::math1("lgamma", &py::add(&v_k.clone(), &Py::Int(1i64))?)?, &py::mul(&Py::Int(2i64), &py::math1("log", &v_k.clone())?)?)?, &py::mul(&Py::Int(2i64), &py::math1("log", &py::abs(&v_h1.clone())?)?)?)?)? } else { Py::Float(1.0) })])?;
    }
    for t2524 in py::range_iter(&[(py::floordiv(&v_k.clone(), &Py::Int(2i64))?).i()?]) {
        v_i = t2524;
        v_v = py::mul(&Py::Float(0.5), &py::sub(&py::getitem(&v_nodes.clone(), &py::sub(&py::sub(&v_k.clone(), &Py::Int(1i64))?, &v_i.clone())?)?, &py::getitem(&v_nodes.clone(), &v_i.clone())?)?)?;
        v_w = py::mul(&Py::Float(0.5), &py::add(&py::getitem(&v_weights.clone(), &v_i.clone())?, &py::getitem(&v_weights.clone(), &py::sub(&py::sub(&v_k.clone(), &Py::Int(1i64))?, &v_i.clone())?)?)?)?;
        let t2525 = (Py::tuple(vec![py::neg(&v_v.clone())?, v_v.clone(), v_w.clone(), v_w.clone()])).items()?;
        if t2525.len() != 4 { return Err(PyErr::Value("unpack".into())); }
        py::setitem(&v_nodes.clone(), &v_i.clone(), t2525[0].clone())?;
        py::setitem(&v_nodes.clone(), &py::sub(&py::sub(&v_k.clone(), &Py::Int(1i64))?, &v_i.clone())?, t2525[1].clone())?;
        py::setitem(&v_weights.clone(), &v_i.clone(), t2525[2].clone())?;
        py::setitem(&v_weights.clone(), &py::sub(&py::sub(&v_k.clone(), &Py::Int(1i64))?, &v_i.clone())?, t2525[3].clone())?;
    }
    if (py::modulo(&v_k.clone(), &Py::Int(2i64))?).truthy() {
        py::setitem(&v_nodes.clone(), &py::floordiv(&v_k.clone(), &Py::Int(2i64))?, Py::Float(0.0))?;
    }
    return Ok(Py::dict(vec![(Py::str("nodes"), v_nodes.clone()), (Py::str("weights"), v_weights.clone())]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_run_gexp_post(mut v_shape: Py, mut v_rate: Py, mut v_times: Py) -> R {
    let mut v_a: Py = Py::None;
    let mut v_a2: Py = Py::None;
    let mut v_b: Py = Py::None;
    let mut v_b2: Py = Py::None;
    let mut v_t: Py = Py::None;
    let t2093 = (Py::tuple(vec![f__pos(v_shape.clone(), Py::str("the prior shape"))?, f__pos(v_rate.clone(), Py::str("the prior rate"))?])).items()?;
    if t2093.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_a = t2093[0].clone();
    v_b = t2093[1].clone();
    v_t = f__vec(v_times.clone(), Py::str("the observed times"), Py::None, Py::Int(1i64))?;
    if (Py::Bool(({ let mut out = vec![]; for t2102 in py::iter(&(v_t.clone()))? { let mut v_v: Py; 
v_v = t2102;
out.push({ let t2103 = v_v.clone(); Py::Bool({ let t2104 = Py::Int(0i64); if py::le(&t2103, &t2104)? { true } else { false } }) }); } Py::list(out) }).items()?.iter().any(|x| x.truthy()))).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("exponential observations must be positive"))));
    }
    let t2105 = (Py::tuple(vec![py::add(&v_a.clone(), &py::len(&v_t.clone())?)?, py::add(&v_b.clone(), &py::fsum(&v_t.clone())?)?])).items()?;
    if t2105.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_a2 = t2105[0].clone();
    v_b2 = t2105[1].clone();
    return Ok(Py::dict(vec![(Py::str("post_shape"), v_a2.clone()), (Py::str("post_rate"), v_b2.clone()), (Py::str("posterior_mean"), py::truediv(&v_a2.clone(), &v_b2.clone())?), (Py::str("posterior_variance"), py::truediv(&v_a2.clone(), &py::pow(&v_b2.clone(), &Py::Int(2i64))?)?), (Py::str("posterior_sd"), py::truediv(&py::math1("sqrt", &v_a2.clone())?, &v_b2.clone())?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_run_gfun(mut v_a: Py) -> R {
    let mut v_V: Py = Py::None;
    let mut v_aa: Py = Py::None;
    let mut v_prod: Py = Py::None;
    let mut v_vi: Py = Py::None;
    v_aa = f__vec(v_a.clone(), Py::str("the parameters a_i"), Py::None, Py::Int(1i64))?;
    if (Py::Bool(({ let mut out = vec![]; for t4373 in py::iter(&(v_aa.clone()))? { let mut v_v: Py; 
v_v = t4373;
out.push({ let t4374 = v_v.clone(); Py::Bool({ let t4375 = Py::Int(0i64); if py::lt(&t4374, &t4375)? { true } else { false } }) }); } Py::list(out) }).items()?.iter().any(|x| x.truthy()))).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the G-function parameters must be >= 0"))));
    }
    v_vi = { let mut out = vec![]; for t4376 in py::iter(&(v_aa.clone()))? { let mut v_v: Py; 
v_v = t4376;
out.push(py::truediv(&Py::Int(1i64), &py::mul(&Py::Int(3i64), &py::pow(&py::add(&Py::Int(1i64), &v_v.clone())?, &Py::Int(2i64))?)?)?); } Py::list(out) };
    v_prod = py::prod(&{ let mut out = vec![]; for t4377 in py::iter(&(v_vi.clone()))? { let mut v_v: Py; 
v_v = t4377;
out.push(py::add(&Py::Int(1i64), &v_v.clone())?); } Py::list(out) })?;
    v_V = py::sub(&v_prod.clone(), &Py::Int(1i64))?;
    return Ok(Py::dict(vec![(Py::str("first_order"), { let mut out = vec![]; for t4378 in py::iter(&(v_vi.clone()))? { let mut v_v: Py; 
v_v = t4378;
out.push(py::truediv(&v_v.clone(), &v_V.clone())?); } Py::list(out) }), (Py::str("total_order"), { let mut out = vec![]; for t4379 in py::iter(&(v_vi.clone()))? { let mut v_v: Py; 
v_v = t4379;
out.push(py::truediv(&py::truediv(&py::mul(&v_v.clone(), &v_prod.clone())?, &py::add(&Py::Int(1i64), &v_v.clone())?)?, &v_V.clone())?); } Py::list(out) }), (Py::str("variance"), v_V.clone())]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_run_gp_ci(mut v_shape: Py, mut v_rate: Py, mut v_counts: Py, mut v_level: Py) -> R {
    let mut v_a: Py = Py::None;
    let mut v_a2: Py = Py::None;
    let mut v_b: Py = Py::None;
    let mut v_b2: Py = Py::None;
    let mut v_c: Py = Py::None;
    let mut v_x: Py = Py::None;
    let t2273 = (f__gp_check(v_shape.clone(), v_rate.clone(), v_counts.clone())?).items()?;
    if t2273.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_a = t2273[0].clone();
    v_b = t2273[1].clone();
    v_x = t2273[2].clone();
    v_c = f__level(v_level.clone())?;
    let t2279 = (Py::tuple(vec![py::add(&v_a.clone(), &py::fsum(&v_x.clone())?)?, py::add(&v_b.clone(), &py::len(&v_x.clone())?)?])).items()?;
    if t2279.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_a2 = t2279[0].clone();
    v_b2 = t2279[1].clone();
    return Ok(Py::dict(vec![(Py::str("post_shape"), v_a2.clone()), (Py::str("post_rate"), v_b2.clone()), (Py::str("lower"), py::truediv(&f__gamma_ppf(py::mul(&Py::Float(0.5), &py::sub(&Py::Int(1i64), &v_c.clone())?)?, v_a2.clone())?, &v_b2.clone())?), (Py::str("upper"), py::truediv(&f__gamma_ppf(py::mul(&Py::Float(0.5), &py::add(&Py::Int(1i64), &v_c.clone())?)?, v_a2.clone())?, &v_b2.clone())?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_run_gp_post(mut v_shape: Py, mut v_rate: Py, mut v_counts: Py) -> R {
    let mut v_a: Py = Py::None;
    let mut v_a2: Py = Py::None;
    let mut v_b: Py = Py::None;
    let mut v_b2: Py = Py::None;
    let mut v_x: Py = Py::None;
    let t2388 = (f__gp_check(v_shape.clone(), v_rate.clone(), v_counts.clone())?).items()?;
    if t2388.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_a = t2388[0].clone();
    v_b = t2388[1].clone();
    v_x = t2388[2].clone();
    let t2389 = (Py::tuple(vec![py::add(&v_a.clone(), &py::fsum(&v_x.clone())?)?, py::add(&v_b.clone(), &py::len(&v_x.clone())?)?])).items()?;
    if t2389.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_a2 = t2389[0].clone();
    v_b2 = t2389[1].clone();
    return Ok(Py::dict(vec![(Py::str("post_shape"), v_a2.clone()), (Py::str("post_rate"), v_b2.clone()), (Py::str("posterior_mean"), py::truediv(&v_a2.clone(), &v_b2.clone())?), (Py::str("posterior_variance"), py::truediv(&v_a2.clone(), &py::pow(&v_b2.clone(), &Py::Int(2i64))?)?), (Py::str("posterior_sd"), py::truediv(&py::math1("sqrt", &v_a2.clone())?, &v_b2.clone())?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_run_gp_pred(mut v_shape: Py, mut v_rate: Py, mut v_events: Py) -> R {
    let mut v_a: Py = Py::None;
    let mut v_at_most: Py = Py::None;
    let mut v_b: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_lp: Py = Py::None;
    let t2433 = (Py::tuple(vec![f__pos(v_shape.clone(), Py::str("the shape"))?, f__pos(v_rate.clone(), Py::str("the rate"))?])).items()?;
    if t2433.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_a = t2433[0].clone();
    v_b = t2433[1].clone();
    v_k = f__whole(v_events.clone(), Py::str("the number of events"), Py::Int(0i64))?;
    v_lp = { let v_a = v_a.clone(); let v_b = v_b.clone(); let v_events = v_events.clone(); let v_k = v_k.clone(); let v_rate = v_rate.clone(); let v_shape = v_shape.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_i: Py = a.get(0).cloned().unwrap_or(Py::None);
            return Ok(py::sub(&py::add(&py::sub(&py::sub(&py::math1("lgamma", &py::add(&v_a.clone(), &v_i.clone())?)?, &py::math1("lgamma", &v_a.clone())?)?, &py::math1("lgamma", &py::add(&v_i.clone(), &Py::Int(1i64))?)?)?, &py::mul(&v_a.clone(), &py::math1("log", &py::truediv(&v_b.clone(), &py::add(&v_b.clone(), &Py::Int(1i64))?)?)?)?)?, &py::mul(&v_i.clone(), &py::math1("log", &py::add(&v_b.clone(), &Py::Int(1i64))?)?)?)?);
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) };
    v_at_most = py::fsum(&{ let mut out = vec![]; for t2439 in py::range_iter(&[(py::add(&v_k.clone(), &Py::Int(1i64))?).i()?]) { let mut v_i: Py; 
v_i = t2439;
out.push(py::math1("exp", &py::call(&v_lp.clone(), &[v_i.clone()])?)?); } Py::list(out) })?;
    return Ok(Py::dict(vec![(Py::str("probability"), py::math1("exp", &py::call(&v_lp.clone(), &[v_k.clone()])?)?), (Py::str("prob_at_most"), py::minmax(&[Py::Float(1.0), v_at_most.clone()], false)?), (Py::str("predictive_mean"), py::truediv(&v_a.clone(), &v_b.clone())?), (Py::str("predictive_variance"), py::mul(&py::truediv(&v_a.clone(), &v_b.clone())?, &py::add(&Py::Int(1i64), &py::truediv(&Py::Int(1i64), &v_b.clone())?)?)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_run_halton(mut v_index: Py, mut v_dimension: Py) -> R {
    let mut v_d: Py = Py::None;
    let mut v_i: Py = Py::None;
    let t2572 = (Py::tuple(vec![f__whole(v_index.clone(), Py::str("the index"), Py::Int(0i64))?, f__whole(v_dimension.clone(), Py::str("the dimension"), Py::Int(1i64))?])).items()?;
    if t2572.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_i = t2572[0].clone();
    v_d = t2572[1].clone();
    if ({ let t2573 = v_d.clone(); Py::Bool({ let t2574 = py::len(&c__PRIMES()?)?; if py::gt(&t2573, &t2574)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str(&["dimension must be at most ".to_string(), py::format(&py::len(&c__PRIMES()?)?, "")?].concat()))));
    }
    if ({ let t2575 = v_i.clone(); Py::Bool({ let t2576 = py::pow(&Py::Int(10i64), &Py::Int(7i64))?; if py::gt(&t2575, &t2576)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("index must be at most 10,000,000"))));
    }
    return Ok(Py::dict(vec![(Py::str("point"), { let mut out = vec![]; for t2577 in py::iter(&(py::slice(&c__PRIMES()?, None, Some((v_d.clone()).i()?), None)?))? { let mut v_b: Py; 
v_b = t2577;
out.push(f__radical_inverse(v_i.clone(), v_b.clone())?); } Py::list(out) }), (Py::str("bases"), { let mut out = vec![]; for t2583 in py::iter(&(py::slice(&c__PRIMES()?, None, Some((v_d.clone()).i()?), None)?))? { let mut v_b: Py; 
v_b = t2583;
out.push(py::float(&v_b.clone())?); } Py::list(out) })]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_run_is(mut v_values: Py, mut v_target_density: Py, mut v_proposal_density: Py) -> R {
    let mut v_est: Py = Py::None;
    let mut v_f: Py = Py::None;
    let mut v_p: Py = Py::None;
    let mut v_q: Py = Py::None;
    let mut v_sw: Py = Py::None;
    let mut v_w: Py = Py::None;
    v_f = f__vec(v_values.clone(), Py::str("the function values"), Py::None, Py::Int(2i64))?;
    v_p = f__vec(v_target_density.clone(), Py::str("the target densities"), py::len(&v_f.clone())?, Py::Int(1i64))?;
    v_q = f__vec(v_proposal_density.clone(), Py::str("the proposal densities"), py::len(&v_f.clone())?, Py::Int(1i64))?;
    if ({ let t2643 = Py::Bool(({ let mut out = vec![]; for t2633 in py::iter(&(v_q.clone()))? { let mut v_v: Py; 
v_v = t2633;
out.push({ let t2634 = v_v.clone(); Py::Bool({ let t2635 = Py::Int(0i64); if py::le(&t2634, &t2635)? { true } else { false } }) }); } Py::list(out) }).items()?.iter().any(|x| x.truthy())); if t2643.truthy() { t2643 } else { { let t2642 = Py::Bool(({ let mut out = vec![]; for t2636 in py::iter(&(v_p.clone()))? { let mut v_v: Py; 
v_v = t2636;
out.push({ let t2637 = v_v.clone(); Py::Bool({ let t2638 = Py::Int(0i64); if py::lt(&t2637, &t2638)? { true } else { false } }) }); } Py::list(out) }).items()?.iter().any(|x| x.truthy())); if t2642.truthy() { t2642 } else { py::not(&Py::Bool(({ let mut out = vec![]; for t2639 in py::iter(&(v_p.clone()))? { let mut v_v: Py; 
v_v = t2639;
out.push({ let t2640 = v_v.clone(); Py::Bool({ let t2641 = Py::Int(0i64); if py::gt(&t2640, &t2641)? { true } else { false } }) }); } Py::list(out) }).items()?.iter().any(|x| x.truthy()))) } } } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("proposal densities must be positive and target densities non-negative (not all zero)"))));
    }
    v_w = { let mut out = vec![]; for t2644 in py::iter(&(py::zip(&[v_p.clone(), v_q.clone()])?))? { let mut v_a: Py; let mut v_b: Py; 
let t2645 = (t2644).items()?;
if t2645.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_a = t2645[0].clone();
v_b = t2645[1].clone();
out.push(py::truediv(&v_a.clone(), &v_b.clone())?); } Py::list(out) };
    v_sw = py::fsum(&v_w.clone())?;
    v_est = py::truediv(&py::fsum(&{ let mut out = vec![]; for t2646 in py::iter(&(py::zip(&[v_w.clone(), v_f.clone()])?))? { let mut v_a: Py; let mut v_b: Py; 
let t2647 = (t2646).items()?;
if t2647.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_a = t2647[0].clone();
v_b = t2647[1].clone();
out.push(py::mul(&v_a.clone(), &v_b.clone())?); } Py::list(out) })?, &v_sw.clone())?;
    return Ok(Py::dict(vec![(Py::str("estimate"), v_est.clone()), (Py::str("unnormalized_estimate"), py::truediv(&py::fsum(&{ let mut out = vec![]; for t2648 in py::iter(&(py::zip(&[v_w.clone(), v_f.clone()])?))? { let mut v_a: Py; let mut v_b: Py; 
let t2649 = (t2648).items()?;
if t2649.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_a = t2649[0].clone();
v_b = t2649[1].clone();
out.push(py::mul(&v_a.clone(), &v_b.clone())?); } Py::list(out) })?, &py::len(&v_f.clone())?)?), (Py::str("effective_sample_size"), py::truediv(&py::mul(&v_sw.clone(), &v_sw.clone())?, &py::fsum(&{ let mut out = vec![]; for t2650 in py::iter(&(v_w.clone()))? { let mut v_v: Py; 
v_v = t2650;
out.push(py::mul(&v_v.clone(), &v_v.clone())?); } Py::list(out) })?)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_run_ishigami(mut v_a: Py, mut v_b: Py) -> R {
    let mut v_p4: Py = Py::None;
    let mut v_p8: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_v1: Py = Py::None;
    let mut v_v13: Py = Py::None;
    let mut v_v2: Py = Py::None;
    let t4470 = (Py::tuple(vec![f__fin(v_a.clone(), Py::str("a"))?, f__fin(v_b.clone(), Py::str("b"))?])).items()?;
    if t4470.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_a = t4470[0].clone();
    v_b = t4470[1].clone();
    let t4471 = (Py::tuple(vec![py::pow(&Py::Float(std::f64::consts::PI), &Py::Int(4i64))?, py::pow(&Py::Float(std::f64::consts::PI), &Py::Int(8i64))?])).items()?;
    if t4471.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_p4 = t4471[0].clone();
    v_p8 = t4471[1].clone();
    v_v1 = py::mul(&Py::Float(0.5), &py::pow(&py::add(&Py::Int(1i64), &py::truediv(&py::mul(&v_b.clone(), &v_p4.clone())?, &Py::Int(5i64))?)?, &Py::Int(2i64))?)?;
    v_v2 = py::truediv(&py::mul(&v_a.clone(), &v_a.clone())?, &Py::Int(8i64))?;
    v_v13 = py::truediv(&py::mul(&py::mul(&py::mul(&Py::Int(8i64), &v_b.clone())?, &v_b.clone())?, &v_p8.clone())?, &Py::Int(225i64))?;
    v_v = py::add(&py::add(&v_v1.clone(), &v_v2.clone())?, &v_v13.clone())?;
    if ({ let t4472 = v_v.clone(); Py::Bool({ let t4473 = Py::Int(0i64); if py::le(&t4472, &t4473)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the model has zero variance"))));
    }
    return Ok(Py::dict(vec![(Py::str("S1"), py::truediv(&v_v1.clone(), &v_v.clone())?), (Py::str("S2"), py::truediv(&v_v2.clone(), &v_v.clone())?), (Py::str("S3"), Py::Float(0.0)), (Py::str("ST1"), py::truediv(&py::add(&v_v1.clone(), &v_v13.clone())?, &v_v.clone())?), (Py::str("ST2"), py::truediv(&v_v2.clone(), &v_v.clone())?), (Py::str("ST3"), py::truediv(&v_v13.clone(), &v_v.clone())?), (Py::str("variance"), v_v.clone())]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_run_jeffreys(mut v_successes: Py, mut v_trials: Py, mut v_level: Py) -> R {
    let mut v_c: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_lo: Py = Py::None;
    let mut v_n: Py = Py::None;
    let mut v_up: Py = Py::None;
    let t2754 = (Py::tuple(vec![f__whole(v_trials.clone(), Py::str("the number of trials"), Py::Int(1i64))?, f__whole(v_successes.clone(), Py::str("the number of successes"), Py::Int(0i64))?])).items()?;
    if t2754.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_n = t2754[0].clone();
    v_k = t2754[1].clone();
    if ({ let t2755 = v_k.clone(); Py::Bool({ let t2756 = v_n.clone(); if py::gt(&t2755, &t2756)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("successes cannot exceed trials"))));
    }
    v_c = f__level(v_level.clone())?;
    v_lo = (if ({ let t2762 = v_k.clone(); Py::Bool({ let t2763 = Py::Int(0i64); if py::eq(&t2762, &t2763) { true } else { false } }) }).truthy() { Py::Float(0.0) } else { f__beta_ppf(py::mul(&Py::Float(0.5), &py::sub(&Py::Int(1i64), &v_c.clone())?)?, py::add(&v_k.clone(), &Py::Float(0.5))?, py::add(&py::sub(&v_n.clone(), &v_k.clone())?, &Py::Float(0.5))?)? });
    v_up = (if ({ let t2843 = v_k.clone(); Py::Bool({ let t2844 = v_n.clone(); if py::eq(&t2843, &t2844) { true } else { false } }) }).truthy() { Py::Float(1.0) } else { f__beta_ppf(py::mul(&Py::Float(0.5), &py::add(&Py::Int(1i64), &v_c.clone())?)?, py::add(&v_k.clone(), &Py::Float(0.5))?, py::add(&py::sub(&v_n.clone(), &v_k.clone())?, &Py::Float(0.5))?)? });
    return Ok(Py::dict(vec![(Py::str("lower"), v_lo.clone()), (Py::str("upper"), v_up.clone()), (Py::str("posterior_mean"), py::truediv(&py::add(&v_k.clone(), &Py::Float(0.5))?, &py::add(&v_n.clone(), &Py::Float(1.0))?)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_run_kl_beta(mut v_alpha1: Py, mut v_beta1: Py, mut v_alpha2: Py, mut v_beta2: Py) -> R {
    let mut v_a1: Py = Py::None;
    let mut v_a2: Py = Py::None;
    let mut v_b1: Py = Py::None;
    let mut v_b2: Py = Py::None;
    let mut v_kl: Py = Py::None;
    let t2890 = ({ let mut out = vec![]; for t2884 in py::iter(&(Py::tuple(vec![Py::tuple(vec![v_alpha1.clone(), Py::str("alpha1")]), Py::tuple(vec![v_beta1.clone(), Py::str("beta1")]), Py::tuple(vec![v_alpha2.clone(), Py::str("alpha2")]), Py::tuple(vec![v_beta2.clone(), Py::str("beta2")])])))? { let mut v_n: Py; let mut v_v: Py; 
let t2885 = (t2884).items()?;
if t2885.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_v = t2885[0].clone();
v_n = t2885[1].clone();
out.push(f__pos(v_v.clone(), v_n.clone())?); } Py::list(out) }).items()?;
    if t2890.len() != 4 { return Err(PyErr::Value("unpack".into())); }
    v_a1 = t2890[0].clone();
    v_b1 = t2890[1].clone();
    v_a2 = t2890[2].clone();
    v_b2 = t2890[3].clone();
    v_kl = py::add(&py::add(&py::add(&py::sub(&f__lbeta(v_a2.clone(), v_b2.clone())?, &f__lbeta(v_a1.clone(), v_b1.clone())?)?, &py::mul(&py::sub(&v_a1.clone(), &v_a2.clone())?, &f__digamma(v_a1.clone())?)?)?, &py::mul(&py::sub(&v_b1.clone(), &v_b2.clone())?, &f__digamma(v_b1.clone())?)?)?, &py::mul(&py::sub(&py::add(&py::sub(&v_a2.clone(), &v_a1.clone())?, &v_b2.clone())?, &v_b1.clone())?, &f__digamma(py::add(&v_a1.clone(), &v_b1.clone())?)?)?)?;
    return Ok(Py::dict(vec![(Py::str("kl"), v_kl.clone())]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_run_kl_normal(mut v_mean1: Py, mut v_sd1: Py, mut v_mean2: Py, mut v_sd2: Py) -> R {
    let mut v_kl: Py = Py::None;
    let mut v_m1: Py = Py::None;
    let mut v_m2: Py = Py::None;
    let mut v_s1: Py = Py::None;
    let mut v_s2: Py = Py::None;
    let t2934 = (Py::tuple(vec![f__fin(v_mean1.clone(), Py::str("mean1"))?, f__fin(v_mean2.clone(), Py::str("mean2"))?])).items()?;
    if t2934.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_m1 = t2934[0].clone();
    v_m2 = t2934[1].clone();
    let t2937 = (Py::tuple(vec![f__pos(v_sd1.clone(), Py::str("sd1"))?, f__pos(v_sd2.clone(), Py::str("sd2"))?])).items()?;
    if t2937.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_s1 = t2937[0].clone();
    v_s2 = t2937[1].clone();
    v_kl = { let v_m1 = v_m1.clone(); let v_m2 = v_m2.clone(); let v_mean1 = v_mean1.clone(); let v_mean2 = v_mean2.clone(); let v_s1 = v_s1.clone(); let v_s2 = v_s2.clone(); let v_sd1 = v_sd1.clone(); let v_sd2 = v_sd2.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_a: Py = a.get(0).cloned().unwrap_or(Py::None);
            let mut v_s: Py = a.get(1).cloned().unwrap_or(Py::None);
            let mut v_b: Py = a.get(2).cloned().unwrap_or(Py::None);
            let mut v_t: Py = a.get(3).cloned().unwrap_or(Py::None);
            return Ok(py::sub(&py::add(&py::math1("log", &py::truediv(&v_t.clone(), &v_s.clone())?)?, &py::truediv(&py::add(&py::mul(&v_s.clone(), &v_s.clone())?, &py::pow(&py::sub(&v_a.clone(), &v_b.clone())?, &Py::Int(2i64))?)?, &py::mul(&py::mul(&Py::Int(2i64), &v_t.clone())?, &v_t.clone())?)?)?, &Py::Float(0.5))?);
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) };
    return Ok(Py::dict(vec![(Py::str("kl_1_to_2"), py::call(&v_kl.clone(), &[v_m1.clone(), v_s1.clone(), v_m2.clone(), v_s2.clone()])?), (Py::str("kl_2_to_1"), py::call(&v_kl.clone(), &[v_m2.clone(), v_s2.clone(), v_m1.clone(), v_s1.clone()])?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_run_laplace_beta(mut v_alpha: Py, mut v_beta: Py, mut v_level: Py) -> R {
    let mut v_a: Py = Py::None;
    let mut v_b: Py = Py::None;
    let mut v_c: Py = Py::None;
    let mut v_m: Py = Py::None;
    let mut v_sd: Py = Py::None;
    let mut v_z: Py = Py::None;
    let t2986 = (Py::tuple(vec![f__pos(v_alpha.clone(), Py::str("alpha"))?, f__pos(v_beta.clone(), Py::str("beta"))?, f__level(v_level.clone())?])).items()?;
    if t2986.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_a = t2986[0].clone();
    v_b = t2986[1].clone();
    v_c = t2986[2].clone();
    if ({ let t2991 = { let t2987 = v_a.clone(); Py::Bool({ let t2988 = Py::Int(1i64); if py::le(&t2987, &t2988)? { true } else { false } }) }; if t2991.truthy() { t2991 } else { { let t2989 = v_b.clone(); Py::Bool({ let t2990 = Py::Int(1i64); if py::le(&t2989, &t2990)? { true } else { false } }) } } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the Laplace approximation needs an interior mode: alpha > 1 and beta > 1"))));
    }
    v_m = py::truediv(&py::sub(&v_a.clone(), &Py::Int(1i64))?, &py::sub(&py::add(&v_a.clone(), &v_b.clone())?, &Py::Int(2i64))?)?;
    v_sd = py::math1("sqrt", &py::truediv(&py::mul(&v_m.clone(), &py::sub(&Py::Int(1i64), &v_m.clone())?)?, &py::sub(&py::add(&v_a.clone(), &v_b.clone())?, &Py::Int(2i64))?)?)?;
    v_z = f__z(py::mul(&Py::Float(0.5), &py::add(&Py::Int(1i64), &v_c.clone())?)?)?;
    return Ok(Py::dict(vec![(Py::str("mode"), v_m.clone()), (Py::str("laplace_sd"), v_sd.clone()), (Py::str("lower"), py::sub(&v_m.clone(), &py::mul(&v_z.clone(), &v_sd.clone())?)?), (Py::str("upper"), py::add(&v_m.clone(), &py::mul(&v_z.clone(), &v_sd.clone())?)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_run_log_ce(mut v_outcomes: Py, mut v_probabilities: Py) -> R {
    let mut v_elu: Py = Py::None;
    let mut v_mean: Py = Py::None;
    let mut v_p: Py = Py::None;
    let mut v_x: Py = Py::None;
    v_x = f__vec(v_outcomes.clone(), Py::str("the outcomes"), Py::None, Py::Int(2i64))?;
    v_p = f__simplex(v_probabilities.clone(), Py::str("the probabilities"), py::len(&v_x.clone())?)?;
    if (Py::Bool(({ let mut out = vec![]; for t1212 in py::iter(&(v_x.clone()))? { let mut v_v: Py; 
v_v = t1212;
out.push({ let t1213 = v_v.clone(); Py::Bool({ let t1214 = Py::Int(0i64); if py::le(&t1213, &t1214)? { true } else { false } }) }); } Py::list(out) }).items()?.iter().any(|x| x.truthy()))).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("logarithmic utility needs positive outcomes (wealth levels)"))));
    }
    v_elu = py::fsum(&{ let mut out = vec![]; for t1215 in py::iter(&(py::zip(&[v_x.clone(), v_p.clone()])?))? { let mut v_a: Py; let mut v_b: Py; 
let t1216 = (t1215).items()?;
if t1216.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_a = t1216[0].clone();
v_b = t1216[1].clone();
out.push(py::mul(&v_b.clone(), &py::math1("log", &v_a.clone())?)?); } Py::list(out) })?;
    v_mean = py::fsum(&{ let mut out = vec![]; for t1217 in py::iter(&(py::zip(&[v_x.clone(), v_p.clone()])?))? { let mut v_a: Py; let mut v_b: Py; 
let t1218 = (t1217).items()?;
if t1218.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_a = t1218[0].clone();
v_b = t1218[1].clone();
out.push(py::mul(&v_a.clone(), &v_b.clone())?); } Py::list(out) })?;
    return Ok(Py::dict(vec![(Py::str("expected_log_utility"), v_elu.clone()), (Py::str("certainty_equivalent"), py::math1("exp", &v_elu.clone())?), (Py::str("risk_premium"), py::sub(&v_mean.clone(), &py::math1("exp", &v_elu.clone())?)?), (Py::str("expected_value"), v_mean.clone())]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_run_lognormal(mut v_mu: Py, mut v_sigma: Py) -> R {
    let mut v_m: Py = Py::None;
    let mut v_s: Py = Py::None;
    let t3123 = (Py::tuple(vec![f__fin(v_mu.clone(), Py::str("mu"))?, f__pos(v_sigma.clone(), Py::str("sigma"))?])).items()?;
    if t3123.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_m = t3123[0].clone();
    v_s = t3123[1].clone();
    if ({ let t3124 = py::add(&v_m.clone(), &py::mul(&v_s.clone(), &v_s.clone())?)?; Py::Bool({ let t3125 = Py::Int(700i64); if py::gt(&t3124, &t3125)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the result overflows"))));
    }
    return Ok(Py::dict(vec![(Py::str("exact_mean"), py::math1("exp", &py::add(&v_m.clone(), &py::truediv(&py::mul(&v_s.clone(), &v_s.clone())?, &Py::Int(2i64))?)?)?), (Py::str("exact_sd"), py::mul(&py::math1("sqrt", &py::math1("expm1", &py::mul(&v_s.clone(), &v_s.clone())?)?)?, &py::math1("exp", &py::add(&v_m.clone(), &py::truediv(&py::mul(&v_s.clone(), &v_s.clone())?, &Py::Int(2i64))?)?)?)?), (Py::str("first_order_mean"), py::math1("exp", &v_m.clone())?), (Py::str("first_order_sd"), py::mul(&py::math1("exp", &v_m.clone())?, &v_s.clone())?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_run_lr(mut v_sensitivity: Py, mut v_specificity: Py) -> R {
    let mut v_ln_: Py = Py::None;
    let mut v_lp: Py = Py::None;
    let mut v_se: Py = Py::None;
    let mut v_sp: Py = Py::None;
    let t3068 = (Py::tuple(vec![f__prob(v_sensitivity.clone(), Py::str("sensitivity"), Py::Bool(false))?, f__prob(v_specificity.clone(), Py::str("specificity"), Py::Bool(false))?])).items()?;
    if t3068.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_se = t3068[0].clone();
    v_sp = t3068[1].clone();
    if ({ let t3076 = { let t3069 = v_sp.clone(); Py::Bool({ let t3070 = Py::Float(1.0); if py::ge(&t3069, &t3070)? { true } else { false } }) }; if t3076.truthy() { t3076 } else { { let t3075 = { let t3071 = v_se.clone(); Py::Bool({ let t3072 = Py::Float(1.0); if py::ge(&t3071, &t3072)? { true } else { false } }) }; if t3075.truthy() { t3075 } else { { let t3073 = v_sp.clone(); Py::Bool({ let t3074 = Py::Float(0.0); if py::le(&t3073, &t3074)? { true } else { false } }) } } } } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("sensitivity and specificity must be below 1 (and specificity above 0) for finite ratios"))));
    }
    let t3077 = (Py::tuple(vec![py::truediv(&v_se.clone(), &py::sub(&Py::Float(1.0), &v_sp.clone())?)?, py::truediv(&py::sub(&Py::Float(1.0), &v_se.clone())?, &v_sp.clone())?])).items()?;
    if t3077.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_lp = t3077[0].clone();
    v_ln_ = t3077[1].clone();
    return Ok(Py::dict(vec![(Py::str("lr_positive"), v_lp.clone()), (Py::str("lr_negative"), v_ln_.clone()), (Py::str("diagnostic_odds_ratio"), (if ({ let t3078 = v_ln_.clone(); Py::Bool({ let t3079 = Py::Int(0i64); if py::gt(&t3078, &t3079)? { true } else { false } }) }).truthy() { py::truediv(&v_lp.clone(), &v_ln_.clone())? } else { py::float(&Py::str("inf"))? }))]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_run_map_bb(mut v_alpha: Py, mut v_beta: Py, mut v_successes: Py, mut v_trials: Py) -> R {
    let mut v_a: Py = Py::None;
    let mut v_a2: Py = Py::None;
    let mut v_b: Py = Py::None;
    let mut v_b2: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_n: Py = Py::None;
    let t3178 = (f__bb_check(v_alpha.clone(), v_beta.clone(), v_successes.clone(), v_trials.clone())?).items()?;
    if t3178.len() != 4 { return Err(PyErr::Value("unpack".into())); }
    v_a = t3178[0].clone();
    v_b = t3178[1].clone();
    v_k = t3178[2].clone();
    v_n = t3178[3].clone();
    let t3179 = (Py::tuple(vec![py::add(&v_a.clone(), &v_k.clone())?, py::sub(&py::add(&v_b.clone(), &v_n.clone())?, &v_k.clone())?])).items()?;
    if t3179.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_a2 = t3179[0].clone();
    v_b2 = t3179[1].clone();
    if ({ let t3184 = { let t3180 = v_a2.clone(); Py::Bool({ let t3181 = Py::Int(1i64); if py::le(&t3180, &t3181)? { true } else { false } }) }; if t3184.truthy() { t3184 } else { { let t3182 = v_b2.clone(); Py::Bool({ let t3183 = Py::Int(1i64); if py::le(&t3182, &t3183)? { true } else { false } }) } } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the posterior mode lies on the boundary (alpha + k <= 1 or beta + n - k <= 1): no interior MAP"))));
    }
    return Ok(Py::dict(vec![(Py::str("map"), py::truediv(&py::sub(&v_a2.clone(), &Py::Int(1i64))?, &py::sub(&py::add(&v_a2.clone(), &v_b2.clone())?, &Py::Int(2i64))?)?), (Py::str("mle"), (if (v_n.clone()).truthy() { py::truediv(&v_k.clone(), &v_n.clone())? } else { Py::None })), (Py::str("posterior_mean"), py::truediv(&v_a2.clone(), &py::add(&v_a2.clone(), &v_b2.clone())?)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_run_map_gp(mut v_shape: Py, mut v_rate: Py, mut v_counts: Py) -> R {
    let mut v_a: Py = Py::None;
    let mut v_a2: Py = Py::None;
    let mut v_b: Py = Py::None;
    let mut v_b2: Py = Py::None;
    let mut v_x: Py = Py::None;
    let t3243 = (f__gp_check(v_shape.clone(), v_rate.clone(), v_counts.clone())?).items()?;
    if t3243.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_a = t3243[0].clone();
    v_b = t3243[1].clone();
    v_x = t3243[2].clone();
    let t3244 = (Py::tuple(vec![py::add(&v_a.clone(), &py::fsum(&v_x.clone())?)?, py::add(&v_b.clone(), &py::len(&v_x.clone())?)?])).items()?;
    if t3244.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_a2 = t3244[0].clone();
    v_b2 = t3244[1].clone();
    if ({ let t3245 = v_a2.clone(); Py::Bool({ let t3246 = Py::Int(1i64); if py::le(&t3245, &t3246)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the posterior mode is at 0 (shape + sum of counts <= 1): no interior MAP"))));
    }
    return Ok(Py::dict(vec![(Py::str("map"), py::truediv(&py::sub(&v_a2.clone(), &Py::Int(1i64))?, &v_b2.clone())?), (Py::str("mle"), f__mean(v_x.clone())?), (Py::str("posterior_mean"), py::truediv(&v_a2.clone(), &v_b2.clone())?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_run_mc_int(mut v_values: Py, mut v_lower: Py, mut v_upper: Py) -> R {
    let mut v_a: Py = Py::None;
    let mut v_b: Py = Py::None;
    let mut v_f: Py = Py::None;
    let mut v_m: Py = Py::None;
    let mut v_s: Py = Py::None;
    v_f = f__vec(v_values.clone(), Py::str("the function values"), Py::None, Py::Int(2i64))?;
    let t3296 = (Py::tuple(vec![f__fin(v_lower.clone(), Py::str("the lower limit"))?, f__fin(v_upper.clone(), Py::str("the upper limit"))?])).items()?;
    if t3296.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_a = t3296[0].clone();
    v_b = t3296[1].clone();
    if ({ let t3297 = v_b.clone(); Py::Bool({ let t3298 = v_a.clone(); if py::le(&t3297, &t3298)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the upper limit must exceed the lower limit"))));
    }
    v_m = f__mean(v_f.clone())?;
    v_s = py::math1("sqrt", &py::truediv(&py::fsum(&{ let mut out = vec![]; for t3299 in py::iter(&(v_f.clone()))? { let mut v_v: Py; 
v_v = t3299;
out.push(py::pow(&py::sub(&v_v.clone(), &v_m.clone())?, &Py::Int(2i64))?); } Py::list(out) })?, &py::sub(&py::len(&v_f.clone())?, &Py::Int(1i64))?)?)?;
    return Ok(Py::dict(vec![(Py::str("estimate"), py::mul(&py::sub(&v_b.clone(), &v_a.clone())?, &v_m.clone())?), (Py::str("standard_error"), py::truediv(&py::mul(&py::sub(&v_b.clone(), &v_a.clone())?, &v_s.clone())?, &py::math1("sqrt", &py::len(&v_f.clone())?)?)?), (Py::str("n"), py::len(&v_f.clone())?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_run_mc_n(mut v_sigma: Py, mut v_half_width: Py, mut v_level: Py) -> R {
    let mut v_c: Py = Py::None;
    let mut v_e: Py = Py::None;
    let mut v_s: Py = Py::None;
    let mut v_z: Py = Py::None;
    let t3348 = (Py::tuple(vec![f__pos(v_sigma.clone(), Py::str("the standard deviation"))?, f__pos(v_half_width.clone(), Py::str("the half-width"))?, f__level(v_level.clone())?])).items()?;
    if t3348.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_s = t3348[0].clone();
    v_e = t3348[1].clone();
    v_c = t3348[2].clone();
    v_z = f__z(py::mul(&Py::Float(0.5), &py::add(&Py::Int(1i64), &v_c.clone())?)?)?;
    return Ok(Py::dict(vec![(Py::str("draws"), py::math1("ceil", &py::sub(&py::pow(&py::truediv(&py::mul(&v_z.clone(), &v_s.clone())?, &v_e.clone())?, &Py::Int(2i64))?, &Py::Float(1e-09))?)?), (Py::str("z"), v_z.clone()), (Py::str("standard_error_target"), py::truediv(&v_e.clone(), &v_z.clone())?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_run_model_probs(mut v_log_evidences: Py, mut v_priors: Py) -> R {
    let mut v_best: Py = Py::None;
    let mut v_le: Py = Py::None;
    let mut v_lse: Py = Py::None;
    let mut v_p: Py = Py::None;
    let mut v_post: Py = Py::None;
    let mut v_z: Py = Py::None;
    v_le = f__vec(v_log_evidences.clone(), Py::str("the log marginal likelihoods"), Py::None, Py::Int(2i64))?;
    v_p = (if ({ let t4012 = v_priors.clone(); Py::Bool({ let t4013 = Py::None; if py::is(&t4012, &t4013) { true } else { false } }) }).truthy() { py::mul(&Py::list(vec![py::truediv(&Py::Float(1.0), &py::len(&v_le.clone())?)?]), &py::len(&v_le.clone())?)? } else { f__simplex(v_priors.clone(), Py::str("the prior model probabilities"), py::len(&v_le.clone())?)? });
    if (Py::Bool(({ let mut out = vec![]; for t4019 in py::iter(&(v_p.clone()))? { let mut v_v: Py; 
v_v = t4019;
out.push({ let t4020 = v_v.clone(); Py::Bool({ let t4021 = Py::Int(0i64); if py::eq(&t4020, &t4021) { true } else { false } }) }); } Py::list(out) }).items()?.iter().all(|x| x.truthy()))).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("at least one model needs a positive prior probability"))));
    }
    v_z = { let mut out = vec![]; for t4022 in py::iter(&(py::zip(&[v_p.clone(), v_le.clone()])?))? { let mut v_v: Py; let mut v_w: Py; 
let t4023 = (t4022).items()?;
if t4023.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_w = t4023[0].clone();
v_v = t4023[1].clone();
out.push((if ({ let t4024 = v_w.clone(); Py::Bool({ let t4025 = Py::Int(0i64); if py::gt(&t4024, &t4025)? { true } else { false } }) }).truthy() { py::add(&py::math1("log", &v_w.clone())?, &v_v.clone())? } else { py::neg(&Py::Float(f64::INFINITY))? })); } Py::list(out) };
    v_lse = f__logsumexp({ let mut out = vec![]; for t4035 in py::iter(&(v_z.clone()))? { let mut v_v: Py; 
v_v = t4035;
if ({ let t4036 = v_v.clone(); Py::Bool({ let t4037 = py::neg(&Py::Float(f64::INFINITY))?; if py::gt(&t4036, &t4037)? { true } else { false } }) }).truthy() { out.push(v_v.clone()); }} Py::list(out) }, Py::None)?;
    v_post = { let mut out = vec![]; for t4038 in py::iter(&(v_z.clone()))? { let mut v_v: Py; 
v_v = t4038;
out.push((if ({ let t4039 = v_v.clone(); Py::Bool({ let t4040 = py::neg(&Py::Float(f64::INFINITY))?; if py::gt(&t4039, &t4040)? { true } else { false } }) }).truthy() { py::math1("exp", &py::sub(&v_v.clone(), &v_lse.clone())?)? } else { Py::Float(0.0) })); } Py::list(out) };
    v_best = py::minmax_key(&[Py::list((py::range_iter(&[(py::len(&v_post.clone())?).i()?])).into_iter().collect::<Vec<Py>>())], &{ let v_le = v_le.clone(); let v_log_evidences = v_log_evidences.clone(); let v_lse = v_lse.clone(); let v_p = v_p.clone(); let v_post = v_post.clone(); let v_priors = v_priors.clone(); let v_z = v_z.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_i: Py = a.get(0).cloned().unwrap_or(Py::None);
            return Ok(py::getitem(&v_post.clone(), &v_i.clone())?);
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) }, true)?;
    return Ok(Py::dict(vec![(Py::str("posterior"), v_post.clone()), (Py::str("best_model"), py::add(&v_best.clone(), &Py::Int(1i64))?), (Py::str("best_probability"), py::getitem(&v_post.clone(), &v_best.clone())?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_run_morris(mut v_outputs: Py, mut v_delta: Py) -> R {
    let mut v_Y: Py = Py::None;
    let mut v_d: Py = Py::None;
    let mut v_ee: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_mu: Py = Py::None;
    let mut v_mus: Py = Py::None;
    let mut v_r: Py = Py::None;
    let mut v_sig: Py = Py::None;
    v_Y = f__mat(v_outputs.clone(), Py::str("the model outputs"), Py::None, Py::None)?;
    v_d = f__pos(v_delta.clone(), Py::str("the step delta"))?;
    let t3439 = (Py::tuple(vec![py::len(&v_Y.clone())?, py::sub(&py::len(&py::getitem(&v_Y.clone(), &Py::Int(0i64))?)?, &Py::Int(1i64))?])).items()?;
    if t3439.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_r = t3439[0].clone();
    v_k = t3439[1].clone();
    if ({ let t3444 = { let t3440 = v_r.clone(); Py::Bool({ let t3441 = Py::Int(2i64); if py::lt(&t3440, &t3441)? { true } else { false } }) }; if t3444.truthy() { t3444 } else { { let t3442 = v_k.clone(); Py::Bool({ let t3443 = Py::Int(1i64); if py::lt(&t3442, &t3443)? { true } else { false } }) } } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("give at least 2 trajectories, each with k + 1 >= 2 outputs"))));
    }
    v_ee = { let mut out = vec![]; for t3445 in py::range_iter(&[(v_k.clone()).i()?]) { let mut v_j: Py; 
v_j = t3445;
out.push({ let mut out = vec![]; for t3446 in py::range_iter(&[(v_r.clone()).i()?]) { let mut v_t: Py; 
v_t = t3446;
out.push(py::truediv(&py::sub(&py::getitem(&py::getitem(&v_Y.clone(), &v_t.clone())?, &py::add(&v_j.clone(), &Py::Int(1i64))?)?, &py::getitem(&py::getitem(&v_Y.clone(), &v_t.clone())?, &v_j.clone())?)?, &v_d.clone())?); } Py::list(out) }); } Py::list(out) };
    v_mu = { let mut out = vec![]; for t3447 in py::iter(&(v_ee.clone()))? { let mut v_e: Py; 
v_e = t3447;
out.push(f__mean(v_e.clone())?); } Py::list(out) };
    v_mus = { let mut out = vec![]; for t3448 in py::iter(&(v_ee.clone()))? { let mut v_e: Py; 
v_e = t3448;
out.push(f__mean({ let mut out = vec![]; for t3449 in py::iter(&(v_e.clone()))? { let mut v_v: Py; 
v_v = t3449;
out.push(py::abs(&v_v.clone())?); } Py::list(out) })?); } Py::list(out) };
    v_sig = { let mut out = vec![]; for t3450 in py::iter(&(py::zip(&[v_ee.clone(), v_mu.clone()])?))? { let mut v_e: Py; let mut v_m: Py; 
let t3451 = (t3450).items()?;
if t3451.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_e = t3451[0].clone();
v_m = t3451[1].clone();
out.push(py::math1("sqrt", &py::truediv(&py::fsum(&{ let mut out = vec![]; for t3452 in py::iter(&(v_e.clone()))? { let mut v_v: Py; 
v_v = t3452;
out.push(py::pow(&py::sub(&v_v.clone(), &v_m.clone())?, &Py::Int(2i64))?); } Py::list(out) })?, &py::sub(&v_r.clone(), &Py::Int(1i64))?)?)?); } Py::list(out) };
    return Ok(Py::dict(vec![(Py::str("mu"), v_mu.clone()), (Py::str("mu_star"), v_mus.clone()), (Py::str("sigma"), v_sig.clone())]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_run_nig(mut v_mu0: Py, mut v_kappa0: Py, mut v_alpha0: Py, mut v_beta0: Py, mut v_data: Py, mut v_level: Py) -> R {
    let mut v_a0: Py = Py::None;
    let mut v_an: Py = Py::None;
    let mut v_b0: Py = Py::None;
    let mut v_bn: Py = Py::None;
    let mut v_c: Py = Py::None;
    let mut v_k0: Py = Py::None;
    let mut v_kn: Py = Py::None;
    let mut v_m0: Py = Py::None;
    let mut v_mn: Py = Py::None;
    let mut v_n: Py = Py::None;
    let mut v_scale: Py = Py::None;
    let mut v_ss: Py = Py::None;
    let mut v_t: Py = Py::None;
    let mut v_x: Py = Py::None;
    let mut v_xb: Py = Py::None;
    let t3496 = (Py::tuple(vec![f__fin(v_mu0.clone(), Py::str("mu0"))?, f__pos(v_kappa0.clone(), Py::str("kappa0"))?, f__pos(v_alpha0.clone(), Py::str("alpha0"))?, f__pos(v_beta0.clone(), Py::str("beta0"))?])).items()?;
    if t3496.len() != 4 { return Err(PyErr::Value("unpack".into())); }
    v_m0 = t3496[0].clone();
    v_k0 = t3496[1].clone();
    v_a0 = t3496[2].clone();
    v_b0 = t3496[3].clone();
    let t3510 = (Py::tuple(vec![f__vec(v_data.clone(), Py::str("the data"), Py::None, Py::Int(1i64))?, f__level(v_level.clone())?])).items()?;
    if t3510.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_x = t3510[0].clone();
    v_c = t3510[1].clone();
    let t3511 = (Py::tuple(vec![py::len(&v_x.clone())?, f__mean(v_x.clone())?])).items()?;
    if t3511.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_n = t3511[0].clone();
    v_xb = t3511[1].clone();
    v_ss = py::fsum(&{ let mut out = vec![]; for t3512 in py::iter(&(v_x.clone()))? { let mut v_v: Py; 
v_v = t3512;
out.push(py::pow(&py::sub(&v_v.clone(), &v_xb.clone())?, &Py::Int(2i64))?); } Py::list(out) })?;
    let t3513 = (Py::tuple(vec![py::add(&v_k0.clone(), &v_n.clone())?, py::add(&v_a0.clone(), &py::truediv(&v_n.clone(), &Py::Int(2i64))?)?])).items()?;
    if t3513.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_kn = t3513[0].clone();
    v_an = t3513[1].clone();
    v_mn = py::truediv(&py::add(&py::mul(&v_k0.clone(), &v_m0.clone())?, &py::mul(&v_n.clone(), &v_xb.clone())?)?, &v_kn.clone())?;
    v_bn = py::add(&py::add(&v_b0.clone(), &py::truediv(&v_ss.clone(), &Py::Int(2i64))?)?, &py::truediv(&py::mul(&py::mul(&v_k0.clone(), &v_n.clone())?, &py::pow(&py::sub(&v_xb.clone(), &v_m0.clone())?, &Py::Int(2i64))?)?, &py::mul(&Py::Int(2i64), &v_kn.clone())?)?)?;
    v_scale = py::math1("sqrt", &py::truediv(&v_bn.clone(), &py::mul(&v_an.clone(), &v_kn.clone())?)?)?;
    v_t = f__t_ppf(py::mul(&Py::Float(0.5), &py::add(&Py::Int(1i64), &v_c.clone())?)?, py::mul(&Py::Int(2i64), &v_an.clone())?)?;
    return Ok(Py::dict(vec![(Py::str("mu_n"), v_mn.clone()), (Py::str("kappa_n"), v_kn.clone()), (Py::str("alpha_n"), v_an.clone()), (Py::str("beta_n"), v_bn.clone()), (Py::str("sigma2_mean"), (if ({ let t3569 = v_an.clone(); Py::Bool({ let t3570 = Py::Int(1i64); if py::gt(&t3569, &t3570)? { true } else { false } }) }).truthy() { py::truediv(&v_bn.clone(), &py::sub(&v_an.clone(), &Py::Int(1i64))?)? } else { Py::None })), (Py::str("lower"), py::sub(&v_mn.clone(), &py::mul(&v_t.clone(), &v_scale.clone())?)?), (Py::str("upper"), py::add(&v_mn.clone(), &py::mul(&v_t.clone(), &v_scale.clone())?)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_run_nn(mut v_prior_mean: Py, mut v_prior_sd: Py, mut v_sigma: Py, mut v_data: Py) -> R {
    let mut v_m: Py = Py::None;
    let mut v_m0: Py = Py::None;
    let mut v_s: Py = Py::None;
    let mut v_t0: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_x: Py = Py::None;
    let t3614 = (f__nn_check(v_prior_mean.clone(), v_prior_sd.clone(), v_sigma.clone())?).items()?;
    if t3614.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_m0 = t3614[0].clone();
    v_t0 = t3614[1].clone();
    v_s = t3614[2].clone();
    v_x = f__vec(v_data.clone(), Py::str("the data"), Py::None, Py::Int(1i64))?;
    let t3623 = (f__nn_post(v_m0.clone(), v_t0.clone(), v_s.clone(), f__mean(v_x.clone())?, py::len(&v_x.clone())?)?).items()?;
    if t3623.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_m = t3623[0].clone();
    v_v = t3623[1].clone();
    return Ok(Py::dict(vec![(Py::str("posterior_mean"), v_m.clone()), (Py::str("posterior_sd"), py::math1("sqrt", &v_v.clone())?), (Py::str("posterior_variance"), v_v.clone())]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_run_nn_pred(mut v_prior_mean: Py, mut v_prior_sd: Py, mut v_sigma: Py, mut v_sample_mean: Py, mut v_n: Py, mut v_threshold: Py) -> R {
    let mut v_c: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_m: Py = Py::None;
    let mut v_m0: Py = Py::None;
    let mut v_s: Py = Py::None;
    let mut v_sd: Py = Py::None;
    let mut v_t0: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_xb: Py = Py::None;
    let mut v_z: Py = Py::None;
    let t3751 = (f__nn_check(v_prior_mean.clone(), v_prior_sd.clone(), v_sigma.clone())?).items()?;
    if t3751.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_m0 = t3751[0].clone();
    v_t0 = t3751[1].clone();
    v_s = t3751[2].clone();
    let t3757 = (Py::tuple(vec![f__fin(v_sample_mean.clone(), Py::str("the sample mean"))?, f__whole(v_n.clone(), Py::str("the sample size"), Py::Int(1i64))?, f__fin(v_threshold.clone(), Py::str("the threshold"))?])).items()?;
    if t3757.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_xb = t3757[0].clone();
    v_k = t3757[1].clone();
    v_c = t3757[2].clone();
    let t3758 = (f__nn_post(v_m0.clone(), v_t0.clone(), v_s.clone(), v_xb.clone(), v_k.clone())?).items()?;
    if t3758.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_m = t3758[0].clone();
    v_v = t3758[1].clone();
    v_sd = py::math1("sqrt", &py::add(&v_v.clone(), &py::mul(&v_s.clone(), &v_s.clone())?)?)?;
    v_z = py::truediv(&py::sub(&v_c.clone(), &v_m.clone())?, &v_sd.clone())?;
    return Ok(Py::dict(vec![(Py::str("predictive_mean"), v_m.clone()), (Py::str("predictive_sd"), v_sd.clone()), (Py::str("prob_above"), py::mul(&Py::Float(0.5), &py::math1("erfc", &py::truediv(&v_z.clone(), &c__SQRT2()?)?)?)?), (Py::str("prob_below"), py::mul(&Py::Float(0.5), &py::math1("erfc", &py::truediv(&py::neg(&v_z.clone())?, &c__SQRT2()?)?)?)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_run_nn_summary(mut v_prior_mean: Py, mut v_prior_sd: Py, mut v_sigma: Py, mut v_sample_mean: Py, mut v_n: Py, mut v_level: Py) -> R {
    let mut v_c: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_m: Py = Py::None;
    let mut v_m0: Py = Py::None;
    let mut v_s: Py = Py::None;
    let mut v_t0: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_xb: Py = Py::None;
    let mut v_z: Py = Py::None;
    let t3667 = (f__nn_check(v_prior_mean.clone(), v_prior_sd.clone(), v_sigma.clone())?).items()?;
    if t3667.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_m0 = t3667[0].clone();
    v_t0 = t3667[1].clone();
    v_s = t3667[2].clone();
    let t3678 = (Py::tuple(vec![f__fin(v_sample_mean.clone(), Py::str("the sample mean"))?, f__whole(v_n.clone(), Py::str("the sample size"), Py::Int(1i64))?, f__level(v_level.clone())?])).items()?;
    if t3678.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_xb = t3678[0].clone();
    v_k = t3678[1].clone();
    v_c = t3678[2].clone();
    let t3679 = (f__nn_post(v_m0.clone(), v_t0.clone(), v_s.clone(), v_xb.clone(), v_k.clone())?).items()?;
    if t3679.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_m = t3679[0].clone();
    v_v = t3679[1].clone();
    v_z = f__z(py::mul(&Py::Float(0.5), &py::add(&Py::Int(1i64), &v_c.clone())?)?)?;
    return Ok(Py::dict(vec![(Py::str("posterior_mean"), v_m.clone()), (Py::str("posterior_sd"), py::math1("sqrt", &v_v.clone())?), (Py::str("lower"), py::sub(&v_m.clone(), &py::mul(&v_z.clone(), &py::math1("sqrt", &v_v.clone())?)?)?), (Py::str("upper"), py::add(&v_m.clone(), &py::mul(&v_z.clone(), &py::math1("sqrt", &v_v.clone())?)?)?), (Py::str("shrinkage"), py::truediv(&py::truediv(&py::mul(&v_s.clone(), &v_s.clone())?, &v_k.clone())?, &py::add(&py::truediv(&py::mul(&v_s.clone(), &v_s.clone())?, &v_k.clone())?, &py::mul(&v_t0.clone(), &v_t0.clone())?)?)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_run_odds(mut v_probability: Py) -> R {
    let mut v_p: Py = Py::None;
    v_p = f__prob(v_probability.clone(), Py::str("the probability"), Py::Bool(true))?;
    return Ok(Py::dict(vec![(Py::str("odds"), py::truediv(&v_p.clone(), &py::sub(&Py::Float(1.0), &v_p.clone())?)?), (Py::str("odds_against"), py::truediv(&py::sub(&Py::Float(1.0), &v_p.clone())?, &v_p.clone())?), (Py::str("log_odds"), py::sub(&py::math1("log", &v_p.clone())?, &py::math1("log1p", &py::neg(&v_p.clone())?)?)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_run_odds_to_prob(mut v_odds: Py) -> R {
    let mut v_o: Py = Py::None;
    v_o = f__fin(v_odds.clone(), Py::str("the odds"))?;
    if ({ let t3800 = v_o.clone(); Py::Bool({ let t3801 = Py::Int(0i64); if py::lt(&t3800, &t3801)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("odds must not be negative"))));
    }
    return Ok(Py::dict(vec![(Py::str("probability"), py::truediv(&v_o.clone(), &py::add(&Py::Float(1.0), &v_o.clone())?)?), (Py::str("complement"), py::truediv(&Py::Float(1.0), &py::add(&Py::Float(1.0), &v_o.clone())?)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_run_poly_normal(mut v_coefficients: Py, mut v_mean: Py, mut v_sd: Py) -> R {
    let mut v_c: Py = Py::None;
    let mut v_m: Py = Py::None;
    let mut v_mu: Py = Py::None;
    let mut v_q: Py = Py::None;
    let mut v_r: Py = Py::None;
    let mut v_s: Py = Py::None;
    let mut v_var: Py = Py::None;
    v_c = f__vec(v_coefficients.clone(), Py::str("the coefficients"), Py::None, Py::Int(1i64))?;
    if ({ let t3851 = py::len(&v_c.clone())?; Py::Bool({ let t3852 = Py::Int(7i64); if py::gt(&t3851, &t3852)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("at most degree 6 (7 coefficients)"))));
    }
    let t3855 = (Py::tuple(vec![f__fin(v_mean.clone(), Py::str("the mean"))?, f__pos(v_sd.clone(), Py::str("the standard deviation"))?])).items()?;
    if t3855.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_m = t3855[0].clone();
    v_s = t3855[1].clone();
    v_q = { let mut out = vec![]; for t3856 in py::range_iter(&[(py::len(&v_c.clone())?).i()?]) { let mut v_k: Py; 
v_k = t3856;
out.push(py::fsum(&{ let mut out = vec![]; for t3857 in py::range_iter(&[(v_k.clone()).i()?, (py::len(&v_c.clone())?).i()?]) { let mut v_j: Py; 
v_j = t3857;
out.push(py::mul(&py::mul(&py::mul(&py::getitem(&v_c.clone(), &v_j.clone())?, &py::comb(&v_j.clone(), &v_k.clone())?)?, &py::pow(&v_m.clone(), &py::sub(&v_j.clone(), &v_k.clone())?)?)?, &py::pow(&v_s.clone(), &v_k.clone())?)?); } Py::list(out) })?); } Py::list(out) };
    v_mu = py::fsum(&{ let mut out = vec![]; for t3858 in py::range_iter(&[(py::len(&v_q.clone())?).i()?]) { let mut v_k: Py; 
v_k = t3858;
out.push(py::mul(&py::getitem(&v_q.clone(), &v_k.clone())?, &f__z_moment(v_k.clone())?)?); } Py::list(out) })?;
    v_r = py::add(&Py::list(vec![py::sub(&py::getitem(&v_q.clone(), &Py::Int(0i64))?, &v_mu.clone())?]), &py::slice(&v_q.clone(), Some((Py::Int(1i64)).i()?), None, None)?)?;
    v_var = py::fsum(&{ let mut out = vec![]; for t3859 in py::range_iter(&[(py::len(&v_r.clone())?).i()?]) { let mut v_i: Py; 
v_i = t3859;
for t3860 in py::range_iter(&[(py::len(&v_r.clone())?).i()?]) { let mut v_j: Py; 
v_j = t3860;
out.push(py::mul(&py::mul(&py::getitem(&v_r.clone(), &v_i.clone())?, &py::getitem(&v_r.clone(), &v_j.clone())?)?, &f__z_moment(py::add(&v_i.clone(), &v_j.clone())?)?)?); }} Py::list(out) })?;
    return Ok(Py::dict(vec![(Py::str("mean"), v_mu.clone()), (Py::str("variance"), py::minmax(&[Py::Float(0.0), v_var.clone()], true)?), (Py::str("sd"), py::math1("sqrt", &py::minmax(&[Py::Float(0.0), v_var.clone()], true)?)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_run_post_lr(mut v_prior: Py, mut v_likelihood_ratio: Py) -> R {
    let mut v_lr: Py = Py::None;
    let mut v_o: Py = Py::None;
    let mut v_p: Py = Py::None;
    v_p = f__prob(v_prior.clone(), Py::str("the pre-test probability"), Py::Bool(true))?;
    v_lr = f__fin(v_likelihood_ratio.clone(), Py::str("the likelihood ratio"))?;
    if ({ let t3961 = v_lr.clone(); Py::Bool({ let t3962 = Py::Int(0i64); if py::lt(&t3961, &t3962)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("a likelihood ratio must not be negative"))));
    }
    v_o = py::mul(&py::truediv(&v_p.clone(), &py::sub(&Py::Float(1.0), &v_p.clone())?)?, &v_lr.clone())?;
    return Ok(Py::dict(vec![(Py::str("posterior"), py::truediv(&v_o.clone(), &py::add(&Py::Float(1.0), &v_o.clone())?)?), (Py::str("prior_odds"), py::truediv(&v_p.clone(), &py::sub(&Py::Float(1.0), &v_p.clone())?)?), (Py::str("posterior_odds"), v_o.clone())]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_run_prob_beta_greater(mut v_alpha_a: Py, mut v_beta_a: Py, mut v_alpha_b: Py, mut v_beta_b: Py) -> R {
    let mut v_aa: Py = Py::None;
    let mut v_ab: Py = Py::None;
    let mut v_ba: Py = Py::None;
    let mut v_bb: Py = Py::None;
    let mut v_p_b: Py = Py::None;
    let mut v_terms: Py = Py::None;
    let t6 = (Py::tuple(vec![f__pos(v_alpha_a.clone(), Py::str("alpha_a"))?, f__pos(v_beta_a.clone(), Py::str("beta_a"))?, f__pos(v_beta_b.clone(), Py::str("beta_b"))?])).items()?;
    if t6.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_aa = t6[0].clone();
    v_ba = t6[1].clone();
    v_bb = t6[2].clone();
    v_ab = f__whole(v_alpha_b.clone(), Py::str("alpha_b"), Py::Int(1i64))?;
    v_terms = { let mut out = vec![]; for t12 in py::range_iter(&[(v_ab.clone()).i()?]) { let mut v_i: Py; 
v_i = t12;
out.push(py::math1("exp", &py::sub(&py::sub(&py::sub(&f__lbeta(py::add(&v_aa.clone(), &v_i.clone())?, py::add(&v_ba.clone(), &v_bb.clone())?)?, &py::math1("log", &py::add(&v_bb.clone(), &v_i.clone())?)?)?, &f__lbeta(py::add(&Py::Int(1i64), &v_i.clone())?, v_bb.clone())?)?, &f__lbeta(v_aa.clone(), v_ba.clone())?)?)?); } Py::list(out) };
    v_p_b = py::fsum(&v_terms.clone())?;
    return Ok(Py::dict(vec![(Py::str("prob_a_greater"), py::sub(&Py::Float(1.0), &v_p_b.clone())?), (Py::str("prob_b_greater"), v_p_b.clone())]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_run_prod_normals(mut v_mean_x: Py, mut v_sd_x: Py, mut v_mean_y: Py, mut v_sd_y: Py) -> R {
    let mut v_fo: Py = Py::None;
    let mut v_mx: Py = Py::None;
    let mut v_my: Py = Py::None;
    let mut v_sx: Py = Py::None;
    let mut v_sy: Py = Py::None;
    let t4200 = (Py::tuple(vec![f__fin(v_mean_x.clone(), Py::str("mean_x"))?, f__fin(v_mean_y.clone(), Py::str("mean_y"))?])).items()?;
    if t4200.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_mx = t4200[0].clone();
    v_my = t4200[1].clone();
    let t4203 = (Py::tuple(vec![f__pos(v_sd_x.clone(), Py::str("sd_x"))?, f__pos(v_sd_y.clone(), Py::str("sd_y"))?])).items()?;
    if t4203.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_sx = t4203[0].clone();
    v_sy = t4203[1].clone();
    v_fo = py::add(&py::pow(&py::mul(&v_mx.clone(), &v_sy.clone())?, &Py::Int(2i64))?, &py::pow(&py::mul(&v_my.clone(), &v_sx.clone())?, &Py::Int(2i64))?)?;
    return Ok(Py::dict(vec![(Py::str("exact_mean"), py::mul(&v_mx.clone(), &v_my.clone())?), (Py::str("exact_sd"), py::math1("sqrt", &py::add(&v_fo.clone(), &py::pow(&py::mul(&v_sx.clone(), &v_sy.clone())?, &Py::Int(2i64))?)?)?), (Py::str("first_order_sd"), py::math1("sqrt", &v_fo.clone())?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_run_sequential(mut v_prevalence: Py, mut v_sensitivity: Py, mut v_specificity: Py, mut v_positives: Py, mut v_negatives: Py) -> R {
    let mut v_count: Py = Py::None;
    let mut v_den: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_lw: Py = Py::None;
    let mut v_m: Py = Py::None;
    let mut v_num: Py = Py::None;
    let mut v_o: Py = Py::None;
    let mut v_post: Py = Py::None;
    let mut v_pv: Py = Py::None;
    let mut v_se: Py = Py::None;
    let mut v_sp: Py = Py::None;
    let t4300 = (Py::tuple(vec![f__prob(v_prevalence.clone(), Py::str("prevalence"), Py::Bool(true))?, f__prob(v_sensitivity.clone(), Py::str("sensitivity"), Py::Bool(false))?, f__prob(v_specificity.clone(), Py::str("specificity"), Py::Bool(false))?])).items()?;
    if t4300.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_pv = t4300[0].clone();
    v_se = t4300[1].clone();
    v_sp = t4300[2].clone();
    let t4306 = (Py::tuple(vec![f__whole(v_positives.clone(), Py::str("the number of positive results"), Py::Int(0i64))?, f__whole(v_negatives.clone(), Py::str("the number of negative results"), Py::Int(0i64))?])).items()?;
    if t4306.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_k = t4306[0].clone();
    v_m = t4306[1].clone();
    if ({ let t4307 = py::add(&v_k.clone(), &v_m.clone())?; Py::Bool({ let t4308 = Py::Int(1i64); if py::lt(&t4307, &t4308)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("at least one test result is needed"))));
    }
    v_lw = py::sub(&py::math1("log", &v_pv.clone())?, &py::math1("log1p", &py::neg(&v_pv.clone())?)?)?;
    for t4309 in py::iter(&(Py::tuple(vec![Py::tuple(vec![v_k.clone(), v_se.clone(), py::sub(&Py::Float(1.0), &v_sp.clone())?]), Py::tuple(vec![v_m.clone(), py::sub(&Py::Float(1.0), &v_se.clone())?, v_sp.clone()])])))? {
        let t4319 = (t4309).items()?;
        if t4319.len() != 3 { return Err(PyErr::Value("unpack".into())); }
        v_count = t4319[0].clone();
        v_num = t4319[1].clone();
        v_den = t4319[2].clone();
        if (v_count.clone()).truthy() {
            if ({ let t4314 = { let t4310 = v_num.clone(); Py::Bool({ let t4311 = Py::Float(0.0); if py::le(&t4310, &t4311)? { true } else { false } }) }; if !t4314.truthy() { t4314 } else { { let t4312 = v_den.clone(); Py::Bool({ let t4313 = Py::Float(0.0); if py::le(&t4312, &t4313)? { true } else { false } }) } } }).truthy() {
                return Err(py::value_error(py::to_str(&Py::str("this result is impossible with these values"))));
            }
            if ({ let t4315 = v_num.clone(); Py::Bool({ let t4316 = Py::Float(0.0); if py::le(&t4315, &t4316)? { true } else { false } }) }).truthy() {
                return Ok(Py::dict(vec![(Py::str("posterior"), Py::Float(0.0)), (Py::str("posterior_odds"), Py::Float(0.0))]));
            }
            if ({ let t4317 = v_den.clone(); Py::Bool({ let t4318 = Py::Float(0.0); if py::le(&t4317, &t4318)? { true } else { false } }) }).truthy() {
                return Ok(Py::dict(vec![(Py::str("posterior"), Py::Float(1.0)), (Py::str("posterior_odds"), py::float(&Py::str("inf"))?)]));
            }
            v_lw = py::iadd(&v_lw, &py::mul(&v_count.clone(), &py::sub(&py::math1("log", &v_num.clone())?, &py::math1("log", &v_den.clone())?)?)?)?;
        }
    }
    v_o = (if ({ let t4320 = v_lw.clone(); Py::Bool({ let t4321 = Py::Int(700i64); if py::lt(&t4320, &t4321)? { true } else { false } }) }).truthy() { py::math1("exp", &v_lw.clone())? } else { py::float(&Py::str("inf"))? });
    v_post = (if ({ let t4322 = v_lw.clone(); Py::Bool({ let t4323 = py::neg(&Py::Int(700i64))?; if py::gt(&t4322, &t4323)? { true } else { false } }) }).truthy() { py::truediv(&Py::Float(1.0), &py::add(&Py::Float(1.0), &py::math1("exp", &py::neg(&v_lw.clone())?)?)?)? } else { Py::Float(0.0) });
    return Ok(Py::dict(vec![(Py::str("posterior"), v_post.clone()), (Py::str("posterior_odds"), v_o.clone())]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_run_sobol_linear(mut v_coefficients: Py, mut v_sds: Py) -> R {
    let mut v_V: Py = Py::None;
    let mut v_c: Py = Py::None;
    let mut v_parts: Py = Py::None;
    let mut v_s: Py = Py::None;
    v_c = f__vec(v_coefficients.clone(), Py::str("the coefficients"), Py::None, Py::Int(2i64))?;
    v_s = f__vec(v_sds.clone(), Py::str("the input standard deviations"), py::len(&v_c.clone())?, Py::Int(1i64))?;
    if (Py::Bool(({ let mut out = vec![]; for t4523 in py::iter(&(v_s.clone()))? { let mut v_v: Py; 
v_v = t4523;
out.push({ let t4524 = v_v.clone(); Py::Bool({ let t4525 = Py::Int(0i64); if py::le(&t4524, &t4525)? { true } else { false } }) }); } Py::list(out) }).items()?.iter().any(|x| x.truthy()))).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("input standard deviations must be positive"))));
    }
    v_parts = { let mut out = vec![]; for t4526 in py::iter(&(py::zip(&[v_c.clone(), v_s.clone()])?))? { let mut v_a: Py; let mut v_b: Py; 
let t4527 = (t4526).items()?;
if t4527.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_a = t4527[0].clone();
v_b = t4527[1].clone();
out.push(py::pow(&py::mul(&v_a.clone(), &v_b.clone())?, &Py::Int(2i64))?); } Py::list(out) };
    v_V = py::fsum(&v_parts.clone())?;
    if ({ let t4528 = v_V.clone(); Py::Bool({ let t4529 = Py::Int(0i64); if py::eq(&t4528, &t4529) { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the model has zero variance"))));
    }
    return Ok(Py::dict(vec![(Py::str("first_order"), { let mut out = vec![]; for t4530 in py::iter(&(v_parts.clone()))? { let mut v_p: Py; 
v_p = t4530;
out.push(py::truediv(&v_p.clone(), &v_V.clone())?); } Py::list(out) }), (Py::str("variance"), v_V.clone())]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_run_strat(mut v_weights: Py, mut v_means: Py, mut v_sds: Py, mut v_sizes: Py) -> R {
    let mut v_m: Py = Py::None;
    let mut v_n: Py = Py::None;
    let mut v_s: Py = Py::None;
    let mut v_w: Py = Py::None;
    v_w = f__simplex(v_weights.clone(), Py::str("the stratum weights"), Py::None)?;
    v_m = f__vec(v_means.clone(), Py::str("the stratum means"), py::len(&v_w.clone())?, Py::Int(1i64))?;
    v_s = f__vec(v_sds.clone(), Py::str("the stratum standard deviations"), py::len(&v_w.clone())?, Py::Int(1i64))?;
    v_n = f__vec(v_sizes.clone(), Py::str("the stratum sample sizes"), py::len(&v_w.clone())?, Py::Int(1i64))?;
    if ({ let t4594 = Py::Bool(({ let mut out = vec![]; for t4585 in py::iter(&(v_s.clone()))? { let mut v_v: Py; 
v_v = t4585;
out.push({ let t4586 = v_v.clone(); Py::Bool({ let t4587 = Py::Int(0i64); if py::lt(&t4586, &t4587)? { true } else { false } }) }); } Py::list(out) }).items()?.iter().any(|x| x.truthy())); if t4594.truthy() { t4594 } else { Py::Bool(({ let mut out = vec![]; for t4588 in py::iter(&(v_n.clone()))? { let mut v_v: Py; 
v_v = t4588;
out.push({ let t4593 = { let t4589 = v_v.clone(); Py::Bool({ let t4590 = Py::Int(1i64); if py::lt(&t4589, &t4590)? { true } else { false } }) }; if t4593.truthy() { t4593 } else { { let t4591 = v_v.clone(); Py::Bool({ let t4592 = py::int(&v_v.clone())?; if py::ne(&t4591, &t4592) { true } else { false } }) } } }); } Py::list(out) }).items()?.iter().any(|x| x.truthy())) } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("standard deviations must be >= 0 and sample sizes whole numbers >= 1"))));
    }
    return Ok(Py::dict(vec![(Py::str("estimate"), py::fsum(&{ let mut out = vec![]; for t4595 in py::iter(&(py::zip(&[v_w.clone(), v_m.clone()])?))? { let mut v_a: Py; let mut v_b: Py; 
let t4596 = (t4595).items()?;
if t4596.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_a = t4596[0].clone();
v_b = t4596[1].clone();
out.push(py::mul(&v_a.clone(), &v_b.clone())?); } Py::list(out) })?), (Py::str("standard_error"), py::math1("sqrt", &py::fsum(&{ let mut out = vec![]; for t4597 in py::iter(&(py::zip(&[v_w.clone(), v_s.clone(), v_n.clone()])?))? { let mut v_a: Py; let mut v_b: Py; let mut v_c: Py; 
let t4598 = (t4597).items()?;
if t4598.len() != 3 { return Err(PyErr::Value("unpack".into())); }
v_a = t4598[0].clone();
v_b = t4598[1].clone();
v_c = t4598[2].clone();
out.push(py::truediv(&py::pow(&py::mul(&v_a.clone(), &v_b.clone())?, &Py::Int(2i64))?, &v_c.clone())?); } Py::list(out) })?)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_run_succession(mut v_successes: Py, mut v_trials: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_n: Py = Py::None;
    v_n = f__whole(v_trials.clone(), Py::str("the number of trials"), Py::Int(0i64))?;
    v_k = f__whole(v_successes.clone(), Py::str("the number of successes"), Py::Int(0i64))?;
    if ({ let t4250 = v_k.clone(); Py::Bool({ let t4251 = v_n.clone(); if py::gt(&t4250, &t4251)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("successes cannot exceed trials"))));
    }
    return Ok(Py::dict(vec![(Py::str("p_next"), py::truediv(&py::add(&v_k.clone(), &Py::Float(1.0))?, &py::add(&v_n.clone(), &Py::Float(2.0))?)?), (Py::str("mle"), (if (v_n.clone()).truthy() { py::truediv(&v_k.clone(), &v_n.clone())? } else { Py::None }))]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_run_threshold(mut v_loss_fp: Py, mut v_loss_fn: Py, mut v_probability: Py) -> R {
    let mut v_a: Py = Py::None;
    let mut v_b: Py = Py::None;
    let mut v_d: Py = Py::None;
    let mut v_p: Py = Py::None;
    let mut v_t: Py = Py::None;
    let t4702 = (Py::tuple(vec![f__pos(v_loss_fp.clone(), Py::str("the loss of a false positive"))?, f__pos(v_loss_fn.clone(), Py::str("the loss of a false negative"))?])).items()?;
    if t4702.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_a = t4702[0].clone();
    v_b = t4702[1].clone();
    v_t = py::truediv(&v_a.clone(), &py::add(&v_a.clone(), &v_b.clone())?)?;
    v_d = Py::None;
    if ({ let t4703 = v_probability.clone(); Py::Bool({ let t4704 = Py::None; if !py::is(&t4703, &t4704) { true } else { false } }) }).truthy() {
        v_p = f__prob(v_probability.clone(), Py::str("the probability of the condition"), Py::Bool(false))?;
        v_d = (if ({ let t4712 = v_p.clone(); Py::Bool({ let t4713 = v_t.clone(); if py::gt(&t4712, &t4713)? { true } else { false } }) }).truthy() { Py::str("treat") } else { (if ({ let t4714 = v_p.clone(); Py::Bool({ let t4715 = v_t.clone(); if py::lt(&t4714, &t4715)? { true } else { false } }) }).truthy() { Py::str("do not treat") } else { Py::str("indifferent") }) });
    }
    return Ok(Py::dict(vec![(Py::str("threshold"), v_t.clone()), (Py::str("decision"), v_d.clone())]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_run_total_probability(mut v_priors: Py, mut v_conditionals: Py) -> R {
    let mut v_c: Py = Py::None;
    let mut v_joint: Py = Py::None;
    let mut v_p: Py = Py::None;
    v_p = f__simplex(v_priors.clone(), Py::str("the probabilities of the partition"), Py::None)?;
    v_c = f__vec(v_conditionals.clone(), Py::str("the conditional probabilities"), py::len(&v_p.clone())?, Py::Int(1i64))?;
    if (Py::Bool(({ let mut out = vec![]; for t4653 in py::iter(&(v_c.clone()))? { let mut v_v: Py; 
v_v = t4653;
out.push(py::not(&{ let t4654 = Py::Float(0.0); Py::Bool({ let t4655 = v_v.clone(); if py::le(&t4654, &t4655)? { { let t4656 = Py::Float(1.0); if py::le(&t4655, &t4656)? { true } else { false } } } else { false } }) })); } Py::list(out) }).items()?.iter().any(|x| x.truthy()))).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the conditional probabilities must be between 0 and 1"))));
    }
    v_joint = { let mut out = vec![]; for t4657 in py::iter(&(py::zip(&[v_p.clone(), v_c.clone()])?))? { let mut v_a: Py; let mut v_b: Py; 
let t4658 = (t4657).items()?;
if t4658.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_a = t4658[0].clone();
v_b = t4658[1].clone();
out.push(py::mul(&v_a.clone(), &v_b.clone())?); } Py::list(out) };
    return Ok(Py::dict(vec![(Py::str("p_b"), py::fsum(&v_joint.clone())?), (Py::str("joint"), v_joint.clone())]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_run_type_b(mut v_half_width: Py, mut v_distribution: Py) -> R {
    let mut v_a: Py = Py::None;
    let mut v_d: Py = Py::None;
    v_a = f__pos(v_half_width.clone(), Py::str("the half-width"))?;
    if ({ let t4759 = v_distribution.clone(); Py::Bool({ let t4760 = c__TYPEB()?; if !py::contains(&t4760, &t4759)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("distribution must be rectangular, triangular or arcsine"))));
    }
    v_d = py::getitem(&Py::dict(vec![(Py::str("rectangular"), py::math1("sqrt", &Py::Int(3i64))?), (Py::str("triangular"), py::math1("sqrt", &Py::Int(6i64))?), (Py::str("arcsine"), py::math1("sqrt", &Py::Int(2i64))?)]), &v_distribution.clone())?;
    return Ok(Py::dict(vec![(Py::str("standard_uncertainty"), py::truediv(&v_a.clone(), &v_d.clone())?), (Py::str("divisor"), v_d.clone())]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_run_u_function(mut v_x: Py, mut v_ux: Py, mut v_function: Py) -> R {
    let mut v_d: Py = Py::None;
    let mut v_f: Py = Py::None;
    let mut v_u: Py = Py::None;
    let t4804 = (Py::tuple(vec![f__fin(v_x.clone(), Py::str("x"))?, f__nonneg(v_ux.clone(), Py::str("u(x)"))?])).items()?;
    if t4804.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_x = t4804[0].clone();
    v_u = t4804[1].clone();
    if ({ let t4809 = { let t4805 = v_function.clone(); Py::Bool({ let t4806 = Py::tuple(vec![Py::str("ln"), Py::str("log10"), Py::str("sqrt")]); if py::contains(&t4806, &t4805)? { true } else { false } }) }; if !t4809.truthy() { t4809 } else { { let t4807 = v_x.clone(); Py::Bool({ let t4808 = Py::Int(0i64); if py::le(&t4807, &t4808)? { true } else { false } }) } } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str(&[py::format(&v_function.clone(), "")?, " needs x > 0".to_string()].concat()))));
    }
    if ({ let t4814 = { let t4810 = v_function.clone(); Py::Bool({ let t4811 = Py::str("reciprocal"); if py::eq(&t4810, &t4811) { true } else { false } }) }; if !t4814.truthy() { t4814 } else { { let t4812 = v_x.clone(); Py::Bool({ let t4813 = Py::Int(0i64); if py::eq(&t4812, &t4813) { true } else { false } }) } } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("1/x needs x != 0"))));
    }
    if ({ let t4819 = { let t4815 = v_function.clone(); Py::Bool({ let t4816 = Py::str("tan"); if py::eq(&t4815, &t4816) { true } else { false } }) }; if !t4819.truthy() { t4819 } else { { let t4817 = py::abs(&py::math1("cos", &v_x.clone())?)?; Py::Bool({ let t4818 = Py::Float(1e-12); if py::lt(&t4817, &t4818)? { true } else { false } }) } } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("tan is undefined at this x"))));
    }
    if ({ let t4820 = v_function.clone(); Py::Bool({ let t4821 = c__FUNS()?; if !py::contains(&t4821, &t4820)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&py::add(&Py::str("function must be one of "), &py::method(&Py::str(", "), "join", &[c__FUNS()?])?)?)));
    }
    let t4822 = (py::call(&py::getitem(&Py::dict(vec![(Py::str("exp"), { let v_function = v_function.clone(); let v_u = v_u.clone(); let v_ux = v_ux.clone(); let v_x = v_x.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            return Ok(Py::tuple(vec![py::math1("exp", &v_x.clone())?, py::math1("exp", &v_x.clone())?]));
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) }), (Py::str("ln"), { let v_function = v_function.clone(); let v_u = v_u.clone(); let v_ux = v_ux.clone(); let v_x = v_x.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            return Ok(Py::tuple(vec![py::math1("log", &v_x.clone())?, py::truediv(&Py::Int(1i64), &v_x.clone())?]));
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) }), (Py::str("log10"), { let v_function = v_function.clone(); let v_u = v_u.clone(); let v_ux = v_ux.clone(); let v_x = v_x.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            return Ok(Py::tuple(vec![py::math1("log10", &v_x.clone())?, py::truediv(&Py::Int(1i64), &py::mul(&v_x.clone(), &py::math1("log", &Py::Int(10i64))?)?)?]));
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) }), (Py::str("sqrt"), { let v_function = v_function.clone(); let v_u = v_u.clone(); let v_ux = v_ux.clone(); let v_x = v_x.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            return Ok(Py::tuple(vec![py::math1("sqrt", &v_x.clone())?, py::truediv(&Py::Float(0.5), &py::math1("sqrt", &v_x.clone())?)?]));
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) }), (Py::str("sin"), { let v_function = v_function.clone(); let v_u = v_u.clone(); let v_ux = v_ux.clone(); let v_x = v_x.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            return Ok(Py::tuple(vec![py::math1("sin", &v_x.clone())?, py::math1("cos", &v_x.clone())?]));
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) }), (Py::str("cos"), { let v_function = v_function.clone(); let v_u = v_u.clone(); let v_ux = v_ux.clone(); let v_x = v_x.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            return Ok(Py::tuple(vec![py::math1("cos", &v_x.clone())?, py::neg(&py::math1("sin", &v_x.clone())?)?]));
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) }), (Py::str("tan"), { let v_function = v_function.clone(); let v_u = v_u.clone(); let v_ux = v_ux.clone(); let v_x = v_x.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            return Ok(Py::tuple(vec![py::math1("tan", &v_x.clone())?, py::truediv(&Py::Int(1i64), &py::pow(&py::math1("cos", &v_x.clone())?, &Py::Int(2i64))?)?]));
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) }), (Py::str("reciprocal"), { let v_function = v_function.clone(); let v_u = v_u.clone(); let v_ux = v_ux.clone(); let v_x = v_x.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            return Ok(Py::tuple(vec![py::truediv(&Py::Int(1i64), &v_x.clone())?, py::truediv(&py::neg(&Py::Int(1i64))?, &py::pow(&v_x.clone(), &Py::Int(2i64))?)?]));
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) })]), &v_function.clone())?, &[])?).items()?;
    if t4822.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_f = t4822[0].clone();
    v_d = t4822[1].clone();
    return Ok(Py::dict(vec![(Py::str("value"), v_f.clone()), (Py::str("uncertainty"), py::mul(&py::abs(&v_d.clone())?, &v_u.clone())?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_run_u_lincorr(mut v_values: Py, mut v_uncertainties: Py, mut v_correlation: Py, mut v_coefficients: Py) -> R {
    let mut v_R: Py = Py::None;
    let mut v_c: Py = Py::None;
    let mut v_u: Py = Py::None;
    let mut v_var: Py = Py::None;
    let mut v_x: Py = Py::None;
    v_x = f__vec(v_values.clone(), Py::str("the values"), Py::None, Py::Int(1i64))?;
    v_u = f__vec(v_uncertainties.clone(), Py::str("the uncertainties"), py::len(&v_x.clone())?, Py::Int(1i64))?;
    v_c = (if ({ let t4872 = v_coefficients.clone(); Py::Bool({ let t4873 = Py::None; if py::is(&t4872, &t4873) { true } else { false } }) }).truthy() { py::mul(&Py::list(vec![Py::Float(1.0)]), &py::len(&v_x.clone())?)? } else { f__vec(v_coefficients.clone(), Py::str("the coefficients"), py::len(&v_x.clone())?, Py::Int(1i64))? });
    v_R = f__mat(v_correlation.clone(), Py::str("the correlation matrix"), py::len(&v_x.clone())?, py::len(&v_x.clone())?)?;
    if (Py::Bool(({ let mut out = vec![]; for t4893 in py::iter(&(v_u.clone()))? { let mut v_v: Py; 
v_v = t4893;
out.push({ let t4894 = v_v.clone(); Py::Bool({ let t4895 = Py::Int(0i64); if py::lt(&t4894, &t4895)? { true } else { false } }) }); } Py::list(out) }).items()?.iter().any(|x| x.truthy()))).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("uncertainties must not be negative"))));
    }
    if ({ let t4903 = Py::Bool(({ let mut out = vec![]; for t4896 in py::range_iter(&[(py::len(&v_x.clone())?).i()?]) { let mut v_i: Py; 
v_i = t4896;
out.push({ let t4897 = py::abs(&py::sub(&py::getitem(&py::getitem(&v_R.clone(), &v_i.clone())?, &v_i.clone())?, &Py::Int(1i64))?)?; Py::Bool({ let t4898 = Py::Float(1e-12); if py::gt(&t4897, &t4898)? { true } else { false } }) }); } Py::list(out) }).items()?.iter().any(|x| x.truthy())); if t4903.truthy() { t4903 } else { Py::Bool(({ let mut out = vec![]; for t4899 in py::iter(&(v_R.clone()))? { let mut v_r: Py; 
v_r = t4899;
for t4900 in py::iter(&(v_r.clone()))? { let mut v_v: Py; 
v_v = t4900;
out.push({ let t4901 = py::abs(&v_v.clone())?; Py::Bool({ let t4902 = Py::Int(1i64); if py::gt(&t4901, &t4902)? { true } else { false } }) }); }} Py::list(out) }).items()?.iter().any(|x| x.truthy())) } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("a correlation matrix has ones on the diagonal and entries between -1 and 1"))));
    }
    let _ = f__cholesky_ok(v_R.clone(), Py::str("the correlation matrix"))?;
    v_var = py::fsum(&{ let mut out = vec![]; for t4923 in py::range_iter(&[(py::len(&v_x.clone())?).i()?]) { let mut v_i: Py; 
v_i = t4923;
for t4924 in py::range_iter(&[(py::len(&v_x.clone())?).i()?]) { let mut v_j: Py; 
v_j = t4924;
out.push(py::mul(&py::mul(&py::mul(&py::mul(&py::getitem(&v_c.clone(), &v_i.clone())?, &py::getitem(&v_c.clone(), &v_j.clone())?)?, &py::getitem(&v_u.clone(), &v_i.clone())?)?, &py::getitem(&v_u.clone(), &v_j.clone())?)?, &py::getitem(&py::getitem(&v_R.clone(), &v_i.clone())?, &v_j.clone())?)?); }} Py::list(out) })?;
    return Ok(Py::dict(vec![(Py::str("value"), py::fsum(&{ let mut out = vec![]; for t4925 in py::iter(&(py::zip(&[v_c.clone(), v_x.clone()])?))? { let mut v_a: Py; let mut v_b: Py; 
let t4926 = (t4925).items()?;
if t4926.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_a = t4926[0].clone();
v_b = t4926[1].clone();
out.push(py::mul(&v_a.clone(), &v_b.clone())?); } Py::list(out) })?), (Py::str("uncertainty"), py::math1("sqrt", &py::minmax(&[Py::Float(0.0), v_var.clone()], true)?)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_run_u_power(mut v_x: Py, mut v_ux: Py, mut v_exponent: Py) -> R {
    let mut v_n: Py = Py::None;
    let mut v_u: Py = Py::None;
    let mut v_uy: Py = Py::None;
    let mut v_y: Py = Py::None;
    let t4970 = (Py::tuple(vec![f__fin(v_x.clone(), Py::str("x"))?, f__nonneg(v_ux.clone(), Py::str("u(x)"))?, f__fin(v_exponent.clone(), Py::str("the exponent"))?])).items()?;
    if t4970.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_x = t4970[0].clone();
    v_u = t4970[1].clone();
    v_n = t4970[2].clone();
    if ({ let t4975 = { let t4971 = v_x.clone(); Py::Bool({ let t4972 = Py::Int(0i64); if py::eq(&t4971, &t4972) { true } else { false } }) }; if !t4975.truthy() { t4975 } else { { let t4973 = v_n.clone(); Py::Bool({ let t4974 = Py::Int(1i64); if py::lt(&t4973, &t4974)? { true } else { false } }) } } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("x must not be 0 for an exponent below 1"))));
    }
    if ({ let t4980 = { let t4976 = v_x.clone(); Py::Bool({ let t4977 = Py::Int(0i64); if py::lt(&t4976, &t4977)? { true } else { false } }) }; if !t4980.truthy() { t4980 } else { { let t4978 = v_n.clone(); Py::Bool({ let t4979 = py::int(&v_n.clone())?; if py::ne(&t4978, &t4979) { true } else { false } }) } } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("a negative x needs a whole-number exponent"))));
    }
    v_y = py::pow(&v_x.clone(), &v_n.clone())?;
    v_uy = py::mul(&py::abs(&py::mul(&v_n.clone(), &py::pow(&v_x.clone(), &py::sub(&v_n.clone(), &Py::Int(1i64))?)?)?)?, &v_u.clone())?;
    return Ok(Py::dict(vec![(Py::str("value"), v_y.clone()), (Py::str("uncertainty"), v_uy.clone()), (Py::str("relative_uncertainty"), (if ({ let t4981 = v_y.clone(); Py::Bool({ let t4982 = Py::Int(0i64); if py::ne(&t4981, &t4982) { true } else { false } }) }).truthy() { py::truediv(&v_uy.clone(), &py::abs(&v_y.clone())?)? } else { Py::None }))]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_run_u_powerlaw(mut v_coefficient: Py, mut v_values: Py, mut v_exponents: Py, mut v_uncertainties: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_p: Py = Py::None;
    let mut v_rel: Py = Py::None;
    let mut v_u: Py = Py::None;
    let mut v_x: Py = Py::None;
    let mut v_y: Py = Py::None;
    v_k = f__fin(v_coefficient.clone(), Py::str("the coefficient"))?;
    v_x = f__vec(v_values.clone(), Py::str("the values"), Py::None, Py::Int(1i64))?;
    v_p = f__vec(v_exponents.clone(), Py::str("the exponents"), py::len(&v_x.clone())?, Py::Int(1i64))?;
    v_u = f__vec(v_uncertainties.clone(), Py::str("the uncertainties"), py::len(&v_x.clone())?, Py::Int(1i64))?;
    if (Py::Bool(({ let mut out = vec![]; for t5032 in py::iter(&(v_x.clone()))? { let mut v_v: Py; 
v_v = t5032;
out.push({ let t5033 = v_v.clone(); Py::Bool({ let t5034 = Py::Int(0i64); if py::le(&t5033, &t5034)? { true } else { false } }) }); } Py::list(out) }).items()?.iter().any(|x| x.truthy()))).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("a power law needs positive values"))));
    }
    if (Py::Bool(({ let mut out = vec![]; for t5035 in py::iter(&(v_u.clone()))? { let mut v_v: Py; 
v_v = t5035;
out.push({ let t5036 = v_v.clone(); Py::Bool({ let t5037 = Py::Int(0i64); if py::lt(&t5036, &t5037)? { true } else { false } }) }); } Py::list(out) }).items()?.iter().any(|x| x.truthy()))).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("uncertainties must not be negative"))));
    }
    v_y = py::mul(&v_k.clone(), &py::prod(&{ let mut out = vec![]; for t5038 in py::iter(&(py::zip(&[v_x.clone(), v_p.clone()])?))? { let mut v_a: Py; let mut v_b: Py; 
let t5039 = (t5038).items()?;
if t5039.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_a = t5039[0].clone();
v_b = t5039[1].clone();
out.push(py::pow(&v_a.clone(), &v_b.clone())?); } Py::list(out) })?)?;
    v_rel = py::math1("sqrt", &py::fsum(&{ let mut out = vec![]; for t5040 in py::iter(&(py::zip(&[v_x.clone(), v_p.clone(), v_u.clone()])?))? { let mut v_a: Py; let mut v_b: Py; let mut v_s: Py; 
let t5041 = (t5040).items()?;
if t5041.len() != 3 { return Err(PyErr::Value("unpack".into())); }
v_a = t5041[0].clone();
v_b = t5041[1].clone();
v_s = t5041[2].clone();
out.push(py::pow(&py::truediv(&py::mul(&v_b.clone(), &v_s.clone())?, &v_a.clone())?, &Py::Int(2i64))?); } Py::list(out) })?)?;
    return Ok(Py::dict(vec![(Py::str("value"), v_y.clone()), (Py::str("uncertainty"), py::mul(&py::abs(&v_y.clone())?, &v_rel.clone())?), (Py::str("relative_uncertainty"), v_rel.clone())]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_run_u_prodquot(mut v_a: Py, mut v_ua: Py, mut v_b: Py, mut v_ub: Py, mut v_rho: Py) -> R {
    let mut v_r: Py = Py::None;
    let mut v_up: Py = Py::None;
    let mut v_uq: Py = Py::None;
    let t5136 = (Py::tuple(vec![f__fin(v_a.clone(), Py::str("a"))?, f__fin(v_b.clone(), Py::str("b"))?])).items()?;
    if t5136.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_a = t5136[0].clone();
    v_b = t5136[1].clone();
    let t5139 = (Py::tuple(vec![f__nonneg(v_ua.clone(), Py::str("u(a)"))?, f__nonneg(v_ub.clone(), Py::str("u(b)"))?])).items()?;
    if t5139.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_ua = t5139[0].clone();
    v_ub = t5139[1].clone();
    v_r = f__fin((if ({ let t5140 = v_rho.clone(); Py::Bool({ let t5141 = Py::None; if py::is(&t5140, &t5141) { true } else { false } }) }).truthy() { Py::Float(0.0) } else { v_rho.clone() }), Py::str("the correlation"))?;
    if (py::not(&{ let t5142 = py::neg(&Py::Int(1i64))?; Py::Bool({ let t5143 = v_r.clone(); if py::le(&t5142, &t5143)? { { let t5144 = Py::Int(1i64); if py::le(&t5143, &t5144)? { true } else { false } } } else { false } }) })).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("a correlation coefficient lies between -1 and 1"))));
    }
    if ({ let t5145 = v_b.clone(); Py::Bool({ let t5146 = Py::Int(0i64); if py::eq(&t5145, &t5146) { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("b must not be 0 (the quotient a/b)"))));
    }
    v_up = py::math1("sqrt", &py::minmax(&[Py::Float(0.0), py::add(&py::add(&py::pow(&py::mul(&v_b.clone(), &v_ua.clone())?, &Py::Int(2i64))?, &py::pow(&py::mul(&v_a.clone(), &v_ub.clone())?, &Py::Int(2i64))?)?, &py::mul(&py::mul(&py::mul(&py::mul(&py::mul(&Py::Int(2i64), &v_a.clone())?, &v_b.clone())?, &v_r.clone())?, &v_ua.clone())?, &v_ub.clone())?)?], true)?)?;
    v_uq = py::math1("sqrt", &py::minmax(&[Py::Float(0.0), py::sub(&py::add(&py::pow(&py::truediv(&v_ua.clone(), &v_b.clone())?, &Py::Int(2i64))?, &py::pow(&py::truediv(&py::mul(&v_a.clone(), &v_ub.clone())?, &py::pow(&v_b.clone(), &Py::Int(2i64))?)?, &Py::Int(2i64))?)?, &py::mul(&py::mul(&py::mul(&py::mul(&Py::Int(2i64), &py::truediv(&v_a.clone(), &py::pow(&v_b.clone(), &Py::Int(3i64))?)?)?, &v_r.clone())?, &v_ua.clone())?, &v_ub.clone())?)?], true)?)?;
    return Ok(Py::dict(vec![(Py::str("product"), py::mul(&v_a.clone(), &v_b.clone())?), (Py::str("u_product"), v_up.clone()), (Py::str("quotient"), py::truediv(&v_a.clone(), &v_b.clone())?), (Py::str("u_quotient"), v_uq.clone())]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_run_u_sum(mut v_values: Py, mut v_uncertainties: Py, mut v_coefficients: Py) -> R {
    let mut v_c: Py = Py::None;
    let mut v_u: Py = Py::None;
    let mut v_x: Py = Py::None;
    v_x = f__vec(v_values.clone(), Py::str("the values"), Py::None, Py::Int(1i64))?;
    v_u = f__vec(v_uncertainties.clone(), Py::str("the uncertainties"), py::len(&v_x.clone())?, Py::Int(1i64))?;
    v_c = (if ({ let t5196 = v_coefficients.clone(); Py::Bool({ let t5197 = Py::None; if py::is(&t5196, &t5197) { true } else { false } }) }).truthy() { py::mul(&Py::list(vec![Py::Float(1.0)]), &py::len(&v_x.clone())?)? } else { f__vec(v_coefficients.clone(), Py::str("the coefficients"), py::len(&v_x.clone())?, Py::Int(1i64))? });
    if (Py::Bool(({ let mut out = vec![]; for t5198 in py::iter(&(v_u.clone()))? { let mut v_v: Py; 
v_v = t5198;
out.push({ let t5199 = v_v.clone(); Py::Bool({ let t5200 = Py::Int(0i64); if py::lt(&t5199, &t5200)? { true } else { false } }) }); } Py::list(out) }).items()?.iter().any(|x| x.truthy()))).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("standard uncertainties must not be negative"))));
    }
    return Ok(Py::dict(vec![(Py::str("value"), py::fsum(&{ let mut out = vec![]; for t5201 in py::iter(&(py::zip(&[v_c.clone(), v_x.clone()])?))? { let mut v_a: Py; let mut v_b: Py; 
let t5202 = (t5201).items()?;
if t5202.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_a = t5202[0].clone();
v_b = t5202[1].clone();
out.push(py::mul(&v_a.clone(), &v_b.clone())?); } Py::list(out) })?), (Py::str("uncertainty"), py::math1("sqrt", &py::fsum(&{ let mut out = vec![]; for t5203 in py::iter(&(py::zip(&[v_c.clone(), v_u.clone()])?))? { let mut v_a: Py; let mut v_b: Py; 
let t5204 = (t5203).items()?;
if t5204.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_a = t5204[0].clone();
v_b = t5204[1].clone();
out.push(py::pow(&py::mul(&v_a.clone(), &v_b.clone())?, &Py::Int(2i64))?); } Py::list(out) })?)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_run_wmean(mut v_values: Py, mut v_uncertainties: Py) -> R {
    let mut v_chi2: Py = Py::None;
    let mut v_m: Py = Py::None;
    let mut v_sw: Py = Py::None;
    let mut v_u: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_x: Py = Py::None;
    v_x = f__vec(v_values.clone(), Py::str("the measurements"), Py::None, Py::Int(2i64))?;
    v_u = f__vec(v_uncertainties.clone(), Py::str("the uncertainties"), py::len(&v_x.clone())?, Py::Int(1i64))?;
    if (Py::Bool(({ let mut out = vec![]; for t2700 in py::iter(&(v_u.clone()))? { let mut v_v: Py; 
v_v = t2700;
out.push({ let t2701 = v_v.clone(); Py::Bool({ let t2702 = Py::Int(0i64); if py::le(&t2701, &t2702)? { true } else { false } }) }); } Py::list(out) }).items()?.iter().any(|x| x.truthy()))).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("uncertainties must be positive"))));
    }
    v_w = { let mut out = vec![]; for t2703 in py::iter(&(v_u.clone()))? { let mut v_v: Py; 
v_v = t2703;
out.push(py::truediv(&Py::Int(1i64), &py::pow(&v_v.clone(), &Py::Int(2i64))?)?); } Py::list(out) };
    v_sw = py::fsum(&v_w.clone())?;
    v_m = py::truediv(&py::fsum(&{ let mut out = vec![]; for t2704 in py::iter(&(py::zip(&[v_w.clone(), v_x.clone()])?))? { let mut v_a: Py; let mut v_b: Py; 
let t2705 = (t2704).items()?;
if t2705.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_a = t2705[0].clone();
v_b = t2705[1].clone();
out.push(py::mul(&v_a.clone(), &v_b.clone())?); } Py::list(out) })?, &v_sw.clone())?;
    v_chi2 = py::fsum(&{ let mut out = vec![]; for t2706 in py::iter(&(py::zip(&[v_w.clone(), v_x.clone()])?))? { let mut v_a: Py; let mut v_b: Py; 
let t2707 = (t2706).items()?;
if t2707.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_a = t2707[0].clone();
v_b = t2707[1].clone();
out.push(py::mul(&v_a.clone(), &py::pow(&py::sub(&v_b.clone(), &v_m.clone())?, &Py::Int(2i64))?)?); } Py::list(out) })?;
    return Ok(Py::dict(vec![(Py::str("weighted_mean"), v_m.clone()), (Py::str("uncertainty"), py::truediv(&Py::Int(1i64), &py::math1("sqrt", &v_sw.clone())?)?), (Py::str("chi_square"), v_chi2.clone()), (Py::str("birge_ratio"), py::math1("sqrt", &py::truediv(&v_chi2.clone(), &py::sub(&py::len(&v_x.clone())?, &Py::Int(1i64))?)?)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_run_ws(mut v_uncertainties: Py, mut v_dof: Py, mut v_level: Py) -> R {
    let mut v_c: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_nu: Py = Py::None;
    let mut v_u: Py = Py::None;
    let mut v_uc2: Py = Py::None;
    let mut v_v: Py = Py::None;
    v_u = f__vec(v_uncertainties.clone(), Py::str("the uncertainty components"), Py::None, Py::Int(1i64))?;
    v_v = f__vec(v_dof.clone(), Py::str("the degrees of freedom"), py::len(&v_u.clone())?, Py::Int(1i64))?;
    v_c = f__level(v_level.clone())?;
    if ({ let t5265 = Py::Bool(({ let mut out = vec![]; for t5259 in py::iter(&(v_u.clone()))? { let mut v_x: Py; 
v_x = t5259;
out.push({ let t5260 = v_x.clone(); Py::Bool({ let t5261 = Py::Int(0i64); if py::le(&t5260, &t5261)? { true } else { false } }) }); } Py::list(out) }).items()?.iter().any(|x| x.truthy())); if t5265.truthy() { t5265 } else { Py::Bool(({ let mut out = vec![]; for t5262 in py::iter(&(v_v.clone()))? { let mut v_x: Py; 
v_x = t5262;
out.push({ let t5263 = v_x.clone(); Py::Bool({ let t5264 = Py::Int(0i64); if py::le(&t5263, &t5264)? { true } else { false } }) }); } Py::list(out) }).items()?.iter().any(|x| x.truthy())) } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("uncertainty components and degrees of freedom must be positive"))));
    }
    v_uc2 = py::fsum(&{ let mut out = vec![]; for t5266 in py::iter(&(v_u.clone()))? { let mut v_x: Py; 
v_x = t5266;
out.push(py::mul(&v_x.clone(), &v_x.clone())?); } Py::list(out) })?;
    v_nu = py::truediv(&py::pow(&v_uc2.clone(), &Py::Int(2i64))?, &py::fsum(&{ let mut out = vec![]; for t5267 in py::iter(&(py::zip(&[v_u.clone(), v_v.clone()])?))? { let mut v_n: Py; let mut v_x: Py; 
let t5268 = (t5267).items()?;
if t5268.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_x = t5268[0].clone();
v_n = t5268[1].clone();
out.push(py::truediv(&py::pow(&v_x.clone(), &Py::Int(4i64))?, &v_n.clone())?); } Py::list(out) })?)?;
    v_k = f__t_ppf(py::mul(&Py::Float(0.5), &py::add(&Py::Int(1i64), &v_c.clone())?)?, v_nu.clone())?;
    return Ok(Py::dict(vec![(Py::str("combined_uncertainty"), py::math1("sqrt", &v_uc2.clone())?), (Py::str("effective_dof"), v_nu.clone()), (Py::str("coverage_factor"), v_k.clone()), (Py::str("expanded_uncertainty"), py::mul(&v_k.clone(), &py::math1("sqrt", &v_uc2.clone())?)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1(mut v_kw: Py) -> R {
    return Ok(f__r12({ let t20 = v_kw.clone(); f_run_prob_beta_greater(py::getitem(&t20, &Py::str("alpha_a"))?, py::getitem(&t20, &Py::str("beta_a"))?, py::getitem(&t20, &Py::str("alpha_b"))?, py::getitem(&t20, &Py::str("beta_b"))?)? })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1005(mut v_kw: Py) -> R {
    return Ok(f__r12({ let t1027 = v_kw.clone(); f_run_beta_from_moments(py::getitem(&t1027, &Py::str("mean"))?, py::getitem(&t1027, &Py::str("sd"))?)? })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1028(mut v_i: Py, mut v_o: Py, mut v__r: Py) -> R {
    return Ok(f__diff(s_1005(v_i.clone())?, v_o.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1058(mut v_kw: Py) -> R {
    return Ok(f__r12({ let t1097 = v_kw.clone(); f_run_break_even(py::getitem(&t1097, &Py::str("payoffs"))?)? })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1098(mut v_i: Py, mut v_o: Py, mut v__r: Py) -> R {
    return Ok(f__diff(s_1058(v_i.clone())?, v_o.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1128(mut v_kw: Py) -> R {
    return Ok(f__r12({ let t1165 = v_kw.clone(); f_run_cara(py::getitem(&t1165, &Py::str("outcomes"))?, py::getitem(&t1165, &Py::str("probabilities"))?, py::getitem(&t1165, &Py::str("risk_tolerance"))?)? })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1166(mut v_i: Py, mut v_o: Py, mut v__r: Py) -> R {
    return Ok(f__diff(s_1128(v_i.clone())?, v_o.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1196(mut v_kw: Py) -> R {
    return Ok(f__r12({ let t1226 = v_kw.clone(); f_run_log_ce(py::getitem(&t1226, &Py::str("outcomes"))?, py::getitem(&t1226, &Py::str("probabilities"))?)? })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1227(mut v_i: Py, mut v_o: Py, mut v__r: Py) -> R {
    return Ok(f__diff(s_1196(v_i.clone())?, v_o.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1257(mut v_kw: Py) -> R {
    return Ok(f__r12({ let t1285 = v_kw.clone(); f_run_cv(py::getitem(&t1285, &Py::str("y"))?, py::getitem(&t1285, &Py::str("c"))?, py::getitem(&t1285, &Py::str("control_mean"))?)? })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_128(mut v_kw: Py) -> R {
    return Ok(f__r12({ let t154 = v_kw.clone(); f_run_bf_simple(py::getitem(&t154, &Py::str("successes"))?, py::getitem(&t154, &Py::str("trials"))?, py::getitem(&t154, &Py::str("p0"))?, py::getitem(&t154, &Py::str("p1"))?)? })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1286(mut v_i: Py, mut v_o: Py, mut v__r: Py) -> R {
    return Ok(f__diff(s_1257(v_i.clone())?, v_o.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1316(mut v_kw: Py) -> R {
    return Ok(f__r12({ let t1374 = v_kw.clone(); f_run_cov_prop(py::getitem(&t1374, &Py::str("matrix"))?, py::getitem(&t1374, &Py::str("covariance"))?)? })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1375(mut v_i: Py, mut v_o: Py, mut v__r: Py) -> R {
    return Ok(f__diff(s_1316(v_i.clone())?, v_o.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1405(mut v_kw: Py) -> R {
    return Ok(f__r12({ let t1466 = v_kw.clone(); f_run_criteria(py::getitem(&t1466, &Py::str("payoffs"))?, py::getitem(&t1466, &Py::str("criterion"))?, py::method(&t1466, "get", &[Py::str("optimism"), Py::None])?)? })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1467(mut v_i: Py, mut v_o: Py, mut v__r: Py) -> R {
    return Ok(f__diff(s_1405(v_i.clone())?, v_o.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1497(mut v_kw: Py) -> R {
    return Ok(f__r12({ let t1521 = v_kw.clone(); f_run_diagnostic(py::getitem(&t1521, &Py::str("prevalence"))?, py::getitem(&t1521, &Py::str("sensitivity"))?, py::getitem(&t1521, &Py::str("specificity"))?)? })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1522(mut v_i: Py, mut v_o: Py, mut v__r: Py) -> R {
    return Ok(f__diff(s_1497(v_i.clone())?, v_o.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_155(mut v_i: Py, mut v_o: Py, mut v__r: Py) -> R {
    return Ok(f__diff(s_128(v_i.clone())?, v_o.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1552(mut v_kw: Py) -> R {
    return Ok(f__r12({ let t1584 = v_kw.clone(); f_run_dm_evidence(py::getitem(&t1584, &Py::str("alpha"))?, py::getitem(&t1584, &Py::str("counts"))?)? })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1585(mut v_i: Py, mut v_o: Py, mut v__r: Py) -> R {
    return Ok(f__diff(s_1552(v_i.clone())?, v_o.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1615(mut v_kw: Py) -> R {
    return Ok(f__r12({ let t1647 = v_kw.clone(); f_run_dm_post(py::getitem(&t1647, &Py::str("alpha"))?, py::getitem(&t1647, &Py::str("counts"))?)? })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1648(mut v_i: Py, mut v_o: Py, mut v__r: Py) -> R {
    return Ok(f__diff(s_1615(v_i.clone())?, v_o.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1678(mut v_kw: Py) -> R {
    return Ok(f__r12({ let t1709 = v_kw.clone(); f_run_discrete_posterior(py::getitem(&t1709, &Py::str("priors"))?, py::getitem(&t1709, &Py::str("likelihoods"))?)? })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1710(mut v_i: Py, mut v_o: Py, mut v__r: Py) -> R {
    return Ok(f__diff(s_1678(v_i.clone())?, v_o.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1740(mut v_kw: Py) -> R {
    return Ok(f__r12({ let t1786 = v_kw.clone(); f_run_ev_decision(py::getitem(&t1786, &Py::str("payoffs"))?, py::getitem(&t1786, &Py::str("probabilities"))?)? })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1787(mut v_i: Py, mut v_o: Py, mut v__r: Py) -> R {
    return Ok(f__diff(s_1740(v_i.clone())?, v_o.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1817(mut v_kw: Py) -> R {
    return Ok(f__r12({ let t1867 = v_kw.clone(); f_run_evpi(py::getitem(&t1867, &Py::str("payoffs"))?, py::getitem(&t1867, &Py::str("probabilities"))?)? })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_185(mut v_kw: Py) -> R {
    return Ok(f__r12({ let t204 = v_kw.clone(); f_run_bf_normal(py::getitem(&t204, &Py::str("sample_mean"))?, py::getitem(&t204, &Py::str("n"))?, py::getitem(&t204, &Py::str("sigma"))?, py::getitem(&t204, &Py::str("mu0"))?, py::getitem(&t204, &Py::str("prior_sd"))?)? })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1868(mut v_i: Py, mut v_o: Py, mut v__r: Py) -> R {
    return Ok(f__diff(s_1817(v_i.clone())?, v_o.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1898(mut v_kw: Py) -> R {
    return Ok(f__r12({ let t1958 = v_kw.clone(); f_run_evsi(py::getitem(&t1958, &Py::str("payoffs"))?, py::getitem(&t1958, &Py::str("priors"))?, py::getitem(&t1958, &Py::str("likelihoods"))?)? })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1959(mut v_i: Py, mut v_o: Py, mut v__r: Py) -> R {
    return Ok(f__diff(s_1898(v_i.clone())?, v_o.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1989(mut v_kw: Py) -> R {
    return Ok(f__r12({ let t2057 = v_kw.clone(); f_run_gamma_ci(py::getitem(&t2057, &Py::str("shape"))?, py::getitem(&t2057, &Py::str("rate"))?, py::method(&t2057, "get", &[Py::str("level"), Py::None])?)? })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_205(mut v_i: Py, mut v_o: Py, mut v__r: Py) -> R {
    return Ok(f__diff(s_185(v_i.clone())?, v_o.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2058(mut v_i: Py, mut v_o: Py, mut v__r: Py) -> R {
    return Ok(f__diff(s_1989(v_i.clone())?, v_o.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2088(mut v_kw: Py) -> R {
    return Ok(f__r12({ let t2113 = v_kw.clone(); f_run_gexp_post(py::getitem(&t2113, &Py::str("shape"))?, py::getitem(&t2113, &Py::str("rate"))?, py::getitem(&t2113, &Py::str("times"))?)? })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_21(mut v_i: Py, mut v_o: Py, mut v__r: Py) -> R {
    return Ok(f__diff(s_1(v_i.clone())?, v_o.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2114(mut v_i: Py, mut v_o: Py, mut v__r: Py) -> R {
    return Ok(f__diff(s_2088(v_i.clone())?, v_o.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2144(mut v_kw: Py) -> R {
    return Ok(f__r12({ let t2222 = v_kw.clone(); f_run_gamma_hdi(py::getitem(&t2222, &Py::str("shape"))?, py::getitem(&t2222, &Py::str("rate"))?, py::method(&t2222, "get", &[Py::str("level"), Py::None])?)? })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2223(mut v_i: Py, mut v_o: Py, mut v__r: Py) -> R {
    return Ok(f__diff(s_2144(v_i.clone())?, v_o.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2253(mut v_kw: Py) -> R {
    return Ok(f__r12({ let t2337 = v_kw.clone(); f_run_gp_ci(py::getitem(&t2337, &Py::str("shape"))?, py::getitem(&t2337, &Py::str("rate"))?, py::getitem(&t2337, &Py::str("counts"))?, py::method(&t2337, "get", &[Py::str("level"), Py::None])?)? })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2338(mut v_i: Py, mut v_o: Py, mut v__r: Py) -> R {
    return Ok(f__diff(s_2253(v_i.clone())?, v_o.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_235(mut v_kw: Py) -> R {
    return Ok(f__r12({ let t267 = v_kw.clone(); f_run_bf_point_null(py::getitem(&t267, &Py::str("successes"))?, py::getitem(&t267, &Py::str("trials"))?, py::getitem(&t267, &Py::str("p0"))?, py::method(&t267, "get", &[Py::str("alpha"), Py::Float(1.0)])?, py::method(&t267, "get", &[Py::str("beta"), Py::Float(1.0)])?)? })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2368(mut v_kw: Py) -> R {
    return Ok(f__r12({ let t2397 = v_kw.clone(); f_run_gp_post(py::getitem(&t2397, &Py::str("shape"))?, py::getitem(&t2397, &Py::str("rate"))?, py::getitem(&t2397, &Py::str("counts"))?)? })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2398(mut v_i: Py, mut v_o: Py, mut v__r: Py) -> R {
    return Ok(f__diff(s_2368(v_i.clone())?, v_o.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2428(mut v_kw: Py) -> R {
    return Ok(f__r12({ let t2447 = v_kw.clone(); f_run_gp_pred(py::getitem(&t2447, &Py::str("shape"))?, py::getitem(&t2447, &Py::str("rate"))?, py::getitem(&t2447, &Py::str("events"))?)? })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2448(mut v_i: Py, mut v_o: Py, mut v__r: Py) -> R {
    return Ok(f__diff(s_2428(v_i.clone())?, v_o.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2478(mut v_kw: Py) -> R {
    return Ok(f__r12({ let t2533 = v_kw.clone(); f_run_gauss_hermite(py::getitem(&t2533, &Py::str("n"))?)? })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2534(mut v_i: Py, mut v_o: Py, mut v__r: Py) -> R {
    return Ok(f__diff(s_2478(v_i.clone())?, v_o.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2564(mut v_kw: Py) -> R {
    return Ok(f__r12({ let t2591 = v_kw.clone(); f_run_halton(py::getitem(&t2591, &Py::str("index"))?, py::getitem(&t2591, &Py::str("dimension"))?)? })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2592(mut v_i: Py, mut v_o: Py, mut v__r: Py) -> R {
    return Ok(f__diff(s_2564(v_i.clone())?, v_o.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2622(mut v_kw: Py) -> R {
    return Ok(f__r12({ let t2658 = v_kw.clone(); f_run_is(py::getitem(&t2658, &Py::str("values"))?, py::getitem(&t2658, &Py::str("target_density"))?, py::getitem(&t2658, &Py::str("proposal_density"))?)? })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2659(mut v_i: Py, mut v_o: Py, mut v__r: Py) -> R {
    return Ok(f__diff(s_2622(v_i.clone())?, v_o.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_268(mut v_i: Py, mut v_o: Py, mut v__r: Py) -> R {
    return Ok(f__diff(s_235(v_i.clone())?, v_o.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2689(mut v_kw: Py) -> R {
    return Ok(f__r12({ let t2715 = v_kw.clone(); f_run_wmean(py::getitem(&t2715, &Py::str("values"))?, py::getitem(&t2715, &Py::str("uncertainties"))?)? })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2716(mut v_i: Py, mut v_o: Py, mut v__r: Py) -> R {
    return Ok(f__diff(s_2689(v_i.clone())?, v_o.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2746(mut v_kw: Py) -> R {
    return Ok(f__r12({ let t2852 = v_kw.clone(); f_run_jeffreys(py::getitem(&t2852, &Py::str("successes"))?, py::getitem(&t2852, &Py::str("trials"))?, py::method(&t2852, "get", &[Py::str("level"), Py::None])?)? })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2853(mut v_i: Py, mut v_o: Py, mut v__r: Py) -> R {
    return Ok(f__diff(s_2746(v_i.clone())?, v_o.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2883(mut v_kw: Py) -> R {
    return Ok(f__r12({ let t2900 = v_kw.clone(); f_run_kl_beta(py::getitem(&t2900, &Py::str("alpha1"))?, py::getitem(&t2900, &Py::str("beta1"))?, py::getitem(&t2900, &Py::str("alpha2"))?, py::getitem(&t2900, &Py::str("beta2"))?)? })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2901(mut v_i: Py, mut v_o: Py, mut v__r: Py) -> R {
    return Ok(f__diff(s_2883(v_i.clone())?, v_o.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2931(mut v_kw: Py) -> R {
    return Ok(f__r12({ let t2945 = v_kw.clone(); f_run_kl_normal(py::getitem(&t2945, &Py::str("mean1"))?, py::getitem(&t2945, &Py::str("sd1"))?, py::getitem(&t2945, &Py::str("mean2"))?, py::getitem(&t2945, &Py::str("sd2"))?)? })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2946(mut v_i: Py, mut v_o: Py, mut v__r: Py) -> R {
    return Ok(f__diff(s_2931(v_i.clone())?, v_o.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2976(mut v_kw: Py) -> R {
    return Ok(f__r12({ let t3027 = v_kw.clone(); f_run_laplace_beta(py::getitem(&t3027, &Py::str("alpha"))?, py::getitem(&t3027, &Py::str("beta"))?, py::method(&t3027, "get", &[Py::str("level"), Py::None])?)? })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_298(mut v_kw: Py) -> R {
    return Ok(f__r12({ let t318 = v_kw.clone(); f_run_bayes(py::getitem(&t318, &Py::str("p_a"))?, py::getitem(&t318, &Py::str("p_b_given_a"))?, py::getitem(&t318, &Py::str("p_b_given_not_a"))?)? })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3028(mut v_i: Py, mut v_o: Py, mut v__r: Py) -> R {
    return Ok(f__diff(s_2976(v_i.clone())?, v_o.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3058(mut v_kw: Py) -> R {
    return Ok(f__r12({ let t3087 = v_kw.clone(); f_run_lr(py::getitem(&t3087, &Py::str("sensitivity"))?, py::getitem(&t3087, &Py::str("specificity"))?)? })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3088(mut v_i: Py, mut v_o: Py, mut v__r: Py) -> R {
    return Ok(f__diff(s_3058(v_i.clone())?, v_o.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3118(mut v_kw: Py) -> R {
    return Ok(f__r12({ let t3133 = v_kw.clone(); f_run_lognormal(py::getitem(&t3133, &Py::str("mu"))?, py::getitem(&t3133, &Py::str("sigma"))?)? })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3134(mut v_i: Py, mut v_o: Py, mut v__r: Py) -> R {
    return Ok(f__diff(s_3118(v_i.clone())?, v_o.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3164(mut v_kw: Py) -> R {
    return Ok(f__r12({ let t3192 = v_kw.clone(); f_run_map_bb(py::getitem(&t3192, &Py::str("alpha"))?, py::getitem(&t3192, &Py::str("beta"))?, py::getitem(&t3192, &Py::str("successes"))?, py::getitem(&t3192, &Py::str("trials"))?)? })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_319(mut v_i: Py, mut v_o: Py, mut v__r: Py) -> R {
    return Ok(f__diff(s_298(v_i.clone())?, v_o.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3193(mut v_i: Py, mut v_o: Py, mut v__r: Py) -> R {
    return Ok(f__diff(s_3164(v_i.clone())?, v_o.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3223(mut v_kw: Py) -> R {
    return Ok(f__r12({ let t3254 = v_kw.clone(); f_run_map_gp(py::getitem(&t3254, &Py::str("shape"))?, py::getitem(&t3254, &Py::str("rate"))?, py::getitem(&t3254, &Py::str("counts"))?)? })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3255(mut v_i: Py, mut v_o: Py, mut v__r: Py) -> R {
    return Ok(f__diff(s_3223(v_i.clone())?, v_o.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3285(mut v_kw: Py) -> R {
    return Ok(f__r12({ let t3307 = v_kw.clone(); f_run_mc_int(py::getitem(&t3307, &Py::str("values"))?, py::getitem(&t3307, &Py::str("lower"))?, py::getitem(&t3307, &Py::str("upper"))?)? })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3308(mut v_i: Py, mut v_o: Py, mut v__r: Py) -> R {
    return Ok(f__diff(s_3285(v_i.clone())?, v_o.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3338(mut v_kw: Py) -> R {
    return Ok(f__r12({ let t3384 = v_kw.clone(); f_run_mc_n(py::getitem(&t3384, &Py::str("sigma"))?, py::getitem(&t3384, &Py::str("half_width"))?, py::method(&t3384, "get", &[Py::str("level"), Py::None])?)? })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3385(mut v_i: Py, mut v_o: Py, mut v__r: Py) -> R {
    return Ok(f__diff(s_3338(v_i.clone())?, v_o.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3415(mut v_kw: Py) -> R {
    return Ok(f__r12({ let t3460 = v_kw.clone(); f_run_morris(py::getitem(&t3460, &Py::str("outputs"))?, py::getitem(&t3460, &Py::str("delta"))?)? })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3461(mut v_i: Py, mut v_o: Py, mut v__r: Py) -> R {
    return Ok(f__diff(s_3415(v_i.clone())?, v_o.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_349(mut v_kw: Py) -> R {
    return Ok(f__r12({ let t456 = v_kw.clone(); f_run_bb_ci(py::getitem(&t456, &Py::str("alpha"))?, py::getitem(&t456, &Py::str("beta"))?, py::getitem(&t456, &Py::str("successes"))?, py::getitem(&t456, &Py::str("trials"))?, py::method(&t456, "get", &[Py::str("level"), Py::None])?)? })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3491(mut v_kw: Py) -> R {
    return Ok(f__r12({ let t3578 = v_kw.clone(); f_run_nig(py::getitem(&t3578, &Py::str("mu0"))?, py::getitem(&t3578, &Py::str("kappa0"))?, py::getitem(&t3578, &Py::str("alpha0"))?, py::getitem(&t3578, &Py::str("beta0"))?, py::getitem(&t3578, &Py::str("data"))?, py::method(&t3578, "get", &[Py::str("level"), Py::None])?)? })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3579(mut v_i: Py, mut v_o: Py, mut v__r: Py) -> R {
    return Ok(f__diff(s_3491(v_i.clone())?, v_o.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3609(mut v_kw: Py) -> R {
    return Ok(f__r12({ let t3631 = v_kw.clone(); f_run_nn(py::getitem(&t3631, &Py::str("prior_mean"))?, py::getitem(&t3631, &Py::str("prior_sd"))?, py::getitem(&t3631, &Py::str("sigma"))?, py::getitem(&t3631, &Py::str("data"))?)? })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3632(mut v_i: Py, mut v_o: Py, mut v__r: Py) -> R {
    return Ok(f__diff(s_3609(v_i.clone())?, v_o.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3662(mut v_kw: Py) -> R {
    return Ok(f__r12({ let t3715 = v_kw.clone(); f_run_nn_summary(py::getitem(&t3715, &Py::str("prior_mean"))?, py::getitem(&t3715, &Py::str("prior_sd"))?, py::getitem(&t3715, &Py::str("sigma"))?, py::getitem(&t3715, &Py::str("sample_mean"))?, py::getitem(&t3715, &Py::str("n"))?, py::method(&t3715, "get", &[Py::str("level"), Py::None])?)? })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3716(mut v_i: Py, mut v_o: Py, mut v__r: Py) -> R {
    return Ok(f__diff(s_3662(v_i.clone())?, v_o.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3746(mut v_kw: Py) -> R {
    return Ok(f__r12({ let t3766 = v_kw.clone(); f_run_nn_pred(py::getitem(&t3766, &Py::str("prior_mean"))?, py::getitem(&t3766, &Py::str("prior_sd"))?, py::getitem(&t3766, &Py::str("sigma"))?, py::getitem(&t3766, &Py::str("sample_mean"))?, py::getitem(&t3766, &Py::str("n"))?, py::getitem(&t3766, &Py::str("threshold"))?)? })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3767(mut v_i: Py, mut v_o: Py, mut v__r: Py) -> R {
    return Ok(f__diff(s_3746(v_i.clone())?, v_o.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3797(mut v_kw: Py) -> R {
    return Ok(f__r12({ let t3809 = v_kw.clone(); f_run_odds_to_prob(py::getitem(&t3809, &Py::str("odds"))?)? })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3810(mut v_i: Py, mut v_o: Py, mut v__r: Py) -> R {
    return Ok(f__diff(s_3797(v_i.clone())?, v_o.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3840(mut v_kw: Py) -> R {
    return Ok(f__r12({ let t3868 = v_kw.clone(); f_run_poly_normal(py::getitem(&t3868, &Py::str("coefficients"))?, py::getitem(&t3868, &Py::str("mean"))?, py::getitem(&t3868, &Py::str("sd"))?)? })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3869(mut v_i: Py, mut v_o: Py, mut v__r: Py) -> R {
    return Ok(f__diff(s_3840(v_i.clone())?, v_o.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3899(mut v_kw: Py) -> R {
    return Ok(f__r12({ let t3920 = v_kw.clone(); f_run_bf_posterior(py::getitem(&t3920, &Py::str("bayes_factor"))?, py::method(&t3920, "get", &[Py::str("prior"), Py::Float(0.5)])?)? })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3921(mut v_i: Py, mut v_o: Py, mut v__r: Py) -> R {
    return Ok(f__diff(s_3899(v_i.clone())?, v_o.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3951(mut v_kw: Py) -> R {
    return Ok(f__r12({ let t3970 = v_kw.clone(); f_run_post_lr(py::getitem(&t3970, &Py::str("prior"))?, py::getitem(&t3970, &Py::str("likelihood_ratio"))?)? })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3971(mut v_i: Py, mut v_o: Py, mut v__r: Py) -> R {
    return Ok(f__diff(s_3951(v_i.clone())?, v_o.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4001(mut v_kw: Py) -> R {
    return Ok(f__r12({ let t4048 = v_kw.clone(); f_run_model_probs(py::getitem(&t4048, &Py::str("log_evidences"))?, py::method(&t4048, "get", &[Py::str("priors"), Py::None])?)? })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4049(mut v_i: Py, mut v_o: Py, mut v__r: Py) -> R {
    return Ok(f__diff(s_4001(v_i.clone())?, v_o.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4079(mut v_kw: Py) -> R {
    return Ok(f__r12({ let t4118 = v_kw.clone(); f_run_gamma_compare(py::getitem(&t4118, &Py::str("shape1"))?, py::getitem(&t4118, &Py::str("rate1"))?, py::getitem(&t4118, &Py::str("shape2"))?, py::getitem(&t4118, &Py::str("rate2"))?)? })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4119(mut v_i: Py, mut v_o: Py, mut v__r: Py) -> R {
    return Ok(f__diff(s_4079(v_i.clone())?, v_o.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4149(mut v_kw: Py) -> R {
    return Ok(f__r12({ let t4166 = v_kw.clone(); f_run_odds(py::getitem(&t4166, &Py::str("probability"))?)? })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4167(mut v_i: Py, mut v_o: Py, mut v__r: Py) -> R {
    return Ok(f__diff(s_4149(v_i.clone())?, v_o.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4197(mut v_kw: Py) -> R {
    return Ok(f__r12({ let t4211 = v_kw.clone(); f_run_prod_normals(py::getitem(&t4211, &Py::str("mean_x"))?, py::getitem(&t4211, &Py::str("sd_x"))?, py::getitem(&t4211, &Py::str("mean_y"))?, py::getitem(&t4211, &Py::str("sd_y"))?)? })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4212(mut v_i: Py, mut v_o: Py, mut v__r: Py) -> R {
    return Ok(f__diff(s_4197(v_i.clone())?, v_o.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4242(mut v_kw: Py) -> R {
    return Ok(f__r12({ let t4259 = v_kw.clone(); f_run_succession(py::getitem(&t4259, &Py::str("successes"))?, py::getitem(&t4259, &Py::str("trials"))?)? })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4260(mut v_i: Py, mut v_o: Py, mut v__r: Py) -> R {
    return Ok(f__diff(s_4242(v_i.clone())?, v_o.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4290(mut v_kw: Py) -> R {
    return Ok(f__r12({ let t4331 = v_kw.clone(); f_run_sequential(py::getitem(&t4331, &Py::str("prevalence"))?, py::getitem(&t4331, &Py::str("sensitivity"))?, py::getitem(&t4331, &Py::str("specificity"))?, py::getitem(&t4331, &Py::str("positives"))?, py::method(&t4331, "get", &[Py::str("negatives"), Py::Int(0i64)])?)? })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4332(mut v_i: Py, mut v_o: Py, mut v__r: Py) -> R {
    return Ok(f__diff(s_4290(v_i.clone())?, v_o.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4362(mut v_kw: Py) -> R {
    return Ok(f__r12({ let t4387 = v_kw.clone(); f_run_gfun(py::getitem(&t4387, &Py::str("a"))?)? })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4388(mut v_i: Py, mut v_o: Py, mut v__r: Py) -> R {
    return Ok(f__diff(s_4362(v_i.clone())?, v_o.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4418(mut v_kw: Py) -> R {
    return Ok(f__r12({ let t4436 = v_kw.clone(); f_run_bilinear(py::getitem(&t4436, &Py::str("a1"))?, py::getitem(&t4436, &Py::str("a2"))?, py::getitem(&t4436, &Py::str("a12"))?, py::getitem(&t4436, &Py::str("mean1"))?, py::getitem(&t4436, &Py::str("sd1"))?, py::getitem(&t4436, &Py::str("mean2"))?, py::getitem(&t4436, &Py::str("sd2"))?)? })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4437(mut v_i: Py, mut v_o: Py, mut v__r: Py) -> R {
    return Ok(f__diff(s_4418(v_i.clone())?, v_o.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4467(mut v_kw: Py) -> R {
    return Ok(f__r12({ let t4481 = v_kw.clone(); f_run_ishigami(py::getitem(&t4481, &Py::str("a"))?, py::getitem(&t4481, &Py::str("b"))?)? })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4482(mut v_i: Py, mut v_o: Py, mut v__r: Py) -> R {
    return Ok(f__diff(s_4467(v_i.clone())?, v_o.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4512(mut v_kw: Py) -> R {
    return Ok(f__r12({ let t4538 = v_kw.clone(); f_run_sobol_linear(py::getitem(&t4538, &Py::str("coefficients"))?, py::getitem(&t4538, &Py::str("sds"))?)? })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4539(mut v_i: Py, mut v_o: Py, mut v__r: Py) -> R {
    return Ok(f__diff(s_4512(v_i.clone())?, v_o.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4569(mut v_kw: Py) -> R {
    return Ok(f__r12({ let t4606 = v_kw.clone(); f_run_strat(py::getitem(&t4606, &Py::str("weights"))?, py::getitem(&t4606, &Py::str("means"))?, py::getitem(&t4606, &Py::str("sds"))?, py::getitem(&t4606, &Py::str("sizes"))?)? })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_457(mut v_i: Py, mut v_o: Py, mut v__r: Py) -> R {
    return Ok(f__diff(s_349(v_i.clone())?, v_o.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4607(mut v_i: Py, mut v_o: Py, mut v__r: Py) -> R {
    return Ok(f__diff(s_4569(v_i.clone())?, v_o.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4637(mut v_kw: Py) -> R {
    return Ok(f__r12({ let t4666 = v_kw.clone(); f_run_total_probability(py::getitem(&t4666, &Py::str("priors"))?, py::getitem(&t4666, &Py::str("conditionals"))?)? })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4667(mut v_i: Py, mut v_o: Py, mut v__r: Py) -> R {
    return Ok(f__diff(s_4637(v_i.clone())?, v_o.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4697(mut v_kw: Py) -> R {
    return Ok(f__r12({ let t4723 = v_kw.clone(); f_run_threshold(py::getitem(&t4723, &Py::str("loss_fp"))?, py::getitem(&t4723, &Py::str("loss_fn"))?, py::method(&t4723, "get", &[Py::str("probability"), Py::None])?)? })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4724(mut v_i: Py, mut v_o: Py, mut v__r: Py) -> R {
    return Ok(f__diff(s_4697(v_i.clone())?, v_o.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4754(mut v_kw: Py) -> R {
    return Ok(f__r12({ let t4768 = v_kw.clone(); f_run_type_b(py::getitem(&t4768, &Py::str("half_width"))?, py::getitem(&t4768, &Py::str("distribution"))?)? })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4769(mut v_i: Py, mut v_o: Py, mut v__r: Py) -> R {
    return Ok(f__diff(s_4754(v_i.clone())?, v_o.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4799(mut v_kw: Py) -> R {
    return Ok(f__r12({ let t4830 = v_kw.clone(); f_run_u_function(py::getitem(&t4830, &Py::str("x"))?, py::getitem(&t4830, &Py::str("ux"))?, py::getitem(&t4830, &Py::str("function"))?)? })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4831(mut v_i: Py, mut v_o: Py, mut v__r: Py) -> R {
    return Ok(f__diff(s_4799(v_i.clone())?, v_o.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4861(mut v_kw: Py) -> R {
    return Ok(f__r12({ let t4934 = v_kw.clone(); f_run_u_lincorr(py::getitem(&t4934, &Py::str("values"))?, py::getitem(&t4934, &Py::str("uncertainties"))?, py::getitem(&t4934, &Py::str("correlation"))?, py::method(&t4934, "get", &[Py::str("coefficients"), Py::None])?)? })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_487(mut v_kw: Py) -> R {
    return Ok(f__r12({ let t509 = v_kw.clone(); f_run_bb_evidence(py::getitem(&t509, &Py::str("alpha"))?, py::getitem(&t509, &Py::str("beta"))?, py::getitem(&t509, &Py::str("successes"))?, py::getitem(&t509, &Py::str("trials"))?)? })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4935(mut v_i: Py, mut v_o: Py, mut v__r: Py) -> R {
    return Ok(f__diff(s_4861(v_i.clone())?, v_o.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4965(mut v_kw: Py) -> R {
    return Ok(f__r12({ let t4990 = v_kw.clone(); f_run_u_power(py::getitem(&t4990, &Py::str("x"))?, py::getitem(&t4990, &Py::str("ux"))?, py::getitem(&t4990, &Py::str("exponent"))?)? })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4991(mut v_i: Py, mut v_o: Py, mut v__r: Py) -> R {
    return Ok(f__diff(s_4965(v_i.clone())?, v_o.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_5021(mut v_kw: Py) -> R {
    return Ok(f__r12({ let t5049 = v_kw.clone(); f_run_u_powerlaw(py::getitem(&t5049, &Py::str("coefficient"))?, py::getitem(&t5049, &Py::str("values"))?, py::getitem(&t5049, &Py::str("exponents"))?, py::getitem(&t5049, &Py::str("uncertainties"))?)? })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_5050(mut v_i: Py, mut v_o: Py, mut v__r: Py) -> R {
    return Ok(f__diff(s_5021(v_i.clone())?, v_o.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_5080(mut v_kw: Py) -> R {
    return Ok(f__r12({ let t5101 = v_kw.clone(); f_run_u_prodquot(py::getitem(&t5101, &Py::str("a"))?, py::getitem(&t5101, &Py::str("ua"))?, py::getitem(&t5101, &Py::str("b"))?, py::getitem(&t5101, &Py::str("ub"))?, py::method(&t5101, "get", &[Py::str("rho"), Py::Float(0.0)])?)? })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_51(mut v_kw: Py) -> R {
    return Ok(f__r12({ let t97 = v_kw.clone(); f_run_bayes_action(py::getitem(&t97, &Py::str("losses"))?, py::getitem(&t97, &Py::str("probabilities"))?)? })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_510(mut v_i: Py, mut v_o: Py, mut v__r: Py) -> R {
    return Ok(f__diff(s_487(v_i.clone())?, v_o.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_5102(mut v_i: Py, mut v_o: Py, mut v__r: Py) -> R {
    return Ok(f__diff(s_5080(v_i.clone())?, v_o.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_5132(mut v_kw: Py) -> R {
    return Ok(f__r12({ let t5154 = v_kw.clone(); s_5133(py::getitem(&t5154, &Py::str("a"))?, py::getitem(&t5154, &Py::str("ua"))?, py::getitem(&t5154, &Py::str("b"))?, py::getitem(&t5154, &Py::str("ub"))?)? })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_5133(mut v_a: Py, mut v_ua: Py, mut v_b: Py, mut v_ub: Py) -> R {
    return Ok(f_run_u_prodquot(v_a.clone(), v_ua.clone(), v_b.clone(), v_ub.clone(), Py::Float(0.0))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_5155(mut v_i: Py, mut v_o: Py, mut v__r: Py) -> R {
    return Ok(f__diff(s_5132(v_i.clone())?, v_o.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_5185(mut v_kw: Py) -> R {
    return Ok(f__r12({ let t5212 = v_kw.clone(); f_run_u_sum(py::getitem(&t5212, &Py::str("values"))?, py::getitem(&t5212, &Py::str("uncertainties"))?, py::method(&t5212, "get", &[Py::str("coefficients"), Py::None])?)? })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_5213(mut v_i: Py, mut v_o: Py, mut v__r: Py) -> R {
    return Ok(f__diff(s_5185(v_i.clone())?, v_o.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_5243(mut v_kw: Py) -> R {
    return Ok(f__r12({ let t5331 = v_kw.clone(); f_run_ws(py::getitem(&t5331, &Py::str("uncertainties"))?, py::getitem(&t5331, &Py::str("dof"))?, py::method(&t5331, "get", &[Py::str("level"), Py::None])?)? })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_5332(mut v_i: Py, mut v_o: Py, mut v__r: Py) -> R {
    return Ok(f__diff(s_5243(v_i.clone())?, v_o.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_540(mut v_kw: Py) -> R {
    return Ok(f__r12({ let t563 = v_kw.clone(); f_run_bb_post(py::getitem(&t563, &Py::str("alpha"))?, py::getitem(&t563, &Py::str("beta"))?, py::getitem(&t563, &Py::str("successes"))?, py::getitem(&t563, &Py::str("trials"))?)? })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_564(mut v_i: Py, mut v_o: Py, mut v__r: Py) -> R {
    return Ok(f__diff(s_540(v_i.clone())?, v_o.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_594(mut v_kw: Py) -> R {
    return Ok(f__r12({ let t617 = v_kw.clone(); f_run_bb_pred(py::getitem(&t617, &Py::str("alpha"))?, py::getitem(&t617, &Py::str("beta"))?, py::getitem(&t617, &Py::str("future_trials"))?, py::getitem(&t617, &Py::str("future_successes"))?)? })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_618(mut v_i: Py, mut v_o: Py, mut v__r: Py) -> R {
    return Ok(f__diff(s_594(v_i.clone())?, v_o.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_648(mut v_kw: Py) -> R {
    return Ok(f__r12({ let t745 = v_kw.clone(); f_run_beta_ci(py::getitem(&t745, &Py::str("alpha"))?, py::getitem(&t745, &Py::str("beta"))?, py::method(&t745, "get", &[Py::str("level"), Py::None])?)? })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_746(mut v_i: Py, mut v_o: Py, mut v__r: Py) -> R {
    return Ok(f__diff(s_648(v_i.clone())?, v_o.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_776(mut v_kw: Py) -> R {
    return Ok(f__r12({ let t888 = v_kw.clone(); f_run_beta_hdi(py::getitem(&t888, &Py::str("alpha"))?, py::getitem(&t888, &Py::str("beta"))?, py::method(&t888, "get", &[Py::str("level"), Py::None])?)? })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_889(mut v_i: Py, mut v_o: Py, mut v__r: Py) -> R {
    return Ok(f__diff(s_776(v_i.clone())?, v_o.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_919(mut v_kw: Py) -> R {
    return Ok(f__r12({ let t974 = v_kw.clone(); f_run_beta_tail(py::getitem(&t974, &Py::str("alpha"))?, py::getitem(&t974, &Py::str("beta"))?, py::getitem(&t974, &Py::str("threshold"))?, py::method(&t974, "get", &[Py::str("successes"), Py::Int(0i64)])?, py::method(&t974, "get", &[Py::str("trials"), Py::Int(0i64)])?)? })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_975(mut v_i: Py, mut v_o: Py, mut v__r: Py) -> R {
    return Ok(f__diff(s_919(v_i.clone())?, v_o.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_98(mut v_i: Py, mut v_o: Py, mut v__r: Py) -> R {
    return Ok(f__diff(s_51(v_i.clone())?, v_o.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

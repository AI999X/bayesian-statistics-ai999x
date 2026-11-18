//! Bayesian Statistics: 15 calculations through the Rust library (the same engine as the desktop app, the
//! command line and the browser). Run from the rust/ folder:
//!     cargo run --release --example readme -p bayesian-statistics-ai999x
//! Inputs are text, read like a form field ("2 m", "0.5", "x^3 - 3x"); a JSON list stays a list.

use serde_json::{json, Value};

/// One calculation: the request in, its outcome and every value with its unit out.
fn show(request: Value) {
    let answer: Value = serde_json::from_str(&bayesian_statistics_ai999x::solve_json(&request.to_string())).unwrap();
    println!("{}", answer["outcome"].as_str().unwrap_or(""));
    for v in answer["values"].as_array().into_iter().flatten() {
        let unit = v[2].as_str().unwrap_or("");
        let unit = if unit.is_empty() || unit == "1" { String::new() } else { format!(" {unit}") };
        println!("{} = {}{}", v[0].as_str().unwrap_or(""), v[1], unit);
    }
}

fn main() {
    show(json!({"key": "bayes_factor_normal_mean", "inputs": {"sample_mean": "0", "n": "1", "sigma": "1", "mu0": "0", "prior_sd": "1"}}));
    println!();
    show(json!({"key": "bayes_theorem", "inputs": {"p_a": "0.3", "p_b_given_a": "0.8", "p_b_given_not_a": "0.1"}}));
    println!();
    show(json!({"key": "ab_test_probability_beta", "inputs": {"alpha_a": "1", "beta_a": "1", "alpha_b": "1", "beta_b": "1"}}));
    println!();
    show(json!({"key": "gamma_poisson_credible_interval", "inputs": {"counts": [0], "shape": "1", "rate": "1", "level": "0.9"}}));
    println!();
    show(json!({"key": "bayes_action_minimum_expected_loss", "inputs": {"losses": [[0, 10], [5, 0]], "probabilities": [0.3, 0.7]}}));
    println!();
    show(json!({"key": "gauss_hermite_rule", "inputs": {"n": "3"}}));
    println!();
    show(json!({"key": "control_variate_estimate", "inputs": {"y": [1, 2, 3], "c": [1, 2, 3], "control_mean": "2.5"}}));
    println!();
    show(json!({"key": "likelihood_ratios", "inputs": {"sensitivity": "0.9", "specificity": "0.8"}}));
    println!();
    show(json!({"key": "laplace_approximation_beta", "inputs": {"alpha": "3", "beta": "3", "level": "0.95"}}));
    println!();
    show(json!({"key": "beta_prior_from_mean_sd", "inputs": {"mean": "0.3", "sd": "0.1"}}));
    println!();
    show(json!({"key": "morris_elementary_effects", "inputs": {"outputs": [[10, 12], [11, 10]], "delta": "1"}}));
    println!();
    show(json!({"key": "covariance_propagation_linear", "inputs": {"matrix": [[1, 1], [1, -1]], "covariance": [[0.04, 0.01], [0.01, 0.09]]}}));
    println!();
    show(json!({"key": "bayes_factor_point_null_binomial", "inputs": {"p0": "0.5", "successes": "2", "trials": "2", "alpha": "1", "beta": "1"}}));
    println!();
    show(json!({"key": "total_probability", "inputs": {"priors": [0.5, 0.3, 0.2], "conditionals": [0.01, 0.02, 0.05]}}));
    println!();
    show(json!({"key": "beta_binomial_posterior", "inputs": {"alpha": "2", "beta": "2", "successes": "7", "trials": "10"}}));
}

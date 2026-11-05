//! Every Bayesian Statistics capability's live figures (src/figures.rs, src/figures_more.rs, plus the generic
//! engine): its worked example gives each published figure kind, a changed input changes the model's own drawings,
//! and no drawing holds a non-finite number or text outside its canvas.

use ax_engine::figures::{figures, Figure};
use ax_engine::Input;
use bayesian_statistics_ai999x::pack;

/// The published figure kinds of every capability (publish/website/models/bayesian-statistics-ai999x/figures,
/// except the generic "curve"), as tools/figure_parity.py lists them.
const KINDS: &[(&str, &[&str])] = &[
    ("ab_test_probability_beta", &["density"]),
    ("bayes_action_minimum_expected_loss", &["lines", "matrix"]),
    ("bayes_factor_binomial_simple", &["stems"]),
    ("bayes_factor_point_null_binomial", &["stems"]),
    ("bayes_theorem", &["area"]),
    ("beta_binomial_credible_interval", &["intervals"]),
    ("beta_binomial_evidence", &["stems"]),
    ("beta_binomial_posterior", &["density", "heatmap"]),
    ("beta_binomial_predictive", &["stems"]),
    ("beta_credible_interval", &["density"]),
    ("beta_hdi", &["density"]),
    ("beta_posterior_tail_probability", &["density"]),
    ("beta_prior_from_mean_sd", &["density"]),
    ("break_even_probability", &["lines"]),
    ("certainty_equivalent_exponential_utility", &["utility"]),
    ("certainty_equivalent_log_utility", &["utility"]),
    ("control_variate_estimate", &["fit"]),
    ("covariance_propagation_linear", &["ellipses", "matrix"]),
    ("decision_criteria_under_uncertainty", &["table"]),
    ("diagnostic_test_predictive_values", &["curves"]),
    ("dirichlet_multinomial_evidence", &["bars"]),
    ("dirichlet_multinomial_posterior", &["bars", "lines"]),
    ("discrete_bayes_posterior", &["bars", "lines"]),
    ("expected_value_decision", &["lines", "matrix"]),
    ("expected_value_of_perfect_information", &["band"]),
    ("expected_value_of_sample_information", &["bars", "matrix"]),
    ("gamma_credible_interval", &["density"]),
    ("gamma_exponential_posterior", &["density"]),
    ("gamma_hdi", &["density"]),
    ("gamma_poisson_credible_interval", &["intervals"]),
    ("gamma_poisson_posterior", &["density"]),
    ("gamma_poisson_predictive", &["stems"]),
    ("gauss_hermite_rule", &["stems"]),
    ("halton_point", &["points"]),
    ("importance_sampling_estimate", &["bars"]),
    ("inverse_variance_weighted_mean", &["intervals"]),
    ("jeffreys_interval", &["intervals"]),
    ("kl_divergence_beta", &["density"]),
    ("kl_divergence_normal", &["density"]),
    ("laplace_approximation_beta", &["density"]),
    ("likelihood_ratios", &["curves"]),
    ("lognormal_propagation", &["curves"]),
    ("map_beta_binomial", &["density"]),
    ("map_gamma_poisson", &["density"]),
    ("monte_carlo_integration", &["field"]),
    ("morris_elementary_effects", &["matrix", "points"]),
    ("normal_inverse_gamma_posterior", &["density"]),
    ("normal_normal_posterior", &["density"]),
    ("normal_normal_summary", &["band"]),
    ("normal_posterior_predictive", &["density"]),
    ("polynomial_of_normal_moments", &["field"]),
    ("posterior_from_bayes_factor", &["field"]),
    ("posterior_from_likelihood_ratio", &["field"]),
    ("posterior_model_probabilities", &["curves"]),
    ("probability_rate_greater_gamma", &["density"]),
    ("product_of_normals_exact_moments", &["curves"]),
    ("sequential_test_posterior", &["stems"]),
    ("sobol_indices_g_function", &["io"]),
    ("sobol_indices_interaction_model", &["bars"]),
    ("sobol_indices_ishigami", &["bars"]),
    ("sobol_indices_linear_model", &["bars"]),
    ("stratified_sampling_estimate", &["intervals"]),
    ("total_probability", &["bars"]),
    ("treatment_threshold", &["decision"]),
    ("type_b_standard_uncertainty", &["bars"]),
    ("uncertainty_function", &["propagation"]),
    ("uncertainty_power", &["propagation"]),
    ("uncertainty_power_law", &["bars"]),
    ("uncertainty_product_correlated", &["curves"]),
    ("uncertainty_product_quotient", &["bars"]),
    ("uncertainty_sum", &["intervals"]),
    ("welch_satterthwaite_expanded_uncertainty", &["curves"]),
];

/// Kinds the generic engine draws for these capabilities (no figure of the model's own): checked for presence only.
const GENERIC: &[(&str, &str)] = &[
    ("covariance_propagation_linear", "matrix"),
    ("morris_elementary_effects", "points"),
    ("treatment_threshold", "decision"),
    ("type_b_standard_uncertainty", "bars"),
];

/// Inputs to change for a capability whose inputs are not plain quantities (a second, different example).
const OTHER: &[(&str, &str, &str)] = &[
    ("bayes_action_minimum_expected_loss", "probabilities", "0.6, 0.4"),
    ("expected_value_decision", "probabilities", "0.7, 0.3"),
    ("expected_value_of_perfect_information", "probabilities", "0.7, 0.3"),
    ("expected_value_of_sample_information", "priors", "0.7, 0.3"),
    ("discrete_bayes_posterior", "priors", "0.2, 0.3, 0.5"),
    ("total_probability", "priors", "0.2, 0.3, 0.5"),
    ("decision_criteria_under_uncertainty", "payoffs", "[[90, -10], [40, 30], [60, 15]]"),
    ("sobol_indices_g_function", "a", "1, 2"),
    ("posterior_model_probabilities", "log_evidences", "-1, -2.5"),
    ("gauss_hermite_rule", "n", "3"),
    ("halton_point", "index", "7"),
    ("sequential_test_posterior", "positives", "3"),
    ("certainty_equivalent_exponential_utility", "outcomes", "800, 100"),
    ("certainty_equivalent_log_utility", "outcomes", "50, 400"),
];

/// The arguments of a structured example: solve('key', a=1, b='0.3, 0.7').
fn example(key: &str) -> Vec<(String, Input)> {
    let p = pack();
    let ex = &p.procedure(key).expect(key).structured_example;
    let cs: Vec<char> = ex.chars().collect();
    let mut i = ex.find(',').map(|x| ex[..x].chars().count() + 1).unwrap_or(cs.len());
    let mut out = vec![];
    while i < cs.len() {
        while i < cs.len() && (cs[i] == ' ' || cs[i] == ',') {
            i += 1;
        }
        if i >= cs.len() || cs[i] == ')' {
            break;
        }
        let st = i;
        while cs[i] != '=' {
            i += 1;
        }
        let name: String = cs[st..i].iter().collect();
        i += 1;
        let value: String;
        if cs[i] == '\'' || cs[i] == '"' {
            let qch = cs[i];
            let s0 = i + 1;
            i += 1;
            while cs[i] != qch {
                i += 1;
            }
            value = cs[s0..i].iter().collect();
            i += 1;
        } else {
            let s0 = i;
            while i < cs.len() && cs[i] != ',' && cs[i] != ')' {
                i += 1;
            }
            value = cs[s0..i].iter().collect();
        }
        out.push((name.trim().to_string(), Input::Text(value.trim().to_string())));
    }
    out
}

/// Every text inside the canvas, no non-finite number (ax_engine::figkit::check_figure).
fn check_texts(f: &Figure) {
    if let Err(e) = ax_engine::figkit::check_figure(f) {
        panic!("{e}");
    }
}

fn own(key: &str, kind: &str) -> bool {
    !GENERIC.iter().any(|g| g.0 == key && g.1 == kind)
}

#[test]
fn every_capability_draws_its_published_kinds() {
    let p = pack();
    let mut missing = vec![];
    for (key, kinds) in KINDS {
        let given = example(key);
        let figs = figures(p, key, &given, None, None);
        for kind in *kinds {
            match figs.iter().find(|f| f.kind == *kind) {
                Some(f) => {
                    if own(key, kind) {
                        assert_eq!(f.slug, format!("{}-{kind}", key.replace('_', "-")));
                        check_texts(f);
                    }
                    assert!(!f.svg.contains("NaN"), "{key} {kind}: a non-finite number");
                }
                None => missing.push(format!("{key}: {kind}")),
            }
        }
        // the model's own figures come first
        if let Some(i) = figs.iter().position(|f| f.slug.contains("-live-")) {
            assert!(figs[i..].iter().all(|f| f.slug.contains("-live-")), "{key}: generic figures before own ones");
        }
    }
    assert!(missing.is_empty(), "missing figures: {missing:?}");
}

/// Numbers in a text multiplied by f ("0.3" -> "0.411"; "[[1, 2]]" -> "[[1.37, 2.74]]").
fn scale_text(t: &str, f: f64) -> Option<String> {
    let mut out = String::new();
    let mut cur = String::new();
    let mut changed = false;
    let flush = |cur: &mut String, out: &mut String, changed: &mut bool| {
        if !cur.is_empty() {
            match cur.parse::<f64>() {
                Ok(x) if x != 0.0 => {
                    out.push_str(&format!("{}", x * f));
                    *changed = true;
                }
                _ => out.push_str(cur),
            }
            cur.clear();
        }
    };
    for c in t.chars() {
        if c.is_ascii_digit() || c == '.' || (c == '-' && cur.is_empty()) || ((c == 'e' || c == 'E') && !cur.is_empty()) {
            cur.push(c);
        } else {
            flush(&mut cur, &mut out, &mut changed);
            out.push(c);
        }
    }
    flush(&mut cur, &mut out, &mut changed);
    changed.then_some(out)
}

#[test]
fn a_changed_input_changes_the_drawing() {
    let p = pack();
    let mut unchanged = vec![];
    for (key, kinds) in KINDS {
        let kinds: Vec<&str> = kinds.iter().copied().filter(|k| own(key, k)).collect();
        if kinds.is_empty() {
            continue;
        }
        let given = example(key);
        let base = figures(p, key, &given, None, None);
        let mut tries: Vec<Vec<(String, Input)>> = vec![];
        if let Some((_, sym, val)) = OTHER.iter().find(|o| o.0 == *key) {
            let mut g = given.clone();
            for e in g.iter_mut() {
                if e.0 == *sym {
                    e.1 = Input::Text(val.to_string());
                }
            }
            tries.push(g);
        }
        for f in [1.37, 0.73] {
            for i in 0..given.len() {
                if let Input::Text(t) = &given[i].1 {
                    if let Some(t2) = scale_text(t, f) {
                        let mut g = given.clone();
                        g[i].1 = Input::Text(t2);
                        tries.push(g);
                    }
                }
            }
        }
        let mut ok = false;
        for g in tries {
            let a = ax_engine::solve(p, key, &g, None);
            if !a.solved() {
                continue;
            }
            let figs = figures(p, key, &g, None, None);
            let differs = kinds.iter().all(|kind| {
                match (base.iter().find(|f| f.kind == *kind), figs.iter().find(|f| f.kind == *kind)) {
                    (Some(a), Some(b)) => a.svg != b.svg,
                    _ => false,
                }
            });
            if differs {
                ok = true;
                for f in figs.iter().filter(|f| !f.slug.contains("-live-")) {
                    assert!(!f.svg.contains("NaN"), "{key}: a non-finite number");
                    check_texts(f);
                }
                break;
            }
        }
        if !ok {
            unchanged.push(*key);
        }
    }
    assert!(unchanged.is_empty(), "drawing did not follow a changed input: {unchanged:?}");
}

#[test]
fn a_refused_input_draws_nothing() {
    // a Beta prior with a negative parameter is refused by the model: no figure at all, never a wrong one
    let p = pack();
    let given: Vec<(String, Input)> = [("alpha", "-1"), ("beta", "2"), ("level", "0.9")]
        .iter().map(|(k, v)| (k.to_string(), Input::Text(v.to_string()))).collect();
    let a = ax_engine::solve(p, "beta_credible_interval", &given, None);
    assert!(!a.solved());
    assert!(figures(p, "beta_credible_interval", &given, None, None).is_empty());
}

#[test]
fn a_density_names_its_distribution_and_median() {
    // the Beta(9, 5) density is drawn from the model's own quantiles and names what it shows
    let p = pack();
    let given: Vec<(String, Input)> = [("alpha", "9"), ("beta", "5"), ("level", "0.95")]
        .iter().map(|(k, v)| (k.to_string(), Input::Text(v.to_string()))).collect();
    let figs = figures(p, "beta_credible_interval", &given, None, None);
    let f = figs.iter().find(|f| f.kind == "density").expect("a density");
    assert!(f.caption.contains("Beta(9, 5)") && f.caption.contains("median"));
}

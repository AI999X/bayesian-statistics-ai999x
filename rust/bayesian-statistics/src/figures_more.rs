//! Bayesian Statistics' own live figures, part two: bars, curves against one input, matrices, value fields over two
//! inputs, and single drawings (Bayes' theorem as areas, a utility curve, a regression correction, covariance
//! ellipses, a table of decision criteria, a Halton point set, an uncertainty carried through a function). Every
//! number drawn is the model's (the answer, the inputs as read, further solves of the model itself).

use crate::figures::{fl, int, int_in, jv, list_text, minmax, out};
use ax_engine::figkit::{self as k, num, Canvas, Plot, Solved, P};
use ax_engine::figures::Figure;
use ax_engine::{Answer, Input};
use serde_json::{json, Value};

pub fn live(s: &Solved) -> Vec<Option<Figure>> {
    match s.key {
        "bayes_action_minimum_expected_loss" => vec![action_lines(s), panels_fig(s)],
        "expected_value_decision" => vec![action_lines(s), panels_fig(s)],
        "break_even_probability" => vec![break_even(s)],
        "expected_value_of_perfect_information" => vec![evpi_band(s)],
        "expected_value_of_sample_information" => vec![evsi_bars(s), panels_fig(s)],
        "morris_elementary_effects" => vec![panels_fig(s)],
        "dirichlet_multinomial_evidence" => vec![dm_evidence(s)],
        "dirichlet_multinomial_posterior" => vec![dm_bars(s), dm_lines(s)],
        "discrete_bayes_posterior" => vec![discrete_bars(s), discrete_lines(s)],
        "importance_sampling_estimate" => vec![importance_bars(s)],
        "sobol_indices_interaction_model" | "sobol_indices_ishigami" | "sobol_indices_linear_model" => vec![sobol_bars(s)],
        "total_probability" => vec![total_bars(s)],
        "uncertainty_power_law" => vec![power_law_bars(s)],
        "uncertainty_product_quotient" => vec![product_quotient_bars(s)],
        "diagnostic_test_predictive_values" | "likelihood_ratios" | "lognormal_propagation"
        | "posterior_model_probabilities" | "product_of_normals_exact_moments" | "uncertainty_product_correlated"
        | "welch_satterthwaite_expanded_uncertainty" => vec![curves(s)],
        "normal_normal_summary" => vec![summary_band(s)],
        "monte_carlo_integration" => vec![field2(s, "upper", Rng::Near, "lower", Rng::Near, "estimate",
                                                 "Monte Carlo estimate over the integration limits")],
        "polynomial_of_normal_moments" => vec![field2(s, "sd", Rng::Decade, "mean", Rng::Near, "variance",
                                                      "Variance of a polynomial of a normal variable over its sd and mean")],
        "posterior_from_bayes_factor" => vec![field2(s, "prior", Rng::Prob, "bayes_factor", Rng::Decade, "posterior",
                                                     "Posterior probability over the prior and the Bayes factor")],
        "posterior_from_likelihood_ratio" => vec![field2(s, "prior", Rng::Prob, "likelihood_ratio", Rng::Decade,
                                                         "posterior", "Post-test probability over the pre-test \
                                                                       probability and the likelihood ratio")],
        "bayes_theorem" => vec![bayes_area(s)],
        "certainty_equivalent_exponential_utility" | "certainty_equivalent_log_utility" => vec![utility(s)],
        "control_variate_estimate" => vec![control_fit(s)],
        "covariance_propagation_linear" => vec![ellipses(s)],
        "decision_criteria_under_uncertainty" => vec![criteria_table(s)],
        "sobol_indices_g_function" => vec![io(s)],
        "halton_point" => vec![halton(s)],
        "uncertainty_function" | "uncertainty_power" => vec![propagation(s)],
        _ => vec![],
    }
}

fn lists(a: &Answer, sym: &str) -> Option<Vec<f64>> {
    k::list_of(a, sym)
}

fn esc(t: &str) -> String {
    t.replace('&', "&amp;").replace('<', "&lt;").replace('>', "&gt;").replace('"', "&quot;")
}

fn mat_text(m: &[Vec<f64>]) -> String {
    format!("[{}]", m.iter().map(|r| format!("[{}]", list_text(r))).collect::<Vec<_>>().join(", "))
}

// ------------------------------------------------------------------------------------------------ actions over p
/// Each action's expected value (or loss) over the probability of state 1 of two states, from the model.
fn action_lines(s: &Solved) -> Option<Figure> {
    let loss = s.key == "bayes_action_minimum_expected_loss";
    let (mname, list, best, best_val) = if loss {
        ("losses", "expected_losses", "bayes_action", "bayes_risk")
    } else {
        ("payoffs", "expected_values", "best_action", "best_expected_value")
    };
    let m = s.mat(mname)?;
    let pr = s.vec("probabilities")?;
    if m.is_empty() || m[0].len() != 2 || pr.len() != 2 || m.len() > 6 {
        return None;
    }
    let (ev0, b0, bv0) = (s.out_list(list)?, s.out(best)?, s.out(best_val)?);
    let xs = k::span(0.0, 1.0, 41, false);
    let mut series: Vec<(String, Vec<P>)> = (0..m.len()).map(|i| (format!("action {}", i + 1), vec![])).collect();
    let mut env = vec![];
    for x in &xs {
        let Some(a) = s.with_input(&[("probabilities", jv(&[*x, 1.0 - x]))]) else { continue };
        let (Some(ev), Some(bv)) = (lists(&a, list), out(&a, best_val)) else { continue };
        if ev.len() != m.len() {
            continue;
        }
        for (i, v) in ev.iter().enumerate() {
            series[i].1.push((*x, *v));
        }
        env.push((*x, bv));
    }
    if env.len() < 10 {
        return None;
    }
    series.push((if loss { "Bayes risk" } else { "best value" }.into(), env));
    let header = format!("action {} at P(state 1) = {}: {}", num(b0), num(pr[0]), num(bv0));
    let body = k::lines(&series, "Probability of state 1", if loss { "Expected loss" } else { "Expected payoff" },
                        &header, Some((pr[0], bv0)), &format!("p = {}", num(pr[0])), false);
    Some(s.chart("lines", if loss { "Expected loss of each action and the Bayes action" } else {
        "Expected payoff of each action against the state probability" },
                 &format!("{} of each action of the table {} (rows: actions, columns: states) as the probability of \
                           state 1 runs from 0 to 1, each of the 41 points computed by the model; the last line is the \
                           {}. Marked: the probabilities given, {}: computed {} {}, {} action {} with {}.",
                          if loss { "Expected loss" } else { "Expected payoff" }, mat_text(&m),
                          if loss { "smallest (the Bayes risk)" } else { "best action's value" }, list_text(&pr), list,
                          list_text(&ev0), if loss { "Bayes" } else { "best" }, num(b0), num(bv0)), &body))
}

fn break_even(s: &Solved) -> Option<Figure> {
    let m = s.mat("payoffs")?;
    let (be, pref, ev) = (s.out("break_even")?, s.out("preferred_above")?, s.out("expected_value_at_break_even")?);
    if m.len() != 2 || m[0].len() != 2 {
        return None;
    }
    let xs = k::span(0.0, 1.0, 41, false);
    let mut series: Vec<(String, Vec<P>)> = vec![("action 1".into(), vec![]), ("action 2".into(), vec![])];
    for x in &xs {
        let Some(a) = s.solve_other("expected_value_decision", &[("payoffs", Input::Json(json!(m))),
                                                                  ("probabilities", jv(&[*x, 1.0 - x]))]) else { continue };
        if let Some(v) = lists(&a, "expected_values") {
            if v.len() == 2 {
                series[0].1.push((*x, v[0]));
                series[1].1.push((*x, v[1]));
            }
        }
    }
    if series[0].1.len() < 10 {
        return None;
    }
    let body = k::lines(&series, "Probability of state 1", "Expected payoff",
                        &format!("break-even p = {}, value {}", num(be), num(ev)), Some((be, ev)),
                        &format!("p = {}", num(be)), false);
    Some(s.chart("lines", "Break-even probability between two actions",
                 &format!("The two actions of the payoff table {}: their expected payoffs over the probability of state 1 \
                           (from the model's expected-value capability, 41 values of p) cross at the computed break-even \
                           probability {}, where both are worth {}; action {} is preferred above it.", mat_text(&m),
                          num(be), num(ev), num(pref)), &body))
}

fn evpi_band(s: &Solved) -> Option<Figure> {
    let m = s.mat("payoffs")?;
    let pr = s.vec("probabilities")?;
    if m.is_empty() || m[0].len() != 2 || pr.len() != 2 {
        return None;
    }
    let (evpi, with, without) = (s.out("evpi")?, s.out("ev_with_perfect_information")?, s.out("best_expected_value")?);
    let mut up = vec![];
    let mut dn = vec![];
    for x in k::span(0.0, 1.0, 41, false) {
        let Some(a) = s.with_input(&[("probabilities", jv(&[x, 1.0 - x]))]) else { continue };
        if let (Some(w), Some(b)) = (out(&a, "ev_with_perfect_information"), out(&a, "best_expected_value")) {
            up.push((x, w));
            dn.push((x, b));
        }
    }
    if up.len() < 10 {
        return None;
    }
    let ys: Vec<f64> = up.iter().chain(&dn).map(|p| p.1).collect();
    let (y0, y1) = minmax(&ys);
    let mut p = Plot::new((0.0, 1.0), (y0, y1), "Probability of state 1", "Expected payoff",
                          &format!("EVPI = {} at p = {}", num(evpi), num(pr[0])), (false, false), true);
    let mut poly = up.clone();
    poly.extend(dn.iter().rev().copied());
    p.polygon(&poly, "axp-s", "");
    p.path(&up, "axp-c");
    p.path(&dn, "axp-c2");
    p.seg((pr[0], without), (pr[0], with), "axp-c3");
    p.dot((pr[0], with), 4.5, "axp-d");
    p.mark((pr[0], without), &format!("{} to {}", num(without), num(with)));
    p.legend(&[("with perfect information".into(), "axp-b"), ("without".into(), "axp-b2")]);
    Some(s.chart("band", "Value of perfect information across state probabilities",
                 &format!("For the payoff table {}: the expected payoff with perfect information about the state and the \
                           best expected payoff without it, over the probability of state 1 (41 computations each); the \
                           shaded gap is the expected value of perfect information. Marked: the probabilities given, {}, \
                           computed values {} and {}, EVPI = {}.", mat_text(&m), list_text(&pr), num(with), num(without),
                          num(evpi)), &p.body()))
}

// ------------------------------------------------------------------------------------------------ matrices
struct Panel {
    name: String,
    m: Vec<Vec<f64>>,
    returned: bool,
    /// row and column names (numbered from 1 when absent)
    labels: Option<(Vec<String>, Vec<String>)>,
}

/// Matrices side by side as heat maps, each shaded by its own largest magnitude (negative values in the second
/// colour), rows and columns numbered, entries written in the cells.
fn draw_panels(ps: &[Panel], header: &str) -> Option<(String, f64)> {
    let ps: Vec<&Panel> = ps.iter().filter(|p| !p.m.is_empty() && !p.m[0].is_empty()).collect();
    if ps.is_empty() {
        return None;
    }
    let rows = ps.iter().map(|p| p.m.len()).max()?;
    let cols: usize = ps.iter().map(|p| p.m.iter().map(|r| r.len()).max().unwrap_or(0)).sum();
    if rows > 16 || cols > 24 || ps.iter().any(|p| p.m.iter().flatten().any(|x| !x.is_finite())) {
        return None;
    }
    let gap = 30.0;
    let avail_w = 520.0 - 44.0 - gap * (ps.len() - 1) as f64;
    let top = 80.0;
    let cell = (avail_w / cols as f64).min(230.0 / rows as f64).min(64.0);
    let widths: Vec<f64> = ps.iter().map(|p| cell * p.m.iter().map(|r| r.len()).max().unwrap_or(0) as f64).collect();
    let total: f64 = widths.iter().sum::<f64>() + gap * (ps.len() - 1) as f64;
    let mut x = 34.0 + (520.0 - 34.0 - 10.0 - total) / 2.0;
    let mut c = Canvas::new(top + cell * rows as f64 + 24.0);
    if !header.is_empty() {
        c.raw(&format!("<text class=\"axp-mt\" x=\"16\" y=\"20\">{}</text>", esc(header)));
    }
    let font = (cell * 0.3).clamp(8.0, 13.0);
    let title = |p: &Panel| if p.labels.is_some() { p.name.clone() } else {
        format!("{} ({})", p.name, if p.returned { "computed" } else { "input" })
    };
    let stagger = ps.iter().zip(&widths).any(|(p, w)| k::text_w(&title(p), false) > w + gap - 4.0);
    for (pi, p) in ps.iter().enumerate() {
        let nc = p.m.iter().map(|r| r.len()).max().unwrap_or(0);
        let w = widths[pi];
        let ty = if stagger && pi % 2 == 1 { 56.0 } else { 40.0 };
        c.text(x + w / 2.0, ty, "middle", if p.returned { "axp-vt" } else { "axp-bt" }, &title(p));
        for j in 0..nc {
            let l = p.labels.as_ref().and_then(|l| l.1.get(j).cloned()).unwrap_or_else(|| (j + 1).to_string());
            c.name(x + cell * (j as f64 + 0.5), top - 6.0, "middle", &l);
        }
        if pi == 0 {
            for i in 0..p.m.len() {
                let l = p.labels.as_ref().and_then(|l| l.0.get(i).cloned()).unwrap_or_else(|| (i + 1).to_string());
                c.name(x - 6.0, top + cell * (i as f64 + 0.5) + 4.0, "end", &l);
            }
        }
        let mx = p.m.iter().flatten().fold(0.0f64, |a, v| a.max(v.abs()));
        for (i, r) in p.m.iter().enumerate() {
            for (j, v) in r.iter().enumerate() {
                let (cx, cy) = (x + cell * j as f64, top + cell * i as f64);
                let op = if mx > 0.0 { 0.08 + 0.47 * v.abs() / mx } else { 0.08 };
                c.raw(&format!("<rect class=\"{}\" x=\"{cx:.1}\" y=\"{cy:.1}\" width=\"{:.1}\" height=\"{:.1}\" \
                                fill-opacity=\"{op:.2}\"/>", if *v < 0.0 { "axp-hn" } else { "axp-hp" }, cell - 1.5,
                               cell - 1.5));
                let t = k::snap(*v, mx);
                let fits = |x: &String| k::text_w(x, true) * font / 13.0 <= cell - 2.0;
                let t = [4, 3, 2].iter().map(|d| ax_engine::num::py_g(t, *d)).find(fits)
                    .unwrap_or_else(|| ax_engine::num::py_g(t, 2));
                if fits(&t) {
                    c.raw(&format!("<text class=\"axp-ht\" x=\"{:.1}\" y=\"{:.1}\" text-anchor=\"middle\" \
                                    font-size=\"{font:.0}\">{}</text>", cx + cell / 2.0, cy + cell / 2.0 + font * 0.35,
                                   esc(&t)));
                }
            }
        }
        x += w + gap;
    }
    let h = c.h;
    Some((c.s, h))
}

fn col(v: &[f64]) -> Vec<Vec<f64>> {
    v.iter().map(|x| vec![*x]).collect()
}

fn panels_fig(s: &Solved) -> Option<Figure> {
    let p = |n: &str, m: Vec<Vec<f64>>, r: bool| Panel { name: n.into(), m, returned: r, labels: None };
    let (ps, header, title, rest) = match s.key {
        "bayes_action_minimum_expected_loss" => {
            (vec![p("losses", s.mat("losses")?, false), p("expected_losses", col(&s.out_list("expected_losses")?), true)],
             format!("Bayes action {}, risk {}", num(s.out("bayes_action")?), num(s.out("bayes_risk")?)),
             "Bayes action: the loss table and the expected losses",
             format!("bayes_action = {}, bayes_risk = {}; probabilities given: {}", num(s.out("bayes_action")?),
                     num(s.out("bayes_risk")?), list_text(&s.vec("probabilities")?)))
        }
        "expected_value_decision" => {
            (vec![p("payoffs", s.mat("payoffs")?, false), p("expected_values", col(&s.out_list("expected_values")?), true)],
             format!("best action {}, value {}", num(s.out("best_action")?), num(s.out("best_expected_value")?)),
             "Best action by expected value: the payoff table and the expected values",
             format!("best_action = {}, best_expected_value = {}; probabilities given: {}", num(s.out("best_action")?),
                     num(s.out("best_expected_value")?), list_text(&s.vec("probabilities")?)))
        }
        "expected_value_of_sample_information" => {
            (vec![p("payoffs", s.mat("payoffs")?, false), p("likelihoods", s.mat("likelihoods")?, false),
                  p("signal_probabilities", col(&s.out_list("signal_probabilities")?), true)],
             format!("EVSI = {}", num(s.out("evsi")?)),
             "Expected value of sample information: the matrices",
             format!("evsi = {}, ev_with_information = {}, ev_without_information = {}; priors given: {}",
                     num(s.out("evsi")?), num(s.out("ev_with_information")?), num(s.out("ev_without_information")?),
                     list_text(&s.vec("priors")?)))
        }
        _ => {
            let o = s.mat("outputs")?;
            let (mu, ms, sg) = (s.out_list("mu")?, s.out_list("mu_star")?, s.out_list("sigma")?);
            let mut ps = vec![p("outputs", o, false)];
            if mu.len() == ms.len() && ms.len() == sg.len() {
                ps.push(p("mu, mu_star, sigma", vec![mu.clone(), ms.clone(), sg.clone()], true));
            }
            (ps, format!("mu* = {}", list_text(&ms)), "Morris elementary-effects screening: the matrices",
             format!("mu = {}, mu_star = {}, sigma = {}; delta given: {}", list_text(&mu), list_text(&ms),
                     list_text(&sg), num(s.inp("delta")?)))
        }
    };
    let (body, h) = draw_panels(&ps, &header)?;
    Some(s.figure("matrix", title,
                  &format!("The matrices as heat maps (rows and columns numbered; each cell shaded by its magnitude, \
                            negative values in the second colour; each panel shaded by its own largest magnitude): {}. \
                            Computed with them: {rest}.",
                           ps.iter().map(|q| format!("{} ({})", q.name, if q.returned { "computed" } else { "input" }))
                               .collect::<Vec<_>>().join("; ")), &body, h))
}

pub fn posterior_mean_heat(s: &Solved) -> Option<Figure> {
    let (a, b) = (s.inp("alpha")?, s.inp("beta")?);
    let mut m = vec![];
    for si in 0..=5i64 {
        let mut row = vec![];
        for fi in 0..=5i64 {
            let v = s.with_input(&[("successes", int(si)), ("trials", int(si + fi))]).and_then(|r| out(&r, "posterior_mean"))?;
            row.push(v);
        }
        m.push(row);
    }
    let ps = [Panel { name: "rows: successes, columns: failures".into(), m: m.clone(), returned: true,
                      labels: Some(((0..=5).map(|i| i.to_string()).collect(), (0..=5).map(|i| i.to_string()).collect())) }];
    let (body, h) = draw_panels(&ps, &format!("posterior mean, prior Beta({}, {})", num(a), num(b)))?;
    Some(s.figure("heatmap", "Posterior mean for every count of successes and failures",
                 &format!("The posterior mean of a success probability under a Beta({}, {}) prior after s successes \
                           (rows, 0 to 5) and f failures (columns, 0 to 5), each cell computed by the model: from {} after \
                           5 failures to {} after 5 successes, {} with no data.", num(a), num(b), num(m[0][5]),
                          num(m[5][0]), num(m[0][0])), &body, h))
}

// ------------------------------------------------------------------------------------------------ bars
fn labels(n: usize, p: &str) -> Vec<String> {
    (1..=n).map(|i| format!("{p}{i}")).collect()
}

fn evsi_bars(s: &Solved) -> Option<Figure> {
    let (evsi, with, without) = (s.out("evsi")?, s.out("ev_with_information")?, s.out("ev_without_information")?);
    let perfect = s.solve_other("expected_value_of_perfect_information",
                                &[("payoffs", Input::Json(json!(s.mat("payoffs")?))), ("probabilities", jv(&s.vec("priors")?))])
        .and_then(|a| Some((out(&a, "ev_with_perfect_information")?, out(&a, "evpi")?)));
    let mut l = vec!["no information".to_string(), "sample information".into()];
    let mut v = vec![without, with];
    if let Some((w, _)) = perfect {
        l.push("perfect information".into());
        v.push(w);
    }
    let body = k::bars(&l, &v, &[1], "", "Expected payoff", &format!("EVSI = {}", num(evsi)));
    Some(s.chart("bars", "Value of sample against perfect information",
                 &format!("For the payoff table {}, priors {} and a signal with likelihoods {}: the computed expected \
                           payoff without information {}, with the sample information {} (EVSI = {}){}.",
                          mat_text(&s.mat("payoffs")?), list_text(&s.vec("priors")?), mat_text(&s.mat("likelihoods")?),
                          num(without), num(with), num(evsi),
                          perfect.map(|(w, e)| format!(" and, from the model's perfect-information capability, {} (EVPI = \
                                                        {}): sample information is worth at most the perfect kind", num(w),
                                                       num(e))).unwrap_or_default()), &body))
}

/// Every way to split n observations over k categories (k <= 4, at most 40 ways).
fn splits(n: i64, k_: usize) -> Option<Vec<Vec<f64>>> {
    fn rec(n: i64, k_: usize, cur: &mut Vec<f64>, out: &mut Vec<Vec<f64>>) {
        if k_ == 1 {
            cur.push(n as f64);
            out.push(cur.clone());
            cur.pop();
            return;
        }
        for i in (0..=n).rev() {
            cur.push(i as f64);
            rec(n - i, k_ - 1, cur, out);
            cur.pop();
        }
    }
    if !(2..=4).contains(&k_) || n < 0 || n > 12 {
        return None;
    }
    let mut v = vec![];
    rec(n, k_, &mut vec![], &mut v);
    (v.len() <= 40).then_some(v)
}

fn dm_evidence(s: &Solved) -> Option<Figure> {
    let (alpha, counts) = (s.vec("alpha")?, s.vec("counts")?);
    let p0 = s.out("probability")?;
    let n: f64 = counts.iter().sum();
    if n.fract() != 0.0 {
        return None;
    }
    let ways = splits(n as i64, counts.len())?;
    let other = vec![4.0; alpha.len()];
    let other = if other == alpha { vec![1.0; alpha.len()] } else { other };
    let mut g1 = vec![];
    let mut g2 = vec![];
    for w in &ways {
        g1.push(s.with_input(&[("counts", jv(w))]).and_then(|a| out(&a, "probability")));
        g2.push(s.with_input(&[("counts", jv(w)), ("alpha", jv(&other))]).and_then(|a| out(&a, "probability")));
    }
    // short labels: the counts written together when every count is one digit ("210" for 2, 1, 0)
    let sep = if n < 10.0 { "" } else { "," };
    let lab: Vec<String> = ways.iter().map(|w| w.iter().map(|x| num(*x)).collect::<Vec<_>>().join(sep)).collect();
    let body = k::bar_groups(&lab, &[(format!("Dirichlet({})", list_text(&alpha)), g1),
                                     (format!("Dirichlet({})", list_text(&other)), g2)],
                             "Counts per category", "Probability",
                             &format!("P({}) = {}", list_text(&counts), num(p0)));
    Some(s.chart("bars", "Probability of every count vector under two priors",
                 &format!("The Dirichlet-multinomial probability of each of the {} ways to split {} observations over {} \
                           categories (each label: the count of every category in turn), under the given Dirichlet({}) \
                           prior and a Dirichlet({}) prior for comparison; \
                           each probability computed by the model. The counts given, {}, have probability {}.", ways.len(),
                          num(n), counts.len(), list_text(&alpha), list_text(&other), list_text(&counts), num(p0)), &body))
}

fn dm_bars(s: &Solved) -> Option<Figure> {
    let (pa, mean, var) = (s.out_list("post_alpha")?, s.out_list("posterior_means")?, s.out_list("posterior_variances")?);
    if mean.len() != var.len() || mean.is_empty() || mean.len() > 12 {
        return None;
    }
    let n = mean.len();
    let sd: Vec<f64> = var.iter().map(|v| v.max(0.0).sqrt()).collect();
    let top = mean.iter().zip(&sd).map(|(m, s)| m + 2.0 * s).fold(0.0, f64::max);
    let names = labels(n, "category ");
    let mut p = Plot::rows((0.0, top.max(1e-9) * 1.12), &names, "Posterior mean of the category probability",
                           &format!("posterior Dirichlet({})", list_text(&pa)), false);
    for (i, (m, d)) in mean.iter().zip(&sd).enumerate() {
        let y = i as f64;
        p.rect((0.0, y - 0.3), (*m, y + 0.3), "axp-b", "");
        p.seg(((m - 2.0 * d).max(0.0), y), (m + 2.0 * d, y), "axp-k");
        p.seg((m + 2.0 * d, y - 0.12), (m + 2.0 * d, y + 0.12), "axp-k");
        p.seg(((m - 2.0 * d).max(0.0), y - 0.12), ((m - 2.0 * d).max(0.0), y + 0.12), "axp-k");
        p.label((m + 2.0 * d, y), 6.0, 4.5, "start", "axp-vt", &num(*m));
    }
    Some(s.chart("bars", "Category probabilities after multinomial counts",
                 &format!("A Dirichlet({}) prior over {n} categories updated with the counts {}: the computed posterior \
                           Dirichlet({}), posterior means {}; whiskers ± 2 posterior standard deviations (square roots of \
                           the computed variances {}).", list_text(&s.vec("alpha")?), list_text(&s.vec("counts")?),
                          list_text(&pa), list_text(&mean), list_text(&var)), &p.body()))
}

fn dm_lines(s: &Solved) -> Option<Figure> {
    let counts = s.vec("counts")?;
    let n = counts.len();
    if n > 6 {
        return None;
    }
    let ms = [1.0, 1.5, 2.0, 3.0, 4.0, 6.0, 8.0, 12.0, 16.0, 23.0, 32.0];
    let mut series: Vec<(String, Vec<P>)> = (0..n).map(|i| (format!("category {}", i + 1), vec![])).collect();
    for m in ms {
        let c: Vec<f64> = counts.iter().map(|x| x * m).collect();
        let Some(v) = s.with_input(&[("counts", jv(&c))]).and_then(|a| lists(&a, "posterior_means")) else { continue };
        if v.len() == n {
            for (i, x) in v.iter().enumerate() {
                series[i].1.push((m, *x));
            }
        }
    }
    // the limit: the same counts a million times over
    let lim: Option<Vec<f64>> = s.with_input(&[("counts", jv(&counts.iter().map(|x| x * 1e6).collect::<Vec<_>>()))])
        .and_then(|a| lists(&a, "posterior_means")).filter(|v| v.len() == n);
    if series[0].1.len() < 4 {
        return None;
    }
    if let Some(l) = &lim {
        for (i, x) in l.iter().enumerate() {
            series.push((String::new(), vec![(1.0, *x), (32.0, *x)]));
            let _ = i;
        }
    }
    let body = k::lines(&series, "Counts multiplied by m", "Posterior mean", &format!("counts {} times m", list_text(&counts)),
                        None, "", true);
    Some(s.chart("lines", "Category means moving from the prior to the observed shares",
                 &format!("Posterior means of {n} category probabilities under the Dirichlet({}) prior when the counts {} \
                           are multiplied by m = 1 to 32, each computed by the model: with more data the means leave the \
                           prior and approach the observed shares{}.", list_text(&s.vec("alpha")?), list_text(&counts),
                          lim.map(|l| format!(" (thin horizontal lines: the model's means for the counts a million times \
                                               over, {})", list_text(&l))).unwrap_or_default()), &body))
}

fn discrete_bars(s: &Solved) -> Option<Figure> {
    let (pr, post) = (s.vec("priors")?, s.out_list("posterior")?);
    if pr.len() != post.len() || pr.len() > 12 {
        return None;
    }
    let lab = labels(pr.len(), "H");
    let body = k::bar_groups(&lab, &[("prior".into(), pr.iter().map(|x| Some(*x)).collect()),
                                     ("posterior".into(), post.iter().map(|x| Some(*x)).collect())],
                             "Hypothesis", "Probability",
                             &format!("most probable: H{} with {}", num(s.out("map_hypothesis")?), num(s.out("map_probability")?)));
    Some(s.chart("bars", "Prior and posterior over the hypotheses",
                 &format!("Bayes' rule over {} hypotheses with priors {} and likelihoods {}: the computed posterior \
                           probabilities {} (evidence {}); the most probable hypothesis is {} with {}.", pr.len(),
                          list_text(&pr), list_text(&s.vec("likelihoods")?), list_text(&post), num(s.out("evidence")?),
                          num(s.out("map_hypothesis")?), num(s.out("map_probability")?)), &body))
}

fn discrete_lines(s: &Solved) -> Option<Figure> {
    let pr = s.vec("priors")?;
    let n = pr.len();
    if n > 6 {
        return None;
    }
    let mut series: Vec<(String, Vec<P>)> = (0..n).map(|i| (format!("H{}", i + 1), vec![(0.0, pr[i])])).collect();
    let mut cur = s.out_list("posterior")?;
    for step in 1..=6 {
        if cur.len() != n {
            return None;
        }
        for (i, x) in cur.iter().enumerate() {
            series[i].1.push((step as f64, *x));
        }
        if step == 6 {
            break;
        }
        cur = s.with_input(&[("priors", jv(&cur))]).and_then(|a| lists(&a, "posterior"))?;
    }
    let body = k::lines(&series, "Number of observations", "Posterior probability", "the posterior fed back as prior",
                        None, "", false);
    Some(s.chart("lines", "Posterior over hypotheses after repeated observations",
                 &format!("Repeated updating over {n} hypotheses with priors {} and likelihoods {}: the posterior of each \
                           step fed back as the next prior, six steps, every probability computed by the model (after 6 \
                           steps: {}).", list_text(&pr), list_text(&s.vec("likelihoods")?), list_text(&cur)), &body))
}

fn importance_bars(s: &Solved) -> Option<Figure> {
    let (x, p, q) = (s.vec("values")?, s.vec("target_density")?, s.vec("proposal_density")?);
    if x.len() != p.len() || p.len() != q.len() || x.len() > 12 {
        return None;
    }
    let lab: Vec<String> = x.iter().map(|v| format!("x = {}", num(*v))).collect();
    let est = s.out("estimate")?;
    let body = k::bar_groups(&lab, &[("target p".into(), p.iter().map(|v| Some(*v)).collect()),
                                     ("proposal q".into(), q.iter().map(|v| Some(*v)).collect())],
                             "Draw", "Density", &format!("estimate {}, ESS {}", num(est), num(s.out("effective_sample_size")?)));
    Some(s.chart("bars", "Target and proposal densities of the draws",
                 &format!("Importance sampling with draws whose values are {}, target densities {} and proposal densities \
                           {}: where the target is higher than the proposal a draw counts more. Computed: self-normalised \
                           estimate {}, unnormalised {}, effective sample size {}.", list_text(&x), list_text(&p),
                          list_text(&q), num(est), num(s.out("unnormalized_estimate")?),
                          num(s.out("effective_sample_size")?)), &body))
}

fn sobol_bars(s: &Solved) -> Option<Figure> {
    match s.key {
        "sobol_indices_interaction_model" => {
            let names = ["S1", "S2", "S12", "ST1", "ST2"];
            let v: Vec<f64> = names.iter().map(|n| s.out(n)).collect::<Option<_>>()?;
            let lab: Vec<String> = names.iter().map(|x| x.to_string()).collect();
            let body = k::bars(&lab, &v, &[3, 4], "Index", "Share of the output variance",
                               &format!("variance {}", num(s.out("variance")?)));
            Some(s.chart("bars", "Main and interaction effects of a two-input model",
                         &format!("Sobol indices of y = a1 x1 + a2 x2 + a12 x1 x2 with a1 = {}, a2 = {}, a12 = {} and \
                                   inputs x1 ~ N({}, {}²), x2 ~ N({}, {}²): computed {}, variance {}. Each total index \
                                   (highlighted) adds the interaction share to the first-order one.", num(s.inp("a1")?),
                                  num(s.inp("a2")?), num(s.inp("a12")?), num(s.inp("mean1")?), num(s.inp("sd1")?),
                                  num(s.inp("mean2")?), num(s.inp("sd2")?),
                                  names.iter().zip(&v).map(|(n, x)| format!("{n} = {}", num(*x))).collect::<Vec<_>>()
                                      .join(", "), num(s.out("variance")?)), &body))
        }
        "sobol_indices_ishigami" => {
            let f: Vec<Option<f64>> = ["S1", "S2", "S3"].iter().map(|n| s.out(n)).collect();
            let t: Vec<Option<f64>> = ["ST1", "ST2", "ST3"].iter().map(|n| s.out(n)).collect();
            if f.iter().chain(&t).any(|x| x.is_none()) {
                return None;
            }
            let body = k::bar_groups(&labels(3, "x"), &[("first order S".into(), f.clone()), ("total ST".into(), t.clone())],
                                     "Input", "Share of the output variance",
                                     &format!("variance {}", num(s.out("variance")?)));
            let tx = |v: &[Option<f64>]| v.iter().flatten().map(|x| num(*x)).collect::<Vec<_>>().join(", ");
            Some(s.chart("bars", "Sobol indices of the Ishigami function",
                         &format!("First- and total-order Sobol indices of the Ishigami function with a = {}, b = {}, as \
                                   computed: S = {}, ST = {}, variance {}. An input whose total index exceeds its \
                                   first-order one acts through interactions.", num(s.inp("a")?), num(s.inp("b")?), tx(&f),
                                  tx(&t), num(s.out("variance")?)), &body))
        }
        _ => {
            let (a, sd) = (s.vec("coefficients")?, s.vec("sds")?);
            let (f, var) = (s.out_list("first_order")?, s.out("variance")?);
            if a.len() != sd.len() || f.len() != a.len() || a.len() > 8 {
                return None;
            }
            let mut a2 = a.clone();
            a2.push(2.0);
            let mut s2 = sd.clone();
            s2.push(1.5);
            let more = s.with_input(&[("coefficients", jv(&a2)), ("sds", jv(&s2))])
                .and_then(|r| Some((lists(&r, "first_order")?, out(&r, "variance")?))).filter(|r| r.0.len() == a2.len());
            let mut g = vec![(format!("given (variance {})", num(var)),
                              f.iter().map(|x| Some(*x)).chain(std::iter::once(None)).collect::<Vec<_>>())];
            if let Some((f2, v2)) = &more {
                g.push((format!("with x{} (variance {})", a2.len(), num(*v2)), f2.iter().map(|x| Some(*x)).collect()));
            }
            let n = if more.is_some() { a2.len() } else { a.len() };
            for x in g.iter_mut() {
                x.1.truncate(n);
            }
            let body = k::bar_groups(&labels(n, "x"), &g, "Input", "First-order index", "share of the output variance");
            Some(s.chart("bars", "Variance shares of the inputs of a linear model",
                         &format!("First-order Sobol indices of a linear model y = Σ a_i x_i with independent inputs: the \
                                   coefficients {} with sds {} give {} (variance {}){}. Each index is that input's share \
                                   of the output variance.", list_text(&a), list_text(&sd), list_text(&f), num(var),
                                  more.map(|(f2, v2)| format!("; adding an input with coefficient 2 and sd 1.5 gives {} \
                                                               (variance {})", list_text(&f2), num(v2)))
                                      .unwrap_or_default()), &body))
        }
    }
}

fn total_bars(s: &Solved) -> Option<Figure> {
    let (pr, cond) = (s.vec("priors")?, s.vec("conditionals")?);
    let (pb, joint) = (s.out("p_b")?, s.out_list("joint")?);
    if joint.len() > 12 {
        return None;
    }
    let mut lab: Vec<String> = (1..=joint.len()).map(|i| format!("B and A{i}")).collect();
    lab.push("P(B)".into());
    let mut v = joint.clone();
    v.push(pb);
    let body = k::bars(&lab, &v, &[joint.len()], "", "Probability", &format!("P(B) = {}", num(pb)));
    Some(s.chart("bars", "Total probability as a sum of joint probabilities",
                 &format!("The law of total probability for priors {} and conditionals P(B | A_i) = {}: the computed \
                           joint probabilities {} and their sum, the computed P(B) = {} (highlighted).", list_text(&pr),
                          list_text(&cond), list_text(&joint), num(pb)), &body))
}

fn power_law_bars(s: &Solved) -> Option<Figure> {
    let (x, u) = (s.vec("values")?, s.vec("uncertainties")?);
    let rel = s.out("relative_uncertainty")?;
    if x.len() != u.len() || x.len() > 10 {
        return None;
    }
    let mut lab = vec![];
    let mut v = vec![];
    for i in 0..x.len() {
        let only: Vec<f64> = (0..x.len()).map(|j| if j == i { u[j] } else { 0.0 }).collect();
        let r = s.with_input(&[("uncertainties", jv(&only))]).and_then(|a| out(&a, "relative_uncertainty"))?;
        lab.push(format!("x{} alone", i + 1));
        v.push(r);
    }
    lab.push("all together".into());
    v.push(rel);
    let n = v.len() - 1;
    let body = k::bars(&lab, &v, &[n], "", "Relative uncertainty of y",
                       &format!("y = {} ± {}", num(s.out("value")?), num(s.out("uncertainty")?)));
    Some(s.chart("bars", "Contributions to the uncertainty of a power law",
                 &format!("The power law y = {} · Π x_i^n_i with values {}, exponents {} and uncertainties {}: the \
                           relative uncertainty of y computed with only one input uncertain at a time ({}), and with all \
                           together, {} (y = {} ± {}): the contributions add in quadrature.", num(s.inp("coefficient")?),
                          list_text(&x), list_text(&s.vec("exponents")?), list_text(&u), list_text(&v[..n]), num(rel),
                          num(s.out("value")?), num(s.out("uncertainty")?)), &body))
}

fn product_quotient_bars(s: &Solved) -> Option<Figure> {
    let (a, ua, b, ub) = (s.inp("a")?, s.inp("ua")?, s.inp("b")?, s.inp("ub")?);
    let rel = |r: &Answer| -> Option<(f64, f64)> {
        let (p, up, q, uq) = (out(r, "product")?, out(r, "u_product")?, out(r, "quotient")?, out(r, "u_quotient")?);
        (p != 0.0 && q != 0.0).then(|| (up / p.abs(), uq / q.abs()))
    };
    let both = rel(s.a)?;
    let only_a = s.with(&[("ub", 0.0)]).and_then(|r| rel(&r));
    let only_b = s.with(&[("ua", 0.0)]).and_then(|r| rel(&r));
    let mut lab = vec![];
    let mut gp = vec![];
    let mut gq = vec![];
    for (l, v) in [("a uncertain", only_a), ("b uncertain", only_b), ("both", Some(both))] {
        if let Some((p, q)) = v {
            lab.push(l.to_string());
            gp.push(Some(p));
            gq.push(Some(q));
        }
    }
    let body = k::bar_groups(&lab, &[("product".into(), gp), ("quotient".into(), gq)], "",
                             "Relative uncertainty", &format!("a·b = {} ± {}", num(s.out("product")?),
                                                              num(s.out("u_product")?)));
    Some(s.chart("bars", "Relative uncertainty of a product and a quotient",
                 &format!("Relative uncertainties (computed uncertainty divided by computed value) for a = {} ± {} and b = \
                           {} ± {}: with only a uncertain, only b uncertain, and both. Computed product {} ± {} and \
                           quotient {} ± {}: product and quotient share the same relative uncertainty.", num(a), num(ua),
                          num(b), num(ub), num(s.out("product")?), num(s.out("u_product")?), num(s.out("quotient")?),
                          num(s.out("u_quotient")?)), &body))
}

// ------------------------------------------------------------------------------------------------ curves
fn curve_pairs(s: &Solved, x: &str, xs: &[f64], ys: &[&str]) -> Vec<(String, Vec<P>)> {
    let mut v: Vec<(String, Vec<P>)> = ys.iter().map(|y| (y.to_string(), vec![])).collect();
    for xv in xs {
        if let Some(a) = s.with(&[(x, *xv)]) {
            for (i, y) in ys.iter().enumerate() {
                if let Some(z) = out(&a, y) {
                    v[i].1.push((*xv, z));
                }
            }
        }
    }
    v
}

fn curves(s: &Solved) -> Option<Figure> {
    let (series, xlab, ylab, header, mark, mlabel, logx, title, cap): (Vec<(String, Vec<P>)>, &str, &str, String,
        Option<P>, String, bool, &str, String);
    match s.key {
        "diagnostic_test_predictive_values" => {
            let pv = s.inp("prevalence")?;
            let (ppv, npv) = (s.out("ppv")?, s.out("npv")?);
            let lo = (pv / 10.0).min(1e-3).max(1e-6);
            let mut sr = curve_pairs(s, "prevalence", &k::span(lo, 0.999, 41, true), &["ppv", "npv"]);
            sr[0].0 = "positive predictive value".into();
            sr[1].0 = "negative predictive value".into();
            series = sr;
            xlab = "Prevalence"; ylab = "Predictive value"; logx = true;
            header = format!("PPV = {}, NPV = {}", num(ppv), num(npv));
            mark = Some((pv, ppv));
            mlabel = format!("prevalence {}", num(pv));
            title = "Predictive values of a test against prevalence";
            cap = format!("Positive and negative predictive values of a test with sensitivity {} and specificity {} over \
                           prevalences from {} to 0.999, each computed by the model: a positive result means little when \
                           the condition is rare. Marked: prevalence {}, computed PPV {} (NPV {}, P(positive) = {}).",
                          num(s.inp("sensitivity")?), num(s.inp("specificity")?), num(lo), num(pv), num(ppv), num(npv),
                          num(s.out("p_positive")?));
        }
        "likelihood_ratios" => {
            let (lp, ln) = (s.out("lr_positive")?, s.out("lr_negative")?);
            let xs = k::span(0.01, 0.99, 50, false);
            let mut pos = vec![];
            let mut neg = vec![];
            for x in &xs {
                let f = |lr: f64| s.solve_other("posterior_from_likelihood_ratio", &[("prior", fl(*x)), ("likelihood_ratio", fl(lr))])
                    .and_then(|a| out(&a, "posterior"));
                if let Some(v) = f(lp) {
                    pos.push((*x, v));
                }
                if let Some(v) = f(ln) {
                    neg.push((*x, v));
                }
            }
            let m = pos.iter().min_by(|a, b| (a.0 - 0.3).abs().partial_cmp(&(b.0 - 0.3).abs()).unwrap()).copied();
            series = vec![("after a positive result".into(), pos), ("after a negative result".into(), neg),
                          ("no test".into(), vec![(0.01, 0.01), (0.99, 0.99)])];
            xlab = "Pre-test probability"; ylab = "Post-test probability"; logx = false;
            header = format!("LR+ = {}, LR- = {}", num(lp), num(ln));
            mark = m;
            mlabel = m.map(|p| format!("{} to {}", num(p.0), num(p.1))).unwrap_or_default();
            title = "Pre-test to post-test probability for both results";
            cap = format!("How far a test result moves a probability: the computed likelihood ratios of a test with \
                           sensitivity {} and specificity {} (LR+ = {}, LR- = {}, diagnostic odds ratio {}) turned into \
                           post-test probabilities by the model's posterior-from-likelihood-ratio capability at 50 \
                           pre-test values.{}", num(s.inp("sensitivity")?), num(s.inp("specificity")?), num(lp), num(ln),
                          num(s.out("diagnostic_odds_ratio")?),
                          m.map(|p| format!(" Marked: pre-test {} becomes {} after a positive result.", num(p.0), num(p.1)))
                              .unwrap_or_default());
        }
        "lognormal_propagation" => {
            let sg = s.inp("sigma")?;
            let (es, fs) = (s.out("exact_sd")?, s.out("first_order_sd")?);
            let hi = (sg * 1.5).max(1.5);
            let mut sr = curve_pairs(s, "sigma", &k::span(hi / 30.0, hi, 41, false), &["exact_sd", "first_order_sd"]);
            sr[0].0 = "exact".into();
            sr[1].0 = "first order".into();
            series = sr;
            xlab = "σ of X"; ylab = "Standard deviation of exp(X)"; logx = false;
            header = format!("exact {}, first order {}", num(es), num(fs));
            mark = Some((sg, es));
            mlabel = format!("σ = {}", num(sg));
            title = "Exact against linearised uncertainty of exp(X)";
            cap = format!("The standard deviation of Y = exp(X) for normal X with mean μ = {} as σ runs from {} to {}: \
                           exact and first-order (linearised) values, each computed by the model; they agree for small σ \
                           and part as it grows. Marked: σ = {}: exact {}, first order {} (means {} and {}).",
                          num(s.inp("mu")?), num(hi / 30.0), num(hi), num(sg), num(es), num(fs), num(s.out("exact_mean")?),
                          num(s.out("first_order_mean")?));
        }
        "posterior_model_probabilities" => {
            let le = s.vec("log_evidences")?;
            if le.len() != 2 {
                return None;
            }
            let p0 = s.out_list("posterior")?;
            let given_pr = s.vec("priors");
            let alt = vec![0.8, 0.2];
            let mut a1 = vec![];
            let mut a2 = vec![];
            for d in k::span(-6.0, 6.0, 49, false) {
                let ev = jv(&[le[1] + d, le[1]]);
                if let Some(v) = s.with_input(&[("log_evidences", ev.clone())]).and_then(|a| lists(&a, "posterior")) {
                    a1.push((d, v[0]));
                }
                if let Some(v) = s.with_input(&[("log_evidences", ev), ("priors", jv(&alt))]).and_then(|a| lists(&a, "posterior")) {
                    a2.push((d, v[0]));
                }
            }
            let gname = given_pr.as_ref().map(|p| format!("priors {}", list_text(p))).unwrap_or_else(|| "equal priors".into());
            series = vec![(gname.clone(), a1), ("priors 0.8, 0.2".into(), a2)];
            xlab = "Log evidence of model 1 minus model 2"; ylab = "Posterior probability of model 1"; logx = false;
            header = format!("P(model 1) = {}", num(p0[0]));
            mark = Some((le[0] - le[1], p0[0]));
            mlabel = format!("difference {}", num(le[0] - le[1]));
            title = "Posterior model probability against the evidence difference";
            cap = format!("The posterior probability of model 1 of two as the difference of their natural-log evidences \
                           runs from -6 to 6, for {gname} and for priors 0.8, 0.2, every value computed by the model. \
                           Marked: the log evidences given, {} (difference {}), give {}.", list_text(&le),
                          num(le[0] - le[1]), num(p0[0]));
        }
        "product_of_normals_exact_moments" => {
            let sx = s.inp("sd_x")?;
            if !(sx > 0.0) {
                return None;
            }
            let (es, fs) = (s.out("exact_sd")?, s.out("first_order_sd")?);
            let mut sr = curve_pairs(s, "sd_x", &k::span(sx * 0.06, sx * 3.0, 41, false), &["exact_sd", "first_order_sd"]);
            sr[0].0 = "exact".into();
            sr[1].0 = "first order".into();
            series = sr;
            xlab = "Standard deviation of X"; ylab = "Standard deviation of X·Y"; logx = false;
            header = format!("exact {}, first order {}", num(es), num(fs));
            mark = Some((sx, es));
            mlabel = format!("sd of X = {}", num(sx));
            title = "Exact against first-order uncertainty of a product";
            cap = format!("The standard deviation of the product of independent normals X (mean {}) and Y (mean {}, sd {}) \
                           as the sd of X runs from {} to {}: exact and first-order values, each computed by the model. \
                           Marked: the sd of X given, {}: exact {}, first order {}.", num(s.inp("mean_x")?),
                          num(s.inp("mean_y")?), num(s.inp("sd_y")?), num(sx * 0.06), num(sx * 3.0), num(sx), num(es),
                          num(fs));
        }
        "uncertainty_product_correlated" => {
            let r = s.inp("rho")?;
            let (up, uq) = (s.out("u_product")?, s.out("u_quotient")?);
            let mut sr = curve_pairs(s, "rho", &k::span(-1.0, 1.0, 41, false), &["u_product", "u_quotient"]);
            sr[0].0 = "u(a·b)".into();
            sr[1].0 = "u(a/b)".into();
            series = sr;
            xlab = "Correlation ρ"; ylab = "Standard uncertainty"; logx = false;
            header = format!("u(a·b) = {}, u(a/b) = {}", num(up), num(uq));
            mark = Some((r, up));
            mlabel = format!("ρ = {}", num(r));
            title = "Product and quotient uncertainty against correlation";
            cap = format!("Uncertainties of the product and the quotient of a = {} ± {} and b = {} ± {} as their \
                           correlation runs from -1 to 1, each computed by the model: positive correlation inflates the \
                           product's uncertainty and cancels the quotient's. Marked: ρ = {}: u(a·b) = {}, u(a/b) = {}.",
                          num(s.inp("a")?), num(s.inp("ua")?), num(s.inp("b")?), num(s.inp("ub")?), num(r), num(up),
                          num(uq));
        }
        _ => {
            let (uc, nu, kf) = (s.out("combined_uncertainty")?, s.out("effective_dof")?, s.out("coverage_factor")?);
            let lv = s.inp("level").unwrap_or(0.95);
            let other = if (lv - 0.99).abs() < 1e-9 { 0.95 } else { 0.99 };
            let dofs = k::span(2.0, 1000.0, 14, true);
            let mut series_v = vec![];
            for l in [Some(lv), Some(other)] {
                let mut pts = vec![];
                for d in &dofs {
                    let mut args = vec![("uncertainties", jv(&[uc])), ("dof", jv(&[d.round()]))];
                    if let Some(l) = l {
                        args.push(("level", fl(l)));
                    }
                    if let Some(v) = s.with_input(&args).and_then(|a| out(&a, "coverage_factor")) {
                        pts.push((d.round(), v));
                    }
                }
                series_v.push((format!("level {}", num(l.unwrap_or(lv))), pts));
            }
            series = series_v;
            xlab = "Effective degrees of freedom"; ylab = "Coverage factor k"; logx = true;
            header = format!("k = {}, U = {}", num(kf), num(s.out("expanded_uncertainty")?));
            mark = Some((nu, kf));
            mlabel = format!("ν = {}", num(nu));
            title = "Coverage factor against degrees of freedom";
            cap = format!("The coverage factor k that turns a combined standard uncertainty into an expanded one, against \
                           the degrees of freedom (14 values from 2 to 1000, levels {} and {}; each computed by the \
                           model): few degrees of freedom need a large k. Marked: the inputs given, {} effective degrees \
                           of freedom: k = {}, expanded uncertainty {}.", num(lv), num(other), num(nu), num(kf),
                          num(s.out("expanded_uncertainty")?));
        }
    }
    if series.iter().all(|x| x.1.len() < 3) {
        return None;
    }
    let body = k::lines(&series, xlab, ylab, &header, mark, &mlabel, logx);
    Some(s.chart("curves", title, &cap, &body))
}

fn summary_band(s: &Solved) -> Option<Figure> {
    let n0 = int_in(s, "n")?;
    let (pm, lo, hi) = (s.out("posterior_mean")?, s.out("lower")?, s.out("upper")?);
    let mut ns: Vec<i64> = k::span(1.0, 100.0f64.max(n0 as f64 * 2.0), 17, true).iter().map(|x| x.round() as i64).collect();
    ns.push(n0);
    ns.sort();
    ns.dedup();
    let mut m = vec![];
    let mut up = vec![];
    let mut dn = vec![];
    for n in &ns {
        let Some(a) = s.with_input(&[("n", int(*n))]) else { continue };
        if let (Some(x), Some(l), Some(u)) = (out(&a, "posterior_mean"), out(&a, "lower"), out(&a, "upper")) {
            m.push((*n as f64, x));
            dn.push((*n as f64, l));
            up.push((*n as f64, u));
        }
    }
    if m.len() < 5 {
        return None;
    }
    let ys: Vec<f64> = up.iter().chain(&dn).map(|p| p.1).collect();
    let (y0, y1) = minmax(&ys);
    let x1 = m.last()?.0;
    let mut p = Plot::new((1.0, x1), (y0, y1), "Sample size n", "Posterior mean and interval",
                          &format!("n = {n0}: mean {}, [{}, {}]", num(pm), num(lo), num(hi)), (true, false), true);
    let mut poly = up.clone();
    poly.extend(dn.iter().rev().copied());
    p.polygon(&poly, "axp-s", "");
    p.path(&m, "axp-c");
    p.dashed(&[(1.0, s.inp("sample_mean")?), (x1, s.inp("sample_mean")?)], "axp-c4");
    p.mark((n0 as f64, pm), &format!("n = {n0}"));
    let lv = s.inp("level").map(|l| format!("{} % ", num(l * 100.0))).unwrap_or_default();
    Some(s.chart("band", "Normal posterior shrinking towards the data as n grows",
                 &format!("A normal prior (mean {}, sd {}) and data with σ = {} and sample mean {} (dashed): the model's \
                           posterior mean and {lv}credible interval (shaded) for {} sample sizes from 1 to {}, moving from \
                           the prior towards the data. Marked: n = {n0}, posterior mean {}, interval [{}, {}].",
                          num(s.inp("prior_mean")?), num(s.inp("prior_sd")?), num(s.inp("sigma")?),
                          num(s.inp("sample_mean")?), m.len(), num(x1), num(pm), num(lo), num(hi)), &p.body()))
}

// ------------------------------------------------------------------------------------------------ fields
enum Rng {
    /// half to one and a half times the value (-1 to 1 for zero)
    Near,
    /// a tenth to ten times (logarithmic)
    Decade,
    /// a probability from 0.02 to 0.98
    Prob,
}

fn range_of(x0: f64, r: &Rng) -> Option<(Vec<f64>, bool)> {
    match r {
        Rng::Near if x0 != 0.0 => Some((k::span((x0 * 0.5).min(x0 * 1.5), (x0 * 0.5).max(x0 * 1.5), 15, false), false)),
        Rng::Near => Some((k::span(-1.0, 1.0, 15, false), false)),
        Rng::Decade if x0 > 0.0 => Some((k::span(x0 / 10.0, x0 * 10.0, 15, true), true)),
        Rng::Prob => Some((k::span(0.02, 0.98, 15, false), false)),
        _ => None,
    }
}

fn meaning(s: &Solved, sym: &str, input: bool) -> String {
    let p = s.p.procedure(s.key);
    let m = if input {
        p.and_then(|p| p.inputs.iter().find(|i| i.symbol == sym)).map(|i| i.meaning.clone())
    } else {
        p.and_then(|p| p.outputs.iter().find(|o| o.symbol == sym)).map(|o| o.meaning.clone())
    };
    m.unwrap_or_else(|| sym.to_string())
}

fn axis(s: &Solved, sym: &str) -> String {
    let m = meaning(s, sym, true);
    let m: String = m.split(" (").next().unwrap_or(&m).to_string();
    let t = format!("{sym}: {m}");
    if t.chars().count() > 46 { sym.to_string() } else { t }
}

/// An output over a grid of two number inputs, each cell one computation of the model.
fn field2(s: &Solved, xn: &str, xr: Rng, yn: &str, yr: Rng, out_: &str, title: &str) -> Option<Figure> {
    let (x0, y0) = (s.inp(xn)?, s.inp(yn)?);
    let z0 = s.out(out_)?;
    let (xs, xlog) = range_of(x0, &xr)?;
    let (ys, ylog) = range_of(y0, &yr)?;
    let mut vals = vec![];
    let mut n = 0;
    for x in &xs {
        let mut cl = vec![];
        for y in &ys {
            let v = s.with(&[(xn, *x), (yn, *y)]).and_then(|a| out(&a, out_));
            n += v.is_some() as usize;
            cl.push(v);
        }
        vals.push(cl);
    }
    if n < 4 {
        return None;
    }
    let all: Vec<f64> = vals.iter().flatten().flatten().copied().collect();
    let (lo, hi) = minmax(&all);
    let logz = lo > 0.0 && hi / lo > 1e3;
    let shade: Vec<Vec<Option<f64>>> = if logz {
        vals.iter().map(|c| c.iter().map(|v| v.map(|x| x.log10())).collect()).collect()
    } else {
        vals.clone()
    };
    let body = k::field(&xs, &ys, &shade, (x0, y0), &axis(s, xn), &axis(s, yn),
                        &format!("{out_} = {}{}", num(z0), if logz { " (shaded by log10)" } else { "" }), (xlog, ylog));
    let span_t = |v: &[f64]| format!("{} to {}", num(v[0]), num(v[v.len() - 1]));
    Some(s.chart("field", title,
                 &format!("{} ({out_}) over {xn} from {} and {yn} from {}{}: each of the {n} shaded cells is one \
                           computation of the model, darker for a larger value ({}from {} to {}; a blank cell: inputs \
                           the model refuses). Marked: the inputs given, {out_} = {}.", meaning(s, out_, false),
                          span_t(&xs), span_t(&ys), if xlog || ylog { " (logarithmic axes)" } else { "" },
                          if logz { "shaded by the logarithm of the value, " } else { "" }, num(lo), num(hi), num(z0)),
                 &body))
}

// ------------------------------------------------------------------------------------------------ single drawings
fn bayes_area(s: &Solved) -> Option<Figure> {
    let (pa, pba, pbn) = (s.inp("p_a")?, s.inp("p_b_given_a")?, s.inp("p_b_given_not_a")?);
    let (post, pb, pnot) = (s.out("p_a_given_b")?, s.out("p_b")?, s.out("p_not_a_given_b")?);
    if !(0.0..=1.0).contains(&pa) || !(0.0..=1.0).contains(&pba) || !(0.0..=1.0).contains(&pbn) {
        return None;
    }
    let (x0, y0, w, h) = (60.0, 60.0, 400.0, 220.0);
    let xa = x0 + w * pa;
    let mut c = Canvas::new(330.0);
    c.raw(&format!("<text class=\"axp-mt\" x=\"16\" y=\"20\">{}</text>",
                   esc(&format!("P(A | B) = {}", num(post)))));
    c.raw(&format!("<rect class=\"axp-ka\" x=\"{x0}\" y=\"{y0}\" width=\"{w}\" height=\"{h}\"/>"));
    let ha = h * pba;
    let hn = h * pbn;
    c.raw(&format!("<rect class=\"axp-b\" x=\"{x0:.1}\" y=\"{:.1}\" width=\"{:.1}\" height=\"{ha:.1}\"/>", y0 + h - ha, xa - x0));
    c.raw(&format!("<rect class=\"axp-b2\" x=\"{xa:.1}\" y=\"{:.1}\" width=\"{:.1}\" height=\"{hn:.1}\"/>", y0 + h - hn,
                   x0 + w - xa));
    c.raw(&format!("<rect class=\"axp-f\" x=\"{x0}\" y=\"{y0}\" width=\"{w}\" height=\"{h}\"/>"));
    c.line((xa, y0), (xa, y0 + h));
    c.name((x0 + xa) / 2.0, y0 - 8.0, "middle", &format!("A: {}", num(pa)));
    c.name((xa + x0 + w) / 2.0, y0 - 8.0, "middle", &format!("not A: {}", num(1.0 - pa)));
    if xa - x0 > 70.0 {
        c.value((x0 + xa) / 2.0, y0 + h - ha / 2.0 + 4.0, "middle", &format!("{} × {}", num(pa), num(pba)));
    }
    if x0 + w - xa > 70.0 {
        c.value((xa + x0 + w) / 2.0, (y0 + h - hn / 2.0 + 4.0).min(y0 + h - 6.0).max(y0 + h - hn - 6.0), "middle",
                &format!("{} × {}", num(1.0 - pa), num(pbn)));
    }
    c.value(260.0, 310.0, "middle", &format!("P(B) = {}; share inside A: P(A | B) = {}", num(pb), num(post)));
    Some(s.figure("area", "Bayes' theorem as areas of a unit square",
                  &format!("Bayes' theorem as areas in the unit square, to scale: the width splits the population into A \
                            (P(A) = {}) and not A; the shaded heights are P(B | A) = {} and P(B | not A) = {}. The shaded \
                            areas together are the computed P(B) = {}; the share of it inside A is the computed P(A | B) \
                            = {} (P(not A | B) = {}).", num(pa), num(pba), num(pbn), num(pb), num(post), num(pnot)),
                  &c.s, 330.0))
}

fn utility(s: &Solved) -> Option<Figure> {
    let log = s.key == "certainty_equivalent_log_utility";
    let (xs, pr) = (s.vec("outcomes")?, s.vec("probabilities")?);
    let eu_sym = if log { "expected_log_utility" } else { "expected_utility" };
    let (eu, ce, rp, ev) = (s.out(eu_sym)?, s.out("certainty_equivalent")?, s.out("risk_premium")?, s.out("expected_value")?);
    let (lo, hi) = minmax(&xs);
    if !(hi > lo) {
        return None;
    }
    let pad = (hi - lo) * 0.05;
    let a = if log { (lo - pad).max(lo * 0.5).max(1e-9) } else { lo - pad };
    let mut u = vec![];
    for x in k::span(a, hi + pad, 61, false) {
        if let Some(v) = s.with_input(&[("outcomes", jv(&[x, x])), ("probabilities", jv(&[0.5, 0.5]))]).and_then(|r| out(&r, eu_sym)) {
            u.push((x, v));
        }
    }
    if u.len() < 10 {
        return None;
    }
    let uat = |x: f64| s.with_input(&[("outcomes", jv(&[x, x])), ("probabilities", jv(&[0.5, 0.5]))]).and_then(|r| out(&r, eu_sym));
    let (ul, uh) = (uat(lo)?, uat(hi)?);
    let ys: Vec<f64> = u.iter().map(|p| p.1).collect();
    let (y0, y1) = minmax(&ys);
    let mut p = Plot::new((a, hi + pad), (y0, y1), "Outcome x", "Utility",
                          &format!("CE = {}, risk premium {}", num(ce), num(rp)), (false, false), true);
    p.path(&u, "axp-c");
    p.dashed(&[(lo, ul), (hi, uh)], "axp-c4");
    for x in &xs {
        if let Some(v) = uat(*x) {
            p.dot((*x, v), 4.0, "axp-d4");
        }
    }
    p.seg((ce, eu), (ev, eu), "axp-c3");
    p.dot((ce, eu), 5.0, "axp-d2");
    p.label((ce, eu), -8.0, -8.0, "end", "axp-vt", &format!("CE {}", num(ce)));
    p.mark((ev, eu), &format!("EV {}", num(ev)));
    let what = if log { "ln x".to_string() } else { format!("1 - exp(-x/{})", num(s.inp("risk_tolerance")?)) };
    Some(s.chart("utility", "Certainty equivalent and risk premium on the utility curve",
                 &format!("A lottery with outcomes {} and probabilities {}: the utility {what} is drawn from the model's \
                           expected utility of a sure outcome at 61 values. The computed expected utility {} sits on the \
                           chord (dashed) at the expected value {}; the sure amount with the same utility is the computed \
                           certainty equivalent {}, and the gap is the computed risk premium {}.", list_text(&xs),
                          list_text(&pr), num(eu), num(ev), num(ce), num(rp)), &p.body()))
}

fn control_fit(s: &Solved) -> Option<Figure> {
    let (y, c) = (s.vec("y")?, s.vec("c")?);
    let mu = s.inp("control_mean")?;
    let (est, b, plain, vr) = (s.out("estimate")?, s.out("coefficient")?, s.out("plain_estimate")?,
                               s.out("variance_reduction")?);
    if y.len() != c.len() || y.len() < 2 {
        return None;
    }
    let (c0, c1) = minmax(&c.iter().copied().chain([mu]).collect::<Vec<_>>());
    let pad = ((c1 - c0) * 0.1).max(1e-9);
    // the estimate the model returns for every known control mean: the regression line read there
    let line = s.curve("control_mean", &k::span(c0 - pad, c1 + pad, 21, false), "estimate", &[]);
    if line.len() < 2 {
        return None;
    }
    let ys: Vec<f64> = y.iter().copied().chain(line.iter().map(|p| p.1)).collect();
    let (y0, y1) = minmax(&ys);
    let mut p = Plot::new((c0 - pad, c1 + pad), (y0, y1), "Control c", "Draw y",
                          &format!("estimate {} (plain mean {})", num(est), num(plain)), (false, false), true);
    p.path(&line, "axp-c");
    p.dashed(&[(c0 - pad, plain), (c1 + pad, plain)], "axp-c4");
    for (a, b_) in c.iter().zip(&y) {
        p.dot((*a, *b_), 4.0, "axp-d4");
    }
    p.vline(mu, "axp-c3", &format!("known mean {}", num(mu)));
    p.mark((mu, est), &format!("{}", num(est)));
    Some(s.chart("fit", "Control variate: correcting the mean along the regression line",
                 &format!("A control-variate estimate for the paired draws y = {} and c = {} with the known mean of c = \
                           {}: the line is the model's estimate for every known mean of c (slope = the computed \
                           coefficient {}); read at the known mean it gives the computed estimate {} instead of the plain \
                           mean {} (dashed); computed variance factor {}.", list_text(&y), list_text(&c), num(mu), num(b),
                          num(est), num(plain), num(vr)), &p.body()))
}

/// The one-standard-deviation contour of a 2 x 2 covariance (None unless positive definite).
fn ellipse(m: &[Vec<f64>]) -> Option<Vec<P>> {
    let (a, b, d) = (m[0][0], (m[0][1] + m[1][0]) / 2.0, m[1][1]);
    if !(a > 0.0) || !(a * d - b * b > 0.0) {
        return None;
    }
    let l11 = a.sqrt();
    let l21 = b / l11;
    let l22 = (d - l21 * l21).sqrt();
    Some((0..=72).map(|i| {
        let t = i as f64 / 72.0 * std::f64::consts::TAU;
        (l11 * t.cos(), l21 * t.cos() + l22 * t.sin())
    }).collect())
}

fn ellipses(s: &Solved) -> Option<Figure> {
    let (am, cov) = (s.mat("matrix")?, s.mat("covariance")?);
    let outc: Vec<Vec<f64>> = s.outj("output_covariance")?.as_array()?.iter()
        .map(|r| r.as_array().map(|x| x.iter().filter_map(|v| v.as_f64().or_else(|| v.as_str()?.parse().ok())).collect()))
        .collect::<Option<_>>()?;
    let u = s.out_list("uncertainties")?;
    if cov.len() != 2 || cov[0].len() != 2 || outc.len() != 2 || outc.iter().any(|r| r.len() != 2) || u.len() != 2 {
        return None;
    }
    let e1 = ellipse(&cov)?;
    let e2 = ellipse(&outc)?;
    let r = e1.iter().chain(&e2).map(|p| p.0.abs().max(p.1.abs())).fold(0.0, f64::max).max(u[0]).max(u[1]) * 1.15;
    let asp = (504.0 - 96.0) / (278.0 - 44.0);
    let mut p = Plot::new((-r * asp, r * asp), (-r, r), "First component", "Second component",
                          &format!("output uncertainties {}, {}", num(u[0]), num(u[1])), (false, false), false);
    p.polygon(&e1, "axp-s3", "axp-c3");
    p.polygon(&e2, "axp-s", "axp-c");
    p.seg((-u[0], 0.0), (u[0], 0.0), "axp-c2");
    p.seg((0.0, -u[1]), (0.0, u[1]), "axp-c2");
    p.legend(&[("input Σ".into(), "axp-b3"), ("output A Σ Aᵀ".into(), "axp-b"), ("± uncertainties".into(), "axp-b2")]);
    Some(s.chart("ellipses", "Uncertainty ellipses before and after a linear map",
                 &format!("Linear propagation of a covariance: the input covariance {} through the matrix A = {} gives the \
                           computed output covariance {} and uncertainties {}, {} (green bars along the axes). Each \
                           ellipse is the one-standard-deviation contour of its covariance matrix (from its Cholesky \
                           factor); the tilt shows the correlation.", mat_text(&cov), mat_text(&am), mat_text(&outc),
                          num(u[0]), num(u[1])), &p.body()))
}

fn criteria_table(s: &Solved) -> Option<Figure> {
    let m = s.mat("payoffs")?;
    let given = s.txt("criterion")?;
    let h = s.inp("optimism").unwrap_or(0.5);
    let crit = ["maximin", "maximax", "laplace", "minimax regret", "hurwicz"];
    if m.len() > 6 {
        return None;
    }
    let mut rows = vec![];
    let mut mark = None;
    let mut notes = vec![];
    for (i, c) in crit.iter().enumerate() {
        let mut args = vec![("criterion", Input::Text(c.to_string()))];
        if *c == "hurwicz" {
            args.push(("optimism", fl(h)));
        }
        let Some(a) = s.with_input(&args) else { continue };
        let (Some(sc), Some(best)) = (lists(&a, "scores"), out(&a, "best_action")) else { continue };
        let mut r = vec![if *c == "hurwicz" { format!("hurwicz {}", num(h)) } else { c.to_string() }];
        r.extend(sc.iter().map(|x| num(*x)));
        r.push(format!("{}", num(best)));
        notes.push(format!("{c}: action {}", num(best)));
        if given.eq_ignore_ascii_case(c) {
            mark = Some(rows.len());
        }
        rows.push(r);
        let _ = i;
    }
    if rows.len() < 2 {
        return None;
    }
    let mut head = vec!["criterion".to_string()];
    head.extend((1..=m.len()).map(|i| format!("action {i}")));
    head.push("best".into());
    let (body, hh) = k::table(&head, &rows, mark, &format!("criterion given: {given}"));
    Some(s.figure("table", "Action scores under five decision criteria",
                  &format!("The score of each action of the payoff table {} under five criteria (Hurwicz optimism {}), \
                            every score computed by the model; the regret row is minimised, the others maximised. Best \
                            action: {}. Outlined: the criterion given.", mat_text(&m), num(h), notes.join("; ")), &body, hh))
}

fn io(s: &Solved) -> Option<Figure> {
    let mut c = Canvas::new(250.0);
    c.block(20.0, 70.0, 170.0, 110.0, "");
    c.block(320.0, 70.0, 180.0, 110.0, "");
    c.arrow((195.0, 125.0), (315.0, 125.0));
    c.bold(105.0, 60.0, "middle", "stated");
    c.bold(410.0, 60.0, "middle", "computed");
    c.name(255.0, 115.0, "middle", "Sobol indices");
    let mut y = 98.0;
    for (n, _) in s.given.iter().take(4) {
        let v = s.vec(n).map(|v| list_text(&v)).or_else(|| s.inp(n).map(num)).unwrap_or_default();
        c.value(30.0, y, "start", &format!("{n} = {v}"));
        y += 20.0;
    }
    let mut y = 98.0;
    for (n, v, _) in s.a.values.iter().take(4) {
        let t = match v {
            Value::Array(a) => a.iter().filter_map(|x| x.as_f64()).map(num).collect::<Vec<_>>().join(", "),
            Value::Number(x) => num(x.as_f64()?),
            other => other.to_string(),
        };
        c.value(330.0, y, "start", &format!("{n} = {}", if t.chars().count() > 18 { format!("{}...", t.chars().take(15).collect::<String>()) } else { t }));
        y += 20.0;
    }
    let first = s.out_list("first_order")?;
    let total = s.out_list("total_order")?;
    c.value(260.0, 220.0, "middle", &format!("first order {}; total {}", list_text(&first), list_text(&total)));
    Some(s.figure("io", "Sobol indices of the G-function: inputs and results",
                  &format!("The inputs stated and the results computed by the model: a = {}; first_order = {}, \
                            total_order = {}, variance = {}.", list_text(&s.vec("a")?), list_text(&first),
                           list_text(&total), num(s.out("variance")?)), &c.s, 250.0))
}

fn halton(s: &Solved) -> Option<Figure> {
    let idx = int_in(s, "index")?;
    let mut pts = vec![];
    let mut mine = None;
    for i in 1..=128i64 {
        let Some(p) = s.with_input(&[("index", int(i)), ("dimension", int(2))]).and_then(|a| lists(&a, "point")) else { continue };
        if p.len() >= 2 {
            pts.push((p[0], p[1]));
            if i == idx {
                mine = Some((p[0], p[1]));
            }
        }
    }
    if mine.is_none() {
        mine = s.with_input(&[("dimension", int(2))]).and_then(|a| lists(&a, "point")).filter(|p| p.len() >= 2)
            .map(|p| (p[0], p[1]));
    }
    if pts.len() < 16 {
        return None;
    }
    let asp = (504.0 - 96.0) / (278.0 - 44.0);
    let mut p = Plot::new((0.5 - 0.5 * asp, 0.5 + 0.5 * asp), (0.0, 1.0), "First coordinate (base 2)",
                          "Second coordinate (base 3)", "the first 128 points", (false, false), false);
    p.polygon(&[(0.0, 0.0), (1.0, 0.0), (1.0, 1.0), (0.0, 1.0)], "", "axp-c4");
    for q in &pts {
        p.dot(*q, 2.6, "axp-d4");
    }
    if let Some(m) = mine {
        p.mark(m, &format!("index {idx}"));
    }
    Some(s.chart("points", "Halton points filling the unit square",
                 &format!("The first {} points of the two-dimensional Halton sequence (bases 2 and 3), each computed by \
                           the model: they fill the unit square evenly, without the clumps and gaps of random points.{}",
                          pts.len(), mine.map(|m| format!(" Marked: index {idx}, point ({}, {}).", num(m.0), num(m.1)))
                              .unwrap_or_default()), &p.body()))
}

fn propagation(s: &Solved) -> Option<Figure> {
    let (x, ux) = (s.inp("x")?, s.inp("ux")?);
    let (f, uf) = (s.out("value")?, s.out("uncertainty")?);
    if !(ux > 0.0) {
        return None;
    }
    let curve = s.curve("x", &k::span(x - 4.0 * ux, x + 4.0 * ux, 61, false), "value", &[]);
    if curve.len() < 10 {
        return None;
    }
    let ys: Vec<f64> = curve.iter().map(|p| p.1).chain([f - uf, f + uf]).collect();
    let (y0, y1) = minmax(&ys);
    let (xa, xb) = (curve[0].0, curve[curve.len() - 1].0);
    let mut p = Plot::new((xa, xb), (y0, y1), "x", "f(x)", &format!("f = {} ± {}", num(f), num(uf)), (false, false), true);
    let big = (y1 - y0).abs() * 10.0 + 1.0;
    p.polygon(&[(x - ux, y0 - big), (x + ux, y0 - big), (x + ux, y1 + big), (x - ux, y1 + big)], "axp-s3", "");
    p.polygon(&[(xa - big, f - uf), (xb + big, f - uf), (xb + big, f + uf), (xa - big, f + uf)], "axp-s2", "");
    p.path(&curve, "axp-c");
    p.mark((x, f), &format!("x = {} ± {}", num(x), num(ux)));
    let what = if s.key == "uncertainty_power" { format!("x^{}", num(s.inp("exponent")?)) } else {
        format!("the function {}", s.txt("function")?)
    };
    Some(s.chart("propagation", &format!("An uncertainty carried through {} by its slope",
                                         if s.key == "uncertainty_power" { "a power" } else { "a function" }),
                 &format!("Uncertainty of {what}: f is drawn from the model's value at 61 points around x = {}; the \
                           vertical band is the stated x ± u(x) = {} ± {}, the horizontal band the computed f = {} ± {}. \
                           The slope of f at x carries one band into the other.", num(x), num(x), num(ux), num(f),
                          num(uf)), &p.body()))
}

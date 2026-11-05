//! Bayesian Statistics' own live figures, part one: densities (each drawn from the model's own quantiles or
//! cumulative probabilities, differenced), stems of whole-number outcomes and credible intervals. Every number drawn
//! is the model's: the answer, the inputs as read, and further solves of the model itself
//! (ax_engine::figkit::Solved). A figure that cannot be drawn faithfully for the inputs is left out.
//! Part two (bars, curves, matrices, fields and the rest) is in figures_more.rs.

use crate::figures_more as more;
use ax_engine::figkit::{self as k, num, Plot, Solved, P};
use ax_engine::figures::Figure;
use ax_engine::{Answer, Input};
use serde_json::json;

pub fn live(s: &Solved) -> Vec<Figure> {
    let v: Vec<Option<Figure>> = match s.key {
        "ab_test_probability_beta" => vec![ab_test(s)],
        "beta_binomial_posterior" => vec![beta_binomial_posterior(s), more::posterior_mean_heat(s)],
        "beta_credible_interval" | "gamma_credible_interval" => vec![credible_density(s)],
        "beta_hdi" | "gamma_hdi" => vec![hdi_density(s)],
        "beta_posterior_tail_probability" => vec![tail_density(s)],
        "beta_prior_from_mean_sd" => vec![beta_prior(s)],
        "gamma_exponential_posterior" | "gamma_poisson_posterior" => vec![gamma_update(s)],
        "kl_divergence_beta" => vec![kl_beta(s)],
        "kl_divergence_normal" => vec![kl_normal(s)],
        "laplace_approximation_beta" => vec![laplace(s)],
        "map_beta_binomial" | "map_gamma_poisson" => vec![map_density(s)],
        "normal_inverse_gamma_posterior" => vec![nig_density(s)],
        "normal_normal_posterior" => vec![normal_update(s)],
        "normal_posterior_predictive" => vec![predictive_density(s)],
        "probability_rate_greater_gamma" => vec![two_rates(s)],
        "bayes_factor_binomial_simple" | "bayes_factor_point_null_binomial" => vec![bf_stems(s)],
        "beta_binomial_evidence" => vec![evidence_stems(s)],
        "beta_binomial_predictive" => vec![predictive_stems(s)],
        "gamma_poisson_predictive" => vec![poisson_predictive_stems(s)],
        "gauss_hermite_rule" => vec![hermite_stems(s)],
        "sequential_test_posterior" => vec![sequential_stems(s)],
        "beta_binomial_credible_interval" => vec![beta_binomial_intervals(s)],
        "gamma_poisson_credible_interval" => vec![gamma_poisson_intervals(s)],
        "jeffreys_interval" => vec![jeffreys_intervals(s)],
        "inverse_variance_weighted_mean" | "uncertainty_sum" | "stratified_sampling_estimate" => {
            vec![measurement_intervals(s)]
        }
        _ => more::live(s),
    };
    v.into_iter().flatten().collect()
}

// ------------------------------------------------------------------------------------------------ helpers
pub(crate) fn fl(x: f64) -> Input {
    Input::Float(x)
}

pub(crate) fn int(n: i64) -> Input {
    Input::Int(n)
}

pub(crate) fn jv(v: &[f64]) -> Input {
    Input::Json(json!(v))
}

/// A whole-number input as read.
pub(crate) fn int_in(s: &Solved, sym: &str) -> Option<i64> {
    match s.val(sym)? {
        ax_engine::Val::Int(i) => Some(i),
        ax_engine::Val::Num(x) if x.fract() == 0.0 => Some(x as i64),
        _ => None,
    }
}

pub(crate) fn out(a: &Answer, sym: &str) -> Option<f64> {
    k::num_of(a, sym)
}

pub(crate) fn list_text(v: &[f64]) -> String {
    v.iter().map(|x| num(*x)).collect::<Vec<_>>().join(", ")
}

pub(crate) fn minmax(v: &[f64]) -> (f64, f64) {
    v.iter().fold((f64::INFINITY, f64::NEG_INFINITY), |(a, b), x| (a.min(*x), b.max(*x)))
}

/// Quantiles (p, x) of a distribution from a capability that returns its equal-tailed interval (lower, upper) at a
/// level (and, when it has one, its median): about 60 cumulative probabilities from 0.0025 to 0.9975.
fn quantiles(s: &Solved, key: &str, args: &[(&str, Input)]) -> Option<Vec<(f64, f64)>> {
    let mut levels: Vec<f64> = (1..=24).map(|i| i as f64 / 25.0).collect();
    levels.extend([0.97, 0.98, 0.99, 0.995, 0.999, 0.9999]);
    let mut q: Vec<(f64, f64)> = vec![];
    for (n, l) in levels.iter().enumerate() {
        let mut a: Vec<(&str, Input)> = args.to_vec();
        a.push(("level", fl(*l)));
        let Some(r) = s.solve_other(key, &a) else { continue };
        let (lo, hi) = (out(&r, "lower"), out(&r, "upper"));
        if let (Some(lo), Some(hi)) = (lo, hi) {
            q.push(((1.0 - l) / 2.0, lo));
            q.push(((1.0 + l) / 2.0, hi));
        }
        if n == 0 {
            if let Some(m) = out(&r, "median") {
                q.push((0.5, m));
            }
        }
    }
    q.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap_or(std::cmp::Ordering::Equal));
    q.dedup_by(|a, b| (a.0 - b.0).abs() < 1e-12);
    // a quantile function never decreases: anything else is not drawn
    if q.len() < 20 || q.windows(2).any(|w| w[1].1 < w[0].1) {
        return None;
    }
    Some(q)
}

/// A density from quantiles: the probability of each small interval divided by its width, at the interval's middle.
fn density_from(q: &[(f64, f64)]) -> Vec<P> {
    q.windows(2).filter(|w| w[1].1 > w[0].1)
        .map(|w| ((w[0].1 + w[1].1) / 2.0, (w[1].0 - w[0].0) / (w[1].1 - w[0].1))).collect()
}

/// A density from cumulative probabilities F(x) at increasing x, differenced.
fn density_from_cdf(c: &[(f64, f64)]) -> Vec<P> {
    c.windows(2).filter(|w| w[1].0 > w[0].0 && w[1].1 >= w[0].1 - 1e-12)
        .map(|w| ((w[0].0 + w[1].0) / 2.0, ((w[1].1 - w[0].1) / (w[1].0 - w[0].0)).max(0.0))).collect()
}

pub(crate) fn beta_density(s: &Solved, a: f64, b: f64) -> Option<Vec<P>> {
    let q = quantiles(s, "beta_credible_interval", &[("alpha", fl(a)), ("beta", fl(b))])?;
    Some(density_from(&q))
}

pub(crate) fn gamma_density(s: &Solved, shape: f64, rate: f64) -> Option<Vec<P>> {
    let q = quantiles(s, "gamma_credible_interval", &[("shape", fl(shape)), ("rate", fl(rate))])?;
    Some(density_from(&q))
}

/// A normal density N(m, sd²) from the model's normal credible interval: a prior N(m, sd²) updated with one
/// observation whose spread is a billion times larger (the posterior is the prior itself, to double precision).
pub(crate) fn normal_density(s: &Solved, m: f64, sd: f64) -> Option<Vec<P>> {
    if !(sd > 0.0) {
        return None;
    }
    let q = quantiles(s, "normal_normal_summary", &[("prior_mean", fl(m)), ("prior_sd", fl(sd)), ("sigma", fl(sd * 1e9)),
                                                     ("sample_mean", fl(m)), ("n", int(1))])?;
    Some(density_from(&q))
}

/// The density's value at x (linear between its points; 0 outside).
fn at(pts: &[P], x: f64) -> f64 {
    for w in pts.windows(2) {
        if x >= w[0].0 && x <= w[1].0 {
            let t = if w[1].0 > w[0].0 { (x - w[0].0) / (w[1].0 - w[0].0) } else { 0.0 };
            return w[0].1 + t * (w[1].1 - w[0].1);
        }
    }
    0.0
}

/// The part of a curve between lo and hi, its ends interpolated.
fn between(pts: &[P], lo: f64, hi: f64) -> Vec<P> {
    let mut v = vec![(lo, at(pts, lo))];
    v.extend(pts.iter().copied().filter(|p| p.0 > lo && p.0 < hi));
    v.push((hi, at(pts, hi)));
    v
}

/// A frame for densities: x over every curve, y from 0 to the highest point.
fn density_plot(curves: &[&[P]], extra_x: &[f64], xlab: &str, header: &str) -> Option<Plot> {
    let xs: Vec<f64> = curves.iter().flat_map(|c| c.iter().map(|p| p.0)).chain(extra_x.iter().copied())
        .filter(|x| x.is_finite()).collect();
    let ys: Vec<f64> = curves.iter().flat_map(|c| c.iter().map(|p| p.1)).filter(|y| y.is_finite()).collect();
    if xs.len() < 4 || ys.is_empty() {
        return None;
    }
    let (x0, x1) = minmax(&xs);
    let (_, y1) = minmax(&ys);
    if !(x1 > x0) || !(y1 > 0.0) {
        return None;
    }
    Some(Plot::new((x0, x1), (0.0, y1 * 1.12), xlab, "Probability density", header, (false, false), false))
}

const LINES: [&str; 4] = ["axp-c", "axp-c2", "axp-c3", "axp-c4"];
const FILLS: [&str; 4] = ["axp-b", "axp-b2", "axp-b3", "axp-b4"];

/// Curves with a legend.
fn draw_curves(p: &mut Plot, curves: &[(String, &[P])]) {
    for (i, (_, c)) in curves.iter().enumerate() {
        p.path(c, LINES[i % 4]);
    }
    if curves.len() > 1 {
        let names: Vec<(String, &str)> = curves.iter().enumerate().map(|(i, c)| (c.0.clone(), FILLS[i % 4])).collect();
        p.legend(&names);
    }
}

fn beta_name(a: f64, b: f64) -> String {
    format!("Beta({}, {})", num(a), num(b))
}

fn gamma_name(a: f64, b: f64) -> String {
    format!("Gamma({}, {})", num(a), num(b))
}

// ------------------------------------------------------------------------------------------------ densities
fn ab_test(s: &Solved) -> Option<Figure> {
    let (aa, ba, ab, bb) = (s.inp("alpha_a")?, s.inp("beta_a")?, s.inp("alpha_b")?, s.inp("beta_b")?);
    let (pa, pb) = (s.out("prob_a_greater")?, s.out("prob_b_greater")?);
    let da = beta_density(s, aa, ba)?;
    let db = beta_density(s, ab, bb)?;
    let header = format!("P(pB > pA) = {}", num(pb));
    let mut p = density_plot(&[&da, &db], &[], "Conversion rate p", &header)?;
    draw_curves(&mut p, &[(format!("A: {}", beta_name(aa, ba)), &da), (format!("B: {}", beta_name(ab, bb)), &db)]);
    Some(s.chart("density", "Two conversion-rate posteriors and the chance B beats A",
                 &format!("The posteriors {} for A and {} for B, each density drawn from the model's quantiles at about \
                           60 cumulative probabilities (the probability of each small interval divided by its width). \
                           Computed: P(pB > pA) = {}, P(pA > pB) = {}.", beta_name(aa, ba), beta_name(ab, bb), num(pb),
                          num(pa)), &p.body()))
}

fn beta_binomial_posterior(s: &Solved) -> Option<Figure> {
    let (a, b) = (s.inp("alpha")?, s.inp("beta")?);
    let (k_, n) = (int_in(s, "successes")?, int_in(s, "trials")?);
    let (pa, pb, mean, sd) = (s.out("post_alpha")?, s.out("post_beta")?, s.out("posterior_mean")?, s.out("posterior_sd")?);
    let prior = beta_density(s, a, b)?;
    let post = beta_density(s, pa, pb)?;
    let ci = s.solve_other("beta_credible_interval", &[("alpha", fl(pa)), ("beta", fl(pb)), ("level", fl(0.95))])?;
    let (lo, hi) = (out(&ci, "lower")?, out(&ci, "upper")?);
    let header = format!("posterior {}, mean {}", beta_name(pa, pb), num(mean));
    let mut p = density_plot(&[&prior, &post], &[], "Success probability θ", &header)?;
    p.area(&between(&post, lo, hi), 0.0, "axp-s2");
    draw_curves(&mut p, &[(format!("prior {}", beta_name(a, b)), &prior), (format!("posterior {}", beta_name(pa, pb)), &post)]);
    p.vline(mean, "axp-c4", &format!("mean {}", num(mean)));
    Some(s.chart("density", &format!("Beta prior and posterior after {k_} successes in {n} trials"),
                 &format!("A {} prior updated with {k_} successes in {n} trials: the posterior {} with mean {} and \
                           standard deviation {}. Shaded: its 95 % equal-tailed credible interval [{}, {}]. Both \
                           densities are drawn from the model's quantiles at about 60 cumulative probabilities (the \
                           probability of each small interval divided by its width).", beta_name(a, b),
                          beta_name(pa, pb), num(mean), num(sd), num(lo), num(hi)), &p.body()))
}

fn credible_density(s: &Solved) -> Option<Figure> {
    let beta = s.key.starts_with("beta");
    let (a, b) = if beta { (s.inp("alpha")?, s.inp("beta")?) } else { (s.inp("shape")?, s.inp("rate")?) };
    let (lo, hi, med) = (s.out("lower")?, s.out("upper")?, s.out("median")?);
    let d = if beta { beta_density(s, a, b)? } else { gamma_density(s, a, b)? };
    let name = if beta { beta_name(a, b) } else { gamma_name(a, b) };
    let level = s.inp("level");
    let lv = level.map(|l| format!("{} % ", num(l * 100.0))).unwrap_or_default();
    let header = format!("{lv}interval [{}, {}]", num(lo), num(hi));
    let mut p = density_plot(&[&d], &[lo, hi], if beta { "θ" } else { "λ" }, &header)?;
    p.area(&between(&d, lo, hi), 0.0, "axp-s");
    p.path(&d, "axp-c");
    p.vline(med, "axp-c4", &format!("median {}", num(med)));
    Some(s.chart("density", &format!("Equal-tailed credible interval of a {} density", if beta { "Beta" } else { "gamma" }),
                 &format!("The {name} density, drawn from the model's quantiles at about 60 cumulative probabilities (the \
                           probability of each small interval divided by its width). Shaded: the computed {lv}equal-tailed \
                           credible interval [{}, {}], the same probability left in each tail; dashed: the computed \
                           median {}.", num(lo), num(hi), num(med)), &p.body()))
}

fn hdi_density(s: &Solved) -> Option<Figure> {
    let beta = s.key.starts_with("beta");
    let (a, b) = if beta { (s.inp("alpha")?, s.inp("beta")?) } else { (s.inp("shape")?, s.inp("rate")?) };
    let (lo, hi, w) = (s.out("lower")?, s.out("upper")?, s.out("width")?);
    let d = if beta { beta_density(s, a, b)? } else { gamma_density(s, a, b)? };
    let name = if beta { beta_name(a, b) } else { gamma_name(a, b) };
    // the equal-tailed interval at the same level, when the level is stated
    let eti = s.inp("level").and_then(|l| {
        let key = if beta { "beta_credible_interval" } else { "gamma_credible_interval" };
        let args = if beta { [("alpha", fl(a)), ("beta", fl(b)), ("level", fl(l))] } else {
            [("shape", fl(a)), ("rate", fl(b)), ("level", fl(l))]
        };
        let r = s.solve_other(key, &args)?;
        Some((out(&r, "lower")?, out(&r, "upper")?))
    });
    let header = format!("highest-density interval [{}, {}]", num(lo), num(hi));
    let mut p = density_plot(&[&d], &[lo, hi], if beta { "θ" } else { "λ" }, &header)?;
    p.area(&between(&d, lo, hi), 0.0, "axp-s");
    p.path(&d, "axp-c");
    p.dashed(&[(lo, at(&d, lo)), (hi, at(&d, hi))], "axp-c3");
    if let Some((e0, e1)) = eti {
        p.vline(e0, "axp-c4", "");
        p.vline(e1, "axp-c4", "equal-tailed");
    }
    let lv = s.inp("level").map(|l| format!(" at level {}", num(l))).unwrap_or_default();
    Some(s.chart("density", &format!("Highest-density against equal-tailed interval ({})", if beta { "Beta" } else { "gamma" }),
                 &format!("The {name} density{lv}, drawn from the model's quantiles at about 60 cumulative probabilities \
                           (the probability of each small interval divided by its width). Shaded: the computed \
                           highest-density interval [{}, {}], width {}; the density is equal at both ends (dashed chord), \
                           so no shorter interval holds the same probability.{}", num(lo), num(hi), num(w),
                          eti.map(|(e0, e1)| format!(" Dashed verticals: the equal-tailed interval [{}, {}] at the same \
                                                       level, from the model's credible-interval capability.", num(e0),
                                                      num(e1))).unwrap_or_default()), &p.body()))
}

fn tail_density(s: &Solved) -> Option<Figure> {
    let c = s.inp("threshold")?;
    let (above, below) = (s.out("prob_above")?, s.out("prob_below")?);
    let xs = k::span(0.0025, 0.9975, 121, false);
    let cdf = s.curve("threshold", &xs, "prob_below", &[]);
    if cdf.len() < 40 {
        return None;
    }
    let d = density_from_cdf(&cdf);
    let header = format!("P(θ > {}) = {}", num(c), num(above));
    let mut p = density_plot(&[&d], &[], "Success probability θ", &header)?;
    let hi = d.last()?.0;
    if c < hi {
        p.area(&between(&d, c.max(d[0].0), hi), 0.0, "axp-s");
    }
    p.path(&d, "axp-c");
    p.vline(c, "axp-c4", &format!("c = {}", num(c)));
    let data = match (int_in(s, "successes"), int_in(s, "trials")) {
        (Some(k_), Some(n)) => format!(" after {k_} successes in {n} trials"),
        _ => String::new(),
    };
    Some(s.chart("density", "Posterior probability above a threshold",
                 &format!("The posterior of a success probability from a {} prior{data}, drawn from the model's P(θ < c) \
                           at 121 values of c (differenced). Shaded: the computed P(θ > {}) = {} (P(θ < {}) = {}).",
                          beta_name(s.inp("alpha")?, s.inp("beta")?), num(c), num(above), num(c), num(below)), &p.body()))
}

fn beta_prior(s: &Solved) -> Option<Figure> {
    let (m, sd) = (s.inp("mean")?, s.inp("sd")?);
    let (a, b, n) = (s.out("alpha")?, s.out("beta")?, s.out("prior_sample_size")?);
    let d = beta_density(s, a, b)?;
    let header = format!("alpha = {}, beta = {}", num(a), num(b));
    let mut p = density_plot(&[&d], &[], "θ", &header)?;
    p.area(&between(&d, (m - sd).max(d[0].0), (m + sd).min(d[d.len() - 1].0)), 0.0, "axp-s");
    p.path(&d, "axp-c");
    p.vline(m, "axp-c4", &format!("mean {}", num(m)));
    Some(s.chart("density", "Beta prior matched to a mean and standard deviation",
                 &format!("The Beta prior with the stated mean {} and standard deviation {}: the computed alpha = {}, beta \
                           = {} (prior sample size {}), its density drawn from the model's quantiles at about 60 \
                           cumulative probabilities. Shaded: mean ± one standard deviation.", num(m), num(sd), num(a),
                          num(b), num(n)), &p.body()))
}

fn gamma_update(s: &Solved) -> Option<Figure> {
    let (a, b) = (s.inp("shape")?, s.inp("rate")?);
    let (pa, pb, mean) = (s.out("post_shape")?, s.out("post_rate")?, s.out("posterior_mean")?);
    let prior = gamma_density(s, a, b)?;
    let post = gamma_density(s, pa, pb)?;
    let ci = s.solve_other("gamma_credible_interval", &[("shape", fl(pa)), ("rate", fl(pb)), ("level", fl(0.95))])?;
    let (lo, hi) = (out(&ci, "lower")?, out(&ci, "upper")?);
    let (data, what) = if s.key == "gamma_poisson_posterior" {
        (format!("the Poisson counts {}", list_text(&s.vec("counts")?)), "a Poisson rate")
    } else {
        (format!("the waiting times {}", list_text(&s.vec("times")?)), "an exponential rate")
    };
    let header = format!("posterior {}, mean {}", gamma_name(pa, pb), num(mean));
    let mut p = density_plot(&[&prior, &post], &[], "Rate λ", &header)?;
    p.area(&between(&post, lo, hi), 0.0, "axp-s2");
    draw_curves(&mut p, &[(format!("prior {}", gamma_name(a, b)), &prior), (format!("posterior {}", gamma_name(pa, pb)), &post)]);
    p.vline(mean, "axp-c4", &format!("mean {}", num(mean)));
    Some(s.chart("density", &format!("Gamma prior and posterior for {what}"),
                 &format!("A gamma prior (shape {}, rate {}) updated with {data}: the computed posterior has shape {} and \
                           rate {}, mean {}. Densities drawn from the model's quantiles at about 60 cumulative \
                           probabilities (the probability of each small interval divided by its width). Shaded: the \
                           posterior 95 % equal-tailed interval [{}, {}].", num(a), num(b), num(pa), num(pb), num(mean),
                          num(lo), num(hi)), &p.body()))
}

fn kl_beta(s: &Solved) -> Option<Figure> {
    let (a1, b1, a2, b2) = (s.inp("alpha1")?, s.inp("beta1")?, s.inp("alpha2")?, s.inp("beta2")?);
    let kl = s.out("kl")?;
    let back = s.with(&[("alpha1", a2), ("beta1", b2), ("alpha2", a1), ("beta2", b1)]).and_then(|a| out(&a, "kl"))?;
    let dp = beta_density(s, a1, b1)?;
    let dq = beta_density(s, a2, b2)?;
    let header = format!("KL(P‖Q) = {}, KL(Q‖P) = {} nats", num(kl), num(back));
    let mut p = density_plot(&[&dp, &dq], &[], "θ", &header)?;
    draw_curves(&mut p, &[(format!("P = {}", beta_name(a1, b1)), &dp), (format!("Q = {}", beta_name(a2, b2)), &dq)]);
    Some(s.chart("density", "Two Beta densities and their divergences",
                 &format!("The densities P = {} and Q = {}, each drawn from the model's quantiles at about 60 cumulative \
                           probabilities. Computed: KL(P‖Q) = {} nats and, with the two swapped, KL(Q‖P) = {} nats; the \
                           divergence is not symmetric.", beta_name(a1, b1), beta_name(a2, b2), num(kl), num(back)),
                 &p.body()))
}

fn kl_normal(s: &Solved) -> Option<Figure> {
    let (m1, s1, m2, s2) = (s.inp("mean1")?, s.inp("sd1")?, s.inp("mean2")?, s.inp("sd2")?);
    let (k12, k21) = (s.out("kl_1_to_2")?, s.out("kl_2_to_1")?);
    let wide = s.with(&[("sd2", s2 * 2.0)]);
    let (w12, w21) = (wide.as_ref().and_then(|a| out(a, "kl_1_to_2")), wide.as_ref().and_then(|a| out(a, "kl_2_to_1")));
    let d1 = normal_density(s, m1, s1)?;
    let d2 = normal_density(s, m2, s2)?;
    let d3 = if w12.is_some() { normal_density(s, m2, s2 * 2.0) } else { None };
    let header = format!("KL(1‖2) = {}, KL(2‖1) = {}", num(k12), num(k21));
    let mut curves: Vec<(String, &[P])> = vec![(format!("N1 = N({}, {}²)", num(m1), num(s1)), &d1),
                                               (format!("N2 = N({}, {}²)", num(m2), num(s2)), &d2)];
    if let Some(d3) = &d3 {
        curves.push((format!("N2' sd {}", num(s2 * 2.0)), d3));
    }
    let all: Vec<&[P]> = curves.iter().map(|c| c.1).collect();
    let mut p = density_plot(&all, &[], "x", &header)?;
    draw_curves(&mut p, &curves);
    let extra = match (w12, w21, &d3) {
        (Some(a), Some(b), Some(_)) => format!(" With the second standard deviation doubled to {} (N2'): KL(1‖2) = {}, \
                                                KL(2‖1) = {}.", num(s2 * 2.0), num(a), num(b)),
        _ => String::new(),
    };
    Some(s.chart("density", "Normal densities and their divergences",
                 &format!("The normal densities N1 = N({}, {}²) and N2 = N({}, {}²), drawn from the model's normal \
                           quantiles at about 60 cumulative probabilities. Computed divergences in nats: KL(1‖2) = {}, \
                           KL(2‖1) = {}.{extra}", num(m1), num(s1), num(m2), num(s2), num(k12), num(k21)), &p.body()))
}

fn laplace(s: &Solved) -> Option<Figure> {
    let (a, b) = (s.inp("alpha")?, s.inp("beta")?);
    let (mode, lsd, lo, hi) = (s.out("mode")?, s.out("laplace_sd")?, s.out("lower")?, s.out("upper")?);
    let exact = beta_density(s, a, b)?;
    let approx = normal_density(s, mode, lsd)?;
    let lv = s.inp("level");
    let eti = lv.and_then(|l| {
        let r = s.solve_other("beta_credible_interval", &[("alpha", fl(a)), ("beta", fl(b)), ("level", fl(l))])?;
        Some((out(&r, "lower")?, out(&r, "upper")?))
    });
    let header = format!("mode {}, Laplace sd {}", num(mode), num(lsd));
    let mut p = density_plot(&[&exact, &approx], &[], "θ", &header)?;
    draw_curves(&mut p, &[(beta_name(a, b), &exact), ("Laplace normal".into(), &approx)]);
    p.vline(lo, "axp-c4", "");
    p.vline(hi, "axp-c4", "Laplace interval");
    if let Some((e0, e1)) = eti {
        p.dot((e0, at(&exact, e0)), 5.0, "axp-d");
        p.dot((e1, at(&exact, e1)), 5.0, "axp-d");
    }
    Some(s.chart("density", "Laplace approximation against the exact Beta density",
                 &format!("The {} density (from the model's quantiles at about 60 cumulative probabilities, differenced) \
                           against its Laplace approximation, a normal density with the computed mode {} and standard \
                           deviation {}. Dashed verticals: the computed Laplace interval [{}, {}]{}.", beta_name(a, b),
                          num(mode), num(lsd), num(lo), num(hi),
                          eti.map(|(e0, e1)| format!("; dots: the exact equal-tailed interval [{}, {}]", num(e0), num(e1)))
                              .unwrap_or_default()), &p.body()))
}

fn map_density(s: &Solved) -> Option<Figure> {
    let (map, mle, mean) = (s.out("map")?, s.out("mle")?, s.out("posterior_mean")?);
    let beta = s.key == "map_beta_binomial";
    let (d, name, what) = if beta {
        let r = s.solve_other("beta_binomial_posterior", &[("alpha", fl(s.inp("alpha")?)), ("beta", fl(s.inp("beta")?)),
                                                            ("successes", int(int_in(s, "successes")?)),
                                                            ("trials", int(int_in(s, "trials")?))])?;
        let (pa, pb) = (out(&r, "post_alpha")?, out(&r, "post_beta")?);
        (beta_density(s, pa, pb)?, beta_name(pa, pb),
         format!("a {} prior and {} successes in {} trials", beta_name(s.inp("alpha")?, s.inp("beta")?),
                 int_in(s, "successes")?, int_in(s, "trials")?))
    } else {
        let counts = s.vec("counts")?;
        let r = s.solve_other("gamma_poisson_posterior", &[("shape", fl(s.inp("shape")?)), ("rate", fl(s.inp("rate")?)),
                                                            ("counts", jv(&counts))])?;
        let (pa, pb) = (out(&r, "post_shape")?, out(&r, "post_rate")?);
        (gamma_density(s, pa, pb)?, gamma_name(pa, pb),
         format!("a {} prior and the counts {}", gamma_name(s.inp("shape")?, s.inp("rate")?), list_text(&counts)))
    };
    let header = format!("MAP {}, MLE {}, mean {}", num(map), num(mle), num(mean));
    let mut p = density_plot(&[&d], &[mle], if beta { "θ" } else { "λ" }, &header)?;
    p.path(&d, "axp-c");
    p.vline(map, "axp-c", "");
    p.vline(mle, "axp-c3", "");
    p.vline(mean, "axp-c2", "");
    p.legend(&[("MAP".into(), "axp-b"), ("MLE".into(), "axp-b3"), ("mean".into(), "axp-b2")]);
    Some(s.chart("density", &format!("MAP, maximum likelihood and posterior mean ({})", if beta { "Beta" } else { "gamma" }),
                 &format!("The posterior {name} for {what} (posterior parameters from the model's posterior capability), \
                           its density drawn from the model's quantiles at about 60 cumulative probabilities. Dashed: the \
                           computed MAP {} (the peak), maximum-likelihood estimate {} and posterior mean {}.", num(map),
                          num(mle), num(mean)), &p.body()))
}

fn nig_density(s: &Solved) -> Option<Figure> {
    let (mu, kn, an, bn) = (s.out("mu_n")?, s.out("kappa_n")?, s.out("alpha_n")?, s.out("beta_n")?);
    let (lo, hi) = (s.out("lower")?, s.out("upper")?);
    // the marginal posterior of the mean from the model's intervals at many levels
    let mut levels: Vec<f64> = (1..=24).map(|i| i as f64 / 25.0).collect();
    levels.extend([0.97, 0.98, 0.99, 0.995]);
    let mut q = vec![(0.5, mu)];
    for l in levels {
        if let Some(a) = s.with(&[("level", l)]) {
            if let (Some(a0), Some(a1)) = (out(&a, "lower"), out(&a, "upper")) {
                q.push(((1.0 - l) / 2.0, a0));
                q.push(((1.0 + l) / 2.0, a1));
            }
        }
    }
    q.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap_or(std::cmp::Ordering::Equal));
    if q.len() < 20 || q.windows(2).any(|w| w[1].1 < w[0].1) {
        return None;
    }
    let d = density_from(&q);
    let lv = s.inp("level");
    let header = format!("μn = {}, interval [{}, {}]", num(mu), num(lo), num(hi));
    let mut p = density_plot(&[&d], &[lo, hi], "Mean μ", &header)?;
    p.area(&between(&d, lo, hi), 0.0, "axp-s");
    p.path(&d, "axp-c");
    Some(s.chart("density", "Marginal posterior of a normal mean with unknown variance",
                 &format!("The marginal posterior of the mean under a normal-inverse-gamma prior (μ0 = {}, κ0 = {}, α0 = \
                           {}, β0 = {}) and the data {}: computed μn = {}, κn = {}, αn = {}, βn = {}. Its density is drawn \
                           from the model's interval limits at about 30 levels (differenced). Shaded: the computed {}\
                           interval [{}, {}].", num(s.inp("mu0")?), num(s.inp("kappa0")?), num(s.inp("alpha0")?),
                          num(s.inp("beta0")?), list_text(&s.vec("data")?), num(mu), num(kn), num(an), num(bn),
                          lv.map(|l| format!("{} % ", num(l * 100.0))).unwrap_or_default(), num(lo), num(hi)), &p.body()))
}

fn normal_update(s: &Solved) -> Option<Figure> {
    let (m0, s0, sig) = (s.inp("prior_mean")?, s.inp("prior_sd")?, s.inp("sigma")?);
    let (mn, sn) = (s.out("posterior_mean")?, s.out("posterior_sd")?);
    // the data alone: the same update under a prior a billion times wider
    let flat = s.with(&[("prior_sd", (s0.abs() + sig.abs()) * 1e9)])?;
    let (md, sd) = (out(&flat, "posterior_mean")?, out(&flat, "posterior_sd")?);
    let dp = normal_density(s, m0, s0)?;
    let dd = normal_density(s, md, sd)?;
    let dn = normal_density(s, mn, sn)?;
    let header = format!("posterior mean {}, sd {}", num(mn), num(sn));
    let mut p = density_plot(&[&dp, &dd, &dn], &[], "Mean μ", &header)?;
    draw_curves(&mut p, &[("prior".into(), &dp), ("data".into(), &dd), ("posterior".into(), &dn)]);
    Some(s.chart("density", "Normal prior, data and posterior for a mean",
                 &format!("A normal prior for a mean (mean {}, sd {}) combined with the data {} of known σ = {}: the \
                           computed posterior mean {} and sd {} lie between prior and data, the posterior narrower than \
                           both. The data curve is the model's posterior under a prior a billion times wider (mean {}, sd \
                           {}); each curve is drawn from the model's normal quantiles.", num(m0), num(s0),
                          list_text(&s.vec("data")?), num(sig), num(mn), num(sn), num(md), num(sd)), &p.body()))
}

fn predictive_density(s: &Solved) -> Option<Figure> {
    let (m, sd, c) = (s.out("predictive_mean")?, s.out("predictive_sd")?, s.inp("threshold")?);
    let (above, below) = (s.out("prob_above")?, s.out("prob_below")?);
    if !(sd > 0.0) {
        return None;
    }
    let xs = k::span(m - 4.0 * sd, m + 4.0 * sd, 121, false);
    let cdf = s.curve("threshold", &xs, "prob_below", &[]);
    if cdf.len() < 40 {
        return None;
    }
    let d = density_from_cdf(&cdf);
    let header = format!("P(x > {}) = {}", num(c), num(above));
    let mut p = density_plot(&[&d], &[], "Next observation x", &header)?;
    let hi = d.last()?.0;
    if c < hi {
        p.area(&between(&d, c.max(d[0].0), hi), 0.0, "axp-s");
    }
    p.path(&d, "axp-c");
    p.vline(c, "axp-c4", &format!("c = {}", num(c)));
    Some(s.chart("density", "Posterior predictive density with a threshold",
                 &format!("The posterior predictive distribution of the next observation (prior mean {}, prior sd {}, σ = \
                           {}, sample mean {} from n = {}), drawn from the model's P(x < c) at 121 thresholds \
                           (differenced); computed predictive mean {}, sd {}. Shaded: P(x > {}) = {} (P(x < {}) = {}).",
                          num(s.inp("prior_mean")?), num(s.inp("prior_sd")?), num(s.inp("sigma")?),
                          num(s.inp("sample_mean")?), int_in(s, "n")?, num(m), num(sd), num(c), num(above), num(c),
                          num(below)), &p.body()))
}

fn two_rates(s: &Solved) -> Option<Figure> {
    let (a1, r1, a2, r2) = (s.inp("shape1")?, s.inp("rate1")?, s.inp("shape2")?, s.inp("rate2")?);
    let (p1, ratio) = (s.out("prob_1_greater")?, s.out("rate_ratio_mean")?);
    let d1 = gamma_density(s, a1, r1)?;
    let d2 = gamma_density(s, a2, r2)?;
    let header = format!("P(λ1 > λ2) = {}", num(p1));
    let mut p = density_plot(&[&d1, &d2], &[], "Rate λ", &header)?;
    draw_curves(&mut p, &[(format!("λ1: {}", gamma_name(a1, r1)), &d1), (format!("λ2: {}", gamma_name(a2, r2)), &d2)]);
    Some(s.chart("density", "Which of two rates is larger",
                 &format!("Two gamma posteriors for rates ({} and {}), each density drawn from the model's quantiles at \
                           about 60 cumulative probabilities. Computed: P(λ1 > λ2) = {}, and the ratio of the posterior \
                           means {}.", gamma_name(a1, r1), gamma_name(a2, r2), num(p1), num(ratio)), &p.body()))
}

// ------------------------------------------------------------------------------------------------ stems
/// Whole numbers from 0 to n, at most 61 of them (evenly spread when n is larger), always holding `keep`.
fn counts_upto(n: i64, keep: i64) -> Vec<i64> {
    if n < 0 {
        return vec![];
    }
    let mut v: Vec<i64> = if n <= 60 { (0..=n).collect() } else {
        (0..=60).map(|i| (i as f64 * n as f64 / 60.0).round() as i64).collect()
    };
    if (0..=n).contains(&keep) && !v.contains(&keep) {
        v.push(keep);
        v.sort();
    }
    v.dedup();
    v
}

/// A stem chart of one or two series (the second offset a little to the right) with the visitor's point marked.
fn stem_plot(series: &[(String, Vec<P>)], xlab: &str, ylab: &str, header: &str, mark: Option<(P, String)>) -> Option<String> {
    let all: Vec<P> = series.iter().flat_map(|s| s.1.iter().copied()).collect();
    if all.len() < 2 {
        return None;
    }
    let (x0, x1) = minmax(&all.iter().map(|p| p.0).collect::<Vec<_>>());
    let (y0, y1) = minmax(&all.iter().map(|p| p.1).collect::<Vec<_>>());
    let pad = ((x1 - x0) * 0.04).max(0.6);
    let (ylo, yhi) = if y1 - y0 == 0.0 && y0 == 0.0 { (-1.0, 1.0) } else { (y0.min(0.0), y1.max(0.0)) };
    let yp = (yhi - ylo).abs() * 0.08;
    let mut p = Plot::new((x0 - pad, x1 + pad), (if ylo < 0.0 { ylo - yp } else { 0.0 }, yhi + yp + 1e-300),
                          xlab, ylab, header, (false, false), false);
    let off = if series.len() > 1 { ((x1 - x0) / all.len() as f64 * 0.6).clamp(0.05, 0.25) } else { 0.0 };
    for (i, (_, pts)) in series.iter().enumerate() {
        let sh: Vec<P> = pts.iter().map(|q| (q.0 + off * i as f64, q.1)).collect();
        p.stems(&sh, ["axp-d", "axp-d2", "axp-d3", "axp-d4"][i % 4]);
    }
    if series.len() > 1 {
        let names: Vec<(String, &str)> = series.iter().enumerate().map(|(i, s)| (s.0.clone(), FILLS[i % 4])).collect();
        p.legend(&names);
    }
    if y0 < 0.0 && y1 > 0.0 {
        p.seg((x0 - pad, 0.0), (x1 + pad, 0.0), "axp-c4");
    }
    if let Some((pt, l)) = mark {
        p.mark(pt, &l);
    }
    Some(p.body())
}

fn bf_stems(s: &Solved) -> Option<Figure> {
    let (k0, n) = (int_in(s, "successes")?, int_in(s, "trials")?);
    // a log10 factor within rounding of 0 is 0 (equal evidence)
    let l0 = k::snap(s.out("log10_bf10")?, 1.0);
    let mut pts = vec![];
    for kk in counts_upto(n, k0) {
        if let Some(a) = s.with_input(&[("successes", int(kk))]) {
            if let Some(v) = out(&a, "log10_bf10") {
                pts.push((kk as f64, k::snap(v, 1.0)));
            }
        }
    }
    if pts.len() < 2 {
        return None;
    }
    let simple = s.key == "bayes_factor_binomial_simple";
    let p0 = s.inp("p0")?;
    let (title, what) = if simple {
        ("Bayes factor of two simple hypotheses for every count",
         format!("the simple hypotheses p = {} against p = {}", num(s.inp("p1")?), num(p0)))
    } else {
        ("Point-null Bayes factor for every count", format!("a Beta alternative against the point null p = {}", num(p0)))
    };
    let body = stem_plot(&[(String::new(), pts.clone())], &format!("k, successes in {n} trials"),
                         "log10 Bayes factor (H1 over H0)", &format!("log10 BF10 = {} at k = {k0}", num(l0)),
                         Some(((k0 as f64, l0), format!("k = {k0}"))))?;
    Some(s.chart("stems", title,
                 &format!("The Bayes factor of {what} for {} counts of successes in {n} trials, as log10 of the computed \
                           factor (above 0 favours H1, below 0 favours H0). Marked: the count given, k = {k0}: log10 BF10 \
                           = {}.", pts.len(), num(l0)), &body))
}

fn evidence_stems(s: &Solved) -> Option<Figure> {
    let (a, b) = (s.inp("alpha")?, s.inp("beta")?);
    let (k0, n) = (int_in(s, "successes")?, int_in(s, "trials")?);
    let e0 = s.out("marginal_likelihood")?;
    let (a2, b2) = if (a, b) == (5.0, 5.0) { (1.0, 1.0) } else { (5.0, 5.0) };
    let mut s1 = vec![];
    let mut s2 = vec![];
    for kk in counts_upto(n, k0) {
        if let Some(r) = s.with_input(&[("successes", int(kk))]).and_then(|r| out(&r, "marginal_likelihood")) {
            s1.push((kk as f64, r));
        }
        if let Some(r) = s.with_input(&[("successes", int(kk)), ("alpha", fl(a2)), ("beta", fl(b2))])
            .and_then(|r| out(&r, "marginal_likelihood")) {
            s2.push((kk as f64, r));
        }
    }
    let body = stem_plot(&[(beta_name(a, b), s1.clone()), (beta_name(a2, b2), s2.clone())],
                         &format!("k, successes in {n} trials"), "Marginal likelihood",
                         &format!("evidence {} at k = {k0}", num(e0)), Some(((k0 as f64, e0), format!("k = {k0}"))))?;
    Some(s.chart("stems", "Evidence of every success count under two priors",
                 &format!("The marginal likelihood (evidence) of k successes in {n} trials for every k, under the given \
                           {} prior and a {} prior for comparison; each value computed by the model. Marked: the count \
                           given, k = {k0}, evidence {}.", beta_name(a, b), beta_name(a2, b2), num(e0)), &body))
}

fn predictive_stems(s: &Solved) -> Option<Figure> {
    let (a, b) = (s.inp("alpha")?, s.inp("beta")?);
    let (m, j) = (int_in(s, "future_trials")?, int_in(s, "future_successes")?);
    let (p0, mean) = (s.out("probability")?, s.out("predictive_mean")?);
    let (a2, b2) = if (a, b) == (8.0, 4.0) { (1.0, 1.0) } else { (8.0, 4.0) };
    let mut s1 = vec![];
    let mut s2 = vec![];
    for kk in counts_upto(m, j) {
        if let Some(r) = s.with_input(&[("future_successes", int(kk))]).and_then(|r| out(&r, "probability")) {
            s1.push((kk as f64, r));
        }
        if let Some(r) = s.with_input(&[("future_successes", int(kk)), ("alpha", fl(a2)), ("beta", fl(b2))])
            .and_then(|r| out(&r, "probability")) {
            s2.push((kk as f64, r));
        }
    }
    let body = stem_plot(&[(beta_name(a, b), s1), (beta_name(a2, b2), s2)], &format!("Successes in {m} future trials"),
                         "Probability", &format!("P(k = {j}) = {}, mean {}", num(p0), num(mean)),
                         Some(((j as f64, p0), format!("k = {j}"))))?;
    Some(s.chart("stems", "Predictive distribution of future successes",
                 &format!("The beta-binomial predictive distribution of the number of successes in {m} future trials, for \
                           the given {} belief and, for comparison, a {} one; each probability computed by the model. \
                           Marked: the count given, P(k = {j}) = {} (predictive mean {}).", beta_name(a, b),
                          beta_name(a2, b2), num(p0), num(mean)), &body))
}

fn poisson_predictive_stems(s: &Solved) -> Option<Figure> {
    let (a, b) = (s.inp("shape")?, s.inp("rate")?);
    let e = int_in(s, "events")?;
    let (p0, mean, var) = (s.out("probability")?, s.out("predictive_mean")?, s.out("predictive_variance")?);
    let (a2, b2) = if (a, b) == (10.0, 2.0) { (1.0, 1.0) } else { (10.0, 2.0) };
    let top = ((mean + 4.0 * var.sqrt()).ceil() as i64).clamp(15, 60).max(e);
    let mut s1 = vec![];
    let mut s2 = vec![];
    for kk in counts_upto(top, e) {
        if let Some(r) = s.with_input(&[("events", int(kk))]).and_then(|r| out(&r, "probability")) {
            s1.push((kk as f64, r));
        }
        if let Some(r) = s.with_input(&[("events", int(kk)), ("shape", fl(a2)), ("rate", fl(b2))])
            .and_then(|r| out(&r, "probability")) {
            s2.push((kk as f64, r));
        }
    }
    let body = stem_plot(&[(gamma_name(a, b), s1), (gamma_name(a2, b2), s2)], "Events in the next period",
                         "Probability", &format!("P(k = {e}) = {}", num(p0)), Some(((e as f64, p0), format!("k = {e}"))))?;
    Some(s.chart("stems", "Predictive distribution of next-period counts",
                 &format!("The negative-binomial predictive distribution of next-period counts for the given {} rate and, \
                           for comparison, a {} rate, k = 0 to {top}, each probability computed by the model. Marked: the \
                           count given, P(k = {e}) = {} (predictive mean {}, variance {}).", gamma_name(a, b),
                          gamma_name(a2, b2), num(p0), num(mean), num(var)), &body))
}

fn hermite_stems(s: &Solved) -> Option<Figure> {
    let n = int_in(s, "n")?;
    let mut series = vec![];
    let mut notes = vec![];
    for m in [n, n * 2, n * 4] {
        if m < 1 || m > 64 {
            continue;
        }
        let a = if m == n { Some(s.a.clone()) } else { s.with_input(&[("n", int(m))]) };
        let Some(a) = a else { continue };
        let (Some(x), Some(w)) = (k::list_of(&a, "nodes"), k::list_of(&a, "weights")) else { continue };
        if x.len() != w.len() {
            continue;
        }
        let (_, wmax) = minmax(&w);
        notes.push(format!("n = {m}: largest node {}, largest weight {}", num(minmax(&x).1), num(wmax)));
        series.push((format!("n = {m}"), x.into_iter().zip(w).collect::<Vec<P>>()));
    }
    if series.is_empty() {
        return None;
    }
    // the series side by side would hide each other: draw them without offset, each its own colour
    let all: Vec<P> = series.iter().flat_map(|s| s.1.iter().copied()).collect();
    let (x0, x1) = minmax(&all.iter().map(|p| p.0).collect::<Vec<_>>());
    let (_, y1) = minmax(&all.iter().map(|p| p.1).collect::<Vec<_>>());
    let pad = ((x1 - x0) * 0.05).max(0.5);
    let mut p = Plot::new((x0 - pad, x1 + pad), (0.0, y1 * 1.1), "Node x", "Weight", &format!("{} nodes", n),
                          (false, false), false);
    for (i, (_, pts)) in series.iter().enumerate().rev() {
        p.stems(pts, ["axp-d", "axp-d2", "axp-d3"][i % 3]);
    }
    let names: Vec<(String, &str)> = series.iter().enumerate().map(|(i, s)| (s.0.clone(), FILLS[i % 3])).collect();
    p.legend(&names);
    Some(s.chart("stems", "Gauss-Hermite nodes and weights",
                 &format!("Gauss-Hermite rules with {} nodes (the n given and wider ones): each node with its weight as \
                           a stem, all computed by the model ({}). Wider rules reach further into the tails with ever \
                           smaller weights.", series.iter().map(|s| s.0.trim_start_matches("n = ").to_string())
                               .collect::<Vec<_>>().join(", "), notes.join("; ")), &p.body()))
}

fn sequential_stems(s: &Solved) -> Option<Figure> {
    let post = s.out("posterior")?;
    let (pos, neg) = (int_in(s, "positives")?, int_in(s, "negatives").unwrap_or(0));
    let mut sp = vec![];
    let mut sn = vec![];
    for kk in 1..=5i64 {
        if let Some(v) = s.with_input(&[("positives", int(kk)), ("negatives", int(0))]).and_then(|a| out(&a, "posterior")) {
            sp.push((kk as f64, v));
        }
        if let Some(v) = s.with_input(&[("positives", int(0)), ("negatives", int(kk))]).and_then(|a| out(&a, "posterior")) {
            sn.push((kk as f64, v));
        }
    }
    let mark = (neg == 0 && (1..=5).contains(&pos)).then(|| ((pos as f64, post), format!("{pos} positive")))
        .or_else(|| (pos == 0 && (1..=5).contains(&neg)).then(|| ((neg as f64 + 0.25, post), format!("{neg} negative"))));
    let body = stem_plot(&[("after k positives".into(), sp.clone()), ("after k negatives".into(), sn.clone())],
                         "Number of results k", "Probability of the condition",
                         &format!("{pos} positive, {neg} negative: {}", num(post)), mark)?;
    Some(s.chart("stems", "Probability of the condition after repeated tests",
                 &format!("Repeated diagnostic tests with prevalence {}, sensitivity {} and specificity {}: the model's \
                           posterior probability of the condition after k = 1 to 5 positive results ({}) and after 1 to 5 \
                           negative results ({}). For the results given ({pos} positive, {neg} negative): {}.",
                          num(s.inp("prevalence")?), num(s.inp("sensitivity")?), num(s.inp("specificity")?),
                          list_text(&sp.iter().map(|p| p.1).collect::<Vec<_>>()),
                          list_text(&sn.iter().map(|p| p.1).collect::<Vec<_>>()), num(post)), &body))
}

// ------------------------------------------------------------------------------------------------ intervals
/// Intervals drawn as rows (top to bottom): (label, lower, upper, a dot inside or None, highlighted).
fn interval_rows(rows: &[(String, f64, f64, Option<f64>, bool)], xlab: &str, header: &str, line: Option<f64>)
                 -> Option<String> {
    let xs: Vec<f64> = rows.iter().flat_map(|r| [r.1, r.2]).chain(line).collect();
    let (x0, x1) = minmax(&xs);
    if !(x1 > x0) || rows.is_empty() || rows.len() > 14 {
        return None;
    }
    let labels: Vec<String> = rows.iter().map(|r| r.0.clone()).collect();
    let mut p = Plot::rows((x0, x1), &labels, xlab, header, true);
    if let Some(v) = line {
        p.vline(v, "axp-c4", "");
    }
    for (i, (_, lo, hi, dot, hot)) in rows.iter().enumerate() {
        let y = i as f64;
        let class = if *hot { "axp-c2" } else { "axp-c" };
        p.seg((*lo, y), (*hi, y), class);
        p.seg((*lo, y - 0.18), (*lo, y + 0.18), class);
        p.seg((*hi, y - 0.18), (*hi, y + 0.18), class);
        if let Some(d) = dot {
            p.dot((*d, y), 4.5, if *hot { "axp-d2" } else { "axp-d" });
        }
        p.label(((*lo + *hi) / 2.0, y), 0.0, -9.0, "middle", "axp-vt", &format!("[{}, {}]", num(*lo), num(*hi)));
    }
    Some(p.body())
}

fn beta_binomial_intervals(s: &Solved) -> Option<Figure> {
    let (k0, n0) = (int_in(s, "successes")?, int_in(s, "trials")?);
    let (lo, hi) = (s.out("lower")?, s.out("upper")?);
    let share = if n0 > 0 { k0 as f64 / n0 as f64 } else { 0.5 };
    let mut rows = vec![(format!("{k0}/{n0}"), lo, hi, None, true)];
    let base = n0.max(10);
    for m in [1i64, 4, 16, 64] {
        let n = base * m;
        if n == n0 {
            continue;
        }
        let kk = (share * n as f64).round() as i64;
        if let Some(a) = s.with_input(&[("successes", int(kk)), ("trials", int(n))]) {
            if let (Some(l), Some(u)) = (out(&a, "lower"), out(&a, "upper")) {
                rows.push((format!("{kk}/{n}"), l, u, None, false));
            }
        }
        if rows.len() >= 5 {
            break;
        }
    }
    let lv = s.inp("level").map(|l| format!("{} % ", num(l * 100.0))).unwrap_or_default();
    let body = interval_rows(&rows, "Success probability θ", &format!("{lv}credible intervals"), None)?;
    Some(s.chart("intervals", "Credible intervals narrowing as trials accumulate",
                 &format!("{}equal-tailed credible intervals for a success probability under a {} prior: first the counts \
                           given ({k0} of {n0}), then the same share {} in ever more trials, each interval computed by \
                           the model: {}. Four times the data roughly halves the width.",
                          if lv.is_empty() { "The ".to_string() } else { lv.clone() },
                          beta_name(s.inp("alpha")?, s.inp("beta")?), num(share),
                          rows.iter().map(|r| format!("{} [{}, {}]", r.0, num(r.1), num(r.2))).collect::<Vec<_>>()
                              .join("; ")), &body))
}

fn gamma_poisson_intervals(s: &Solved) -> Option<Figure> {
    let counts = s.vec("counts")?;
    let (lo, hi) = (s.out("lower")?, s.out("upper")?);
    if counts.is_empty() {
        return None;
    }
    let mut rows = vec![];
    let n = counts.len().min(10);
    for j in 1..=n {
        let part = &counts[..j];
        let r = if j == counts.len() { Some((lo, hi)) } else {
            s.with_input(&[("counts", jv(part))]).and_then(|a| Some((out(&a, "lower")?, out(&a, "upper")?)))
        };
        if let Some((l, u)) = r {
            rows.push((format!("first {j}"), l, u, None, j == counts.len()));
        }
    }
    if counts.len() > n {
        rows.push((format!("all {}", counts.len()), lo, hi, None, true));
    }
    // the same counts observed again, twice and four times over
    for m in [2usize, 4] {
        if rows.len() >= 6 {
            break;
        }
        let rep: Vec<f64> = counts.iter().cycle().take(counts.len() * m).copied().collect();
        if let Some(a) = s.with_input(&[("counts", jv(&rep))]) {
            if let (Some(l), Some(u)) = (out(&a, "lower"), out(&a, "upper")) {
                rows.push((format!("{m}x repeated"), l, u, None, false));
            }
        }
    }
    let lv = s.inp("level").map(|l| format!("{} % ", num(l * 100.0))).unwrap_or_default();
    let body = interval_rows(&rows, "Rate λ", &format!("{lv}credible intervals for the rate"), None)?;
    Some(s.chart("intervals", "Rate interval narrowing as counts arrive",
                 &format!("{lv}credible intervals for a Poisson rate under a {} prior as the counts {} arrive (the first j \
                           counts), then the same counts observed again (repeated twice and four times), each computed by \
                           the model: {}.", gamma_name(s.inp("shape")?, s.inp("rate")?), list_text(&counts),
                          rows.iter().map(|r| format!("{} [{}, {}]", r.0, num(r.1), num(r.2))).collect::<Vec<_>>()
                              .join("; ")), &body))
}

fn jeffreys_intervals(s: &Solved) -> Option<Figure> {
    let (k0, n0) = (int_in(s, "successes")?, int_in(s, "trials")?);
    let mut rows = vec![];
    for m in [1i64, 2, 4, 8, 16] {
        let a = if m == 1 { Some(s.a.clone()) } else {
            s.with_input(&[("successes", int(k0 * m)), ("trials", int(n0 * m))])
        };
        if let Some(a) = a {
            if let (Some(l), Some(u), Some(pm)) = (out(&a, "lower"), out(&a, "upper"), out(&a, "posterior_mean")) {
                rows.push((format!("{}/{}", k0 * m, n0 * m), l, u, Some(pm), m == 1));
            }
        }
    }
    let lv = s.inp("level").map(|l| format!("{} % ", num(l * 100.0))).unwrap_or_default();
    let body = interval_rows(&rows, "Proportion p", &format!("{lv}Jeffreys intervals"), None)?;
    Some(s.chart("intervals", "Jeffreys intervals for the same proportion with more data",
                 &format!("Jeffreys {lv}intervals for a proportion, starting from the counts given ({k0} of {n0}) and \
                           multiplying both by 2, 4, 8 and 16; intervals and posterior means (dots) computed by the model: \
                           {}.", rows.iter().map(|r| format!("{} [{}, {}]", r.0, num(r.1), num(r.2)))
                               .collect::<Vec<_>>().join("; ")), &body))
}

fn measurement_intervals(s: &Solved) -> Option<Figure> {
    let mut rows = vec![];
    let (title, cap, line);
    match s.key {
        "inverse_variance_weighted_mean" => {
            let (x, u) = (s.vec("values")?, s.vec("uncertainties")?);
            let (m, um) = (s.out("weighted_mean")?, s.out("uncertainty")?);
            if x.len() != u.len() {
                return None;
            }
            for (i, (a, b)) in x.iter().zip(&u).enumerate().take(12) {
                rows.push((format!("x{}", i + 1), a - b, a + b, Some(*a), false));
            }
            rows.push(("mean".into(), m - um, m + um, Some(m), true));
            title = "Measurements and their inverse-variance weighted mean";
            cap = format!("The {} measurements {} with standard uncertainties {}, each drawn as value ± uncertainty, and \
                           the computed inverse-variance weighted mean {} ± {} (bottom row and dashed line): the most \
                           precise measurement pulls hardest. Computed χ² = {}, Birge ratio {}.", x.len(), list_text(&x),
                          list_text(&u), num(m), num(um), num(s.out("chi_square")?),
                          s.out("birge_ratio").map(num).unwrap_or_else(|| "not defined".into()));
            line = Some(m);
        }
        "uncertainty_sum" => {
            let (x, u) = (s.vec("values")?, s.vec("uncertainties")?);
            let (y, uy) = (s.out("value")?, s.out("uncertainty")?);
            if x.len() != u.len() {
                return None;
            }
            for (i, (a, b)) in x.iter().zip(&u).enumerate().take(12) {
                rows.push((format!("x{}", i + 1), a - b, a + b, Some(*a), false));
            }
            rows.push(("y".into(), y - uy, y + uy, Some(y), true));
            title = "Inputs and result of an uncertain sum";
            cap = format!("A weighted sum y = Σ c_i x_i of the inputs {} with standard uncertainties {}{}, each drawn as \
                           value ± uncertainty: the computed y = {} ± {}. Uncertainties add in quadrature, so the result \
                           is less uncertain than their plain sum.", list_text(&x), list_text(&u),
                          s.vec("coefficients").map(|c| format!(" and coefficients {}", list_text(&c))).unwrap_or_default(),
                          num(y), num(uy));
            line = None;
        }
        _ => {
            let (w, m, sd, n) = (s.vec("weights")?, s.vec("means")?, s.vec("sds")?, s.vec("sizes")?);
            let (est, se) = (s.out("estimate")?, s.out("standard_error")?);
            if m.len() != sd.len() || w.len() != m.len() {
                return None;
            }
            for (i, (a, b)) in m.iter().zip(&sd).enumerate().take(12) {
                rows.push((format!("stratum {}", i + 1), a - b, a + b, Some(*a), false));
            }
            rows.push(("estimate".into(), est - 2.0 * se, est + 2.0 * se, Some(est), true));
            title = "Strata and the stratified estimate";
            cap = format!("Stratified sampling with weights {}, stratum means {}, standard deviations {} and sample \
                           sizes {}: each stratum's mean ± sd, and the computed overall estimate \
                           {} with standard error {} (bottom row: ± 2 standard errors).", list_text(&w), list_text(&m),
                          list_text(&sd), list_text(&n), num(est), num(se));
            line = Some(est);
        }
    }
    let header = format!("{} = {} ± {}", rows.last()?.0, num(rows.last()?.3?), num((rows.last()?.2 - rows.last()?.1) / 2.0));
    let body = interval_rows(&rows, "Value", &header, line)?;
    Some(s.chart("intervals", title, &cap, &body))
}

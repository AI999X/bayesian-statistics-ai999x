"""Bayesian statistics procedures: deterministic algorithms with typed inputs (probabilities, counts, data vectors,
payoff and correlation matrices, choices) and several outputs. Domain code, loaded by the pack; a procedure is exposed
only once tools/certify_procedures.py has certified it.

Covers Bayes' theorem and diagnostic reasoning, odds and likelihood ratios, conjugate updating (beta-binomial,
gamma-Poisson, gamma-exponential, normal-normal, normal-inverse-gamma, Dirichlet-multinomial), credible and
highest-density intervals, posterior predictive probabilities, MAP and Laplace approximations, Bayes factors,
decision analysis (expected value, EVPI, EVSI, utility, decision criteria), first-order uncertainty propagation (GUM),
deterministic Monte Carlo tools (sample size, importance sampling, control variates, stratification, Halton points,
Gauss-Hermite rules) and variance-based sensitivity analysis (Sobol indices for analytic models, Morris effects).

Every procedure states its inputs, outputs, conditions, method and LaTeX, the documented question forms it reads,
worked examples, an INDEPENDENT reference computation (scipy, numpy, uncertainties, SALib, chaospy, sympy; imported
inside the reference only, used by certification only) and a deliberately wrong variant that certification shows the
checks reject. The shipped code is plain Python: the beta, gamma, normal and Student-t distribution functions and
quantiles are computed here (continued fractions, series, safeguarded Newton inversion) and certified against scipy.
Monte Carlo answers are never drawn from a random generator here: they are exact, quadrature or given-sample formulas,
so every platform gives the same answer.
"""
from __future__ import annotations

import math
import re

PROCEDURES: dict = {}
LEVELS: dict = {}

# ============================================================================================ special functions ====
_TINY = 1e-300
_SQRT2 = math.sqrt(2.0)
_LN_SQRT_2PI = 0.5 * math.log(2.0 * math.pi)


def _lbeta(a, b):
    return math.lgamma(a) + math.lgamma(b) - math.lgamma(a + b)


def _bcf(a, b, x):
    """Continued fraction of the regularized incomplete beta function (modified Lentz)."""
    qab, qap, qam = a + b, a + 1.0, a - 1.0
    c, d = 1.0, 1.0 - qab * x / qap
    if abs(d) < _TINY:
        d = _TINY
    d = 1.0 / d
    h = d
    for m in range(1, 100000):
        m2 = 2 * m
        aa = m * (b - m) * x / ((qam + m2) * (a + m2))
        d = 1.0 + aa * d
        d = _TINY if abs(d) < _TINY else d
        c = 1.0 + aa / c
        c = _TINY if abs(c) < _TINY else c
        d = 1.0 / d
        h *= d * c
        aa = -(a + m) * (qab + m) * x / ((a + m2) * (qap + m2))
        d = 1.0 + aa * d
        d = _TINY if abs(d) < _TINY else d
        c = 1.0 + aa / c
        c = _TINY if abs(c) < _TINY else c
        d = 1.0 / d
        de = d * c
        h *= de
        if abs(de - 1.0) < 3e-16:
            return h
    raise ArithmeticError("the incomplete beta function did not converge")


def _ibeta(a, b, x, y=None):
    """(I_x(a, b), 1 - I_x(a, b)); y = 1 - x may be passed separately so that neither tail loses precision."""
    if y is None:
        y = 1.0 - x
    if x <= 0.0:
        return 0.0, 1.0
    if y <= 0.0:
        return 1.0, 0.0
    lf = a * math.log(x) + b * math.log(y) - _lbeta(a, b)
    if x < (a + 1.0) / (a + b + 2.0):
        v = math.exp(lf) * _bcf(a, b, x) / a
        return v, 1.0 - v
    v = math.exp(lf) * _bcf(b, a, y) / b
    return 1.0 - v, v


def _igamma(a, x):
    """(P(a, x), _Q(a, x)): the regularized lower and upper incomplete gamma functions."""
    if x <= 0.0:
        return 0.0, 1.0
    lf = a * math.log(x) - x - math.lgamma(a)
    if x < a + 1.0:
        ap, s = a, 1.0 / a
        d = s
        for _ in range(100000):
            ap += 1.0
            d *= x / ap
            s += d
            if abs(d) < abs(s) * 1e-17:
                break
        p = s * math.exp(lf)
        return p, 1.0 - p
    b = x + 1.0 - a
    c, d = 1.0 / _TINY, 1.0 / b
    h = d
    for i in range(1, 100000):
        an = -i * (i - a)
        b += 2.0
        d = an * d + b
        d = _TINY if abs(d) < _TINY else d
        c = b + an / c
        c = _TINY if abs(c) < _TINY else c
        d = 1.0 / d
        de = d * c
        h *= de
        if abs(de - 1.0) < 3e-16:
            break
    q = math.exp(lf) * h
    return 1.0 - q, q


def _invert(tails, pdf, p, lo, hi, x0):
    """x with F(x) = p for an increasing distribution: safeguarded Newton on the SMALLER tail (no cancellation),
    bracketed so that it always converges. tails(x) -> (F, 1 - F)."""
    lower = p <= 0.5
    target = p if lower else 1.0 - p

    def g(x):
        f, s = tails(x)
        return (f - target) if lower else (target - s)
    x = x0 if lo < x0 < hi else (0.5 * (lo + hi) if math.isfinite(hi) else lo + 1.0)
    for _ in range(400):
        gx = g(x)
        if gx == 0.0:
            return x
        if gx > 0.0:
            hi = x
        else:
            lo = x
        d = pdf(x)
        xn = x - gx / d if d > 0.0 and math.isfinite(d) else float("nan")
        if not (lo < xn < hi):
            xn = 0.5 * (lo + hi) if math.isfinite(hi) else max(2.0 * x, x + 1.0)
        if abs(xn - x) <= 2e-16 * abs(x) or xn == lo or xn == hi:
            return xn
        x = xn
    return x


def _beta_pdf(x, a, b):
    if x <= 0.0 or x >= 1.0:
        if (x <= 0.0 and a < 1) or (x >= 1.0 and b < 1):
            return float("inf")
        if (x <= 0.0 and a == 1) or (x >= 1.0 and b == 1):
            return math.exp(-_lbeta(a, b))
        return 0.0
    return math.exp((a - 1.0) * math.log(x) + (b - 1.0) * math.log1p(-x) - _lbeta(a, b))


def _beta_logpdf(x, a, b):
    return (a - 1.0) * math.log(x) + (b - 1.0) * math.log1p(-x) - _lbeta(a, b)


def _beta_ppf(p, a, b):
    if p <= 0.0:
        return 0.0
    if p >= 1.0:
        return 1.0
    m = a / (a + b)
    return _invert(lambda x: _ibeta(a, b, x), lambda x: _beta_pdf(x, a, b), p, 0.0, 1.0, m)


def _gamma_logpdf(x, k):
    return (k - 1.0) * math.log(x) - x - math.lgamma(k)


def _gamma_ppf(p, k):
    """Quantile of the standard gamma distribution (shape k, rate 1)."""
    if p <= 0.0:
        return 0.0
    hi = max(1.0, k)
    while _igamma(k, hi)[0] < p and hi < 1e300:
        hi *= 2.0
    return _invert(lambda x: _igamma(k, x), lambda x: math.exp(_gamma_logpdf(x, k)) if x > 0 else 0.0,
                   p, 0.0, hi, k)


def _phi(z):
    return 0.5 * math.erfc(-z / _SQRT2)


def _norm_pdf(z):
    return math.exp(-0.5 * z * z - _LN_SQRT_2PI)


def _z(p):
    """Standard normal quantile (safeguarded Newton on the exact tails)."""
    if not 0.0 < p < 1.0:
        raise ValueError("a probability strictly between 0 and 1 is required")
    return _invert(lambda z: (0.5 * math.erfc(-z / _SQRT2), 0.5 * math.erfc(z / _SQRT2)), _norm_pdf, p, -40.0, 40.0, 0.0)


def _t_tails(t, v):
    x, y = v / (v + t * t), t * t / (v + t * t)
    i, _c = _ibeta(0.5 * v, 0.5, x, y)
    return (1.0 - 0.5 * i, 0.5 * i) if t > 0 else (0.5 * i, 1.0 - 0.5 * i)


def _t_pdf(t, v):
    return math.exp(math.lgamma(0.5 * (v + 1)) - math.lgamma(0.5 * v) - 0.5 * math.log(v * math.pi)
                    - 0.5 * (v + 1) * math.log1p(t * t / v))


def _t_ppf(p, v):
    if not 0.0 < p < 1.0:
        raise ValueError("a probability strictly between 0 and 1 is required")
    return _invert(lambda t: _t_tails(t, v), lambda t: _t_pdf(t, v), p, -1e12, 1e12, 0.0)


def _digamma(x):
    r = 0.0
    while x < 10.0:
        r -= 1.0 / x
        x += 1.0
    f = 1.0 / (x * x)
    return r + math.log(x) - 0.5 / x - f * (1 / 12 - f * (1 / 120 - f * (1 / 252 - f * (1 / 240 - f * (1 / 132)))))


def _logsumexp(vals, weights=None):
    weights = weights or [1.0] * len(vals)
    m = max(v for v, w in zip(vals, weights) if w > 0)
    return m + math.log(math.fsum(w * math.exp(v - m) for v, w in zip(vals, weights) if w > 0))


# ======================================================================================= input checks ====
def _fin(v, name):
    if isinstance(v, bool) or not isinstance(v, (int, float)) or not math.isfinite(v):
        raise ValueError(f"{name} must be a finite number")
    return float(v)


def _prob(v, name, open_=False):
    v = _fin(v, name)
    if open_ and not 0.0 < v < 1.0:
        raise ValueError(f"{name} must be strictly between 0 and 1")
    if not 0.0 <= v <= 1.0:
        raise ValueError(f"{name} must be a probability between 0 and 1")
    return v


def _pos(v, name):
    v = _fin(v, name)
    if v <= 0:
        raise ValueError(f"{name} must be positive")
    return v


def _nonneg(v, name):
    v = _fin(v, name)
    if v < 0:
        raise ValueError(f"{name} must not be negative")
    return v


def _whole(v, name, lo=0):
    v = _fin(v, name)
    if v != int(v) or v < lo:
        raise ValueError(f"{name} must be a whole number >= {lo}")
    return int(v)


def _level(v):
    v = 0.95 if v is None else _fin(v, "level")
    if not 0.0 < v < 1.0:
        raise ValueError("the credibility level must be strictly between 0 and 1 (0.95 for 95%)")
    return v


def _vec(x, name, n=None, min_len=1):
    if not isinstance(x, (list, tuple)):
        raise ValueError(f"{name} must be a list of numbers")
    x = [_fin(v, name) for v in x]
    if len(x) < min_len:
        raise ValueError(f"{name} needs at least {min_len} values")
    if n is not None and len(x) != n:
        raise ValueError(f"{name} must have {n} values (one per entry of the other list)")
    return x


def _simplex(p, name="the probabilities", n=None):
    p = _vec(p, name, n)
    if any(v < 0 for v in p):
        raise ValueError(f"{name} must not be negative")
    if abs(math.fsum(p) - 1.0) > 1e-9:
        raise ValueError(f"{name} must sum to 1 (they sum to {math.fsum(p):.6g})")
    return p


def _mat(m, name, rows=None, cols=None):
    if not isinstance(m, (list, tuple)) or not m or not all(isinstance(r, (list, tuple)) for r in m):
        raise ValueError(f"{name} must be a matrix (rows of numbers)")
    m = [[_fin(v, name) for v in r] for r in m]
    if any(len(r) != len(m[0]) for r in m) or not m[0]:
        raise ValueError(f"{name} must be rectangular")
    if rows is not None and len(m) != rows:
        raise ValueError(f"{name} must have {rows} rows")
    if cols is not None and len(m[0]) != cols:
        raise ValueError(f"{name} must have {cols} columns")
    return m


def _cholesky_ok(s, name):
    """A symmetric positive semi-definite matrix (checked by a Cholesky factorization with a small tolerance)."""
    n = len(s)
    if any(len(r) != n for r in s):
        raise ValueError(f"{name} must be square")
    scale = max(1.0, max(abs(s[i][i]) for i in range(n)))
    for i in range(n):
        for j in range(i):
            if abs(s[i][j] - s[j][i]) > 1e-12 * scale:
                raise ValueError(f"{name} must be symmetric")
    L = [[0.0] * n for _ in range(n)]
    for j in range(n):
        d = s[j][j] - math.fsum(L[j][k] ** 2 for k in range(j))
        if d < -1e-12 * scale:
            raise ValueError(f"{name} must be positive semi-definite")
        L[j][j] = math.sqrt(max(d, 0.0))
        for i in range(j + 1, n):
            v = s[i][j] - math.fsum(L[i][k] * L[j][k] for k in range(j))
            if L[j][j] > 1e-15 * scale:
                L[i][j] = v / L[j][j]
            elif abs(v) > 1e-9 * scale:
                raise ValueError(f"{name} must be positive semi-definite")
    return s


def _mean(x):
    return math.fsum(x) / len(x)


# ===================================================================================== comparison (verify) ====
def _diff(a, b):
    """Largest relative difference between two outputs (numbers, lists, dicts, text, None)."""
    if isinstance(a, dict) or isinstance(b, dict):
        if not (isinstance(a, dict) and isinstance(b, dict)) or a.keys() != b.keys():
            return 1.0
        return max((_diff(a[k], b[k]) for k in a), default=0.0)
    if isinstance(a, (list, tuple)) or isinstance(b, (list, tuple)):
        if not (isinstance(a, (list, tuple)) and isinstance(b, (list, tuple))) or len(a) != len(b):
            return 1.0
        return max((_diff(u, v) for u, v in zip(a, b)), default=0.0)
    if a is None or b is None:
        return 0.0 if a is b else 1.0
    if isinstance(a, str) or isinstance(b, str):
        return 0.0 if a == b else 1.0
    try:
        a, b = float(a), float(b)
    except (TypeError, ValueError):
        return 1.0
    if not (math.isfinite(a) and math.isfinite(b)):
        return 0.0 if a == b else 1.0
    return abs(a - b) / max(1e-15, abs(a), abs(b))


# ================================================================================ reading documented questions ====
_NUM = r"[-+]?(?:\d+(?:\.\d*)?|\.\d+)(?:e[-+]?\d+)?"
_ROUTES: list = []          # (key, required patterns, avoided patterns, priority)


def _norm(question: str) -> str:
    q = question.lower()
    for a, b in (("−", "-"), ("–", "-"), ("—", "-"), ("σ", "sigma "), ("μ", "mu "), ("≤", "<="), ("≥", ">="),
                 ("α", "alpha "), ("λ", "lambda "), ("β", "beta "), ("τ", "tau "), ("κ", "kappa "), ("ρ", "rho "),
                 ("ν", "nu "), ("¬", "not "), ("’", "'"), ("∣", "|"), ("×", "x")):
        q = q.replace(a, b)
    return q


def _route(q: str) -> list:
    hits = [(prio, key) for key, need, avoid, prio in _ROUTES
            if all(re.search(p, q) for p in need) and not any(re.search(p, q) for p in avoid)]
    if not hits:
        return []
    best = max(p for p, _k in hits)
    return [k for p, k in hits if p == best]


_SEP = r"\s*(?:=|:|\bis\b|\bof\b|\bequals\b)?\s*"


def _value(q, alias):
    # the value starts after a separator, never inside a name: "bf10" is the name of BF10, not "bf" = 10
    m = re.search(r"(?<![a-z0-9_])(?:" + alias + r")" + _SEP + r"(?<![a-z0-9_])(" + _NUM + r")(\s*%)?(?![\d.]*\d)", q)
    if not m:
        return None
    v = float(m.group(1))
    return v / 100.0 if m.group(2) else v


def _list(q, alias):
    m = re.search(r"(?<![a-z0-9_])(?:" + alias + r")\s*(?:=|:|\bis\b|\bare\b)?\s*(\[[^\[\]]*\])", q)
    return m.group(1) if m else None


def _matrix(q, alias):
    m = re.search(r"(?<![a-z0-9_])(?:" + alias + r")\s*(?:=|:|\bis\b)?\s*(\[\s*\[.*?\]\s*\])", q)
    return m.group(1) if m else None


def _choice(q, alias, choices):
    alts = "|".join(re.escape(c.lower()) for c in sorted(choices, key=len, reverse=True))
    m = re.search(r"(?<![a-z0-9_])(?:" + alias + r")\s*(?:=|:|\bis\b)?\s*(" + alts + r")\b", q)
    return m.group(1) if m else None


def _make_parse(key, inputs, aliases, extract):
    def parse(question):
        if not isinstance(question, str):
            return None
        q = _norm(question)
        if key not in _route(q):
            return None
        d = {}
        for i in inputs:
            s, t = i["symbol"], i.get("type", "number")
            al = aliases.get(s, re.escape(s).replace("_", "[ _]"))
            if t == "matrix":
                v = _matrix(q, al)
            elif t == "vector":
                v = _list(q, al)
            elif t == "choice":
                v = _choice(q, al, i["choices"])
            else:
                v = _value(q, al)
            if v is not None:
                d[s] = v
        if extract is not None:
            extra = extract(q, d)
            if isinstance(extra, dict) and "_out_of_scope" in extra:
                return extra
        return d
    return parse


def _x_level(q, d):
    m = re.search(r"(\d+(?:\.\d+)?)\s*%\s*(?:equal[- ]tailed\s+|central\s+|two-sided\s+|highest[- ]density\s+|hpd\s+|"
                  r"hdi\s+|bayesian\s+|posterior\s+)*(?:credible|probability interval|interval|hdi|hpd|coverage|"
                  r"confidence|level)", q)
    if m and "level" not in d:
        d["level"] = float(m.group(1)) / 100.0


def _x_trials(q, d):
    m = re.search(r"(\d+)\s+successes?\s+(?:in|out of|from|among)\s+(\d+)\s+(?:trials|attempts|patients|tosses|"
                  r"flips|draws|observations|cases|visitors|users)", q) or \
        re.search(r"(\d+)\s+(?:out of|of)\s+(\d+)\s+(?:trials|patients|tosses|flips|users|visitors)\s+(?:were\s+)?"
                  r"(?:successes|successful|positive|converted|responded)", q)
    if m:
        d.setdefault("successes", int(m.group(1)))
        d.setdefault("trials", int(m.group(2)))
    else:                                       # "7 successes" with no number of trials: the trials are asked for
        k = set(re.findall(r"(?<![\w.])(\d+)\s+successes?\b", q))
        if len(k) == 1:
            d.setdefault("successes", int(k.pop()))


def _x_beta_prior(q, d):
    m = re.search(r"beta\(\s*(" + _NUM + r")\s*,\s*(" + _NUM + r")\s*\)", q)
    if m:
        d.setdefault("alpha", float(m.group(1)))
        d.setdefault("beta", float(m.group(2)))
    elif re.search(r"\b(uniform|flat) prior\b", q):
        d.setdefault("alpha", 1.0)
        d.setdefault("beta", 1.0)
    elif re.search(r"\bjeffreys prior\b", q):
        d.setdefault("alpha", 0.5)
        d.setdefault("beta", 0.5)


def _x_gamma_prior(q, d):
    m = re.search(r"gamma\(\s*(?:shape\s*=\s*)?(" + _NUM + r")\s*,\s*(?:rate\s*=\s*)?(" + _NUM + r")\s*\)", q)
    if m:
        d.setdefault("shape", float(m.group(1)))
        d.setdefault("rate", float(m.group(2)))


def _chain(*fs):
    def f(q, d):
        for g in fs:
            r = g(q, d)
            if isinstance(r, dict):
                return r
    return f


# ================================================================================================= the builder ====
def _X(inputs, outputs=None):
    """A worked example: inputs as a user writes them, and outputs checked against the run."""
    return {"inputs": inputs, "outputs": outputs or {}}


def _Q(text, outputs=None, outcome=None):
    return {"q": text, "outcome": outcome} if outcome else {"q": text, "outputs": outputs or {}}


def _r12(v):
    """Round to 12 significant digits (identical answers on every platform; far above the 1e-9 checks)."""
    if isinstance(v, dict):
        return {k: _r12(x) for k, x in v.items()}
    if isinstance(v, (list, tuple)):
        return [_r12(x) for x in v]
    if isinstance(v, float) and math.isfinite(v) and v != 0.0:
        return float(f"{v:.12g}")
    return v


def _cap(key, *, level, name, topic, purpose, inputs, outputs, primary, method, latex, run, reference, wrong,
         rnd, need, templates, examples, questions, conditions=(), avoid=(), prio=5, aliases=None, extract=None,
         keywords=(), verify=None):
    aliases = aliases or {}
    run = (lambda _r: lambda **kw: _r12(_r(**kw)))(run)
    PROCEDURES[key] = {
        "name": name, "topic": topic, "purpose": purpose, "inputs": inputs, "outputs": outputs, "primary": primary,
        "conditions": list(conditions), "method": method, "latex": latex, "run": run,
        "verify": verify or (lambda i, o, _r=run: _diff(_r(**i), o)),
        "parse": _make_parse(key, inputs, aliases, extract), "reference": reference, "wrong": wrong,
        "random_inputs": rnd, "templates": list(templates), "examples": list(examples), "questions": list(questions),
        "keywords": list(keywords)}
    LEVELS[key] = level
    _ROUTES.append((key, [re.compile(p) for p in ([need] if isinstance(need, str) else need)],
                    [re.compile(p) for p in ([avoid] if isinstance(avoid, str) else avoid)], prio))


def _Num(sym, meaning, required=True):
    return {"symbol": sym, "kind": "number", "meaning": meaning, "required": required}


def _Int(sym, meaning, required=True, lo=0):
    return {"symbol": sym, "type": "integer", "kind": "number", "meaning": meaning, "required": required, "min": lo}


def _Vec(sym, meaning, min_length=1, required=True, max_length=None):
    d = {"symbol": sym, "type": "vector", "kind": "number", "meaning": meaning, "required": required,
         "min_length": min_length}
    if max_length:
        d["max_length"] = max_length
    return d


def _Mat(sym, meaning, required=True):
    return {"symbol": sym, "type": "matrix", "kind": "number", "meaning": meaning, "required": required}


def _Ch(sym, meaning, choices, required=True):
    return {"symbol": sym, "type": "choice", "kind": "number", "meaning": meaning, "required": required,
            "choices": list(choices)}


def _Out(sym, meaning):
    return {"symbol": sym, "kind": "number", "meaning": meaning}


LEVEL_IN = _Num("level", "credibility level as a fraction (0.95 for 95%; default 0.95)", required=False)


# --------------------------------------------------------------------------------------------- random helpers ----
def _rp(rng, lo=0.001, hi=0.999, dec=4):
    return round(rng.uniform(lo, hi), dec)


def _rsimplex(rng, n, dec=4):
    w = [rng.uniform(0.05, 1.0) for _ in range(n)]
    s = sum(w)
    p = [round(v / s, dec) for v in w[:-1]]
    return p + [1.0 - math.fsum(p)] if 1.0 - math.fsum(p) >= 0 else _rsimplex(rng, n, dec)


def _rlevel(rng):
    return rng.choice([0.5, 0.8, 0.9, 0.95, 0.95, 0.99, round(rng.uniform(0.5, 0.995), 3)])


# ==================================================================================== Bayes' theorem (basic) ====
_TB = "Bayes' theorem and conditional probability"


def run_bayes(p_a, p_b_given_a, p_b_given_not_a):
    pa, l1, l0 = _prob(p_a, "P(A)"), _prob(p_b_given_a, "P(B|A)"), _prob(p_b_given_not_a, "P(B|not A)")
    pb = pa * l1 + (1.0 - pa) * l0
    if pb <= 0.0:
        raise ValueError("P(B) is 0: B cannot occur, so P(A|B) is undefined")
    return {"p_a_given_b": pa * l1 / pb, "p_b": pb, "p_not_a_given_b": (1.0 - pa) * l0 / pb}


def ref_bayes(p_a, p_b_given_a, p_b_given_not_a):
    """Odds form of Bayes' theorem (posterior odds = prior odds x likelihood ratio), numpy for the total probability."""
    import numpy as np
    odds = p_a / (1 - p_a) * p_b_given_a / p_b_given_not_a
    post = odds / (1 + odds)
    return {"p_a_given_b": post, "p_not_a_given_b": 1 - post,
            "p_b": float(np.dot([p_a, 1 - p_a], [p_b_given_a, p_b_given_not_a]))}


_cap("bayes_theorem", level="beginner", name="Bayes' theorem for an event and its complement", topic=_TB,
     purpose="The posterior probability P(A|B) from the prior P(A) and the likelihoods P(B|A) and P(B|not A).",
     inputs=[_Num("p_a", "prior probability P(A)"), _Num("p_b_given_a", "likelihood P(B|A)"),
             _Num("p_b_given_not_a", "likelihood P(B|not A)")],
     outputs=[_Out("p_a_given_b", "posterior probability P(A|B)"), _Out("p_b", "total probability P(B)"),
              _Out("p_not_a_given_b", "P(not A|B) = 1 - P(A|B)")],
     primary="p_a_given_b", conditions=["probabilities between 0 and 1", "P(B) > 0"],
     method="law of total probability for P(B), then Bayes' theorem", run=run_bayes, reference=ref_bayes,
     latex=r"P(A\mid B) = \frac{P(B\mid A)\,P(A)}{P(B\mid A)\,P(A) + P(B\mid \neg A)\,(1 - P(A))}",
     wrong=lambda p_a, p_b_given_a, p_b_given_not_a: dict(run_bayes(p_a, p_b_given_a, p_b_given_not_a),
                                                          p_a_given_b=p_b_given_a / (p_b_given_a + p_b_given_not_a)),
     rnd=lambda rng: {"p_a": _rp(rng), "p_b_given_a": _rp(rng), "p_b_given_not_a": _rp(rng)},
     need=[r"p\(b\s*\|\s*a\)", r"p\(a\)"], avoid=[r"sensitiv|specific|prevalence"],
     aliases={"p_a": r"p\(a\)", "p_b_given_a": r"p\(b\s*\|\s*a\)",
              "p_b_given_not_a": r"p\(b\s*\|\s*(?:not\s*a|~\s*a|a'|a\^c|a\s*complement)\)"},
     templates=["Bayes' theorem: P(A) = 0.3, P(B|A) = 0.8, P(B|not A) = 0.1. Find P(A|B).",
                "Given P(A) = 0.02, P(B|A) = 0.95 and P(B|not A) = 0.1, what is P(A|B)?"],
     examples=[_X({"p_a": 0.3, "p_b_given_a": 0.8, "p_b_given_not_a": 0.1}, {"p_a_given_b": 0.24 / 0.31, "p_b": 0.31}),
               _X({"p_a": 0.5, "p_b_given_a": 0.5, "p_b_given_not_a": 0.5}, {"p_a_given_b": 0.5})],
     questions=[_Q("Bayes' theorem: P(A) = 0.3, P(B|A) = 0.8, P(B|not A) = 0.1. Find P(A|B).", {"p_a_given_b": 0.24 / 0.31}),
                _Q("Given P(A) = 2%, P(B|A) = 95% and P(B|not A) = 10%, what is P(A|B)?",
                  {"p_a_given_b": 0.019 / (0.019 + 0.098)}),
                _Q("Bayes' theorem: P(A) = 0.3, P(B|A) = 1.2, P(B|not A) = 0.1. Find P(A|B).", outcome="INVALID_INPUT"),
                _Q("Bayes' theorem: P(A) = 0.3, P(B|A) = 0.8. Find P(A|B).", outcome="NEEDS_INPUT")])


def _dx(prevalence, sensitivity, specificity):
    pv, se, sp = _prob(prevalence, "prevalence"), _prob(sensitivity, "sensitivity"), _prob(specificity, "specificity")
    pos = pv * se + (1.0 - pv) * (1.0 - sp)
    neg = 1.0 - pos
    if pos <= 0.0 or neg <= 0.0:
        raise ValueError("a positive (or negative) result is impossible with these values: a predictive value is undefined")
    return pv, se, sp, pos, neg


def run_diagnostic(prevalence, sensitivity, specificity):
    pv, se, sp, pos, neg = _dx(prevalence, sensitivity, specificity)
    ppv = pv * se / pos
    npv = (1.0 - pv) * sp / neg
    return {"ppv": ppv, "npv": npv, "p_positive": pos, "false_discovery_rate": 1.0 - ppv,
            "false_omission_rate": 1.0 - npv}


def ref_diagnostic(prevalence, sensitivity, specificity):
    """Odds form: post-test odds = pre-test odds x LR+ (or LR-); numpy for the probability of a positive result."""
    import numpy as np
    pre = prevalence / (1 - prevalence)
    o_pos = pre * sensitivity / (1 - specificity)
    o_neg_healthy = (1 / pre) * specificity / (1 - sensitivity)
    ppv, npv = o_pos / (1 + o_pos), o_neg_healthy / (1 + o_neg_healthy)
    return {"ppv": ppv, "npv": npv, "false_discovery_rate": 1 - ppv, "false_omission_rate": 1 - npv,
            "p_positive": float(np.array([prevalence, 1 - prevalence]) @ np.array([sensitivity, 1 - specificity]))}


_DX_IN = [_Num("prevalence", "prevalence (pre-test probability of the condition)"),
          _Num("sensitivity", "sensitivity P(positive | condition)"),
          _Num("specificity", "specificity P(negative | no condition)")]
_DX_AL = {"prevalence": r"prevalence|pre-test probability|pretest probability|base rate|prior probability",
          "sensitivity": r"sensitivity|true positive rate", "specificity": r"specificity|true negative rate"}

# counts of test results ("3 positive results and 1 negative result", "tests positive 3 times"): several results name
# the repeated-test task, never the single-result predictive values
_RESULT_NOUN = r"(?:test\s+)?(?:results?|tests?|outcomes?|readings?)\b"
_RESULT_COUNTS = (r"(?<![\w.])\d+\s+(?:positive|negative)\s+" + _RESULT_NOUN +
                  r"|\b(?:positive|negative)\s+\d+\s+times?\b")


def _x_results(q, d):
    """positives / negatives from counts written in words; a count written twice differently is left unread (so the
    question is refused: every stated number must be bound)."""
    for sym, word in (("positives", "positive"), ("negatives", "negative")):
        if sym in d:
            continue
        found = set(re.findall(r"(?<![\w.])(\d+)\s+" + word + r"\s+" + _RESULT_NOUN, q)) | \
            set(re.findall(r"\b" + word + r"\s+(\d+)\s+times?\b", q))
        if len(found) == 1:
            d[sym] = int(found.pop())

_cap("diagnostic_test_predictive_values", level="beginner", name="Positive and negative predictive values of a test",
     topic=_TB,
     purpose="The probability of the condition after a positive result (PPV) and of no condition after a negative one "
             "(NPV), from prevalence, sensitivity and specificity.",
     inputs=_DX_IN,
     outputs=[_Out("ppv", "positive predictive value P(condition | positive)"),
              _Out("npv", "negative predictive value P(no condition | negative)"),
              _Out("p_positive", "probability of a positive result"),
              _Out("false_discovery_rate", "1 - PPV"), _Out("false_omission_rate", "1 - NPV")],
     primary="ppv", conditions=["probabilities between 0 and 1", "both results possible"],
     method="Bayes' theorem with the total probability of a positive result", run=run_diagnostic,
     reference=ref_diagnostic,
     latex=r"\mathrm{PPV} = \frac{\mathrm{Se}\,\pi}{\mathrm{Se}\,\pi + (1-\mathrm{Sp})(1-\pi)},\quad "
           r"\mathrm{NPV} = \frac{\mathrm{Sp}\,(1-\pi)}{\mathrm{Sp}\,(1-\pi) + (1-\mathrm{Se})\,\pi}",
     wrong=lambda prevalence, sensitivity, specificity: dict(
         run_diagnostic(prevalence, sensitivity, specificity),
         ppv=sensitivity / (sensitivity + 1 - specificity)),
     rnd=lambda rng: {"prevalence": _rp(rng, 0.001, 0.6), "sensitivity": _rp(rng, 0.5, 0.999),
                      "specificity": _rp(rng, 0.5, 0.999)},
     need=[r"sensitiv", r"specific", r"prevalence|pre-?test probability|base rate|prior probability"],
     avoid=[r"positives\s*=|repeated|consecutive|sequential|likelihood ratio", _RESULT_COUNTS], aliases=_DX_AL,
     templates=["A test has sensitivity 0.9 and specificity 0.95; the prevalence is 0.01. What is the positive "
                "predictive value?",
                "Prevalence = 2%, sensitivity = 99%, specificity = 98%. Find the PPV and NPV."],
     examples=[_X({"prevalence": 0.01, "sensitivity": 0.9, "specificity": 0.95},
                 {"ppv": 0.009 / (0.009 + 0.0495), "p_positive": 0.0585}),
               _X({"prevalence": 0.5, "sensitivity": 0.8, "specificity": 0.8}, {"ppv": 0.8, "npv": 0.8})],
     questions=[_Q("A test has sensitivity 0.9 and specificity 0.95; the prevalence is 0.01. What is the positive "
                  "predictive value?", {"ppv": 0.009 / (0.009 + 0.0495)}),
                _Q("Prevalence = 50%, sensitivity = 80%, specificity = 80%. Find the PPV and NPV.",
                  {"ppv": 0.8, "npv": 0.8}),
                _Q("Prevalence = 0.1, sensitivity = 1.3, specificity = 0.9. Find the PPV.", outcome="INVALID_INPUT"),
                _Q("The prevalence is 0.1 and the sensitivity is 0.9. What is the PPV given the specificity?",
                  outcome="NEEDS_INPUT")])


def run_total_probability(priors, conditionals):
    p = _simplex(priors, "the probabilities of the partition")
    c = _vec(conditionals, "the conditional probabilities", len(p))
    if any(not 0.0 <= v <= 1.0 for v in c):
        raise ValueError("the conditional probabilities must be between 0 and 1")
    joint = [a * b for a, b in zip(p, c)]
    return {"p_b": math.fsum(joint), "joint": joint}


_cap("total_probability", level="beginner", name="Law of total probability", topic=_TB,
     purpose="P(B) = sum of P(B|A_i) P(A_i) over a partition A_1, ..., A_k (and the joint probabilities).",
     inputs=[_Vec("priors", "probabilities P(A_i) of the partition (sum to 1)"),
             _Vec("conditionals", "conditional probabilities P(B|A_i), one per part")],
     outputs=[_Out("p_b", "total probability P(B)"), _Out("joint", "joint probabilities P(B and A_i)")],
     primary="p_b", conditions=["the P(A_i) sum to 1", "same number of conditionals as parts"],
     method="weighted sum over the partition", run=run_total_probability,
     reference=lambda priors, conditionals: (lambda np: {
         "p_b": float(np.dot(priors, conditionals)),
         "joint": list(np.multiply(priors, conditionals))})(__import__("numpy")),
     latex=r"P(B) = \sum_i P(B\mid A_i)\,P(A_i)",
     wrong=lambda priors, conditionals: dict(run_total_probability(priors, conditionals),
                                            p_b=sum(conditionals) / len(conditionals)),
     rnd=lambda rng: (lambda n: {"priors": _rsimplex(rng, n), "conditionals": [_rp(rng) for _ in range(n)]})(
         rng.randint(2, 6)),
     need=r"total probability", aliases={"priors": r"priors|p\(a_i\)|part probabilities|partition probabilities",
                                         "conditionals": r"conditionals|p\(b\s*\|\s*a_i\)|conditional probabilities"},
     templates=["Law of total probability: priors = [0.5, 0.3, 0.2], conditionals = [0.01, 0.02, 0.05]. Find P(B).",
                "Use total probability with priors [0.6, 0.4] and conditional probabilities [0.1, 0.3]."],
     examples=[_X({"priors": "0.5, 0.3, 0.2", "conditionals": "0.01, 0.02, 0.05"}, {"p_b": 0.021}),
               _X({"priors": [0.6, 0.4], "conditionals": [0.1, 0.3]}, {"p_b": 0.18})],
     questions=[_Q("Law of total probability: priors = [0.5, 0.3, 0.2], conditionals = [0.01, 0.02, 0.05]. Find P(B).",
                  {"p_b": 0.021}),
                _Q("Use total probability with priors [0.6, 0.4] and conditional probabilities [0.1, 0.3].", {"p_b": 0.18}),
                _Q("Law of total probability: priors = [0.5, 0.3], conditionals = [0.01, 0.02]. Find P(B).",
                  outcome="INVALID_INPUT"),
                _Q("Law of total probability: priors = [0.5, 0.5]. Find P(B).", outcome="NEEDS_INPUT")])


def run_discrete_posterior(priors, likelihoods):
    p = _simplex(priors, "the prior probabilities")
    lik = _vec(likelihoods, "the likelihoods", len(p))
    if any(v < 0 for v in lik):
        raise ValueError("the likelihoods must not be negative")
    joint = [a * b for a, b in zip(p, lik)]
    ev = math.fsum(joint)
    if ev <= 0:
        raise ValueError("the evidence is 0: no hypothesis gives the observation a positive probability")
    post = [j / ev for j in joint]
    best = max(range(len(post)), key=lambda i: post[i])
    return {"posterior": post, "evidence": ev, "map_hypothesis": best + 1, "map_probability": post[best]}


def ref_discrete_posterior(priors, likelihoods):
    """Log-space softmax (scipy.special.softmax of log prior + log likelihood)."""
    import numpy as np
    from scipy.special import logsumexp, softmax
    with np.errstate(divide="ignore"):
        z = np.log(np.array(priors)) + np.log(np.array(likelihoods))
    post = softmax(z)
    return {"posterior": list(post), "evidence": float(np.exp(logsumexp(z))),
            "map_hypothesis": int(np.argmax(post)) + 1, "map_probability": float(post.max())}


_cap("discrete_bayes_posterior", level="intermediate", name="Posterior over several discrete hypotheses",
     topic=_TB,
     purpose="Posterior probabilities of k hypotheses from their priors and the likelihood of the observed data under "
             "each; the evidence and the most probable hypothesis (MAP).",
     inputs=[_Vec("priors", "prior probabilities of the hypotheses (sum to 1)", 2),
             _Vec("likelihoods", "likelihood of the data under each hypothesis", 2)],
     outputs=[_Out("posterior", "posterior probabilities"), _Out("evidence", "marginal probability of the data"),
              _Out("map_hypothesis", "number (1-based) of the most probable hypothesis"),
              _Out("map_probability", "its posterior probability")],
     primary="evidence", conditions=["priors sum to 1", "likelihoods >= 0, not all zero where the prior is positive"],
     method="Bayes' theorem: multiply prior by likelihood and normalize", run=run_discrete_posterior,
     reference=ref_discrete_posterior,
     latex=r"P(H_i\mid D) = \frac{P(D\mid H_i)\,P(H_i)}{\sum_j P(D\mid H_j)\,P(H_j)}",
     wrong=lambda priors, likelihoods: dict(run_discrete_posterior(priors, likelihoods),
                                            posterior=[v / sum(likelihoods) for v in likelihoods],
                                            evidence=sum(likelihoods) / len(likelihoods)),
     rnd=lambda rng: (lambda n: {"priors": _rsimplex(rng, n), "likelihoods": [_rp(rng, 0.001, 1.0) for _ in range(n)]})(
         rng.randint(2, 7)),
     need=[r"posterior", r"priors\s*(?:=|:)?\s*\[", r"likelihoods"],
     avoid=[r"log|grid|marginal likelihoods"],
     aliases={"priors": r"priors", "likelihoods": r"likelihoods"},
     templates=["Posterior probabilities with priors = [0.5, 0.3, 0.2] and likelihoods = [0.1, 0.4, 0.7].",
                "Three hypotheses have priors [0.2, 0.3, 0.5] and likelihoods [0.9, 0.5, 0.1]: find the posterior."],
     examples=[_X({"priors": "0.5, 0.3, 0.2", "likelihoods": "0.1, 0.4, 0.7"},
                 {"posterior": [0.05 / 0.31, 0.12 / 0.31, 0.14 / 0.31], "evidence": 0.31, "map_hypothesis": 3}),
               _X({"priors": [0.5, 0.5], "likelihoods": [0.2, 0.2]}, {"posterior": [0.5, 0.5]})],
     questions=[_Q("Posterior probabilities with priors = [0.5, 0.3, 0.2] and likelihoods = [0.1, 0.4, 0.7].",
                  {"evidence": 0.31, "map_hypothesis": 3}),
                _Q("Three hypotheses have priors [0.2, 0.3, 0.5] and likelihoods [0.9, 0.5, 0.1]: find the posterior.",
                  {"evidence": 0.38, "map_hypothesis": 1}),
                _Q("Posterior with priors = [0.5, 0.6] and likelihoods = [0.1, 0.4].", outcome="INVALID_INPUT"),
                _Q("Posterior probabilities with priors = [0.5, 0.5]; the likelihoods are not known yet.", outcome="NEEDS_INPUT")])


# ----------------------------------------------------------------------------------- odds and likelihood ratios ----
_TO = "odds and likelihood ratios"


def run_odds(probability):
    p = _prob(probability, "the probability", open_=True)
    return {"odds": p / (1.0 - p), "odds_against": (1.0 - p) / p, "log_odds": math.log(p) - math.log1p(-p)}


_cap("probability_to_odds", level="beginner", name="Odds and log-odds of a probability", topic=_TO,
     purpose="The odds p/(1-p), the odds against and the log-odds (logit) of a probability.",
     inputs=[_Num("probability", "probability p, strictly between 0 and 1")],
     outputs=[_Out("odds", "odds in favour p/(1-p)"), _Out("odds_against", "odds against (1-p)/p"),
              _Out("log_odds", "natural log-odds ln(p/(1-p))")],
     primary="odds", conditions=["0 < p < 1"], method="definition of odds", run=run_odds,
     reference=lambda probability: (lambda sp: {"log_odds": float(sp.logit(probability)),
                                                "odds": float(math.exp(sp.logit(probability))),
                                                "odds_against": float(math.exp(-sp.logit(probability)))})(
         __import__("scipy.special", fromlist=["logit"])),
     latex=r"\text{odds} = \frac{p}{1-p},\quad \operatorname{logit} p = \ln\frac{p}{1-p}",
     wrong=lambda probability: dict(run_odds(probability), odds=(1 - probability) / probability),
     rnd=lambda rng: {"probability": _rp(rng, 0.001, 0.999)},
     need=[r"\bodds\b", r"probability"],
     avoid=[r"likelihood ratio|bayes factor|posterior|prior|to (?:a )?probability|corresponds to odds|odds\s*(?:=|of)\s*\d"],
     aliases={"probability": r"probability"},
     templates=["Convert the probability 0.2 to odds.", "What are the odds for a probability of 75%?"],
     examples=[_X({"probability": 0.2}, {"odds": 0.25, "odds_against": 4.0}), _X({"probability": 0.75}, {"odds": 3.0})],
     questions=[_Q("Convert the probability 0.2 to odds.", {"odds": 0.25}),
                _Q("What are the odds for a probability of 75%?", {"odds": 3.0, "log_odds": math.log(3.0)}),
                _Q("Convert the probability 1.5 to odds.", outcome="INVALID_INPUT")])


def run_odds_to_prob(odds):
    o = _fin(odds, "the odds")
    if o < 0:
        raise ValueError("odds must not be negative")
    return {"probability": o / (1.0 + o), "complement": 1.0 / (1.0 + o)}


_cap("odds_to_probability", level="beginner", name="Probability from odds", topic=_TO,
     purpose="The probability o/(1+o) that corresponds to odds o in favour.",
     inputs=[_Num("odds", "odds in favour (>= 0)")],
     outputs=[_Out("probability", "probability o/(1+o)"), _Out("complement", "1 - probability")],
     primary="probability", conditions=["odds >= 0"], method="p = o/(1+o)", run=run_odds_to_prob,
     reference=lambda odds: (lambda sp: {"probability": float(sp.expit(math.log(odds))) if odds > 0 else 0.0,
                                         "complement": float(sp.expit(-math.log(odds))) if odds > 0 else 1.0})(
         __import__("scipy.special", fromlist=["expit"])),
     latex=r"p = \frac{o}{1+o}",
     wrong=lambda odds: dict(run_odds_to_prob(odds), probability=1 / (1 + odds)),
     rnd=lambda rng: {"odds": round(rng.choice([rng.uniform(0.01, 1.0), rng.uniform(1.0, 50.0)]), 4)},
     need=[r"\bodds\b", r"(?:to|into|as) (?:a )?probability|probability (?:for|from|of) odds|corresponds to odds"], prio=6,
     avoid=[r"likelihood ratio|bayes factor|posterior|prior"], aliases={"odds": r"odds"},
     templates=["Convert odds of 3 to a probability.", "What probability corresponds to odds = 0.25?"],
     examples=[_X({"odds": 3}, {"probability": 0.75}), _X({"odds": 0.25}, {"probability": 0.2})],
     questions=[_Q("Convert odds of 3 to a probability.", {"probability": 0.75}),
                _Q("What probability corresponds to odds = 0.25?", {"probability": 0.2}),
                _Q("Convert odds of -2 to a probability.", outcome="INVALID_INPUT")])


def run_lr(sensitivity, specificity):
    se, sp = _prob(sensitivity, "sensitivity"), _prob(specificity, "specificity")
    if sp >= 1.0 or se >= 1.0 or sp <= 0.0:
        raise ValueError("sensitivity and specificity must be below 1 (and specificity above 0) for finite ratios")
    lp, ln_ = se / (1.0 - sp), (1.0 - se) / sp
    return {"lr_positive": lp, "lr_negative": ln_, "diagnostic_odds_ratio": lp / ln_ if ln_ > 0 else float("inf")}


def ref_lr(sensitivity, specificity):
    """Odds-ratio identity: at a pre-test probability of 1/2, post-test odds equal the likelihood ratio; the
    predictive values are computed by normalizing a numpy joint table."""
    import numpy as np
    joint = np.array([[0.5 * sensitivity, 0.5 * (1 - sensitivity)], [0.5 * (1 - specificity), 0.5 * specificity]])
    ppv = joint[0, 0] / joint[:, 0].sum()
    fomr = joint[0, 1] / joint[:, 1].sum()
    lp, ln_ = ppv / (1 - ppv), fomr / (1 - fomr)
    return {"lr_positive": float(lp), "lr_negative": float(ln_), "diagnostic_odds_ratio": float(lp / ln_)}


_cap("likelihood_ratios", level="beginner", name="Likelihood ratios of a diagnostic test", topic=_TO,
     purpose="Positive and negative likelihood ratios and the diagnostic odds ratio from sensitivity and specificity.",
     inputs=[_DX_IN[1], _DX_IN[2]],
     outputs=[_Out("lr_positive", "LR+ = sensitivity / (1 - specificity)"),
              _Out("lr_negative", "LR- = (1 - sensitivity) / specificity"),
              _Out("diagnostic_odds_ratio", "DOR = LR+ / LR-")],
     primary="lr_positive", conditions=["0 < specificity < 1", "sensitivity < 1"],
     method="ratios of the probabilities of a result with and without the condition", run=run_lr, reference=ref_lr,
     latex=r"LR^{+} = \frac{\mathrm{Se}}{1-\mathrm{Sp}},\quad LR^{-} = \frac{1-\mathrm{Se}}{\mathrm{Sp}}",
     wrong=lambda sensitivity, specificity: dict(run_lr(sensitivity, specificity),
                                                 lr_positive=sensitivity / specificity),
     rnd=lambda rng: {"sensitivity": _rp(rng, 0.3, 0.995), "specificity": _rp(rng, 0.3, 0.995)},
     need=[r"likelihood ratio|diagnostic odds", r"sensitiv", r"specific"],
     avoid=[r"prevalence|pre-?test|prior|posterior|post-test"], aliases=_DX_AL,
     templates=["Find the likelihood ratios of a test with sensitivity 0.9 and specificity 0.8.",
                "Sensitivity = 95%, specificity = 90%: what are LR+, LR- and the diagnostic odds ratio?"],
     examples=[_X({"sensitivity": 0.9, "specificity": 0.8}, {"lr_positive": 4.5, "lr_negative": 0.125}),
               _X({"sensitivity": 0.95, "specificity": 0.9}, {"lr_positive": 9.5, "diagnostic_odds_ratio": 171.0})],
     questions=[_Q("Find the likelihood ratios of a test with sensitivity 0.9 and specificity 0.8.",
                  {"lr_positive": 4.5, "lr_negative": 0.125, "diagnostic_odds_ratio": 36.0}),
                _Q("Sensitivity = 95%, specificity = 90%: what are LR+, LR- and the diagnostic odds ratio?",
                  {"lr_positive": 9.5}),
                _Q("Find the likelihood ratios of a test with sensitivity 0.9 and specificity 1.", outcome="INVALID_INPUT"),
                _Q("Find the likelihood ratios of a test with sensitivity 0.9 and an unknown specificity.", outcome="NEEDS_INPUT")])


def run_post_lr(prior, likelihood_ratio):
    p = _prob(prior, "the pre-test probability", open_=True)
    lr = _fin(likelihood_ratio, "the likelihood ratio")
    if lr < 0:
        raise ValueError("a likelihood ratio must not be negative")
    o = p / (1.0 - p) * lr
    return {"posterior": o / (1.0 + o), "prior_odds": p / (1.0 - p), "posterior_odds": o}


_cap("posterior_from_likelihood_ratio", level="beginner", name="Post-test probability from a likelihood ratio",
     topic=_TO,
     purpose="Bayes' theorem in odds form: post-test odds = pre-test odds x likelihood ratio, and the post-test "
             "probability.",
     inputs=[_Num("prior", "pre-test (prior) probability, 0 < p < 1"), _Num("likelihood_ratio", "likelihood ratio (>= 0)")],
     outputs=[_Out("posterior", "post-test (posterior) probability"), _Out("prior_odds", "pre-test odds"),
              _Out("posterior_odds", "post-test odds")],
     primary="posterior", conditions=["0 < prior < 1", "likelihood ratio >= 0"],
     method="odds form of Bayes' theorem", run=run_post_lr,
     reference=lambda prior, likelihood_ratio: (lambda sp: {
         "posterior": float(sp.expit(sp.logit(prior) + math.log(likelihood_ratio))),
         "prior_odds": float(math.exp(sp.logit(prior))),
         "posterior_odds": float(math.exp(sp.logit(prior) + math.log(likelihood_ratio)))})(
         __import__("scipy.special", fromlist=["expit"])),
     latex=r"\frac{P(H\mid E)}{1-P(H\mid E)} = \frac{P(H)}{1-P(H)}\times LR",
     wrong=lambda prior, likelihood_ratio: dict(run_post_lr(prior, likelihood_ratio),
                                                posterior=prior * likelihood_ratio),
     rnd=lambda rng: {"prior": _rp(rng, 0.01, 0.95), "likelihood_ratio": round(rng.choice(
         [rng.uniform(0.02, 1.0), rng.uniform(1.0, 30.0)]), 3)},
     need=[r"likelihood ratio", r"pre-?test probability|prior probability|prior\s*="],
     avoid=[r"sensitiv|specific|bayes factor"],
     aliases={"prior": r"pre-?test probability|prior probability|prior",
              "likelihood_ratio": r"likelihood ratio|lr"},
     templates=["The pre-test probability is 0.2 and the likelihood ratio is 6. What is the post-test probability?",
                "Prior probability = 10%, likelihood ratio = 0.2: find the posterior probability."],
     examples=[_X({"prior": 0.2, "likelihood_ratio": 6}, {"posterior": 0.6, "posterior_odds": 1.5}),
               _X({"prior": 0.1, "likelihood_ratio": 0.2}, {"posterior": (0.2 / 9) / (1 + 0.2 / 9)})],
     questions=[_Q("The pre-test probability is 0.2 and the likelihood ratio is 6. What is the post-test probability?",
                  {"posterior": 0.6}),
                _Q("Prior probability = 10%, likelihood ratio = 0.2: find the posterior probability.",
                  {"posterior": (0.2 / 9) / (1 + 0.2 / 9)}),
                _Q("The pre-test probability is 0.2 and the likelihood ratio is -1. What is the post-test probability?",
                  outcome="INVALID_INPUT"),
                _Q("The likelihood ratio is 6. What is the post-test probability given the pre-test probability?",
                  outcome="NEEDS_INPUT")])


def run_sequential(prevalence, sensitivity, specificity, positives, negatives=0):
    pv, se, sp = (_prob(prevalence, "prevalence", open_=True), _prob(sensitivity, "sensitivity"),
                  _prob(specificity, "specificity"))
    k, m = _whole(positives, "the number of positive results"), _whole(negatives, "the number of negative results")
    if k + m < 1:
        raise ValueError("at least one test result is needed")
    lw = math.log(pv) - math.log1p(-pv)
    for count, num, den in ((k, se, 1.0 - sp), (m, 1.0 - se, sp)):
        if count:
            if num <= 0.0 and den <= 0.0:
                raise ValueError("this result is impossible with these values")
            if num <= 0.0:
                return {"posterior": 0.0, "posterior_odds": 0.0}
            if den <= 0.0:
                return {"posterior": 1.0, "posterior_odds": float("inf")}
            lw += count * (math.log(num) - math.log(den))
    o = math.exp(lw) if lw < 700 else float("inf")
    post = 1.0 / (1.0 + math.exp(-lw)) if lw > -700 else 0.0
    return {"posterior": post, "posterior_odds": o}


def ref_sequential(prevalence, sensitivity, specificity, positives, negatives=0):
    """Update a numpy probability vector one result at a time (renormalizing after each test)."""
    import numpy as np
    p = np.array([prevalence, 1 - prevalence])
    for lik in [np.array([sensitivity, 1 - specificity])] * positives + [np.array([1 - sensitivity, specificity])] * negatives:
        p = p * lik / np.dot(p, lik)
    return {"posterior": float(p[0]), "posterior_odds": float(p[0] / p[1])}


_cap("sequential_test_posterior", level="intermediate", name="Posterior after repeated independent tests",
     topic=_TB,
     purpose="Probability of the condition after several conditionally independent test results (k positive and m "
             "negative), by multiplying the pre-test odds by LR+ for each positive and LR- for each negative.",
     inputs=_DX_IN + [_Int("positives", "number of positive results"),
                      _Int("negatives", "number of negative results (default 0)", required=False)],
     outputs=[_Out("posterior", "probability of the condition after all results"), _Out("posterior_odds", "its odds")],
     primary="posterior", conditions=["0 < prevalence < 1", "results conditionally independent given the status"],
     method="odds form, applied once per result (in log space)", run=run_sequential, reference=ref_sequential,
     latex=r"\text{odds}_{post} = \frac{\pi}{1-\pi}\,(LR^{+})^{k}\,(LR^{-})^{m}",
     wrong=lambda prevalence, sensitivity, specificity, positives, negatives=0: run_sequential(
         prevalence, sensitivity, specificity, 1, 0) if positives + negatives > 1 else run_sequential(
         prevalence, sensitivity, specificity, 2, 0),
     rnd=lambda rng: {"prevalence": _rp(rng, 0.001, 0.5), "sensitivity": _rp(rng, 0.6, 0.99),
                      "specificity": _rp(rng, 0.6, 0.99), "positives": rng.randint(1, 4), "negatives": rng.randint(0, 2)},
     need=[r"positives\s*(?:=|:)|" + _RESULT_COUNTS, r"sensitiv", r"specific"], prio=6,
     aliases=dict(_DX_AL, positives=r"positives", negatives=r"negatives"), extract=_x_results,
     templates=["Repeated testing: prevalence 0.01, sensitivity 0.9, specificity 0.95, positives = 2, negatives = 0. "
                "Posterior probability of the disease?",
                "Prevalence = 5%, sensitivity = 80%, specificity = 90%, positives = 1, negatives = 1: probability of "
                "the condition?"],
     examples=[_X({"prevalence": 0.01, "sensitivity": 0.9, "specificity": 0.95, "positives": 2, "negatives": 0},
                 {"posterior": (0.01 * 324) / (0.99 + 0.01 * 324)}),
               _X({"prevalence": 0.05, "sensitivity": 0.8, "specificity": 0.9, "positives": 1, "negatives": 1},
                 {"posterior": (0.05 * 8 * (0.2 / 0.9)) / (0.95 + 0.05 * 8 * (0.2 / 0.9))})],
     questions=[_Q("Repeated testing: prevalence 0.01, sensitivity 0.9, specificity 0.95, positives = 2, negatives = 0. "
                  "Posterior probability of the disease?", {"posterior": (0.01 * 324) / (0.99 + 0.01 * 324)}),
                _Q("Prevalence = 5%, sensitivity = 80%, specificity = 90%, positives = 3: probability of the condition?",
                  {"posterior": (0.05 * 512) / (0.95 + 0.05 * 512)}),
                _Q("Prevalence = 0.1, sensitivity = 0.9, specificity = 0.9, positives = 1.5: posterior?",
                  outcome="INVALID_INPUT"),
                _Q("Prevalence = 0.1, sensitivity = 0.9, positives = 2: posterior given the specificity?",
                  outcome="NEEDS_INPUT")])


def run_bf_posterior(bayes_factor, prior=0.5):
    b = _fin(bayes_factor, "the Bayes factor")
    if b <= 0:
        raise ValueError("a Bayes factor must be positive")
    p = _prob(0.5 if prior is None else prior, "the prior probability of H1", open_=True)
    o = p / (1.0 - p) * b
    return {"posterior": o / (1.0 + o), "posterior_odds": o, "posterior_h0": 1.0 / (1.0 + o)}


_cap("posterior_from_bayes_factor", level="beginner", name="Posterior probability of a hypothesis from a Bayes factor",
     topic="Bayes factors and model comparison",
     purpose="Posterior probability of H1 from the Bayes factor BF10 and the prior probability of H1 (default 1/2).",
     inputs=[_Num("bayes_factor", "Bayes factor BF10 in favour of H1 (> 0)"),
             _Num("prior", "prior probability of H1 (default 0.5)", required=False)],
     outputs=[_Out("posterior", "posterior probability of H1"), _Out("posterior_odds", "posterior odds H1 : H0"),
              _Out("posterior_h0", "posterior probability of H0")],
     primary="posterior", conditions=["BF10 > 0", "0 < prior < 1"], method="posterior odds = BF10 x prior odds",
     run=run_bf_posterior,
     reference=lambda bayes_factor, prior=0.5: (lambda sp: {
         "posterior": float(sp.expit(math.log(bayes_factor) + sp.logit(prior))),
         "posterior_h0": float(sp.expit(-math.log(bayes_factor) - sp.logit(prior))),
         "posterior_odds": float(math.exp(math.log(bayes_factor) + sp.logit(prior)))})(
         __import__("scipy.special", fromlist=["expit"])),
     latex=r"\frac{P(H_1\mid D)}{P(H_0\mid D)} = BF_{10}\,\frac{P(H_1)}{P(H_0)}",
     wrong=lambda bayes_factor, prior=0.5: dict(run_bf_posterior(bayes_factor, prior), posterior=prior * bayes_factor),
     rnd=lambda rng: {"bayes_factor": round(rng.choice([rng.uniform(0.01, 1.0), rng.uniform(1.0, 100.0)]), 4),
                      "prior": _rp(rng, 0.05, 0.95)},
     need=[r"bayes factor", r"posterior probability"],
     avoid=[r"successes|trials|p0\s*=|p1\s*=|sample mean|xbar|marginal likelihoods|bic",
            # the input is BF10 (in favour of H1): a Bayes factor stated the other way is not read as BF10
            r"\bbf\s*_?\s*01\b|\bb01\b|in favou?r of (?:h0|the null)|(?:supports?|favou?rs?) (?:h0|the null)"],
     aliases={"bayes_factor": r"bayes factor(?: bf10)?|bf10|bf", "prior": r"prior probability(?: of h1)?|prior"},
     templates=["A Bayes factor of 6 with prior probability 0.5: what is the posterior probability of H1?",
                "Bayes factor = 0.2 and prior probability = 0.3. Find the posterior probability."],
     examples=[_X({"bayes_factor": 6, "prior": 0.5}, {"posterior": 6 / 7}),
               _X({"bayes_factor": 0.2, "prior": 0.3}, {"posterior": (0.3 / 0.7 * 0.2) / (1 + 0.3 / 0.7 * 0.2)})],
     questions=[_Q("A Bayes factor of 6 with prior probability 0.5: what is the posterior probability of H1?",
                  {"posterior": 6 / 7}),
                _Q("Bayes factor = 3. Find the posterior probability of H1 with equal prior odds.", {"posterior": 0.75}),
                _Q("Bayes factor = -2 and prior probability = 0.3. Find the posterior probability.",
                  outcome="INVALID_INPUT")])


def run_succession(successes, trials):
    n = _whole(trials, "the number of trials")
    k = _whole(successes, "the number of successes")
    if k > n:
        raise ValueError("successes cannot exceed trials")
    return {"p_next": (k + 1.0) / (n + 2.0), "mle": k / n if n else None}


_cap("rule_of_succession", level="beginner", name="Laplace's rule of succession",
     topic="conjugate priors: beta-binomial",
     purpose="Probability that the next trial is a success after k successes in n trials, with a uniform prior on the "
             "success probability: (k + 1)/(n + 2).",
     inputs=[_Int("successes", "number of successes k"), _Int("trials", "number of trials n")],
     outputs=[_Out("p_next", "posterior predictive probability that the next trial succeeds"),
              _Out("mle", "observed proportion k/n (none when n = 0)")],
     primary="p_next", conditions=["0 <= k <= n", "uniform Beta(1, 1) prior"],
     method="posterior mean of Beta(k + 1, n - k + 1)", run=run_succession,
     reference=lambda successes, trials: {"p_next": float(__import__("scipy.stats", fromlist=["betabinom"]).betabinom.pmf(
         1, 1, successes + 1, trials - successes + 1))},
     latex=r"P(\text{next success}) = \frac{k+1}{n+2}",
     wrong=lambda successes, trials: dict(run_succession(successes, trials),
                                          p_next=successes / trials if trials else 0.0),
     rnd=lambda rng: (lambda n: {"trials": n, "successes": rng.randint(0, n)})(rng.randint(1, 200)),
     need=r"rule of succession|next (?:trial|toss|draw|one) (?:is|will be) a success",
     aliases={"successes": r"successes", "trials": r"trials"}, extract=_x_trials,
     templates=["Rule of succession: 7 successes in 10 trials. Probability the next trial is a success?",
                "After successes = 0 and trials = 5, what is the probability that the next trial is a success "
                "(rule of succession)?"],
     examples=[_X({"successes": 7, "trials": 10}, {"p_next": 8 / 12}), _X({"successes": 0, "trials": 5}, {"p_next": 1 / 7})],
     questions=[_Q("Rule of succession: 7 successes in 10 trials. Probability the next trial is a success?",
                  {"p_next": 8 / 12}),
                _Q("The sun has risen on 9 successes in 9 trials; by the rule of succession, what is the probability "
                  "of the next?", {"p_next": 10 / 11}),
                _Q("Rule of succession: 12 successes in 10 trials.", outcome="INVALID_INPUT"),
                _Q("Rule of succession with successes = 4.", outcome="NEEDS_INPUT")])

# ============================================================== reference helpers (certification only) ====
def _ref_grid(logf, lo, hi):
    """Mode, scale and break points of an unnormalized log-density (for numerical integration in references)."""
    from scipy import optimize
    raw = logf
    logf = lambda x: raw(x) if lo < x < hi else -math.inf          # noqa: E731
    top = hi if math.isfinite(hi) else 1e4
    eps = 1e-12 * (top - lo)
    c = optimize.minimize_scalar(lambda x: -logf(x), bounds=(lo + eps, top - eps), method="bounded",
                                 options={"xatol": 1e-12 * max(1.0, top)}).x
    h = 1e-4 * max(min(c - lo, (top - c) if math.isfinite(hi) else c), 1e-8)
    d2 = (logf(c + h) - 2 * logf(c) + logf(c - h)) / (h * h)
    s = 1 / math.sqrt(-d2) if d2 < 0 and math.isfinite(d2) else (top - lo) / 10
    pts = sorted({min(max(c + k * s, lo), top) for k in (-40, -12, -5, -2, 0, 2, 5, 12, 40)} - {lo, hi})
    return c, s, pts


def _ref_integral(g, lo, hi, pts):
    from scipy import integrate
    edges = [lo] + [p for p in pts if lo < p < hi] + ([hi] if math.isfinite(hi) else [])
    tot = math.fsum(integrate.quad(g, a, b, epsabs=0, epsrel=1e-13, limit=500)[0] for a, b in zip(edges, edges[1:]))
    if not math.isfinite(hi):
        tot += integrate.quad(g, edges[-1], math.inf, epsabs=0, epsrel=1e-13, limit=500)[0]
    return tot


def _ref_moments(logf, lo, hi):
    """Posterior mean and variance by numerical integration of an unnormalized log posterior (prior x likelihood)."""
    c, s, pts = _ref_grid(logf, lo, hi)
    l0 = logf(c)
    g = lambda x: math.exp(logf(x) - l0) if lo < x < hi else 0.0          # noqa: E731
    z = _ref_integral(g, lo, hi, pts)
    m = _ref_integral(lambda x: x * g(x), lo, hi, pts) / z
    v = _ref_integral(lambda x: (x - m) ** 2 * g(x), lo, hi, pts) / z
    return m, v


def _ref_quantile(logf, lo, hi, probs):
    """Quantiles of a posterior given by an unnormalized log density: integrated CDF, solved with brentq."""
    from scipy import optimize
    c, s, pts = _ref_grid(logf, lo, hi)
    l0 = logf(c)
    g = lambda x: math.exp(logf(x) - l0) if lo < x < hi else 0.0          # noqa: E731
    z = _ref_integral(g, lo, hi, pts)
    out = []
    for p in probs:
        f = lambda x: _ref_integral(g, lo, x, pts) / z - p if p <= 0.5 else \
            p - 1 + _ref_integral(g, x, hi, pts) / z                         # noqa: E731
        a, b = lo + 1e-300, (hi if math.isfinite(hi) else c + 200 * s)
        a = max(a, c - 60 * s) if lo == -math.inf else a
        out.append(optimize.brentq(f, a, b, xtol=1e-15, rtol=8.9e-16, maxiter=500))
    return out


def _log_beta_prior_binom(a, b, k, n):
    """log prior density x binomial likelihood, written from their definitions (no conjugate update)."""
    lc = math.lgamma(n + 1) - math.lgamma(k + 1) - math.lgamma(n - k + 1)
    lb = math.lgamma(a) + math.lgamma(b) - math.lgamma(a + b)
    return lambda p: ((a - 1) * math.log(p) + (b - 1) * math.log1p(-p) - lb
                      + lc + k * math.log(p) + (n - k) * math.log1p(-p))


# ===================================================================================== conjugate: beta-binomial ====
_TBB = "conjugate priors: beta-binomial"
_BB_IN = [_Num("alpha", "prior Beta alpha (> 0)"), _Num("beta", "prior Beta beta (> 0)"),
          _Int("successes", "number of successes k"), _Int("trials", "number of trials n")]
_BB_AL = {"alpha": r"alpha|a0", "beta": r"beta|b0", "successes": r"successes|k", "trials": r"trials|n"}
_BB_X = _chain(_x_beta_prior, _x_trials, _x_level)


def _bb_check(alpha, beta, successes, trials):
    a, b = _pos(alpha, "alpha"), _pos(beta, "beta")
    n, k = _whole(trials, "the number of trials"), _whole(successes, "the number of successes")
    if k > n:
        raise ValueError("successes cannot exceed trials")
    return a, b, k, n


def run_bb_post(alpha, beta, successes, trials):
    a, b, k, n = _bb_check(alpha, beta, successes, trials)
    a2, b2 = a + k, b + n - k
    m = a2 / (a2 + b2)
    v = a2 * b2 / ((a2 + b2) ** 2 * (a2 + b2 + 1))
    return {"post_alpha": a2, "post_beta": b2, "posterior_mean": m, "posterior_variance": v,
            "posterior_sd": math.sqrt(v)}


def ref_bb_post(alpha, beta, successes, trials):
    """Numerical integration of prior density x binomial likelihood (scipy.integrate.quad); the posterior Beta
    parameters recovered from its mean and variance by the method of moments."""
    m, v = _ref_moments(_log_beta_prior_binom(alpha, beta, successes, trials), 0.0, 1.0)
    nu = m * (1 - m) / v - 1
    return {"posterior_mean": m, "posterior_variance": v, "posterior_sd": math.sqrt(v), "post_alpha": m * nu,
            "post_beta": (1 - m) * nu}


def _rbb(rng, lo=1.05):
    n = rng.randint(1, 60)
    return {"alpha": round(rng.uniform(lo, 10), 2), "beta": round(rng.uniform(lo, 10), 2), "trials": n,
            "successes": rng.randint(0, n)}


_BB_AVOID = (r"interval|credible|hdi|hpd|highest|mode\b|\bmap\b|maximum a posteriori|probability that|exceed|"
             r"predictive|future|next|evidence|marginal likelihood|laplace|bayes factor|succession")

_cap("beta_binomial_posterior", level="intermediate", name="Beta-binomial conjugate posterior", topic=_TBB,
     purpose="Update a Beta(alpha, beta) prior on a success probability with k successes in n trials: posterior "
             "parameters, mean, variance and standard deviation.",
     inputs=_BB_IN,
     outputs=[_Out("post_alpha", "posterior alpha = alpha + k"), _Out("post_beta", "posterior beta = beta + n - k"),
              _Out("posterior_mean", "posterior mean"), _Out("posterior_variance", "posterior variance"),
              _Out("posterior_sd", "posterior standard deviation")],
     primary="posterior_mean", conditions=["alpha, beta > 0", "0 <= k <= n"],
     method="conjugate update of the beta prior with the binomial likelihood", run=run_bb_post, reference=ref_bb_post,
     latex=r"p\mid k \sim \mathrm{Beta}(\alpha + k,\ \beta + n - k),\quad E[p\mid k] = \frac{\alpha + k}{\alpha + \beta + n}",
     wrong=lambda alpha, beta, successes, trials: (lambda a2, b2: {
         "post_alpha": a2, "post_beta": b2, "posterior_mean": a2 / (a2 + b2),
         "posterior_variance": a2 * b2 / ((a2 + b2) ** 2 * (a2 + b2 + 1)),
         "posterior_sd": math.sqrt(a2 * b2 / ((a2 + b2) ** 2 * (a2 + b2 + 1)))})(alpha + successes, beta + trials),
     rnd=_rbb, need=[r"posterior", r"beta\(|beta prior|alpha\s*=|uniform prior|jeffreys prior",
                     r"successes|trials"], avoid=[_BB_AVOID], aliases=_BB_AL, extract=_BB_X,
     templates=["Beta-binomial posterior: prior Beta(2, 2), 7 successes in 10 trials. Find the posterior mean.",
                "With a uniform prior and 3 successes in 20 trials, what is the posterior distribution of p?",
                "Beta prior alpha = 1, beta = 1; successes = 4, trials = 9: posterior mean and variance?"],
     examples=[_X({"alpha": 2, "beta": 2, "successes": 7, "trials": 10},
                 {"post_alpha": 9.0, "post_beta": 5.0, "posterior_mean": 9 / 14, "posterior_variance": 45 / (196 * 15)}),
               _X({"alpha": 1, "beta": 1, "successes": 3, "trials": 20}, {"posterior_mean": 4 / 22})],
     questions=[_Q("Beta-binomial posterior: prior Beta(2, 2), 7 successes in 10 trials. Find the posterior mean.",
                   {"posterior_mean": 9 / 14, "post_alpha": 9.0, "post_beta": 5.0}),
                _Q("With a uniform prior and 3 successes in 20 trials, what is the posterior distribution of p?",
                   {"post_alpha": 4.0, "post_beta": 18.0}),
                _Q("Beta-binomial posterior: prior Beta(2, 2), 12 successes in 10 trials. Posterior mean?",
                   outcome="INVALID_INPUT"),
                _Q("Beta-binomial posterior: prior Beta(2, 2), successes = 7. Posterior mean?", outcome="NEEDS_INPUT")])


def run_beta_ci(alpha, beta, level=None):
    a, b, c = _pos(alpha, "alpha"), _pos(beta, "beta"), _level(level)
    return {"lower": _beta_ppf(0.5 * (1 - c), a, b), "upper": _beta_ppf(0.5 * (1 + c), a, b),
            "median": _beta_ppf(0.5, a, b)}


def ref_beta_ci(alpha, beta, level=None):
    from scipy.stats import beta as B
    c = 0.95 if level is None else level
    return {"lower": float(B.ppf((1 - c) / 2, alpha, beta)), "upper": float(B.isf((1 - c) / 2, alpha, beta)),
            "median": float(B.median(alpha, beta))}


def _wrong_ci_normal(m, sd, c):
    z = _z(0.5 * (1 + c))
    return m - z * sd, m + z * sd


_cap("beta_credible_interval", level="intermediate", name="Equal-tailed credible interval of a beta distribution",
     topic="credible intervals",
     purpose="The central (equal-tailed) credible interval and the median of a Beta(alpha, beta) posterior.",
     inputs=[_Num("alpha", "Beta alpha (> 0)"), _Num("beta", "Beta beta (> 0)"), LEVEL_IN],
     outputs=[_Out("lower", "lower limit (quantile (1 - level)/2)"), _Out("upper", "upper limit (quantile (1 + level)/2)"),
              _Out("median", "posterior median")],
     primary="lower", conditions=["alpha, beta > 0", "0 < level < 1"],
     method="beta quantiles by safeguarded Newton inversion of the regularized incomplete beta function",
     run=run_beta_ci, reference=ref_beta_ci,
     latex=r"\left[F^{-1}_{\mathrm{Beta}(\alpha,\beta)}\!\left(\tfrac{1-c}{2}\right),\ F^{-1}_{\mathrm{Beta}(\alpha,\beta)}\!\left(\tfrac{1+c}{2}\right)\right]",
     wrong=lambda alpha, beta, level=None: (lambda c, m, sd: dict(zip(("lower", "upper"), _wrong_ci_normal(m, sd, c)),
                                                                 median=m))(
         0.95 if level is None else level, alpha / (alpha + beta),
         math.sqrt(alpha * beta / ((alpha + beta) ** 2 * (alpha + beta + 1)))),
     rnd=lambda rng: {"alpha": round(rng.uniform(0.5, 50), 2), "beta": round(rng.uniform(0.5, 50), 2),
                      "level": _rlevel(rng)},
     need=[r"credible|probability interval", r"beta\(|beta distribution|beta posterior"],
     avoid=[r"successes|trials|highest|hdi|hpd|shortest|narrowest|jeffreys|laplace"], aliases={"alpha": r"alpha", "beta": r"beta"},
     extract=_chain(_x_beta_prior, _x_level),
     templates=["Equal-tailed 95% credible interval for a Beta(3, 9) posterior.",
                "Find the 90% credible interval of a beta distribution with alpha = 12 and beta = 4."],
     examples=[_X({"alpha": 1, "beta": 1, "level": 0.9}, {"lower": 0.05, "upper": 0.95, "median": 0.5}),
               _X({"alpha": 3, "beta": 9}, {})],
     questions=[_Q("Equal-tailed 90% credible interval for a Beta(1, 1) posterior.", {"lower": 0.05, "upper": 0.95}),
                _Q("Find the 50% credible interval of a beta distribution with alpha = 2 and beta = 1.",
                   {"lower": math.sqrt(0.25), "upper": math.sqrt(0.75)}),
                _Q("Equal-tailed 95% credible interval for a Beta(0, 9) posterior.", outcome="INVALID_INPUT"),
                _Q("Find the credible interval of a beta distribution with alpha = 12.", outcome="NEEDS_INPUT")])


def run_bb_ci(alpha, beta, successes, trials, level=None):
    a, b, k, n = _bb_check(alpha, beta, successes, trials)
    r = run_beta_ci(a + k, b + n - k, level)
    return {"post_alpha": a + k, "post_beta": b + n - k, "lower": r["lower"], "upper": r["upper"]}


def ref_bb_ci(alpha, beta, successes, trials, level=None):
    """Quantiles of prior x likelihood integrated numerically (quad), solved with brentq; parameters by moments."""
    c = 0.95 if level is None else level
    logf = _log_beta_prior_binom(alpha, beta, successes, trials)
    lo, up = _ref_quantile(logf, 0.0, 1.0, [(1 - c) / 2, (1 + c) / 2])
    m, v = _ref_moments(logf, 0.0, 1.0)
    nu = m * (1 - m) / v - 1
    return {"lower": lo, "upper": up, "post_alpha": m * nu, "post_beta": (1 - m) * nu}


_cap("beta_binomial_credible_interval", level="intermediate",
     name="Credible interval for a proportion (beta-binomial)", topic="credible intervals",
     purpose="Equal-tailed credible interval of a success probability after k successes in n trials with a "
             "Beta(alpha, beta) prior.",
     inputs=_BB_IN + [LEVEL_IN],
     outputs=[_Out("post_alpha", "posterior alpha"), _Out("post_beta", "posterior beta"),
              _Out("lower", "lower credible limit"), _Out("upper", "upper credible limit")],
     primary="lower", conditions=["alpha, beta > 0", "0 <= k <= n", "0 < level < 1"],
     method="conjugate update, then beta quantiles", run=run_bb_ci, reference=ref_bb_ci,
     latex=r"p\mid k \sim \mathrm{Beta}(\alpha+k,\beta+n-k),\quad [q_{(1-c)/2},\ q_{(1+c)/2}]",
     wrong=lambda alpha, beta, successes, trials, level=None: dict(
         run_bb_ci(alpha, beta, successes, trials, level),
         **{k_: v for k_, v in run_beta_ci(alpha + successes, beta + trials, level).items() if k_ != "median"}),
     rnd=lambda rng: dict(_rbb(rng), level=_rlevel(rng)),
     need=[r"credible|probability interval", r"successes|trials"], avoid=[r"highest|hdi|hpd|shortest|narrowest|jeffreys|laplace"],
     aliases=_BB_AL, extract=_BB_X,
     templates=["95% credible interval for p: prior Beta(2, 2), 7 successes in 10 trials.",
                "With a uniform prior and 15 successes in 50 trials, find the 90% credible interval for the proportion."],
     examples=[_X({"alpha": 1, "beta": 1, "successes": 0, "trials": 0, "level": 0.9}, {"lower": 0.05, "upper": 0.95}),
               _X({"alpha": 2, "beta": 2, "successes": 7, "trials": 10, "level": 0.95}, {"post_alpha": 9.0})],
     questions=[_Q("90% credible interval for p: prior Beta(1, 1), 1 successes in 1 trials.",
                   {"lower": math.sqrt(0.05), "upper": math.sqrt(0.95)}),
                _Q("With a uniform prior and 15 successes in 50 trials, find the 90% credible interval for the "
                   "proportion.", {"post_alpha": 16.0, "post_beta": 36.0}),
                _Q("95% credible interval for p: prior Beta(2, 2), 11 successes in 10 trials.", outcome="INVALID_INPUT"),
                _Q("95% credible interval for p: prior Beta(2, 2) and 7 successes.", outcome="NEEDS_INPUT")])


def run_beta_tail(alpha, beta, threshold, successes=0, trials=0):
    a, b, k, n = _bb_check(alpha, beta, successes or 0, trials or 0)
    c = _prob(threshold, "the threshold", open_=True)
    lo_, up_ = _ibeta(a + k, b + n - k, c)
    return {"prob_above": up_, "prob_below": lo_}


def ref_beta_tail(alpha, beta, threshold, successes=0, trials=0):
    """Tail areas of prior x likelihood by numerical integration (quad) on each side of the threshold."""
    logf = _log_beta_prior_binom(alpha, beta, successes or 0, trials or 0)
    c, s, pts = _ref_grid(logf, 0.0, 1.0)
    l0 = logf(c)
    g = lambda x: math.exp(logf(x) - l0) if 0 < x < 1 else 0.0          # noqa: E731
    pts = sorted(set(pts) | {threshold})
    below, above = _ref_integral(g, 0.0, threshold, pts), _ref_integral(g, threshold, 1.0, pts)
    return {"prob_above": above / (below + above), "prob_below": below / (below + above)}


def _rtail(rng):
    x = _rbb(rng)
    if rng.random() < 0.3:
        x["successes"], x["trials"] = 0, 0
    a2, b2 = x["alpha"] + x["successes"], x["beta"] + x["trials"] - x["successes"]
    m, sd = a2 / (a2 + b2), math.sqrt(a2 * b2 / ((a2 + b2) ** 2 * (a2 + b2 + 1)))
    x["threshold"] = round(min(0.99, max(0.01, m + rng.uniform(-1.5, 1.5) * sd)), 3)
    return x


_cap("beta_posterior_tail_probability", level="intermediate", name="Posterior probability that p exceeds a value",
     topic=_TBB,
     purpose="P(p > c) and P(p < c) under a Beta(alpha, beta) posterior, optionally after k successes in n trials.",
     inputs=[_Num("alpha", "Beta alpha (> 0)"), _Num("beta", "Beta beta (> 0)"), _Num("threshold", "the value c, 0 < c < 1"),
             _Int("successes", "successes to add (default 0)", required=False),
             _Int("trials", "trials to add (default 0)", required=False)],
     outputs=[_Out("prob_above", "posterior probability P(p > c)"), _Out("prob_below", "posterior probability P(p < c)")],
     primary="prob_above", conditions=["alpha, beta > 0", "0 < c < 1", "0 <= k <= n"],
     method="regularized incomplete beta function of the (updated) posterior", run=run_beta_tail,
     reference=ref_beta_tail,
     latex=r"P(p > c \mid k) = 1 - I_c(\alpha + k,\ \beta + n - k)",
     wrong=lambda alpha, beta, threshold, successes=0, trials=0: (lambda r: {
         "prob_above": r["prob_below"], "prob_below": r["prob_above"]})(
         run_beta_tail(alpha, beta, threshold, successes, trials)),
     rnd=_rtail, need=[r"beta\(|alpha\s*=|uniform prior|jeffreys prior",
                       r"probability that (?:p|the (?:success )?(?:probability|rate|proportion)) "
                       r"(?:exceeds|is (?:greater|more|less|smaller|above|below|larger))|p\s*[<>]"],
     avoid=[r"credible|interval|hdi|predictive|bayes factor|beta_a|beta_b|alpha_a|alpha_b"], aliases=dict(
         _BB_AL, threshold=r"threshold|c"),
     extract=_chain(_x_beta_prior, _x_trials, lambda q, d: d.setdefault("threshold", _thr(q))
                    if _thr(q) is not None else None),
     templates=["For a Beta(8, 4) posterior, what is the probability that p exceeds 0.5?",
                "Prior Beta(1, 1) and 7 successes in 10 trials: what is the probability that p is greater than 0.6?"],
     examples=[_X({"alpha": 1, "beta": 1, "threshold": 0.3}, {"prob_above": 0.7}),
               _X({"alpha": 2, "beta": 1, "threshold": 0.5}, {"prob_above": 0.75, "prob_below": 0.25})],
     questions=[_Q("For a Beta(2, 1) posterior, what is the probability that p exceeds 0.5?", {"prob_above": 0.75}),
                _Q("Prior Beta(1, 1) and 1 successes in 1 trials: what is the probability that p is less than 0.5?",
                   {"prob_below": 0.25}),
                _Q("For a Beta(8, 4) posterior, what is the probability that p exceeds 1.5?", outcome="INVALID_INPUT")])


def _thr(q):
    m = re.search(r"(?:exceeds|greater than|more than|above|larger than|less than|smaller than|below|[<>])\s*(?:=\s*)?(" +
                  _NUM + r")(\s*%)?", q)
    if not m:
        return None
    return float(m.group(1)) / (100.0 if m.group(2) else 1.0)


def run_jeffreys(successes, trials, level=None):
    n, k = _whole(trials, "the number of trials", 1), _whole(successes, "the number of successes")
    if k > n:
        raise ValueError("successes cannot exceed trials")
    c = _level(level)
    lo = 0.0 if k == 0 else _beta_ppf(0.5 * (1 - c), k + 0.5, n - k + 0.5)
    up = 1.0 if k == n else _beta_ppf(0.5 * (1 + c), k + 0.5, n - k + 0.5)
    return {"lower": lo, "upper": up, "posterior_mean": (k + 0.5) / (n + 1.0)}


def ref_jeffreys(successes, trials, level=None):
    from scipy.stats import beta as B
    c = 0.95 if level is None else level
    a, b = successes + 0.5, trials - successes + 0.5
    return {"lower": 0.0 if successes == 0 else float(B.ppf((1 - c) / 2, a, b)),
            "upper": 1.0 if successes == trials else float(B.isf((1 - c) / 2, a, b)),
            "posterior_mean": float(B.mean(a, b))}


_cap("jeffreys_interval", level="intermediate", name="Jeffreys interval for a binomial proportion",
     topic="credible intervals",
     purpose="The equal-tailed Bayesian interval for a proportion under the Jeffreys Beta(1/2, 1/2) prior (lower "
             "limit 0 when k = 0 and upper limit 1 when k = n).",
     inputs=[_Int("successes", "number of successes k"), _Int("trials", "number of trials n (>= 1)", lo=1), LEVEL_IN],
     outputs=[_Out("lower", "lower limit"), _Out("upper", "upper limit"), _Out("posterior_mean", "(k + 1/2)/(n + 1)")],
     primary="upper", conditions=["0 <= k <= n", "n >= 1", "0 < level < 1"],
     method="quantiles of Beta(k + 1/2, n - k + 1/2)", run=run_jeffreys, reference=ref_jeffreys,
     latex=r"\left[B^{-1}\!\left(\tfrac{1-c}{2};\,k+\tfrac12,\,n-k+\tfrac12\right),\ "
           r"B^{-1}\!\left(\tfrac{1+c}{2};\,k+\tfrac12,\,n-k+\tfrac12\right)\right]",
     wrong=lambda successes, trials, level=None: dict(run_jeffreys(successes, trials, level), **{
         "lower": _beta_ppf(0.5 * (1 - (level or 0.95)), successes + 1, trials - successes + 1),
         "upper": _beta_ppf(0.5 * (1 + (level or 0.95)), successes + 1, trials - successes + 1)}),
     rnd=lambda rng: (lambda n: {"trials": n, "successes": rng.randint(0, n), "level": _rlevel(rng)})(rng.randint(1, 300)),
     need=r"jeffreys interval", aliases={"successes": r"successes|k", "trials": r"trials|n"},
     extract=_chain(_x_trials, _x_level),
     templates=["Jeffreys interval at 95% for 7 successes in 50 trials.",
                "Find the 90% Jeffreys interval with successes = 0 and trials = 20."],
     examples=[_X({"successes": 7, "trials": 50, "level": 0.95}, {"posterior_mean": 7.5 / 51}),
               _X({"successes": 0, "trials": 20, "level": 0.9}, {"lower": 0.0})],
     questions=[_Q("Jeffreys interval at 95% for 7 successes in 50 trials.", {"posterior_mean": 7.5 / 51}),
                _Q("Find the 90% Jeffreys interval with successes = 20 and trials = 20.", {"upper": 1.0}),
                _Q("Jeffreys interval at 95% for 9 successes in 5 trials.", outcome="INVALID_INPUT"),
                _Q("Jeffreys interval at 95% with successes = 9.", outcome="NEEDS_INPUT")])


def run_bb_pred(alpha, beta, future_trials, future_successes):
    a, b = _pos(alpha, "alpha"), _pos(beta, "beta")
    m, j = _whole(future_trials, "the number of future trials", 1), _whole(future_successes, "the number of future successes")
    if j > m:
        raise ValueError("future successes cannot exceed future trials")
    lc = lambda i: math.lgamma(m + 1) - math.lgamma(i + 1) - math.lgamma(m - i + 1)     # noqa: E731
    pmf = lambda i: math.exp(lc(i) + _lbeta(a + i, b + m - i) - _lbeta(a, b))          # noqa: E731
    lower = math.fsum(pmf(i) for i in range(0, j + 1))
    upper = math.fsum(pmf(i) for i in range(j + 1, m + 1))
    return {"probability": pmf(j), "prob_at_most": lower / (lower + upper), "prob_more": upper / (lower + upper),
            "predictive_mean": m * a / (a + b)}


def ref_bb_pred(alpha, beta, future_trials, future_successes):
    from scipy.stats import betabinom
    d = betabinom(future_trials, alpha, beta)
    return {"probability": float(d.pmf(future_successes)), "prob_at_most": float(d.cdf(future_successes)),
            "prob_more": float(d.sf(future_successes)), "predictive_mean": float(d.mean())}


def _rbbpred(rng):
    a, b = round(rng.uniform(1, 40), 2), round(rng.uniform(1, 40), 2)
    m = rng.randint(1, 40)
    j = min(m, max(0, int(round(m * a / (a + b) + rng.uniform(-1, 1) * math.sqrt(m) * 0.5))))
    return {"alpha": a, "beta": b, "future_trials": m, "future_successes": j}


_cap("beta_binomial_predictive", level="advanced", name="Posterior predictive probability (beta-binomial)",
     topic=_TBB,
     purpose="Probability of exactly j successes in m future trials when the success probability has a "
             "Beta(alpha, beta) posterior; the cumulative probability and the predictive mean.",
     inputs=[_Num("alpha", "posterior alpha (> 0)"), _Num("beta", "posterior beta (> 0)"),
             _Int("future_trials", "number of future trials m (>= 1)", lo=1),
             _Int("future_successes", "number of future successes j")],
     outputs=[_Out("probability", "P(exactly j successes in m trials)"), _Out("prob_at_most", "P(at most j)"),
              _Out("prob_more", "P(more than j)"), _Out("predictive_mean", "expected number of successes m alpha/(alpha+beta)")],
     primary="probability", conditions=["alpha, beta > 0", "0 <= j <= m"],
     method="beta-binomial distribution (the binomial averaged over the beta posterior)", run=run_bb_pred,
     reference=ref_bb_pred,
     latex=r"P(J = j) = \binom{m}{j}\frac{B(\alpha + j,\ \beta + m - j)}{B(\alpha, \beta)}",
     wrong=lambda alpha, beta, future_trials, future_successes: (lambda p: dict(
         run_bb_pred(alpha, beta, future_trials, future_successes),
         probability=math.comb(future_trials, future_successes) * p ** future_successes * (1 - p) ** (
             future_trials - future_successes)))(alpha / (alpha + beta)),
     rnd=_rbbpred, need=[r"predictive|next \d+ trials|future", r"beta\(|alpha\s*="],
     avoid=[r"credible|interval|bayes factor|marginal likelihood|evidence"],
     aliases={"alpha": r"alpha", "beta": r"beta", "future_trials": r"future trials|m",
              "future_successes": r"future successes|j"},
     extract=_chain(_x_beta_prior, lambda q, d: (lambda mm: (d.setdefault("future_successes", int(mm.group(1))),
                                                             d.setdefault("future_trials", int(mm.group(2))))
                                                 if mm else None)(
         re.search(r"(\d+) successes in (?:the )?next (\d+) trials", q))),
     templates=["Posterior Beta(8, 4): probability of exactly 3 successes in the next 5 trials (posterior predictive)?",
                "Posterior predictive with alpha = 2, beta = 3, future trials = 10, future successes = 4."],
     examples=[_X({"alpha": 1, "beta": 1, "future_trials": 4, "future_successes": 2}, {"probability": 0.2}),
               _X({"alpha": 8, "beta": 4, "future_trials": 5, "future_successes": 3}, {})],
     questions=[_Q("Posterior Beta(1, 1): probability of exactly 2 successes in the next 4 trials (posterior predictive)?",
                   {"probability": 0.2, "prob_at_most": 0.6}),
                _Q("Posterior predictive with alpha = 2, beta = 3, future trials = 1, future successes = 1.",
                   {"probability": 0.4}),
                _Q("Posterior predictive with alpha = 2, beta = 3, future trials = 3, future successes = 4.",
                   outcome="INVALID_INPUT")])


def run_bb_evidence(alpha, beta, successes, trials):
    a, b, k, n = _bb_check(alpha, beta, successes, trials)
    lml = (math.lgamma(n + 1) - math.lgamma(k + 1) - math.lgamma(n - k + 1) + _lbeta(a + k, b + n - k) - _lbeta(a, b))
    return {"marginal_likelihood": math.exp(lml), "log_marginal_likelihood": lml}


def ref_bb_evidence(alpha, beta, successes, trials):
    """Integral of prior density x binomial likelihood over p (quad), done relative to its peak."""
    logf = _log_beta_prior_binom(alpha, beta, successes, trials)
    c, s, pts = _ref_grid(logf, 0.0, 1.0)
    l0 = logf(c)
    z = _ref_integral(lambda x: math.exp(logf(x) - l0) if 0 < x < 1 else 0.0, 0.0, 1.0, pts)
    return {"marginal_likelihood": z * math.exp(l0), "log_marginal_likelihood": math.log(z) + l0}


_cap("beta_binomial_evidence", level="advanced", name="Marginal likelihood (evidence) of binomial data under a beta prior",
     topic="Bayes factors and model comparison",
     purpose="Prior predictive probability of k successes in n trials when p ~ Beta(alpha, beta), and its logarithm.",
     inputs=_BB_IN,
     outputs=[_Out("marginal_likelihood", "P(k | n, alpha, beta)"), _Out("log_marginal_likelihood", "its natural log")],
     primary="log_marginal_likelihood", conditions=["alpha, beta > 0", "0 <= k <= n"],
     method="closed form with log-beta functions", run=run_bb_evidence, reference=ref_bb_evidence,
     latex=r"p(k) = \binom{n}{k}\frac{B(\alpha + k,\ \beta + n - k)}{B(\alpha,\beta)}",
     wrong=lambda alpha, beta, successes, trials: (lambda l: {"marginal_likelihood": math.exp(l),
                                                             "log_marginal_likelihood": l})(
         _lbeta(alpha + successes, beta + trials - successes) - _lbeta(alpha, beta)),
     rnd=lambda rng: (lambda x: x if 0 < x["successes"] < x["trials"] else dict(x, successes=x["trials"] // 2))(
         dict(_rbb(rng, 1.05), trials=rng.randint(2, 60))),
     need=[r"marginal likelihood|evidence", r"successes|trials"], avoid=[r"bayes factor|log marginal likelihoods"],
     aliases=_BB_AL, extract=_BB_X,
     templates=["Marginal likelihood of 7 successes in 10 trials under a Beta(2, 2) prior.",
                "Evidence for successes = 3, trials = 12 with prior alpha = 1, beta = 1."],
     examples=[_X({"alpha": 1, "beta": 1, "successes": 3, "trials": 12}, {"marginal_likelihood": 1 / 13}),
               _X({"alpha": 2, "beta": 2, "successes": 7, "trials": 10}, {})],
     questions=[_Q("Marginal likelihood of 7 successes in 10 trials under a Beta(1, 1) prior.",
                   {"marginal_likelihood": 1 / 11}),
                _Q("Evidence for successes = 3, trials = 12 with prior alpha = 1, beta = 1.",
                   {"log_marginal_likelihood": -math.log(13)}),
                _Q("Marginal likelihood of 7 successes in 10 trials under a Beta(-1, 2) prior.", outcome="INVALID_INPUT")])


def run_map_bb(alpha, beta, successes, trials):
    a, b, k, n = _bb_check(alpha, beta, successes, trials)
    a2, b2 = a + k, b + n - k
    if a2 <= 1 or b2 <= 1:
        raise ValueError("the posterior mode lies on the boundary (alpha + k <= 1 or beta + n - k <= 1): no interior MAP")
    return {"map": (a2 - 1) / (a2 + b2 - 2), "mle": k / n if n else None, "posterior_mean": a2 / (a2 + b2)}


def ref_map_bb(alpha, beta, successes, trials):
    """Exact symbolic derivation (sympy): zero of the derivative of log prior + log likelihood."""
    import sympy as sp
    from fractions import Fraction
    p = sp.Symbol("p", positive=True)
    R = lambda v: sp.Rational(Fraction(v).limit_denominator(10 ** 9))      # noqa: E731
    a, b, k, n = R(alpha), R(beta), sp.Integer(successes), sp.Integer(trials)
    lp = (a - 1) * sp.log(p) + (b - 1) * sp.log(1 - p) + k * sp.log(p) + (n - k) * sp.log(1 - p)
    roots = [r for r in sp.solve(sp.diff(lp, p), p) if 0 < r < 1]
    out = {"map": float(roots[0])}
    if 0 < successes < trials:
        ll = k * sp.log(p) + (n - k) * sp.log(1 - p)
        out["mle"] = float([r for r in sp.solve(sp.diff(ll, p), p) if 0 < r < 1][0])
    return out


_cap("map_beta_binomial", level="intermediate", name="MAP estimate of a proportion (beta-binomial)", topic=_TBB,
     purpose="The posterior mode (maximum a posteriori estimate) of p after k successes in n trials with a "
             "Beta(alpha, beta) prior, with the maximum-likelihood estimate and the posterior mean for comparison.",
     inputs=_BB_IN,
     outputs=[_Out("map", "posterior mode (alpha + k - 1)/(alpha + beta + n - 2)"), _Out("mle", "k/n"),
              _Out("posterior_mean", "(alpha + k)/(alpha + beta + n)")],
     primary="map", conditions=["alpha + k > 1 and beta + n - k > 1 (interior mode)"],
     method="maximize the log posterior (closed form of its stationary point)", run=run_map_bb, reference=ref_map_bb,
     latex=r"\hat p_{MAP} = \frac{\alpha + k - 1}{\alpha + \beta + n - 2}",
     wrong=lambda alpha, beta, successes, trials: dict(run_map_bb(alpha, beta, successes, trials),
                                                       map=(alpha + successes) / (alpha + beta + trials)),
     rnd=lambda rng: (lambda x: x)(dict(_rbb(rng, lo=1.2), trials=rng.randint(1, 60))),
     need=[r"\bmap\b|maximum a posteriori|posterior mode", r"successes|trials"], aliases=_BB_AL, extract=_BB_X,
     avoid=[r"counts|poisson"],
     templates=["MAP estimate of p with prior Beta(2, 2) and 7 successes in 10 trials.",
                "Find the posterior mode with alpha = 3, beta = 5, successes = 4, trials = 12."],
     examples=[_X({"alpha": 2, "beta": 2, "successes": 7, "trials": 10}, {"map": 8 / 12, "mle": 0.7}),
               _X({"alpha": 3, "beta": 5, "successes": 4, "trials": 12}, {"map": 6 / 18})],
     questions=[_Q("MAP estimate of p with prior Beta(2, 2) and 7 successes in 10 trials.", {"map": 8 / 12}),
                _Q("Find the posterior mode with alpha = 3, beta = 5, successes = 4, trials = 12.", {"map": 6 / 18}),
                _Q("MAP estimate of p with prior Beta(1, 1) and 0 successes in 10 trials.", outcome="INVALID_INPUT")])


def run_laplace_beta(alpha, beta, level=None):
    a, b, c = _pos(alpha, "alpha"), _pos(beta, "beta"), _level(level)
    if a <= 1 or b <= 1:
        raise ValueError("the Laplace approximation needs an interior mode: alpha > 1 and beta > 1")
    m = (a - 1) / (a + b - 2)
    sd = math.sqrt(m * (1 - m) / (a + b - 2))
    z = _z(0.5 * (1 + c))
    return {"mode": m, "laplace_sd": sd, "lower": m - z * sd, "upper": m + z * sd}


def ref_laplace_beta(alpha, beta, level=None):
    """sympy: stationary point and exact second derivative of the log density; scipy normal quantile."""
    import sympy as sp
    from fractions import Fraction
    from scipy.stats import norm
    p = sp.Symbol("p", positive=True)
    R = lambda v: sp.Rational(Fraction(v).limit_denominator(10 ** 9))      # noqa: E731
    lp = (R(alpha) - 1) * sp.log(p) + (R(beta) - 1) * sp.log(1 - p)
    m = [r for r in sp.solve(sp.diff(lp, p), p) if 0 < r < 1][0]
    sd = float(1 / sp.sqrt(-sp.diff(lp, p, 2).subs(p, m)))
    z = norm.ppf(0.5 * (1 + (0.95 if level is None else level)))
    return {"mode": float(m), "laplace_sd": sd, "lower": float(m) - z * sd, "upper": float(m) + z * sd}


_cap("laplace_approximation_beta", level="advanced", name="Laplace (normal) approximation of a beta posterior",
     topic="posterior approximation",
     purpose="Approximate a Beta(alpha, beta) posterior by a normal distribution centred at its mode with variance "
             "-1/(second derivative of the log density); the approximate credible interval.",
     inputs=[_Num("alpha", "Beta alpha (> 1)"), _Num("beta", "Beta beta (> 1)"), LEVEL_IN],
     outputs=[_Out("mode", "posterior mode"), _Out("laplace_sd", "Laplace standard deviation"),
              _Out("lower", "approximate lower limit"), _Out("upper", "approximate upper limit")],
     primary="laplace_sd", conditions=["alpha > 1 and beta > 1"],
     method="second-order Taylor expansion of the log posterior at its mode", run=run_laplace_beta,
     reference=ref_laplace_beta,
     latex=r"\hat p = \frac{\alpha-1}{\alpha+\beta-2},\quad \sigma^2 = \left[\frac{\alpha-1}{\hat p^2} + "
           r"\frac{\beta-1}{(1-\hat p)^2}\right]^{-1}",
     wrong=lambda alpha, beta, level=None: (lambda r: dict(r, laplace_sd=math.sqrt(
         alpha * beta / ((alpha + beta) ** 2 * (alpha + beta + 1)))))(run_laplace_beta(alpha, beta, level)),
     rnd=lambda rng: {"alpha": round(rng.uniform(1.5, 60), 2), "beta": round(rng.uniform(1.5, 60), 2),
                      "level": _rlevel(rng)},
     need=[r"laplace approximation", r"beta\(|alpha\s*="], aliases={"alpha": r"alpha", "beta": r"beta"},
     extract=_chain(_x_beta_prior, _x_level),
     templates=["Laplace approximation of a Beta(10, 4) posterior with a 95% interval.",
                "Use the Laplace approximation for a beta posterior with alpha = 21 and beta = 11."],
     examples=[_X({"alpha": 3, "beta": 3, "level": 0.95}, {"mode": 0.5, "laplace_sd": math.sqrt(0.25 / 4)}),
               _X({"alpha": 10, "beta": 4}, {"mode": 9 / 12})],
     questions=[_Q("Laplace approximation of a Beta(3, 3) posterior with a 95% interval.",
                   {"mode": 0.5, "laplace_sd": 0.25}),
                _Q("Use the Laplace approximation for a beta posterior with alpha = 21 and beta = 11.", {"mode": 20 / 30}),
                _Q("Laplace approximation of a Beta(1, 4) posterior.", outcome="INVALID_INPUT")])


def run_beta_hdi(alpha, beta, level=None):
    a, b, c = _pos(alpha, "alpha"), _pos(beta, "beta"), _level(level)
    if a <= 1 or b <= 1:
        raise ValueError("the highest-density interval is computed for unimodal Beta with alpha > 1 and beta > 1")

    def g(l):
        u = _beta_ppf(_ibeta(a, b, l)[0] + c, a, b)
        return (_beta_logpdf(u, a, b) if u < 1 else -math.inf) - _beta_logpdf(l, a, b), u
    lo, hi = 0.0, _beta_ppf(1 - c, a, b)
    u = 1.0
    for _ in range(200):
        mid = 0.5 * (lo + hi)
        if mid in (lo, hi):
            break
        gm, um = g(mid)
        if gm > 0:
            lo = mid
        else:
            hi, u = mid, um
    l = 0.5 * (lo + hi)
    u = _beta_ppf(_ibeta(a, b, l)[0] + c, a, b)
    return {"lower": l, "upper": u, "width": u - l}


def ref_beta_hdi(alpha, beta, level=None):
    """scipy: brentq on equal density at both ends of an interval of the stated probability."""
    from scipy.optimize import brentq
    from scipy.stats import beta as B
    c = 0.95 if level is None else level
    f = lambda l: B.logpdf(B.ppf(B.cdf(l, alpha, beta) + c, alpha, beta), alpha, beta) - B.logpdf(l, alpha, beta)  # noqa: E731
    top = B.ppf(1 - c, alpha, beta)
    l = brentq(f, top * 1e-9, top * (1 - 1e-12), xtol=1e-17, rtol=8.9e-16, maxiter=500)
    u = B.ppf(B.cdf(l, alpha, beta) + c, alpha, beta)
    return {"lower": l, "upper": float(u), "width": float(u - l)}


def _rhdi(rng):
    a = round(rng.uniform(1.5, 40), 2)
    b = round(a + rng.choice([-1, 1]) * rng.uniform(1.0, 30), 2)
    return {"alpha": a, "beta": b if b > 1.2 else round(a + rng.uniform(1.0, 30), 2), "level": _rlevel(rng)}


_cap("beta_hdi", level="advanced", name="Highest-density interval of a beta posterior", topic="credible intervals",
     purpose="The shortest interval holding a stated posterior probability of a unimodal Beta(alpha, beta): equal "
             "density at both ends.",
     inputs=[_Num("alpha", "Beta alpha (> 1)"), _Num("beta", "Beta beta (> 1)"), LEVEL_IN],
     outputs=[_Out("lower", "lower limit"), _Out("upper", "upper limit"), _Out("width", "interval width")],
     primary="lower", conditions=["alpha > 1 and beta > 1 (unimodal, zero density at both ends)", "0 < level < 1"],
     method="bisection on the lower limit l with u = F^-1(F(l) + level) until f(l) = f(u)", run=run_beta_hdi,
     reference=ref_beta_hdi,
     latex=r"F(u) - F(l) = c,\quad f(l) = f(u)",
     wrong=lambda alpha, beta, level=None: (lambda r: {"lower": r["lower"], "upper": r["upper"],
                                                        "width": r["upper"] - r["lower"]})(run_beta_ci(alpha, beta, level)),
     rnd=_rhdi, need=[r"highest[- ]density|\bhdi\b|\bhpd\b|shortest|narrowest", r"beta\(|alpha\s*="], avoid=[r"successes|trials|equal[- ]tailed|central"],
     aliases={"alpha": r"alpha", "beta": r"beta"}, extract=_chain(_x_beta_prior, _x_level),
     templates=["95% highest-density interval of a Beta(3, 9) posterior.",
                "Find the 90% HDI for a beta posterior with alpha = 12 and beta = 4."],
     examples=[_X({"alpha": 2, "beta": 2, "level": 0.5}, {}), _X({"alpha": 3, "beta": 9, "level": 0.95}, {})],
     questions=[_Q("95% highest-density interval of a Beta(3, 9) posterior.", {}),
                _Q("Find the 90% HDI for a beta posterior with alpha = 1 and beta = 4.", outcome="INVALID_INPUT"),
                _Q("Find the 90% HDI for a beta posterior with alpha = 12.", outcome="NEEDS_INPUT")])


def run_beta_from_moments(mean, sd):
    m, s = _prob(mean, "the prior mean", open_=True), _pos(sd, "the prior standard deviation")
    if s * s >= m * (1 - m):
        raise ValueError("the variance must be smaller than mean x (1 - mean) for a beta distribution")
    nu = m * (1 - m) / (s * s) - 1
    return {"alpha": m * nu, "beta": (1 - m) * nu, "prior_sample_size": nu}


def ref_beta_from_moments(mean, sd):
    """brentq on the prior sample size so that scipy's Beta(mean*nu, (1-mean)*nu) has the stated variance."""
    from scipy.optimize import brentq
    from scipy.stats import beta as B
    f = lambda nu: B(mean * nu, (1 - mean) * nu).var() - sd * sd      # noqa: E731
    nu = brentq(f, 1e-9, 1e12, xtol=1e-14, rtol=8.9e-16, maxiter=1000)
    return {"alpha": mean * nu, "beta": (1 - mean) * nu, "prior_sample_size": nu}


_cap("beta_prior_from_mean_sd", level="intermediate", name="Beta prior from a mean and standard deviation",
     topic="prior elicitation",
     purpose="The Beta(alpha, beta) prior with a stated mean and standard deviation (method of moments) and its "
             "effective prior sample size alpha + beta.",
     inputs=[_Num("mean", "prior mean, 0 < m < 1"), _Num("sd", "prior standard deviation, sd^2 < m(1 - m)")],
     outputs=[_Out("alpha", "alpha"), _Out("beta", "beta"), _Out("prior_sample_size", "alpha + beta")],
     primary="alpha", conditions=["0 < mean < 1", "0 < sd^2 < mean (1 - mean)"],
     method="match the beta mean and variance", run=run_beta_from_moments, reference=ref_beta_from_moments,
     latex=r"\nu = \frac{m(1-m)}{s^2} - 1,\quad \alpha = m\nu,\quad \beta = (1-m)\nu",
     wrong=lambda mean, sd: (lambda nu: {"alpha": mean * nu, "beta": (1 - mean) * nu, "prior_sample_size": nu})(
         mean * (1 - mean) / (sd * sd)),
     rnd=lambda rng: (lambda m: {"mean": m, "sd": round(math.sqrt(m * (1 - m)) * rng.uniform(0.05, 0.9), 4)})(
         _rp(rng, 0.05, 0.95, 3)),
     need=[r"beta prior|beta distribution", r"\bmean\b", r"standard deviation|\bsd\b"],
     avoid=[r"successes|trials|credible|posterior"],
     aliases={"mean": r"mean", "sd": r"standard deviation|sd"},
     templates=["Find the beta prior with mean = 0.3 and standard deviation = 0.1.",
                "Which beta distribution has mean 0.8 and sd 0.05?"],
     examples=[_X({"mean": 0.3, "sd": 0.1}, {"prior_sample_size": 20.0, "alpha": 6.0, "beta": 14.0}),
               _X({"mean": 0.5, "sd": 0.5 / math.sqrt(3)}, {"alpha": 1.0, "beta": 1.0})],
     questions=[_Q("Find the beta prior with mean = 0.3 and standard deviation = 0.1.", {"alpha": 6.0, "beta": 14.0}),
                _Q("Which beta distribution has mean 0.8 and sd 0.05?", {"prior_sample_size": 63.0}),
                _Q("Find the beta prior with mean = 0.5 and standard deviation = 0.6.", outcome="INVALID_INPUT"),
                _Q("Find the beta prior with mean = 0.5 and an unknown standard deviation.", outcome="NEEDS_INPUT")])


def run_prob_beta_greater(alpha_a, beta_a, alpha_b, beta_b):
    aa, ba, bb = _pos(alpha_a, "alpha_a"), _pos(beta_a, "beta_a"), _pos(beta_b, "beta_b")
    ab = _whole(alpha_b, "alpha_b", 1)
    terms = [math.exp(_lbeta(aa + i, ba + bb) - math.log(bb + i) - _lbeta(1 + i, bb) - _lbeta(aa, ba))
             for i in range(ab)]
    p_b = math.fsum(terms)                       # P(p_B > p_A)
    return {"prob_a_greater": 1.0 - p_b, "prob_b_greater": p_b}


def ref_prob_beta_greater(alpha_a, beta_a, alpha_b, beta_b):
    """Numerical integration of f_A(x) F_B(x) over (0, 1) with scipy."""
    from scipy import integrate
    from scipy.stats import beta as B
    pa = integrate.quad(lambda x: B.pdf(x, alpha_a, beta_a) * B.cdf(x, alpha_b, beta_b), 0, 1, epsabs=0,
                        epsrel=1e-13, limit=500, points=[alpha_a / (alpha_a + beta_a)])[0]
    pb = integrate.quad(lambda x: B.pdf(x, alpha_a, beta_a) * B.sf(x, alpha_b, beta_b), 0, 1, epsabs=0,
                        epsrel=1e-13, limit=500, points=[alpha_a / (alpha_a + beta_a)])[0]
    return {"prob_a_greater": pa, "prob_b_greater": pb}


def _rab(rng):
    na, nb = rng.randint(10, 300), rng.randint(10, 300)
    p = rng.uniform(0.1, 0.6)
    ka = max(0, min(na, int(round(na * p))))
    kb = max(0, min(nb, int(round(nb * (p + rng.uniform(-1.2, 1.2) * math.sqrt(p * (1 - p) * (1 / na + 1 / nb)))))))
    return {"alpha_a": float(ka + 1), "beta_a": float(na - ka + 1), "alpha_b": float(kb + 1), "beta_b": float(nb - kb + 1)}


_cap("ab_test_probability_beta", level="advanced", name="Bayesian A/B test: probability that A beats B",
     topic=_TBB,
     purpose="P(p_A > p_B) for independent Beta posteriors of two success probabilities (alpha_b a whole number, as "
             "after a uniform or any integer-parameter prior).",
     inputs=[_Num("alpha_a", "posterior alpha of A"), _Num("beta_a", "posterior beta of A"),
             _Num("alpha_b", "posterior alpha of B (whole number)"), _Num("beta_b", "posterior beta of B")],
     outputs=[_Out("prob_a_greater", "P(p_A > p_B)"), _Out("prob_b_greater", "P(p_B > p_A)")],
     primary="prob_a_greater", conditions=["all parameters > 0", "alpha_b a whole number"],
     method="exact finite sum of beta functions (Miller's formula)", run=run_prob_beta_greater,
     reference=ref_prob_beta_greater,
     latex=r"P(p_B > p_A) = \sum_{i=0}^{\alpha_B - 1}\frac{B(\alpha_A + i,\ \beta_A + \beta_B)}{(\beta_B + i)\,"
           r"B(1 + i,\ \beta_B)\,B(\alpha_A, \beta_A)}",
     wrong=lambda alpha_a, beta_a, alpha_b, beta_b: (lambda ma, mb, va, vb: (lambda p: {
         "prob_a_greater": p, "prob_b_greater": 1 - p})(_phi((ma - mb) / math.sqrt(va + vb))))(
         alpha_a / (alpha_a + beta_a), alpha_b / (alpha_b + beta_b),
         alpha_a * beta_a / ((alpha_a + beta_a) ** 2 * (alpha_a + beta_a + 1)),
         alpha_b * beta_b / ((alpha_b + beta_b) ** 2 * (alpha_b + beta_b + 1))),
     rnd=_rab, need=[r"alpha_a", r"alpha_b", r"beat|greater|better|exceed|higher"],
     aliases={"alpha_a": r"alpha_a", "beta_a": r"beta_a", "alpha_b": r"alpha_b", "beta_b": r"beta_b"},
     templates=["Bayesian A/B test: alpha_a = 21, beta_a = 81, alpha_b = 31, beta_b = 71. Probability that A beats B?",
                "With alpha_a = 5, beta_a = 3, alpha_b = 4, beta_b = 4, what is the probability that p_A is greater "
                "than p_B?"],
     examples=[_X({"alpha_a": 1, "beta_a": 1, "alpha_b": 1, "beta_b": 1}, {"prob_a_greater": 0.5}),
               _X({"alpha_a": 2, "beta_a": 1, "alpha_b": 1, "beta_b": 1}, {"prob_a_greater": 2 / 3})],
     questions=[_Q("Bayesian A/B test: alpha_a = 1, beta_a = 1, alpha_b = 1, beta_b = 1. Probability that A beats B?",
                   {"prob_a_greater": 0.5}),
                _Q("With alpha_a = 2, beta_a = 1, alpha_b = 1, beta_b = 1, what is the probability that p_A is "
                   "greater than p_B?", {"prob_a_greater": 2 / 3}),
                _Q("Bayesian A/B test: alpha_a = 21, beta_a = 81, alpha_b = 3.5, beta_b = 71. Probability that A "
                   "beats B?", outcome="INVALID_INPUT")])

# ================================================================================== conjugate: gamma family ====
_TGP = "conjugate priors: gamma-Poisson and gamma-exponential"
_G_AL = {"shape": r"shape|alpha", "rate": r"rate|beta", "counts": r"counts|data|observations",
         "times": r"times|waiting times|data|observations|lifetimes", "events": r"events|k"}


def _gp_check(shape, rate, counts):
    a, b = _pos(shape, "the prior shape"), _pos(rate, "the prior rate")
    x = _vec(counts, "the counts")
    if any(v < 0 or v != int(v) for v in x):
        raise ValueError("Poisson counts must be whole numbers >= 0")
    return a, b, x


def run_gp_post(shape, rate, counts):
    a, b, x = _gp_check(shape, rate, counts)
    a2, b2 = a + math.fsum(x), b + len(x)
    return {"post_shape": a2, "post_rate": b2, "posterior_mean": a2 / b2, "posterior_variance": a2 / b2 ** 2,
            "posterior_sd": math.sqrt(a2) / b2}


def _log_gamma_prior_poisson(shape, rate, counts):
    """log gamma prior density x Poisson likelihood, written from their definitions (no conjugate update)."""
    lp0 = shape * math.log(rate) - math.lgamma(shape)
    lf = math.fsum(math.lgamma(c + 1) for c in counts)
    return lambda lam: (lp0 + (shape - 1) * math.log(lam) - rate * lam
                        + math.fsum(c * math.log(lam) - lam for c in counts) - lf)


def ref_gp_post(shape, rate, counts):
    """Numerical integration of gamma prior x Poisson likelihood (quad); parameters from mean and variance."""
    m, v = _ref_moments(_log_gamma_prior_poisson(shape, rate, counts), 0.0, math.inf)
    return {"posterior_mean": m, "posterior_variance": v, "posterior_sd": math.sqrt(v), "post_shape": m * m / v,
            "post_rate": m / v}


def _rgp(rng):
    lam = rng.uniform(0.3, 15)
    return {"shape": round(rng.uniform(1.05, 10), 2), "rate": round(rng.uniform(0.1, 5), 2),
            "counts": [float(min(60, max(0, int(round(rng.gauss(lam, math.sqrt(lam))))))) for _ in range(rng.randint(1, 15))]}


_GP_X = _chain(_x_gamma_prior, _x_level)

_cap("gamma_poisson_posterior", level="intermediate", name="Gamma-Poisson conjugate posterior of a rate", topic=_TGP,
     purpose="Update a Gamma(shape, rate) prior on a Poisson rate with observed counts: posterior parameters, mean, "
             "variance and standard deviation.",
     inputs=[_Num("shape", "prior shape alpha (> 0)"), _Num("rate", "prior rate beta (> 0)"),
             _Vec("counts", "observed counts (whole numbers), one per unit interval")],
     outputs=[_Out("post_shape", "alpha + sum of counts"), _Out("post_rate", "beta + number of intervals"),
              _Out("posterior_mean", "posterior mean of the rate"), _Out("posterior_variance", "posterior variance"),
              _Out("posterior_sd", "posterior standard deviation")],
     primary="posterior_mean", conditions=["shape, rate > 0", "counts whole numbers >= 0"],
     method="conjugate update of the gamma prior with the Poisson likelihood", run=run_gp_post, reference=ref_gp_post,
     latex=r"\lambda\mid x \sim \mathrm{Gamma}\!\left(\alpha + \textstyle\sum x_i,\ \beta + n\right)",
     wrong=lambda shape, rate, counts: (lambda a2, b2: {"post_shape": a2, "post_rate": b2, "posterior_mean": a2 / b2,
                                                        "posterior_variance": a2 / b2 ** 2,
                                                        "posterior_sd": math.sqrt(a2) / b2})(shape + sum(counts), rate),
     rnd=_rgp, need=[r"gamma\(|shape\s*=", r"counts\s*(?:=|:)?\s*\["],
     avoid=[r"interval|credible|hdi|highest|\bmap\b|mode\b|maximum a posteriori|predictive|next|probability of"],
     aliases=_G_AL, extract=_GP_X,
     templates=["Gamma-Poisson posterior: prior Gamma(2, 1), counts = [3, 5, 4]. Posterior mean of the rate?",
                "With shape = 1 and rate = 0.5 and Poisson counts = [0, 2, 1, 1], find the posterior of the rate."],
     examples=[_X({"shape": 2, "rate": 1, "counts": "3, 5, 4"}, {"post_shape": 14.0, "post_rate": 4.0,
                                                                 "posterior_mean": 3.5, "posterior_variance": 14 / 16}),
               _X({"shape": 1, "rate": 0.5, "counts": [0, 2, 1, 1]}, {"posterior_mean": 5 / 4.5})],
     questions=[_Q("Gamma-Poisson posterior: prior Gamma(2, 1), counts = [3, 5, 4]. Posterior mean of the rate?",
                   {"posterior_mean": 3.5, "post_shape": 14.0}),
                _Q("With shape = 1 and rate = 0.5 and Poisson counts = [0, 2, 1, 1], find the posterior of the rate.",
                   {"post_rate": 4.5}),
                _Q("Gamma-Poisson posterior: prior Gamma(2, 1), counts = [3, -5, 4]. Posterior mean?",
                   outcome="INVALID_INPUT"),
                _Q("Gamma-Poisson posterior with shape = 2 and counts = [3, 5].", outcome="NEEDS_INPUT")])


def run_gp_ci(shape, rate, counts, level=None):
    a, b, x = _gp_check(shape, rate, counts)
    c = _level(level)
    a2, b2 = a + math.fsum(x), b + len(x)
    return {"post_shape": a2, "post_rate": b2, "lower": _gamma_ppf(0.5 * (1 - c), a2) / b2,
            "upper": _gamma_ppf(0.5 * (1 + c), a2) / b2}


def ref_gp_ci(shape, rate, counts, level=None):
    c = 0.95 if level is None else level
    logf = _log_gamma_prior_poisson(shape, rate, counts)
    lo, up = _ref_quantile(logf, 0.0, math.inf, [(1 - c) / 2, (1 + c) / 2])
    m, v = _ref_moments(logf, 0.0, math.inf)
    return {"lower": lo, "upper": up, "post_shape": m * m / v, "post_rate": m / v}


_cap("gamma_poisson_credible_interval", level="intermediate", name="Credible interval for a Poisson rate",
     topic="credible intervals",
     purpose="Equal-tailed credible interval of a Poisson rate after observed counts with a Gamma(shape, rate) prior.",
     inputs=[_Num("shape", "prior shape (> 0)"), _Num("rate", "prior rate (> 0)"), _Vec("counts", "observed counts"),
             LEVEL_IN],
     outputs=[_Out("post_shape", "posterior shape"), _Out("post_rate", "posterior rate"),
              _Out("lower", "lower limit"), _Out("upper", "upper limit")],
     primary="lower", conditions=["shape, rate > 0", "counts whole numbers >= 0", "0 < level < 1"],
     method="conjugate update, then gamma quantiles (inverse regularized incomplete gamma)", run=run_gp_ci,
     reference=ref_gp_ci,
     latex=r"\left[\frac{G^{-1}_{\alpha'}\!\left(\frac{1-c}{2}\right)}{\beta'},\ \frac{G^{-1}_{\alpha'}\!\left(\frac{1+c}{2}\right)}{\beta'}\right]",
     wrong=lambda shape, rate, counts, level=None: (lambda r: dict(r, lower=_gamma_ppf(0.5 * (1 - (level or 0.95)),
                                                                                       r["post_shape"]) / rate,
                                                                    upper=_gamma_ppf(0.5 * (1 + (level or 0.95)),
                                                                                     r["post_shape"]) / rate))(
         run_gp_ci(shape, rate, counts, level)),
     rnd=lambda rng: dict(_rgp(rng), level=_rlevel(rng)),
     need=[r"credible|probability interval", r"counts\s*(?:=|:)?\s*\["], avoid=[r"highest|hdi|hpd|shortest|narrowest|dirichlet"],
     aliases=_G_AL, extract=_GP_X,
     templates=["95% credible interval for a Poisson rate: prior Gamma(2, 1), counts = [3, 5, 4].",
                "Find the 90% credible interval of the rate with shape = 0.5, rate = 0.001 and counts = [7]."],
     examples=[_X({"shape": 1, "rate": 1, "counts": "0", "level": 0.9}, {"lower": -math.log(0.95) / 2,
                                                                          "upper": -math.log(0.05) / 2}),
               _X({"shape": 2, "rate": 1, "counts": [3, 5, 4]}, {"post_shape": 14.0})],
     questions=[_Q("90% credible interval for a Poisson rate: prior Gamma(1, 1), counts = [0].",
                   {"lower": -math.log(0.95) / 2, "upper": -math.log(0.05) / 2}),
                _Q("Find the 90% credible interval of the rate with shape = 0.5, rate = 0.001 and counts = [7].",
                   {"post_shape": 7.5}),
                _Q("95% credible interval for a Poisson rate: prior Gamma(2, 0), counts = [3].", outcome="INVALID_INPUT")])


def run_gamma_ci(shape, rate, level=None):
    a, b, c = _pos(shape, "the shape"), _pos(rate, "the rate"), _level(level)
    return {"lower": _gamma_ppf(0.5 * (1 - c), a) / b, "upper": _gamma_ppf(0.5 * (1 + c), a) / b,
            "median": _gamma_ppf(0.5, a) / b}


_cap("gamma_credible_interval", level="intermediate", name="Equal-tailed credible interval of a gamma distribution",
     topic="credible intervals",
     purpose="The central credible interval and median of a Gamma(shape, rate) posterior.",
     inputs=[_Num("shape", "shape (> 0)"), _Num("rate", "rate (> 0)"), LEVEL_IN],
     outputs=[_Out("lower", "lower limit"), _Out("upper", "upper limit"), _Out("median", "median")],
     primary="lower", conditions=["shape, rate > 0", "0 < level < 1"],
     method="gamma quantiles by inversion of the regularized incomplete gamma function", run=run_gamma_ci,
     reference=lambda shape, rate, level=None: (lambda G, c: {
         "lower": float(G.ppf((1 - c) / 2, shape, scale=1 / rate)), "upper": float(G.isf((1 - c) / 2, shape, scale=1 / rate)),
         "median": float(G.median(shape, scale=1 / rate))})(__import__("scipy.stats", fromlist=["gamma"]).gamma,
                                                             0.95 if level is None else level),
     latex=r"\left[G^{-1}\!\left(\tfrac{1-c}{2}\right),\ G^{-1}\!\left(\tfrac{1+c}{2}\right)\right]",
     wrong=lambda shape, rate, level=None: (lambda r: dict(r, lower=r["lower"] * rate * rate, upper=r["upper"] * rate * rate))(
         run_gamma_ci(shape, rate, level)) if abs(rate - 1) > 0.05 else (lambda r: dict(
         r, lower=r["lower"] * 2, upper=r["upper"] * 2))(run_gamma_ci(shape, rate, level)),
     rnd=lambda rng: {"shape": round(rng.uniform(0.3, 60), 2), "rate": round(rng.uniform(0.05, 10), 3),
                      "level": _rlevel(rng)},
     need=[r"credible|probability interval", r"gamma\(|gamma distribution|gamma posterior"],
     avoid=[r"counts|kappa0|inverse-gamma|highest|hdi|hpd|shortest|narrowest"], aliases={"shape": r"shape|alpha", "rate": r"rate|beta"}, extract=_GP_X,
     templates=["95% credible interval of a Gamma(5, 2) posterior.",
                "Find the 90% credible interval of a gamma distribution with shape = 3 and rate = 0.5."],
     examples=[_X({"shape": 1, "rate": 1, "level": 0.9}, {"lower": -math.log(0.95), "upper": -math.log(0.05),
                                                          "median": math.log(2)}),
               _X({"shape": 5, "rate": 2}, {})],
     questions=[_Q("90% credible interval of a Gamma(1, 2) posterior.", {"lower": -math.log(0.95) / 2,
                                                                      "median": math.log(2) / 2}),
                _Q("Find the 90% credible interval of a gamma distribution with shape = 3 and rate = 0.5.", {}),
                _Q("95% credible interval of a Gamma(5, -2) posterior.", outcome="INVALID_INPUT")])


def run_gamma_hdi(shape, rate, level=None):
    k, b, c = _pos(shape, "the shape"), _pos(rate, "the rate"), _level(level)
    if k <= 1:
        raise ValueError("the highest-density interval is computed for a unimodal gamma with shape > 1")

    def upper_of(l):
        p = _igamma(k, l)[0] + c
        return math.inf if p >= 1.0 else _gamma_ppf(p, k)

    def g(l):
        u = upper_of(l)
        return (_gamma_logpdf(u, k) if math.isfinite(u) else -math.inf) - _gamma_logpdf(l, k)
    lo, hi = 0.0, _gamma_ppf(1 - c, k)
    for _ in range(300):
        mid = 0.5 * (lo + hi)
        if mid in (lo, hi):
            break
        if g(mid) > 0:
            lo = mid
        else:
            hi = mid
    l = 0.5 * (lo + hi)
    u = upper_of(l)
    return {"lower": l / b, "upper": u / b, "width": (u - l) / b}


def ref_gamma_hdi(shape, rate, level=None):
    from scipy.optimize import brentq
    from scipy.stats import gamma as G
    c = 0.95 if level is None else level
    f = lambda l: G.logpdf(G.ppf(G.cdf(l, shape) + c, shape), shape) - G.logpdf(l, shape)   # noqa: E731
    top = G.ppf(1 - c, shape)
    l = brentq(f, top * 1e-9, top * (1 - 1e-12), xtol=1e-17, rtol=8.9e-16, maxiter=500)
    u = G.ppf(G.cdf(l, shape) + c, shape)
    return {"lower": l / rate, "upper": float(u) / rate, "width": float(u - l) / rate}


_cap("gamma_hdi", level="expert", name="Highest-density interval of a gamma posterior", topic="credible intervals",
     purpose="The shortest interval holding a stated probability of a unimodal Gamma(shape, rate): equal density at "
             "both ends.",
     inputs=[_Num("shape", "shape (> 1)"), _Num("rate", "rate (> 0)"), LEVEL_IN],
     outputs=[_Out("lower", "lower limit"), _Out("upper", "upper limit"), _Out("width", "interval width")],
     primary="lower", conditions=["shape > 1", "rate > 0", "0 < level < 1"],
     method="bisection on the lower limit with u = G^-1(G(l) + level) until the densities are equal", run=run_gamma_hdi,
     reference=ref_gamma_hdi, latex=r"G(u) - G(l) = c,\quad g(l) = g(u)",
     wrong=lambda shape, rate, level=None: (lambda r: {"lower": r["lower"], "upper": r["upper"],
                                                        "width": r["upper"] - r["lower"]})(run_gamma_ci(shape, rate, level)),
     rnd=lambda rng: {"shape": round(rng.uniform(1.3, 40), 2), "rate": round(rng.uniform(0.05, 10), 3),
                      "level": _rlevel(rng)},
     need=[r"highest[- ]density|\bhdi\b|\bhpd\b|shortest|narrowest", r"gamma\(|gamma distribution|gamma posterior|shape\s*="],
     avoid=[r"counts|kappa0|inverse-gamma|equal[- ]tailed|central"],
     aliases={"shape": r"shape|alpha", "rate": r"rate|beta"}, extract=_GP_X,
     templates=["95% highest-density interval of a Gamma(5, 2) posterior.",
                "Find the 90% HDI of a gamma posterior with shape = 3 and rate = 0.5."],
     examples=[_X({"shape": 5, "rate": 2, "level": 0.95}, {}), _X({"shape": 2, "rate": 1, "level": 0.5}, {})],
     questions=[_Q("95% highest-density interval of a Gamma(5, 2) posterior.", {}),
                _Q("Find the 90% HDI of a gamma posterior with shape = 0.5 and rate = 0.5.", outcome="INVALID_INPUT")])


def run_gp_pred(shape, rate, events):
    a, b = _pos(shape, "the shape"), _pos(rate, "the rate")
    k = _whole(events, "the number of events")
    lp = lambda i: (math.lgamma(a + i) - math.lgamma(a) - math.lgamma(i + 1) + a * math.log(b / (b + 1))   # noqa: E731
                    - i * math.log(b + 1))
    at_most = math.fsum(math.exp(lp(i)) for i in range(k + 1))
    return {"probability": math.exp(lp(k)), "prob_at_most": min(1.0, at_most), "predictive_mean": a / b,
            "predictive_variance": a / b * (1 + 1 / b)}


def ref_gp_pred(shape, rate, events):
    from scipy.stats import nbinom
    d = nbinom(shape, rate / (rate + 1))
    return {"probability": float(d.pmf(events)), "prob_at_most": float(d.cdf(events)), "predictive_mean": float(d.mean()),
            "predictive_variance": float(d.var())}


def _rgppred(rng):
    a, b = round(rng.uniform(1, 40), 2), round(rng.uniform(0.2, 8), 2)
    m = a / b
    return {"shape": a, "rate": b, "events": max(0, int(round(m + rng.uniform(-1, 1) * math.sqrt(m * (1 + 1 / b)))))}


_cap("gamma_poisson_predictive", level="advanced", name="Posterior predictive count (negative binomial)", topic=_TGP,
     purpose="Probability of exactly k events in the next unit interval when the Poisson rate has a Gamma(shape, "
             "rate) posterior; the cumulative probability and the predictive mean and variance.",
     inputs=[_Num("shape", "posterior shape (> 0)"), _Num("rate", "posterior rate (> 0)"),
             _Int("events", "number of events k in the next interval")],
     outputs=[_Out("probability", "P(exactly k events)"), _Out("prob_at_most", "P(at most k events)"),
              _Out("predictive_mean", "shape/rate"), _Out("predictive_variance", "shape/rate (1 + 1/rate)")],
     primary="probability", conditions=["shape, rate > 0", "k whole number >= 0"],
     method="negative binomial distribution (the Poisson averaged over the gamma posterior)", run=run_gp_pred,
     reference=ref_gp_pred,
     latex=r"P(X = k) = \frac{\Gamma(\alpha + k)}{\Gamma(\alpha)\,k!}\left(\frac{\beta}{\beta+1}\right)^{\alpha}"
           r"\left(\frac{1}{\beta+1}\right)^{k}",
     wrong=lambda shape, rate, events: dict(run_gp_pred(shape, rate, events), probability=math.exp(
         -shape / rate + events * math.log(shape / rate) - math.lgamma(events + 1))),
     rnd=_rgppred, need=[r"predictive|next (?:unit )?interval|next period|next day|next hour", r"gamma\(|shape\s*="],
     avoid=[r"credible|interval:|hdi"], aliases=_G_AL,
     extract=_chain(_x_gamma_prior, lambda q, d: (lambda mm: d.setdefault("events", int(mm.group(1))) if mm else None)(
         re.search(r"exactly (\d+) events", q))),
     templates=["Posterior Gamma(12, 4) for a Poisson rate: probability of exactly 2 events in the next interval "
                "(posterior predictive)?",
                "Posterior predictive with shape = 3, rate = 1, events = 0."],
     examples=[_X({"shape": 1, "rate": 1, "events": 0}, {"probability": 0.5, "prob_at_most": 0.5}),
               _X({"shape": 12, "rate": 4, "events": 2}, {"predictive_mean": 3.0})],
     questions=[_Q("Posterior Gamma(1, 1) for a Poisson rate: probability of exactly 1 events in the next interval "
                   "(posterior predictive)?", {"probability": 0.25, "prob_at_most": 0.75}),
                _Q("Posterior predictive with shape = 3, rate = 1, events = 0.", {"probability": 0.125}),
                _Q("Posterior predictive with shape = 3, rate = 1, events = 1.5.", outcome="INVALID_INPUT")])


def run_map_gp(shape, rate, counts):
    a, b, x = _gp_check(shape, rate, counts)
    a2, b2 = a + math.fsum(x), b + len(x)
    if a2 <= 1:
        raise ValueError("the posterior mode is at 0 (shape + sum of counts <= 1): no interior MAP")
    return {"map": (a2 - 1) / b2, "mle": _mean(x), "posterior_mean": a2 / b2}


def ref_map_gp(shape, rate, counts):
    """Exact symbolic derivation (sympy): zero of the derivative of log prior + log Poisson likelihood."""
    import sympy as sp
    from fractions import Fraction
    lam = sp.Symbol("lam", positive=True)
    R = lambda v: sp.Rational(Fraction(v).limit_denominator(10 ** 9))      # noqa: E731
    a, b = R(shape), R(rate)
    ll = sum(sp.Integer(int(c)) * sp.log(lam) - lam for c in counts)
    lp = (a - 1) * sp.log(lam) - b * lam + ll
    out = {"map": float(sp.solve(sp.diff(lp, lam), lam)[0])}
    if sum(counts) > 0:
        out["mle"] = float(sp.solve(sp.diff(ll, lam), lam)[0])
    return out


_cap("map_gamma_poisson", level="intermediate", name="MAP estimate of a Poisson rate (gamma prior)", topic=_TGP,
     purpose="Posterior mode of a Poisson rate with a Gamma(shape, rate) prior and observed counts, with the MLE and "
             "the posterior mean for comparison.",
     inputs=[_Num("shape", "prior shape (> 0)"), _Num("rate", "prior rate (> 0)"), _Vec("counts", "observed counts")],
     outputs=[_Out("map", "(shape + sum - 1)/(rate + n)"), _Out("mle", "sample mean of the counts"),
              _Out("posterior_mean", "(shape + sum)/(rate + n)")],
     primary="map", conditions=["shape + sum of counts > 1"], method="stationary point of the log posterior",
     run=run_map_gp, reference=ref_map_gp, latex=r"\hat\lambda_{MAP} = \frac{\alpha + \sum x_i - 1}{\beta + n}",
     wrong=lambda shape, rate, counts: dict(run_map_gp(shape, rate, counts),
                                            map=(shape + sum(counts)) / (rate + len(counts))),
     rnd=_rgp, need=[r"\bmap\b|maximum a posteriori|posterior mode", r"counts\s*(?:=|:)?\s*\["],
     aliases=_G_AL, extract=_GP_X,
     templates=["MAP estimate of the Poisson rate with prior Gamma(2, 1) and counts = [3, 5, 4].",
                "Posterior mode of the rate with shape = 1, rate = 0.5, counts = [0, 2, 1, 1]."],
     examples=[_X({"shape": 2, "rate": 1, "counts": "3, 5, 4"}, {"map": 13 / 4, "mle": 4.0}),
               _X({"shape": 1, "rate": 0.5, "counts": [0, 2, 1, 1]}, {"map": 4 / 4.5})],
     questions=[_Q("MAP estimate of the Poisson rate with prior Gamma(2, 1) and counts = [3, 5, 4].", {"map": 3.25}),
                _Q("Posterior mode of the rate with shape = 1, rate = 0.5, counts = [0, 2, 1, 1].", {"map": 4 / 4.5}),
                _Q("MAP estimate of the Poisson rate with prior Gamma(0.5, 1) and counts = [0, 0].",
                   outcome="INVALID_INPUT")])


def run_gexp_post(shape, rate, times):
    a, b = _pos(shape, "the prior shape"), _pos(rate, "the prior rate")
    t = _vec(times, "the observed times")
    if any(v <= 0 for v in t):
        raise ValueError("exponential observations must be positive")
    a2, b2 = a + len(t), b + math.fsum(t)
    return {"post_shape": a2, "post_rate": b2, "posterior_mean": a2 / b2, "posterior_variance": a2 / b2 ** 2,
            "posterior_sd": math.sqrt(a2) / b2}


def ref_gexp_post(shape, rate, times):
    """Numerical integration of gamma prior x exponential likelihood (quad); parameters from mean and variance."""
    lp0 = shape * math.log(rate) - math.lgamma(shape)
    logf = lambda lam: lp0 + (shape - 1) * math.log(lam) - rate * lam + math.fsum(math.log(lam) - lam * t for t in times)  # noqa: E731
    m, v = _ref_moments(logf, 0.0, math.inf)
    return {"posterior_mean": m, "posterior_variance": v, "posterior_sd": math.sqrt(v), "post_shape": m * m / v,
            "post_rate": m / v}


_cap("gamma_exponential_posterior", level="intermediate", name="Gamma-exponential conjugate posterior of a rate",
     topic=_TGP,
     purpose="Update a Gamma(shape, rate) prior on the rate of exponential waiting times with observed times.",
     inputs=[_Num("shape", "prior shape (> 0)"), _Num("rate", "prior rate (> 0)"),
             _Vec("times", "observed waiting times (> 0)")],
     outputs=[_Out("post_shape", "shape + n"), _Out("post_rate", "rate + sum of times"),
              _Out("posterior_mean", "posterior mean of the rate"), _Out("posterior_variance", "posterior variance"),
              _Out("posterior_sd", "posterior standard deviation")],
     primary="posterior_mean", conditions=["shape, rate > 0", "times > 0"],
     method="conjugate update of the gamma prior with the exponential likelihood", run=run_gexp_post,
     reference=ref_gexp_post,
     latex=r"\lambda\mid t \sim \mathrm{Gamma}\!\left(\alpha + n,\ \beta + \textstyle\sum t_i\right)",
     wrong=lambda shape, rate, times: (lambda a2, b2: {"post_shape": a2, "post_rate": b2, "posterior_mean": a2 / b2,
                                                       "posterior_variance": a2 / b2 ** 2,
                                                       "posterior_sd": math.sqrt(a2) / b2})(
         shape + sum(times), rate + len(times)),
     rnd=lambda rng: (lambda lam: {"shape": round(rng.uniform(1.05, 8), 2), "rate": round(rng.uniform(0.2, 10), 2),
                                   "times": [round(rng.expovariate(lam), 3) + 0.001 for _ in range(rng.randint(1, 15))]})(
         rng.uniform(0.2, 3)),
     need=[r"exponential", r"gamma\(|shape\s*=", r"times\s*(?:=|:)?\s*\["],
     avoid=[r"interval|credible|hdi|map\b|predictive"], aliases=_G_AL, extract=_GP_X,
     templates=["Gamma-exponential posterior: prior Gamma(2, 1), exponential waiting times = [0.5, 1.2, 0.8]. "
                "Posterior mean of the rate?",
                "Exponential lifetimes times = [3, 5] with prior shape = 1 and rate = 2: posterior of the rate?"],
     examples=[_X({"shape": 2, "rate": 1, "times": "0.5, 1.2, 0.8"}, {"post_shape": 5.0, "post_rate": 3.5,
                                                                      "posterior_mean": 5 / 3.5}),
               _X({"shape": 1, "rate": 2, "times": [3, 5]}, {"posterior_mean": 0.3})],
     questions=[_Q("Gamma-exponential posterior: prior Gamma(2, 1), exponential waiting times = [0.5, 1.2, 0.8]. "
                   "Posterior mean of the rate?", {"posterior_mean": 5 / 3.5}),
                _Q("Exponential lifetimes times = [3, 5] with prior shape = 1 and rate = 2: posterior of the rate?",
                   {"post_shape": 3.0, "post_rate": 10.0}),
                _Q("Exponential lifetimes times = [3, -5] with prior shape = 1 and rate = 2: posterior of the rate?",
                   outcome="INVALID_INPUT")])


# ================================================================================= conjugate: normal models ====
_TNN = "conjugate priors: normal models"
_NN_AL = {"prior_mean": r"prior mean|mu0", "prior_sd": r"prior (?:standard deviation|sd)|tau0|tau",
          "sigma": r"known (?:standard deviation|sd|sigma)|measurement (?:standard deviation|sd)|data (?:standard "
                   r"deviation|sd)|sigma",
          "data": r"data|observations|measurements", "sample_mean": r"sample mean|xbar", "n": r"sample size|n",
          "threshold": r"threshold|c", "mu0": r"mu0", "kappa0": r"kappa0", "alpha0": r"alpha0", "beta0": r"beta0"}


def _nn_check(prior_mean, prior_sd, sigma):
    return _fin(prior_mean, "the prior mean"), _pos(prior_sd, "the prior standard deviation"), _pos(sigma, "sigma")


def _nn_post(m0, t0, s, xbar, n):
    prec = 1 / t0 ** 2 + n / s ** 2
    v = 1 / prec
    return v * (m0 / t0 ** 2 + n * xbar / s ** 2), v


def run_nn(prior_mean, prior_sd, sigma, data):
    m0, t0, s = _nn_check(prior_mean, prior_sd, sigma)
    x = _vec(data, "the data")
    m, v = _nn_post(m0, t0, s, _mean(x), len(x))
    return {"posterior_mean": m, "posterior_sd": math.sqrt(v), "posterior_variance": v}


def ref_nn(prior_mean, prior_sd, sigma, data):
    """Gaussian conditioning on the joint covariance of (theta, x_1, ..., x_n) with numpy.linalg."""
    import numpy as np
    n = len(data)
    S = prior_sd ** 2 * np.ones((n + 1, n + 1))
    S[1:, 1:] += sigma ** 2 * np.eye(n)
    w = np.linalg.solve(S[1:, 1:], S[0, 1:])
    m = prior_mean + w @ (np.array(data, float) - prior_mean)
    v = prior_sd ** 2 - S[0, 1:] @ w
    return {"posterior_mean": float(m), "posterior_variance": float(v), "posterior_sd": float(math.sqrt(v))}


def _rnn(rng):
    s = round(rng.uniform(0.5, 10), 2)
    m0 = round(rng.uniform(-50, 50), 2)
    t0 = round(s * rng.uniform(0.1, 4), 2)
    mu = m0 + rng.gauss(0, t0)
    return {"prior_mean": m0, "prior_sd": t0, "sigma": s, "data": [round(rng.gauss(mu, s), 2) for _ in range(rng.randint(1, 25))]}


_cap("normal_normal_posterior", level="intermediate", name="Normal mean with known variance: posterior from data",
     topic=_TNN,
     purpose="Posterior of a normal mean theta with a N(prior mean, prior sd^2) prior, given observations with known "
             "standard deviation sigma.",
     inputs=[_Num("prior_mean", "prior mean mu0"), _Num("prior_sd", "prior standard deviation tau0 (> 0)"),
             _Num("sigma", "known standard deviation of each observation (> 0)"), _Vec("data", "the observations")],
     outputs=[_Out("posterior_mean", "posterior mean"), _Out("posterior_sd", "posterior standard deviation"),
              _Out("posterior_variance", "posterior variance")],
     primary="posterior_mean", conditions=["prior sd > 0", "sigma > 0 known"],
     method="precision-weighted average of the prior mean and the sample mean", run=run_nn, reference=ref_nn,
     latex=r"\frac{1}{\tau_n^2} = \frac{1}{\tau_0^2} + \frac{n}{\sigma^2},\quad \mu_n = \tau_n^2\left(\frac{\mu_0}{\tau_0^2} + \frac{n\bar x}{\sigma^2}\right)",
     wrong=lambda prior_mean, prior_sd, sigma, data: (lambda m, v: {"posterior_mean": m, "posterior_sd": math.sqrt(v),
                                                                     "posterior_variance": v})(
         *_nn_post(prior_mean, prior_sd, sigma, sum(data) / len(data), 1)) if len(data) > 1 else
     (lambda m, v: {"posterior_mean": m, "posterior_sd": math.sqrt(v), "posterior_variance": v})(
         *_nn_post(prior_mean, prior_sd, sigma, data[0], 2)),
     rnd=_rnn, need=[r"prior mean|mu0", r"data\s*(?:=|:)?\s*\[|observations\s*(?:=|:)?\s*\[|measurements\s*(?:=|:)?\s*\["],
     avoid=[r"kappa0|alpha0|beta0|predictive|bayes factor|dirichlet"], aliases=_NN_AL,
     templates=["Normal-normal posterior: prior mean = 10, prior sd = 2, known sigma = 3, data = [12, 9, 14]. "
                "Posterior mean?",
                "Known-variance normal model with prior mean 0, prior standard deviation 1, sigma = 1 and data = [1.2]: "
                "find the posterior."],
     examples=[_X({"prior_mean": 0, "prior_sd": 1, "sigma": 1, "data": "1.2"}, {"posterior_mean": 0.6,
                                                                                "posterior_variance": 0.5}),
               _X({"prior_mean": 10, "prior_sd": 2, "sigma": 3, "data": [12, 9, 14]},
                  {"posterior_mean": (10 / 4 + 3 * (35 / 3) / 9) / (1 / 4 + 3 / 9)})],
     questions=[_Q("Normal-normal posterior: prior mean = 10, prior sd = 2, known sigma = 3, data = [12, 9, 14]. "
                   "Posterior mean?", {"posterior_mean": (10 / 4 + 3 * (35 / 3) / 9) / (1 / 4 + 3 / 9)}),
                _Q("Known-variance normal model with prior mean 0, prior standard deviation 1, sigma = 1 and data = "
                   "[1.2]: find the posterior.", {"posterior_mean": 0.6, "posterior_variance": 0.5}),
                _Q("Normal-normal posterior: prior mean = 10, prior sd = -2, known sigma = 3, data = [12]. Posterior?",
                   outcome="INVALID_INPUT"),
                _Q("Normal-normal posterior: prior mean = 10, known sigma = 3, data = [12]. Posterior given the prior sd?",
                   outcome="NEEDS_INPUT")])


def run_nn_summary(prior_mean, prior_sd, sigma, sample_mean, n, level=None):
    m0, t0, s = _nn_check(prior_mean, prior_sd, sigma)
    xb, k, c = _fin(sample_mean, "the sample mean"), _whole(n, "the sample size", 1), _level(level)
    m, v = _nn_post(m0, t0, s, xb, k)
    z = _z(0.5 * (1 + c))
    return {"posterior_mean": m, "posterior_sd": math.sqrt(v), "lower": m - z * math.sqrt(v),
            "upper": m + z * math.sqrt(v), "shrinkage": (s * s / k) / (s * s / k + t0 * t0)}


def ref_nn_summary(prior_mean, prior_sd, sigma, sample_mean, n, level=None):
    """Conditioning of the bivariate normal (theta, xbar) with numpy, scipy normal quantiles; the shrinkage factor
    as the slope of the posterior mean in the prior mean (finite difference of the conditioning)."""
    import numpy as np
    from scipy.stats import norm
    S = np.array([[prior_sd ** 2, prior_sd ** 2], [prior_sd ** 2, prior_sd ** 2 + sigma ** 2 / n]])
    def cond(m0):
        return m0 + S[0, 1] / S[1, 1] * (sample_mean - m0)
    m, v = cond(prior_mean), S[0, 0] - S[0, 1] ** 2 / S[1, 1]
    c = 0.95 if level is None else level
    lo, up = norm.interval(c, loc=m, scale=math.sqrt(v))
    return {"posterior_mean": float(m), "posterior_sd": float(math.sqrt(v)), "lower": float(lo), "upper": float(up),
            "shrinkage": float(cond(1.0) - cond(0.0))}


def _rnns(rng):
    x = _rnn(rng)
    return {"prior_mean": x["prior_mean"], "prior_sd": x["prior_sd"], "sigma": x["sigma"],
            "sample_mean": round(x["prior_mean"] + rng.gauss(0, x["prior_sd"] + x["sigma"]), 2),
            "n": rng.randint(1, 200), "level": _rlevel(rng)}


_cap("normal_normal_summary", level="intermediate",
     name="Normal mean with known variance: posterior and credible interval from a sample mean", topic=_TNN,
     purpose="Posterior mean, standard deviation, credible interval and shrinkage factor of a normal mean from the "
             "sample mean of n observations with known sigma and a normal prior.",
     inputs=[_Num("prior_mean", "prior mean"), _Num("prior_sd", "prior standard deviation (> 0)"),
             _Num("sigma", "known standard deviation of one observation (> 0)"), _Num("sample_mean", "sample mean"),
             _Int("n", "sample size (>= 1)", lo=1), LEVEL_IN],
     outputs=[_Out("posterior_mean", "posterior mean"), _Out("posterior_sd", "posterior standard deviation"),
              _Out("lower", "lower credible limit"), _Out("upper", "upper credible limit"),
              _Out("shrinkage", "weight on the prior mean (sigma^2/n)/(sigma^2/n + tau0^2)")],
     primary="posterior_mean", conditions=["prior sd > 0", "sigma > 0", "n >= 1", "0 < level < 1"],
     method="normal-normal conjugate update, interval from normal quantiles", run=run_nn_summary,
     reference=ref_nn_summary,
     latex=r"\mu_n = B\mu_0 + (1-B)\bar x,\ B = \frac{\sigma^2/n}{\sigma^2/n + \tau_0^2},\quad \mu_n \pm z_{(1+c)/2}\,\tau_n",
     wrong=lambda prior_mean, prior_sd, sigma, sample_mean, n, level=None: dict(
         run_nn_summary(prior_mean, prior_sd, sigma, sample_mean, n, level),
         posterior_mean=(prior_mean + sample_mean) / 2 if abs(prior_mean - sample_mean) > 1e-6 else prior_mean + 1),
     rnd=_rnns, need=[r"prior mean|mu0", r"sample mean|xbar"], avoid=[r"predictive|bayes factor|threshold|exceed"],
     aliases=_NN_AL, extract=_x_level,
     templates=["Normal prior with prior mean = 100, prior sd = 15; sample mean = 110 from n = 25 with known sigma = 20. "
                "Posterior mean and 95% credible interval?",
                "Prior mean 0, prior standard deviation 2, known sigma = 4, sample mean 1.5, sample size 16: posterior?"],
     examples=[_X({"prior_mean": 0, "prior_sd": 1, "sigma": 2, "sample_mean": 1, "n": 4},
                  {"posterior_mean": 0.5, "posterior_sd": math.sqrt(0.5), "shrinkage": 0.5}),
               _X({"prior_mean": 100, "prior_sd": 15, "sigma": 20, "sample_mean": 110, "n": 25, "level": 0.95}, {})],
     questions=[_Q("Normal prior with prior mean = 0, prior sd = 1; sample mean = 1 from n = 4 with known sigma = 2. "
                   "Posterior mean and 95% credible interval?", {"posterior_mean": 0.5, "shrinkage": 0.5}),
                _Q("Prior mean 0, prior standard deviation 2, known sigma = 4, sample mean 1.5, sample size 16: posterior?",
                   {"posterior_mean": 1.5 * 4 / 5}),
                _Q("Prior mean 0, prior standard deviation 2, known sigma = 4, sample mean 1.5, sample size 0: posterior?",
                   outcome="INVALID_INPUT")])


def run_nn_pred(prior_mean, prior_sd, sigma, sample_mean, n, threshold):
    m0, t0, s = _nn_check(prior_mean, prior_sd, sigma)
    xb, k, c = _fin(sample_mean, "the sample mean"), _whole(n, "the sample size", 1), _fin(threshold, "the threshold")
    m, v = _nn_post(m0, t0, s, xb, k)
    sd = math.sqrt(v + s * s)
    z = (c - m) / sd
    return {"predictive_mean": m, "predictive_sd": sd, "prob_above": 0.5 * math.erfc(z / _SQRT2),
            "prob_below": 0.5 * math.erfc(-z / _SQRT2)}


def ref_nn_pred(prior_mean, prior_sd, sigma, sample_mean, n, threshold):
    """Conditioning of the trivariate normal (theta, xbar, y_new) with numpy; scipy normal tails."""
    import numpy as np
    from scipy.stats import norm
    t2, s2 = prior_sd ** 2, sigma ** 2
    S = np.array([[t2, t2, t2], [t2, t2 + s2 / n, t2], [t2, t2, t2 + s2]])
    m = prior_mean + S[2, 1] / S[1, 1] * (sample_mean - prior_mean)
    v = S[2, 2] - S[2, 1] ** 2 / S[1, 1]
    return {"predictive_mean": float(m), "predictive_sd": float(math.sqrt(v)),
            "prob_above": float(norm.sf(threshold, m, math.sqrt(v))), "prob_below": float(norm.cdf(threshold, m, math.sqrt(v)))}


def _rnnp(rng):
    x = _rnns(rng)
    x.pop("level")
    m, v = _nn_post(x["prior_mean"], x["prior_sd"], x["sigma"], x["sample_mean"], x["n"])
    x["threshold"] = round(m + rng.uniform(-1.5, 1.5) * math.sqrt(v + x["sigma"] ** 2), 2)
    return x


_cap("normal_posterior_predictive", level="advanced", name="Posterior predictive distribution of a new observation",
     topic=_TNN,
     purpose="Mean, standard deviation and tail probabilities of the next observation in the normal model with known "
             "sigma, after a sample mean of n observations and a normal prior.",
     inputs=[_Num("prior_mean", "prior mean"), _Num("prior_sd", "prior standard deviation (> 0)"),
             _Num("sigma", "known standard deviation (> 0)"), _Num("sample_mean", "sample mean"),
             _Int("n", "sample size (>= 1)", lo=1), _Num("threshold", "value c for P(Y > c)")],
     outputs=[_Out("predictive_mean", "posterior mean"), _Out("predictive_sd", "sqrt(posterior variance + sigma^2)"),
              _Out("prob_above", "P(Y_new > c)"), _Out("prob_below", "P(Y_new < c)")],
     primary="predictive_sd", conditions=["prior sd > 0", "sigma > 0", "n >= 1"],
     method="predictive variance = posterior variance + sampling variance", run=run_nn_pred, reference=ref_nn_pred,
     latex=r"Y_{new}\mid \bar x \sim N\!\left(\mu_n,\ \tau_n^2 + \sigma^2\right)",
     wrong=lambda prior_mean, prior_sd, sigma, sample_mean, n, threshold: (lambda m, v: (lambda sd: {
         "predictive_mean": m, "predictive_sd": sd, "prob_above": 0.5 * math.erfc((threshold - m) / sd / _SQRT2),
         "prob_below": 0.5 * math.erfc(-(threshold - m) / sd / _SQRT2)})(math.sqrt(v)))(
         *_nn_post(prior_mean, prior_sd, sigma, sample_mean, n)),
     rnd=_rnnp, need=[r"predictive|new observation|next observation", r"prior mean|mu0", r"sample mean|xbar"],
     aliases=_NN_AL,
     extract=lambda q, d: d.setdefault("threshold", _thr(q)) if _thr(q) is not None and "threshold" not in d else None,
     templates=["Posterior predictive: prior mean = 0, prior sd = 1, known sigma = 2, sample mean = 1, n = 4. "
                "Probability that the next observation exceeds 2?",
                "Predictive distribution of a new observation with prior mean 10, prior sd 3, sigma = 2, sample mean 12, "
                "sample size 5 and threshold = 15."],
     examples=[_X({"prior_mean": 0, "prior_sd": 1, "sigma": 2, "sample_mean": 1, "n": 4, "threshold": 0.5},
                  {"predictive_mean": 0.5, "predictive_sd": math.sqrt(4.5), "prob_above": 0.5}),
               _X({"prior_mean": 10, "prior_sd": 3, "sigma": 2, "sample_mean": 12, "n": 5, "threshold": 15}, {})],
     questions=[_Q("Posterior predictive: prior mean = 0, prior sd = 1, known sigma = 2, sample mean = 1, n = 4. "
                   "Probability that the next observation exceeds 0.5?", {"prob_above": 0.5,
                                                                         "predictive_sd": math.sqrt(4.5)}),
                _Q("Predictive distribution of a new observation with prior mean 10, prior sd 3, sigma = 0, sample "
                   "mean 12, sample size 5 and threshold = 15.", outcome="INVALID_INPUT")])


def run_nig(mu0, kappa0, alpha0, beta0, data, level=None):
    m0, k0, a0, b0 = _fin(mu0, "mu0"), _pos(kappa0, "kappa0"), _pos(alpha0, "alpha0"), _pos(beta0, "beta0")
    x, c = _vec(data, "the data"), _level(level)
    n, xb = len(x), _mean(x)
    ss = math.fsum((v - xb) ** 2 for v in x)
    kn, an = k0 + n, a0 + n / 2
    mn = (k0 * m0 + n * xb) / kn
    bn = b0 + ss / 2 + k0 * n * (xb - m0) ** 2 / (2 * kn)
    scale = math.sqrt(bn / (an * kn))
    t = _t_ppf(0.5 * (1 + c), 2 * an)
    return {"mu_n": mn, "kappa_n": kn, "alpha_n": an, "beta_n": bn,
            "sigma2_mean": bn / (an - 1) if an > 1 else None, "lower": mn - t * scale, "upper": mn + t * scale}


def ref_nig(mu0, kappa0, alpha0, beta0, data, level=None):
    """Sequential one-observation updates (an independent algebraic route to the batch formulas); scipy's Student t
    quantile for the marginal of mu and scipy's inverse-gamma mean for sigma^2."""
    from scipy.stats import invgamma, t as T
    m, k, a, b = mu0, kappa0, alpha0, beta0
    for x in data:
        b += k * (x - m) ** 2 / (2 * (k + 1))
        m = (k * m + x) / (k + 1)
        k += 1
        a += 0.5
    c = 0.95 if level is None else level
    lo, up = T.interval(c, 2 * a, loc=m, scale=math.sqrt(b / (a * k)))
    out = {"mu_n": m, "kappa_n": k, "alpha_n": a, "beta_n": b, "lower": float(lo), "upper": float(up)}
    if a > 1:
        out["sigma2_mean"] = float(invgamma.mean(a, scale=b))
    return out


_cap("normal_inverse_gamma_posterior", level="expert",
     name="Normal model with unknown mean and variance (normal-inverse-gamma)", topic=_TNN,
     purpose="Conjugate normal-inverse-gamma update for normal data with unknown mean and variance: posterior "
             "parameters, posterior mean of the variance and the Student-t credible interval for the mean.",
     inputs=[_Num("mu0", "prior location of the mean"), _Num("kappa0", "prior pseudo-observations for the mean (> 0)"),
             _Num("alpha0", "prior inverse-gamma shape (> 0)"), _Num("beta0", "prior inverse-gamma scale (> 0)"),
             _Vec("data", "the observations"), LEVEL_IN],
     outputs=[_Out("mu_n", "posterior location"), _Out("kappa_n", "kappa0 + n"), _Out("alpha_n", "alpha0 + n/2"),
              _Out("beta_n", "posterior scale"), _Out("sigma2_mean", "posterior mean of sigma^2 (alpha_n > 1)"),
              _Out("lower", "lower credible limit for the mean"), _Out("upper", "upper credible limit for the mean")],
     primary="beta_n", conditions=["kappa0, alpha0, beta0 > 0", "0 < level < 1"],
     method="conjugate update; marginal posterior of the mean is Student t with 2 alpha_n degrees of freedom",
     run=run_nig, reference=ref_nig,
     latex=r"\beta_n = \beta_0 + \tfrac12\sum(x_i-\bar x)^2 + \frac{\kappa_0 n(\bar x-\mu_0)^2}{2(\kappa_0+n)},\quad "
           r"\mu\mid x \sim t_{2\alpha_n}\!\left(\mu_n,\ \frac{\beta_n}{\alpha_n\kappa_n}\right)",
     wrong=lambda mu0, kappa0, alpha0, beta0, data, level=None: dict(
         run_nig(mu0, kappa0, alpha0, beta0, data, level),
         beta_n=beta0 + sum((v - sum(data) / len(data)) ** 2 for v in data) / 2 + 1.0),
     rnd=lambda rng: (lambda mu, s: {"mu0": round(rng.uniform(-20, 20), 2), "kappa0": round(rng.uniform(0.1, 10), 2),
                                     "alpha0": round(rng.uniform(1, 10), 2), "beta0": round(rng.uniform(0.5, 20), 2),
                                     "data": [round(rng.gauss(mu, s), 2) for _ in range(rng.randint(2, 20))],
                                     "level": _rlevel(rng)})(rng.uniform(-20, 20), rng.uniform(0.5, 6)),
     need=[r"mu0", r"kappa0", r"alpha0", r"beta0"], aliases=_NN_AL, extract=_x_level,
     templates=["Normal-inverse-gamma posterior: mu0 = 0, kappa0 = 1, alpha0 = 2, beta0 = 2, data = [1.2, 0.8, 1.5, "
                "0.9]. 95% credible interval for the mean?",
                "Unknown mean and variance with mu0 = 10, kappa0 = 0.5, alpha0 = 3, beta0 = 4 and data = [9, 11, 12]."],
     examples=[_X({"mu0": 0, "kappa0": 1, "alpha0": 2, "beta0": 2, "data": "1, -1"},
                  {"mu_n": 0.0, "kappa_n": 3.0, "alpha_n": 3.0, "beta_n": 3.0, "sigma2_mean": 1.5}),
               _X({"mu0": 10, "kappa0": 0.5, "alpha0": 3, "beta0": 4, "data": [9, 11, 12]}, {"kappa_n": 3.5})],
     questions=[_Q("Normal-inverse-gamma posterior: mu0 = 0, kappa0 = 1, alpha0 = 2, beta0 = 2, data = [1, -1]. "
                   "95% credible interval for the mean?", {"mu_n": 0.0, "beta_n": 3.0}),
                _Q("Unknown mean and variance with mu0 = 10, kappa0 = 0, alpha0 = 3, beta0 = 4 and data = [9, 11, 12].",
                   outcome="INVALID_INPUT")])


# ======================================================================== conjugate: Dirichlet-multinomial ====
_TDM = "conjugate priors: Dirichlet-multinomial"


def _dm_check(alpha, counts):
    a = _vec(alpha, "the prior concentrations", min_len=2)
    x = _vec(counts, "the counts", len(a))
    if any(v <= 0 for v in a):
        raise ValueError("Dirichlet concentrations must be positive")
    if any(v < 0 or v != int(v) for v in x):
        raise ValueError("counts must be whole numbers >= 0")
    return a, x


def run_dm_post(alpha, counts):
    a, x = _dm_check(alpha, counts)
    post = [u + v for u, v in zip(a, x)]
    s = math.fsum(post)
    return {"post_alpha": post, "posterior_means": [p / s for p in post],
            "posterior_variances": [p * (s - p) / (s * s * (s + 1)) for p in post]}


def ref_dm_post(alpha, counts):
    """Posterior moments as predictive ratios of Dirichlet-multinomial probabilities under the PRIOR (scipy):
    E[theta_j | x] = P(x + e_j)/P(x) and E[theta_j^2 | x] = P(x + 2 e_j)/P(x), adjusted for the multinomial
    coefficients; the posterior concentrations recovered from mean and variance."""
    import numpy as np
    from scipy.stats import dirichlet_multinomial as DM
    x = np.array(counts, float)
    n = int(x.sum())
    l0 = DM.logpmf(x, alpha, n)
    means, var, post = [], [], []
    for j in range(len(x)):
        e = np.zeros(len(x))
        e[j] = 1
        m = math.exp(DM.logpmf(x + e, alpha, n + 1) - l0) * (x[j] + 1) / (n + 1)
        m2 = math.exp(DM.logpmf(x + 2 * e, alpha, n + 2) - l0) * (x[j] + 1) * (x[j] + 2) / ((n + 1) * (n + 2))
        means.append(m)
        var.append(m2 - m * m)
    a0 = means[0] * (1 - means[0]) / var[0] - 1
    return {"posterior_means": means, "posterior_variances": var, "post_alpha": [m * a0 for m in means]}


def _rdm(rng):
    k = rng.randint(2, 6)
    p = _rsimplex(rng, k)
    n = rng.randint(1, 60)
    return {"alpha": [round(rng.uniform(0.5, 5), 2) for _ in range(k)],
            "counts": [float(int(round(n * v + rng.uniform(-2, 2))) if n * v > 2 else rng.randint(0, 3)) for v in p]}


_cap("dirichlet_multinomial_posterior", level="advanced", name="Dirichlet-multinomial conjugate posterior",
     topic=_TDM,
     purpose="Update a Dirichlet(alpha_1, ..., alpha_k) prior on category probabilities with category counts: "
             "posterior concentrations, means and variances.",
     inputs=[_Vec("alpha", "prior concentrations alpha_j (> 0)", 2), _Vec("counts", "observed category counts", 2)],
     outputs=[_Out("post_alpha", "alpha_j + n_j"), _Out("posterior_means", "posterior means (alpha_j + n_j)/(alpha_0 + n)"),
              _Out("posterior_variances", "posterior variances")],
     primary="posterior_means", conditions=["alpha_j > 0", "counts whole numbers >= 0", "same length"],
     method="conjugate update of the Dirichlet prior with the multinomial likelihood", run=run_dm_post,
     reference=ref_dm_post,
     latex=r"\theta\mid n \sim \mathrm{Dir}(\alpha_1 + n_1, \ldots, \alpha_k + n_k),\quad E[\theta_j] = \frac{\alpha_j + n_j}{\alpha_0 + n}",
     wrong=lambda alpha, counts: (lambda r: dict(r, posterior_means=[(c + 1) / (sum(counts) + len(counts))
                                                                     for c in counts]) if any(abs(a - 1) > 1e-9 for a in alpha)
                                 else dict(r, posterior_means=[c / max(sum(counts), 1) for c in counts]))(
         run_dm_post(alpha, counts)),
     rnd=_rdm, need=[r"dirichlet", r"counts\s*(?:=|:)?\s*\["],
     avoid=[r"evidence|marginal likelihood|probability of (?:the|these) counts|pmf"],
     aliases={"alpha": r"alpha|concentrations|prior", "counts": r"counts"},
     templates=["Dirichlet-multinomial posterior with alpha = [1, 1, 1] and counts = [10, 5, 3]: posterior means?",
                "Dirichlet prior concentrations [2, 2, 2, 2] updated with counts [3, 0, 7, 1]."],
     examples=[_X({"alpha": "1, 1, 1", "counts": "10, 5, 3"}, {"posterior_means": [11 / 21, 6 / 21, 4 / 21],
                                                                "post_alpha": [11.0, 6.0, 4.0]}),
               _X({"alpha": [2, 2, 2, 2], "counts": [3, 0, 7, 1]}, {"post_alpha": [5.0, 2.0, 9.0, 3.0]})],
     questions=[_Q("Dirichlet-multinomial posterior with alpha = [1, 1, 1] and counts = [10, 5, 3]: posterior means?",
                   {"posterior_means": [11 / 21, 6 / 21, 4 / 21]}),
                _Q("Dirichlet prior concentrations [2, 2, 2, 2] updated with counts [3, 0, 7, 1].",
                   {"post_alpha": [5.0, 2.0, 9.0, 3.0]}),
                _Q("Dirichlet-multinomial posterior with alpha = [1, 1, 1] and counts = [10, 5].",
                   outcome="INVALID_INPUT")])


def run_dm_evidence(alpha, counts):
    a, x = _dm_check(alpha, counts)
    n, a0 = math.fsum(x), math.fsum(a)
    lp = (math.lgamma(n + 1) - math.fsum(math.lgamma(v + 1) for v in x) + math.lgamma(a0) - math.lgamma(a0 + n)
          + math.fsum(math.lgamma(u + v) - math.lgamma(u) for u, v in zip(a, x)))
    return {"probability": math.exp(lp), "log_probability": lp}


_cap("dirichlet_multinomial_evidence", level="advanced",
     name="Dirichlet-multinomial probability of a count vector (evidence)", topic=_TDM,
     purpose="Prior predictive probability (marginal likelihood) of observed category counts when the category "
             "probabilities have a Dirichlet prior.",
     inputs=[_Vec("alpha", "prior concentrations (> 0)", 2), _Vec("counts", "category counts", 2)],
     outputs=[_Out("probability", "P(counts | alpha)"), _Out("log_probability", "its natural log")],
     primary="log_probability", conditions=["alpha_j > 0", "counts whole numbers >= 0"],
     method="closed form with log-gamma functions", run=run_dm_evidence,
     reference=lambda alpha, counts: (lambda DM: {
         "probability": float(DM.pmf(counts, alpha, int(sum(counts)))),
         "log_probability": float(DM.logpmf(counts, alpha, int(sum(counts))))})(
         __import__("scipy.stats", fromlist=["dirichlet_multinomial"]).dirichlet_multinomial),
     latex=r"P(n\mid\alpha) = \frac{n!\,\Gamma(\alpha_0)}{\Gamma(n + \alpha_0)}\prod_j\frac{\Gamma(n_j + \alpha_j)}{n_j!\,\Gamma(\alpha_j)}",
     wrong=lambda alpha, counts: (lambda r: dict(r, log_probability=r["log_probability"] - math.lgamma(sum(counts) + 1)
                                                 + sum(math.lgamma(c + 1) for c in counts),
                                                 probability=math.exp(r["log_probability"] - math.lgamma(sum(counts) + 1)
                                                                      + sum(math.lgamma(c + 1) for c in counts))))(
         run_dm_evidence(alpha, counts)),
     rnd=lambda rng: (lambda x: x if sum(1 for c in x["counts"] if c > 0) >= 2 else dict(
         x, counts=[c + 1 for c in x["counts"]]))(_rdm(rng)),
     need=[r"dirichlet", r"evidence|marginal likelihood|probability of (?:the|these) counts"],
     aliases={"alpha": r"alpha|concentrations|prior", "counts": r"counts"},
     templates=["Dirichlet-multinomial evidence for counts = [2, 1, 0] with alpha = [1, 1, 1].",
                "Marginal likelihood of counts [3, 3] under a Dirichlet prior with alpha = [0.5, 0.5]."],
     examples=[_X({"alpha": "1, 1, 1", "counts": "2, 1, 0"}, {"probability": 0.1}),
               _X({"alpha": [1, 1], "counts": [3, 3]}, {"probability": 1 / 7})],
     questions=[_Q("Dirichlet-multinomial evidence for counts = [2, 1, 0] with alpha = [1, 1, 1].",
                   {"probability": 0.1}),
                _Q("Marginal likelihood of counts [3, 3] under a Dirichlet prior with alpha = [1, 1].",
                   {"probability": 1 / 7}),
                _Q("Dirichlet-multinomial evidence for counts = [2, 1, 0] with alpha = [1, -1, 1].",
                   outcome="INVALID_INPUT")])


# ============================================================================================= Bayes factors ====
_TBF = "Bayes factors and model comparison"


def run_bf_simple(successes, trials, p0, p1):
    n, k = _whole(trials, "the number of trials", 1), _whole(successes, "the number of successes")
    if k > n:
        raise ValueError("successes cannot exceed trials")
    a, b = _prob(p0, "p0", open_=True), _prob(p1, "p1", open_=True)
    lbf = k * (math.log(b) - math.log(a)) + (n - k) * (math.log1p(-b) - math.log1p(-a))
    return {"bf10": math.exp(lbf), "log10_bf10": lbf / math.log(10), "posterior_h1": 1 / (1 + math.exp(-lbf))}


def ref_bf_simple(successes, trials, p0, p1):
    from scipy.stats import binom
    l1, l0 = binom.logpmf(successes, trials, p1), binom.logpmf(successes, trials, p0)
    return {"bf10": float(math.exp(l1 - l0)), "log10_bf10": float((l1 - l0) / math.log(10)),
            "posterior_h1": float(math.exp(binom.logpmf(successes, trials, p1)) /
                                  (math.exp(l1) + math.exp(l0)))}


def _rbfs(rng):
    p0 = _rp(rng, 0.1, 0.8, 3)
    p1 = round(min(0.95, max(0.05, p0 + rng.choice([-1, 1]) * rng.uniform(0.05, 0.3))), 3)
    n = rng.randint(5, 80)
    p = rng.choice([p0, p1, (p0 + p1) / 2])
    return {"successes": max(0, min(n, int(round(n * p + rng.uniform(-2, 2))))), "trials": n, "p0": p0, "p1": p1}


_cap("bayes_factor_binomial_simple", level="intermediate", name="Bayes factor for two simple binomial hypotheses",
     topic=_TBF,
     purpose="Bayes factor BF10 for H1: p = p1 against H0: p = p0 after k successes in n trials, its log10 and the "
             "posterior probability of H1 with equal prior odds.",
     inputs=[_Int("successes", "successes k"), _Int("trials", "trials n (>= 1)", lo=1),
             _Num("p0", "success probability under H0"), _Num("p1", "success probability under H1")],
     outputs=[_Out("bf10", "Bayes factor H1 : H0"), _Out("log10_bf10", "log10 of BF10"),
              _Out("posterior_h1", "P(H1 | data) with prior odds 1")],
     primary="log10_bf10", conditions=["0 < p0, p1 < 1", "0 <= k <= n"],
     method="likelihood ratio of the two simple hypotheses", run=run_bf_simple, reference=ref_bf_simple,
     latex=r"BF_{10} = \left(\frac{p_1}{p_0}\right)^{k}\left(\frac{1-p_1}{1-p_0}\right)^{n-k}",
     wrong=lambda successes, trials, p0, p1: (lambda l: {"bf10": math.exp(l), "log10_bf10": l / math.log(10),
                                                         "posterior_h1": 1 / (1 + math.exp(-l))})(
         successes * (math.log(p1) - math.log(p0))),
     rnd=_rbfs, need=[r"bayes factor", r"p0\s*=", r"p1\s*="], avoid=[r"alpha|beta\("],
     aliases={"successes": r"successes|k", "trials": r"trials|n", "p0": r"p0", "p1": r"p1"}, extract=_x_trials,
     templates=["Bayes factor for p1 = 0.7 against p0 = 0.5 after 14 successes in 20 trials.",
                "Compute the Bayes factor with successes = 3, trials = 10, p0 = 0.5, p1 = 0.2."],
     examples=[_X({"successes": 1, "trials": 1, "p0": 0.5, "p1": 0.8}, {"bf10": 1.6, "posterior_h1": 1.6 / 2.6}),
               _X({"successes": 14, "trials": 20, "p0": 0.5, "p1": 0.7},
                  {"bf10": (0.7 / 0.5) ** 14 * (0.3 / 0.5) ** 6})],
     questions=[_Q("Bayes factor for p1 = 0.7 against p0 = 0.5 after 14 successes in 20 trials.",
                   {"bf10": (0.7 / 0.5) ** 14 * (0.3 / 0.5) ** 6}),
                _Q("Compute the Bayes factor with successes = 1, trials = 1, p0 = 0.5, p1 = 0.8.", {"bf10": 1.6}),
                _Q("Compute the Bayes factor with successes = 3, trials = 10, p0 = 1.5, p1 = 0.2.",
                   outcome="INVALID_INPUT")])


def run_bf_point_null(successes, trials, p0, alpha=1.0, beta=1.0):
    n, k = _whole(trials, "the number of trials", 1), _whole(successes, "the number of successes")
    if k > n:
        raise ValueError("successes cannot exceed trials")
    q = _prob(p0, "p0", open_=True)
    a, b = _pos(1.0 if alpha is None else alpha, "alpha"), _pos(1.0 if beta is None else beta, "beta")
    lbf = _lbeta(a + k, b + n - k) - _lbeta(a, b) - k * math.log(q) - (n - k) * math.log1p(-q)
    return {"bf10": math.exp(lbf), "bf01": math.exp(-lbf), "log10_bf10": lbf / math.log(10),
            "posterior_h1": 1 / (1 + math.exp(-lbf))}


def ref_bf_point_null(successes, trials, p0, alpha=1.0, beta=1.0):
    """Savage-Dickey density ratio: BF01 = posterior density at p0 / prior density at p0 (scipy)."""
    from scipy.stats import beta as B
    a, b = alpha or 1.0, beta or 1.0
    l01 = B.logpdf(p0, a + successes, b + trials - successes) - B.logpdf(p0, a, b)
    return {"bf01": float(math.exp(l01)), "bf10": float(math.exp(-l01)), "log10_bf10": float(-l01 / math.log(10)),
            "posterior_h1": float(1 / (1 + math.exp(l01)))}


def _rbfpn(rng):
    n = rng.randint(5, 100)
    p0 = _rp(rng, 0.1, 0.9, 2)
    p = min(0.98, max(0.02, p0 + rng.uniform(-2.5, 2.5) * math.sqrt(p0 * (1 - p0) / n)))
    k = int(round(n * p))
    k = min(n - 1, max(1, k))
    return {"successes": k, "trials": n, "p0": p0, "alpha": round(rng.uniform(0.5, 5), 2),
            "beta": round(rng.uniform(0.5, 5), 2)}


_cap("bayes_factor_point_null_binomial", level="advanced",
     name="Bayes factor for a point null on a proportion (beta prior under H1)", topic=_TBF,
     purpose="BF10 for H1: p ~ Beta(alpha, beta) against H0: p = p0 after k successes in n trials (default uniform "
             "prior), with BF01 and the posterior probability of H1 at equal prior odds.",
     inputs=[_Int("successes", "successes k"), _Int("trials", "trials n (>= 1)", lo=1), _Num("p0", "null value p0"),
             _Num("alpha", "Beta alpha under H1 (default 1)", required=False),
             _Num("beta", "Beta beta under H1 (default 1)", required=False)],
     outputs=[_Out("bf10", "Bayes factor H1 : H0"), _Out("bf01", "1/BF10"), _Out("log10_bf10", "log10 BF10"),
              _Out("posterior_h1", "P(H1 | data) with prior odds 1")],
     primary="log10_bf10", conditions=["0 < p0 < 1", "alpha, beta > 0", "0 <= k <= n"],
     method="ratio of the beta-binomial marginal likelihood to the binomial likelihood at p0", run=run_bf_point_null,
     reference=ref_bf_point_null,
     latex=r"BF_{10} = \frac{B(\alpha + k,\ \beta + n - k)/B(\alpha,\beta)}{p_0^{k}(1-p_0)^{n-k}}",
     wrong=lambda successes, trials, p0, alpha=1.0, beta=1.0: (lambda r: (lambda l: {
         "bf10": math.exp(l), "bf01": math.exp(-l), "log10_bf10": l / math.log(10), "posterior_h1": 1 / (1 + math.exp(-l))})(
         math.log(r["bf10"]) + math.log(math.comb(trials, successes))))(
         run_bf_point_null(successes, trials, p0, alpha, beta)),
     rnd=_rbfpn, need=[r"bayes factor", r"p0\s*=|point null|null value"], avoid=[r"p1\s*="],
     aliases={"successes": r"successes|k", "trials": r"trials|n", "p0": r"p0|null value", "alpha": r"alpha",
              "beta": r"beta"},
     extract=_chain(_x_trials, _x_beta_prior),
     templates=["Bayes factor against the point null p0 = 0.5 with a uniform prior: 60 successes in 100 trials.",
                "Compute the Bayes factor for p0 = 0.25 with successes = 9, trials = 20, alpha = 2, beta = 2."],
     examples=[_X({"successes": 1, "trials": 1, "p0": 0.5}, {"bf10": 1.0}),
               _X({"successes": 2, "trials": 2, "p0": 0.5}, {"bf10": (1 / 3) / 0.25})],
     questions=[_Q("Bayes factor against the point null p0 = 0.5 with a uniform prior: 2 successes in 2 trials.",
                   {"bf10": 4 / 3}),
                _Q("Compute the Bayes factor for p0 = 0.25 with successes = 9, trials = 20, alpha = 2, beta = 2.", {}),
                _Q("Compute the Bayes factor for p0 = 0 with successes = 9, trials = 20.", outcome="INVALID_INPUT")])


def run_bf_normal(sample_mean, n, sigma, mu0, prior_sd):
    xb, k, s, m0, t = (_fin(sample_mean, "the sample mean"), _whole(n, "the sample size", 1), _pos(sigma, "sigma"),
                       _fin(mu0, "mu0"), _pos(prior_sd, "the prior standard deviation"))
    m, v = _nn_post(m0, t, s, xb, k)
    l01 = (-0.5 * (m0 - m) ** 2 / v - 0.5 * math.log(v)) - (-0.5 * math.log(t * t))   # Savage-Dickey
    return {"bf10": math.exp(-l01), "bf01": math.exp(l01), "log10_bf10": -l01 / math.log(10),
            "posterior_h1": 1 / (1 + math.exp(l01))}


def ref_bf_normal(sample_mean, n, sigma, mu0, prior_sd):
    """Ratio of the marginal densities of the sample mean under H1 (N(mu0, tau^2 + sigma^2/n)) and H0 (scipy)."""
    from scipy.stats import norm
    se = sigma / math.sqrt(n)
    l10 = norm.logpdf(sample_mean, mu0, math.sqrt(prior_sd ** 2 + se ** 2)) - norm.logpdf(sample_mean, mu0, se)
    return {"bf10": float(math.exp(l10)), "bf01": float(math.exp(-l10)), "log10_bf10": float(l10 / math.log(10)),
            "posterior_h1": float(1 / (1 + math.exp(-l10)))}


def _rbfn(rng):
    s, n = round(rng.uniform(0.5, 20), 2), rng.randint(2, 200)
    m0 = round(rng.uniform(-20, 20), 2)
    return {"sample_mean": round(m0 + rng.uniform(-3.5, 3.5) * s / math.sqrt(n), 3), "n": n, "sigma": s, "mu0": m0,
            "prior_sd": round(s * rng.uniform(0.05, 3), 3)}


_cap("bayes_factor_normal_mean", level="advanced", name="Bayes factor for a normal mean (known variance)",
     topic=_TBF,
     purpose="BF10 for H1: mu ~ N(mu0, tau^2) against H0: mu = mu0, from the sample mean of n observations with "
             "known sigma; BF01 and the posterior probability of H1 at equal prior odds.",
     inputs=[_Num("sample_mean", "sample mean"), _Int("n", "sample size (>= 1)", lo=1),
             _Num("sigma", "known standard deviation (> 0)"), _Num("mu0", "null value mu0"),
             _Num("prior_sd", "prior standard deviation tau of mu under H1 (> 0)")],
     outputs=[_Out("bf10", "Bayes factor H1 : H0"), _Out("bf01", "1/BF10"), _Out("log10_bf10", "log10 BF10"),
              _Out("posterior_h1", "P(H1 | data) with prior odds 1")],
     primary="log10_bf10", conditions=["sigma, tau > 0", "n >= 1"],
     method="Savage-Dickey density ratio of the posterior and prior at mu0", run=run_bf_normal, reference=ref_bf_normal,
     latex=r"BF_{01} = \frac{p(\mu_0\mid \bar x, H_1)}{p(\mu_0\mid H_1)}",
     wrong=lambda sample_mean, n, sigma, mu0, prior_sd: run_bf_normal(sample_mean, 1, sigma, mu0, prior_sd)
     if n > 1 else run_bf_normal(sample_mean, 4, sigma, mu0, prior_sd),
     rnd=_rbfn, need=[r"bayes factor", r"sample mean|xbar", r"mu0"],
     aliases=dict(_NN_AL, prior_sd=r"prior (?:standard deviation|sd)|tau"),
     templates=["Bayes factor for a normal mean: sample mean = 0.5, n = 25, sigma = 2, mu0 = 0, prior sd = 1.",
                "Compute the Bayes factor with xbar = 101.2, n = 50, known sigma = 10, mu0 = 100 and tau = 5."],
     examples=[_X({"sample_mean": 0, "n": 1, "sigma": 1, "mu0": 0, "prior_sd": 1}, {"bf10": 1 / math.sqrt(2)}),
               _X({"sample_mean": 0.5, "n": 25, "sigma": 2, "mu0": 0, "prior_sd": 1}, {})],
     questions=[_Q("Bayes factor for a normal mean: sample mean = 0, n = 1, sigma = 1, mu0 = 0, prior sd = 1.",
                   {"bf10": 1 / math.sqrt(2)}),
                _Q("Compute the Bayes factor with xbar = 101.2, n = 50, known sigma = 10, mu0 = 100 and tau = 5.", {}),
                _Q("Compute the Bayes factor with xbar = 101.2, n = 50, known sigma = -10, mu0 = 100 and tau = 5.",
                   outcome="INVALID_INPUT")])


def run_model_probs(log_evidences, priors=None):
    le = _vec(log_evidences, "the log marginal likelihoods", min_len=2)
    p = [1.0 / len(le)] * len(le) if priors is None else _simplex(priors, "the prior model probabilities", len(le))
    if all(v == 0 for v in p):
        raise ValueError("at least one model needs a positive prior probability")
    z = [math.log(w) + v if w > 0 else -math.inf for w, v in zip(p, le)]
    lse = _logsumexp([v for v in z if v > -math.inf])
    post = [math.exp(v - lse) if v > -math.inf else 0.0 for v in z]
    best = max(range(len(post)), key=lambda i: post[i])
    return {"posterior": post, "best_model": best + 1, "best_probability": post[best]}


def ref_model_probs(log_evidences, priors=None):
    import numpy as np
    from scipy.special import softmax
    p = np.full(len(log_evidences), 1 / len(log_evidences)) if priors is None else np.array(priors, float)
    with np.errstate(divide="ignore"):
        post = softmax(np.array(log_evidences, float) + np.log(p))
    return {"posterior": list(post), "best_model": int(np.argmax(post)) + 1, "best_probability": float(post.max())}


_cap("posterior_model_probabilities", level="advanced", name="Posterior model probabilities from log evidences",
     topic=_TBF,
     purpose="Posterior probabilities of several models from their log marginal likelihoods and prior model "
             "probabilities (default equal), computed stably in log space.",
     inputs=[_Vec("log_evidences", "natural-log marginal likelihoods of the models", 2),
             _Vec("priors", "prior model probabilities (default equal)", 2, required=False)],
     outputs=[_Out("posterior", "posterior model probabilities"), _Out("best_model", "number of the most probable model"),
              _Out("best_probability", "its posterior probability")],
     primary="best_probability", conditions=["prior probabilities sum to 1"],
     method="log-sum-exp normalization of log prior + log evidence", run=run_model_probs, reference=ref_model_probs,
     latex=r"P(M_i\mid D) = \frac{P(D\mid M_i)P(M_i)}{\sum_j P(D\mid M_j)P(M_j)}",
     wrong=lambda log_evidences, priors=None: (lambda s: dict(run_model_probs(log_evidences, priors),
                                                             posterior=[v / s for v in log_evidences],
                                                             best_probability=max(log_evidences) / s))(sum(log_evidences)),
     rnd=lambda rng: (lambda k: {"log_evidences": [round(rng.uniform(-60, -10), 3) for _ in range(k)],
                                 "priors": _rsimplex(rng, k)})(rng.randint(2, 5)),
     need=[r"log (?:marginal likelihoods|evidences)|log_evidences"], aliases={
         "log_evidences": r"log marginal likelihoods|log evidences|log_evidences", "priors": r"prior model probabilities|priors"},
     templates=["Posterior model probabilities from log marginal likelihoods = [-10.2, -12.5, -11.1].",
                "Models with log evidences [-100, -101] and prior model probabilities [0.3, 0.7]: posterior "
                "probabilities?"],
     examples=[_X({"log_evidences": "-1, -1"}, {"posterior": [0.5, 0.5]}),
               _X({"log_evidences": [0, -math.log(3)], "priors": [0.5, 0.5]}, {"posterior": [0.75, 0.25],
                                                                              "best_model": 1})],
     questions=[_Q("Posterior model probabilities from log marginal likelihoods = [-10, -10, -10].",
                   {"best_probability": 1 / 3}),
                _Q("Models with log evidences [-100, -101] and prior model probabilities [0.3, 0.7]: posterior "
                   "probabilities?", {"best_model": 1, "best_probability": 0.3 / (0.3 + 0.7 * math.exp(-1))}),
                _Q("Models with log evidences [-100, -101] and prior model probabilities [0.3, 0.8]: posterior?",
                   outcome="INVALID_INPUT")])

# ========================================================================================= decision analysis ====
_TDA = "decision analysis"
_DA_AL = {"payoffs": r"payoffs?(?: matrix)?|payoff table", "probabilities": r"(?:state )?probabilities|probs",
          "losses": r"loss(?:es)?(?: matrix)?", "priors": r"prior probabilities|priors",
          "likelihoods": r"likelihoods?(?: matrix)?|test accuracy", "criterion": r"criterion|rule",
          "optimism": r"optimism(?: coefficient)?|alpha", "outcomes": r"outcomes|payoffs|wealth levels",
          "risk_tolerance": r"risk tolerance|r", "loss_fp": r"loss_fp|false[- ]positive loss|cost of a false positive",
          "loss_fn": r"loss_fn|false[- ]negative loss|cost of a false negative",
          "probability": r"probability(?: of (?:disease|the condition))?|p"}


def _payoff_check(payoffs, probabilities, name="the payoff matrix"):
    m = _mat(payoffs, name)
    p = _simplex(probabilities, "the state probabilities", len(m[0]))
    return m, p


def _ev(m, p):
    return [math.fsum(a * b for a, b in zip(r, p)) for r in m]


def _argbest(v, sign=1):
    best = max(range(len(v)), key=lambda i: sign * v[i])
    return best


def run_ev_decision(payoffs, probabilities):
    m, p = _payoff_check(payoffs, probabilities)
    ev = _ev(m, p)
    b = _argbest(ev)
    return {"expected_values": ev, "best_action": b + 1, "best_expected_value": ev[b]}


def ref_ev_decision(payoffs, probabilities):
    import numpy as np
    ev = np.array(payoffs, float) @ np.array(probabilities, float)
    return {"expected_values": list(ev), "best_action": int(np.argmax(ev)) + 1, "best_expected_value": float(ev.max())}


def _rpay(rng, rows=None, cols=None):
    r, c = rows or rng.randint(2, 5), cols or rng.randint(2, 5)
    return [[round(rng.uniform(-100, 300), 2) for _ in range(c)] for _ in range(r)]


def _rpay_p(rng):
    m = _rpay(rng)
    return {"payoffs": m, "probabilities": _rsimplex(rng, len(m[0]))}


_cap("expected_value_decision", level="beginner", name="Best action by expected monetary value", topic=_TDA,
     purpose="Expected payoff of each action over uncertain states with known probabilities, and the action with the "
             "highest expected value.",
     inputs=[_Mat("payoffs", "payoff matrix: one row per action, one column per state"),
             _Vec("probabilities", "state probabilities (sum to 1)", 2)],
     outputs=[_Out("expected_values", "expected payoff of each action"),
              _Out("best_action", "number (1-based) of the best action (first if tied)"),
              _Out("best_expected_value", "its expected payoff")],
     primary="best_expected_value", conditions=["probabilities sum to 1", "one probability per column"],
     method="expected value of each row", run=run_ev_decision, reference=ref_ev_decision,
     latex=r"EV(a) = \sum_s P(s)\,u(a, s),\quad a^* = \arg\max_a EV(a)",
     wrong=lambda payoffs, probabilities: (lambda r: dict(r, best_expected_value=max(max(row) for row in payoffs)))(
         run_ev_decision(payoffs, probabilities)),
     rnd=_rpay_p, need=[r"expected (?:monetary )?value|\bemv\b", r"payoff"],
     avoid=[r"perfect information|evpi|sample information|evsi|regret|maximin|maximax|hurwicz|laplace|criterion|"
            r"break[- ]even|indifferen|likelihood"],
     aliases=_DA_AL,
     templates=["Expected value decision: payoffs = [[100, -20], [40, 30]], probabilities = [0.4, 0.6]. Best action?",
                "Which action maximizes expected monetary value with payoff matrix [[5, 1, 0], [3, 3, 3]] and state "
                "probabilities [0.2, 0.5, 0.3]?"],
     examples=[_X({"payoffs": "[[100, -20], [40, 30]]", "probabilities": "0.4, 0.6"},
                  {"expected_values": [28.0, 34.0], "best_action": 2, "best_expected_value": 34.0}),
               _X({"payoffs": [[5, 1, 0], [3, 3, 3]], "probabilities": [0.2, 0.5, 0.3]}, {"best_action": 2})],
     questions=[_Q("Expected value decision: payoffs = [[100, -20], [40, 30]], probabilities = [0.4, 0.6]. Best action?",
                   {"best_action": 2, "best_expected_value": 34.0}),
                _Q("Which action maximizes expected monetary value with payoff matrix [[5, 1, 0], [3, 3, 3]] and "
                   "state probabilities [0.2, 0.5, 0.3]?", {"best_action": 2, "best_expected_value": 3.0}),
                _Q("Expected value decision: payoffs = [[100, -20], [40, 30]], probabilities = [0.4, 0.4].",
                   outcome="INVALID_INPUT"),
                _Q("Expected value decision: payoffs = [[100, -20], [40, 30]]. Best action?", outcome="NEEDS_INPUT")])


_CRIT = ["maximin", "maximax", "laplace", "minimax regret", "hurwicz"]


def run_criteria(payoffs, criterion, optimism=None):
    m = _mat(payoffs, "the payoff matrix")
    if criterion not in _CRIT:
        raise ValueError("criterion must be one of " + ", ".join(_CRIT))
    if criterion == "maximin":
        s, sign = [min(r) for r in m], 1
    elif criterion == "maximax":
        s, sign = [max(r) for r in m], 1
    elif criterion == "laplace":
        s, sign = [math.fsum(r) / len(r) for r in m], 1
    elif criterion == "hurwicz":
        if optimism is None:
            raise ValueError("the Hurwicz criterion needs the optimism coefficient alpha (0 to 1)")
        a = _prob(optimism, "the optimism coefficient")
        s, sign = [a * max(r) + (1 - a) * min(r) for r in m], 1
    else:
        col = [max(r[j] for r in m) for j in range(len(m[0]))]
        s, sign = [max(col[j] - r[j] for j in range(len(r))) for r in m], -1
    b = _argbest(s, sign)
    return {"scores": s, "best_action": b + 1, "criterion_value": s[b]}


def ref_criteria(payoffs, criterion, optimism=None):
    import numpy as np
    m = np.array(payoffs, float)
    if criterion == "maximin":
        s = m.min(axis=1)
    elif criterion == "maximax":
        s = m.max(axis=1)
    elif criterion == "laplace":
        s = m.mean(axis=1)
    elif criterion == "hurwicz":
        s = optimism * m.max(axis=1) + (1 - optimism) * m.min(axis=1)
    else:
        s = (m.max(axis=0) - m).max(axis=1)
        b = int(np.argmin(s))
        return {"scores": list(s), "best_action": b + 1, "criterion_value": float(s[b])}
    b = int(np.argmax(s))
    return {"scores": list(s), "best_action": b + 1, "criterion_value": float(s[b])}


_cap("decision_criteria_under_uncertainty", level="beginner",
     name="Decision under uncertainty: maximin, maximax, Laplace, minimax regret, Hurwicz", topic=_TDA,
     purpose="The best action and its criterion value when state probabilities are unknown, by a stated criterion.",
     inputs=[_Mat("payoffs", "payoff matrix: one row per action, one column per state"),
             _Ch("criterion", "maximin, maximax, laplace, minimax regret or hurwicz", _CRIT),
             _Num("optimism", "Hurwicz optimism coefficient alpha (Hurwicz only)", required=False)],
     outputs=[_Out("scores", "criterion score of each action (maximum regret for minimax regret)"),
              _Out("best_action", "number of the chosen action (first if tied)"),
              _Out("criterion_value", "criterion value of the chosen action")],
     primary="criterion_value", conditions=["0 <= alpha <= 1 for Hurwicz"],
     method="row minima / maxima / means / Hurwicz mixtures, or maximum regret per row", run=run_criteria,
     reference=ref_criteria,
     latex=r"\text{maximin: }\max_a\min_s u,\quad \text{regret } r(a,s) = \max_{a'}u(a',s) - u(a,s)",
     wrong=lambda payoffs, criterion, optimism=None: run_criteria(
         payoffs, {"maximin": "maximax", "maximax": "maximin", "laplace": "maximin", "minimax regret": "maximin",
                   "hurwicz": "laplace"}[criterion], optimism),
     rnd=lambda rng: (lambda c: dict({"payoffs": _rpay(rng), "criterion": c},
                                     **({"optimism": _rp(rng, 0.05, 0.95, 2)} if c == "hurwicz" else {})))(
         rng.choice(_CRIT)),
     need=[r"payoff", r"maximin|maximax|laplace|minimax regret|hurwicz"], aliases=_DA_AL,
     extract=lambda q, d: d.setdefault("criterion", next(c for c in sorted(_CRIT, key=len, reverse=True) if c in q))
     if any(c in q for c in _CRIT) else None,
     templates=["Use the maximin criterion with payoffs = [[100, -20], [40, 30], [60, 10]].",
                "Minimax regret decision for payoff matrix [[5, 1, 0], [3, 3, 3]].",
                "Hurwicz criterion with optimism = 0.6 and payoffs = [[100, -20], [40, 30]]."],
     examples=[_X({"payoffs": "[[100, -20], [40, 30], [60, 10]]", "criterion": "maximin"},
                  {"best_action": 2, "criterion_value": 30.0}),
               _X({"payoffs": [[5, 1, 0], [3, 3, 3]], "criterion": "minimax regret"},
                  {"scores": [3.0, 2.0], "best_action": 2, "criterion_value": 2.0})],
     questions=[_Q("Use the maximin criterion with payoffs = [[100, -20], [40, 30], [60, 10]].",
                   {"best_action": 2, "criterion_value": 30.0}),
                _Q("Minimax regret decision for payoff matrix [[5, 1, 0], [3, 3, 3]].", {"best_action": 2}),
                _Q("Hurwicz criterion with optimism = 0.6 and payoffs = [[100, -20], [40, 30]].",
                   {"best_action": 1, "criterion_value": 52.0}),
                _Q("Hurwicz criterion with payoffs = [[100, -20], [40, 30]].", outcome="INVALID_INPUT")])


def run_evpi(payoffs, probabilities):
    m, p = _payoff_check(payoffs, probabilities)
    ev = _ev(m, p)
    b = _argbest(ev)
    wpi = math.fsum(max(r[j] for r in m) * p[j] for j in range(len(p)))
    evpi = math.fsum(p[j] * (max(r[j] for r in m) - m[b][j]) for j in range(len(p)))   # exact 0 when b dominates
    return {"evpi": evpi, "ev_with_perfect_information": wpi, "best_expected_value": ev[b]}


def ref_evpi(payoffs, probabilities):
    import numpy as np
    m, p = np.array(payoffs, float), np.array(probabilities, float)
    ev = m @ p
    b = int(np.argmax(ev))
    wpi = float(m.max(axis=0) @ p)
    return {"evpi": float(np.sum(p * (m.max(axis=0) - m[b]))), "ev_with_perfect_information": wpi,
            "best_expected_value": float(ev[b])}


_cap("expected_value_of_perfect_information", level="intermediate", name="Expected value of perfect information (EVPI)",
     topic=_TDA,
     purpose="The most a decision maker should pay to learn the true state before acting: expected value with "
             "perfect information minus the best expected value without it.",
     inputs=[_Mat("payoffs", "payoff matrix: one row per action, one column per state"),
             _Vec("probabilities", "state probabilities (sum to 1)", 2)],
     outputs=[_Out("evpi", "expected value of perfect information"),
              _Out("ev_with_perfect_information", "expected value of the best action in each state"),
              _Out("best_expected_value", "best expected value without information")],
     primary="evpi", conditions=["probabilities sum to 1"], method="E[max over actions] - max over actions of E",
     run=run_evpi, reference=ref_evpi,
     latex=r"EVPI = \sum_s P(s)\max_a u(a,s) - \max_a\sum_s P(s)\,u(a,s)",
     wrong=lambda payoffs, probabilities: (lambda r: dict(r, evpi=r["ev_with_perfect_information"] - sum(
         _ev(payoffs, probabilities)) / len(payoffs)))(run_evpi(payoffs, probabilities)),
     rnd=_rpay_p, need=[r"perfect information|evpi"], aliases=_DA_AL,
     templates=["Expected value of perfect information: payoffs = [[100, -20], [40, 30]], probabilities = [0.4, 0.6].",
                "Find the EVPI for payoff matrix [[5, 1, 0], [3, 3, 3]] and state probabilities [0.2, 0.5, 0.3]."],
     examples=[_X({"payoffs": "[[100, -20], [40, 30]]", "probabilities": "0.4, 0.6"},
                  {"evpi": 24.0, "ev_with_perfect_information": 58.0}),
               _X({"payoffs": [[5, 1, 0], [3, 3, 3]], "probabilities": [0.2, 0.5, 0.3]}, {"evpi": 0.4})],
     questions=[_Q("Expected value of perfect information: payoffs = [[100, -20], [40, 30]], probabilities = [0.4, 0.6].",
                   {"evpi": 24.0}),
                _Q("Find the EVPI for payoff matrix [[5, 1, 0], [3, 3, 3]] and state probabilities [0.2, 0.5, 0.3].",
                   {"evpi": 0.4}),
                _Q("Find the EVPI for payoff matrix [[5, 1, 0], [3, 3, 3]] and state probabilities [0.2, 0.8].",
                   outcome="INVALID_INPUT")])


def run_evsi(payoffs, priors, likelihoods):
    m, p = _payoff_check(payoffs, priors)
    L = _mat(likelihoods, "the likelihood matrix", cols=len(p))
    for j in range(len(p)):
        col = [r[j] for r in L]
        if any(v < 0 for v in col) or abs(math.fsum(col) - 1) > 1e-9:
            raise ValueError("each column of the likelihood matrix (P(signal | state)) must sum to 1")
    ev0 = _ev(m, p)
    a0 = _argbest(ev0)
    without = ev0[a0]
    total, acts, probs, gain = [], [], [], []
    for row in L:
        ps = math.fsum(row[j] * p[j] for j in range(len(p)))
        probs.append(ps)
        if ps <= 0:
            acts.append(0)
            continue
        post = [row[j] * p[j] / ps for j in range(len(p))]
        ev = _ev(m, post)
        b = _argbest(ev)
        acts.append(b + 1)
        total.append(ps * ev[b])
        gain.append(ps * (ev[b] - ev[a0]))                 # exact 0 when the prior best action stays best
    with_ = math.fsum(total)
    return {"evsi": math.fsum(gain), "ev_with_information": with_, "ev_without_information": without,
            "signal_probabilities": probs, "best_action_per_signal": acts}


def ref_evsi(payoffs, priors, likelihoods):
    """Joint-probability formulation with numpy (no posteriors): sum over signals of max over actions of
    sum_s u(a, s) P(signal, s)."""
    import numpy as np
    m, p, L = np.array(payoffs, float), np.array(priors, float), np.array(likelihoods, float)
    joint = L * p
    per = m @ joint.T                                    # actions x signals
    a0 = int(np.argmax(m @ p))
    with_ = float(per.max(axis=0).sum())
    without = float((m @ p)[a0])
    return {"evsi": float(np.sum(per.max(axis=0) - per[a0])), "ev_with_information": with_,
            "ev_without_information": without,
            "signal_probabilities": list(joint.sum(axis=1))}


def _revsi(rng):
    k = rng.randint(2, 3)
    m = _rpay(rng, cols=k)
    L = [[0.0] * k for _ in range(k)]
    for j in range(k):
        col = _rsimplex(rng, k)
        col[j], col[0] = col[0], col[j]
        acc = max(col)
        i = col.index(acc)
        col[i], col[j] = col[j], col[i]
        for s in range(k):
            L[s][j] = col[s]
    return {"payoffs": m, "priors": _rsimplex(rng, k), "likelihoods": L}


_cap("expected_value_of_sample_information", level="advanced", name="Expected value of sample information (EVSI)",
     topic=_TDA,
     purpose="Value of an imperfect test before acting: for each possible signal, update the state probabilities by "
             "Bayes' theorem, choose the best action, and compare the expected value with that of acting now.",
     inputs=[_Mat("payoffs", "payoff matrix: one row per action, one column per state"),
             _Vec("priors", "prior state probabilities (sum to 1)", 2),
             _Mat("likelihoods", "likelihood matrix P(signal | state): one row per signal, one column per state")],
     outputs=[_Out("evsi", "expected value of sample information"), _Out("ev_with_information", "expected value with the test"),
              _Out("ev_without_information", "best expected value without the test"),
              _Out("signal_probabilities", "probability of each signal"),
              _Out("best_action_per_signal", "best action after each signal")],
     primary="evsi", conditions=["priors sum to 1", "each likelihood column sums to 1"],
     method="preposterior analysis: Bayes' theorem per signal, best action per posterior, weighted by P(signal)",
     run=run_evsi, reference=ref_evsi,
     latex=r"EVSI = \sum_z P(z)\max_a\sum_s P(s\mid z)\,u(a,s) - \max_a\sum_s P(s)\,u(a,s)",
     wrong=lambda payoffs, priors, likelihoods: (lambda r: (lambda w: dict(r, evsi=w - r["ev_without_information"],
                                                                          ev_with_information=w))(
         sum(max(_ev(payoffs, [row[j] * priors[j] / max(1e-300, sum(row[i] * priors[i] for i in range(len(priors))))
                               for j in range(len(priors))])) for row in likelihoods) / len(likelihoods)))(
         run_evsi(payoffs, priors, likelihoods)),
     rnd=_revsi, need=[r"sample information|evsi|value of (?:the )?test"], aliases=_DA_AL,
     templates=["Expected value of sample information: payoffs = [[100, -20], [40, 30]], priors = [0.4, 0.6], "
                "likelihoods = [[0.8, 0.3], [0.2, 0.7]].",
                "Find the EVSI with payoff matrix [[50, -10], [0, 0]], prior probabilities [0.5, 0.5] and likelihood "
                "matrix [[0.9, 0.2], [0.1, 0.8]]."],
     examples=[_X({"payoffs": "[[100, -20], [40, 30]]", "priors": "0.4, 0.6", "likelihoods": "[[0.8, 0.3], [0.2, 0.7]]"},
                  {"ev_without_information": 34.0, "signal_probabilities": [0.5, 0.5]}),
               _X({"payoffs": [[50, -10], [0, 0]], "priors": [0.5, 0.5], "likelihoods": [[0.9, 0.2], [0.1, 0.8]]},
                  {"ev_with_information": 0.5 * 45 - 0.5 * 2, "evsi": 21.5 - 20.0})],
     questions=[_Q("Find the EVSI with payoff matrix [[50, -10], [0, 0]], prior probabilities [0.5, 0.5] and "
                   "likelihood matrix [[0.9, 0.2], [0.1, 0.8]].", {"evsi": 1.5}),
                _Q("Expected value of sample information: payoffs = [[100, -20], [40, 30]], priors = [0.4, 0.6], "
                   "likelihoods = [[0.8, 0.3], [0.3, 0.7]].", outcome="INVALID_INPUT")])


def run_bayes_action(losses, probabilities):
    m, p = _payoff_check(losses, probabilities, "the loss matrix")
    el = _ev(m, p)
    b = _argbest(el, -1)
    return {"expected_losses": el, "bayes_action": b + 1, "bayes_risk": el[b]}


_cap("bayes_action_minimum_expected_loss", level="intermediate", name="Bayes action: minimum posterior expected loss",
     topic=_TDA,
     purpose="Posterior expected loss of each action from a loss matrix and (posterior) state probabilities; the "
             "Bayes action minimizes it.",
     inputs=[_Mat("losses", "loss matrix: one row per action, one column per state"),
             _Vec("probabilities", "posterior state probabilities (sum to 1)", 2)],
     outputs=[_Out("expected_losses", "expected loss of each action"), _Out("bayes_action", "action with least expected loss"),
              _Out("bayes_risk", "its expected loss")],
     primary="bayes_risk", conditions=["probabilities sum to 1"], method="expected loss per row, minimum",
     run=run_bayes_action,
     reference=lambda losses, probabilities: (lambda np: (lambda el: {
         "expected_losses": list(el), "bayes_action": int(np.argmin(el)) + 1, "bayes_risk": float(el.min())})(
         np.array(losses, float) @ np.array(probabilities, float)))(__import__("numpy")),
     latex=r"\rho(a) = \sum_s P(s\mid x)\,L(a,s),\quad a^* = \arg\min_a \rho(a)",
     wrong=lambda losses, probabilities: (lambda r: dict(r, bayes_risk=max(r["expected_losses"]),
                                                         bayes_action=r["expected_losses"].index(max(r["expected_losses"])) + 1))(
         run_bayes_action(losses, probabilities)),
     rnd=lambda rng: (lambda m: {"losses": [[abs(v) for v in r] for r in m], "probabilities": _rsimplex(rng, len(m[0]))})(
         _rpay(rng)),
     need=[r"loss(?:es)?(?: matrix)?\s*(?:=|:)?\s*\[\s*\[", r"expected loss|bayes action|bayes risk"], aliases=_DA_AL,
     templates=["Bayes action: losses = [[0, 10], [5, 0]], probabilities = [0.3, 0.7]. Minimum expected loss?",
                "Which action minimizes the expected loss with loss matrix [[0, 4, 8], [2, 2, 2]] and probabilities "
                "[0.5, 0.3, 0.2]?"],
     examples=[_X({"losses": "[[0, 10], [5, 0]]", "probabilities": "0.3, 0.7"},
                  {"expected_losses": [7.0, 1.5], "bayes_action": 2, "bayes_risk": 1.5}),
               _X({"losses": [[0, 4, 8], [2, 2, 2]], "probabilities": [0.5, 0.3, 0.2]}, {"bayes_action": 2, "bayes_risk": 2.0})],
     questions=[_Q("Bayes action: losses = [[0, 10], [5, 0]], probabilities = [0.3, 0.7]. Minimum expected loss?",
                   {"bayes_action": 2, "bayes_risk": 1.5}),
                _Q("Which action minimizes the expected loss with loss matrix [[0, 4, 8], [2, 2, 2]] and probabilities "
                   "[0.5, 0.3, 0.2]?", {"bayes_action": 2, "bayes_risk": 2.0}),
                _Q("Bayes action: losses = [[0, 10], [5, 0]], probabilities = [0.3, 0.9].", outcome="INVALID_INPUT")])


def run_cara(outcomes, probabilities, risk_tolerance):
    x = _vec(outcomes, "the outcomes", min_len=2)
    p = _simplex(probabilities, "the probabilities", len(x))
    r = _pos(risk_tolerance, "the risk tolerance")
    lse = _logsumexp([-v / r for v in x], p)
    ce = -r * lse
    mean = math.fsum(a * b for a, b in zip(x, p))
    return {"expected_utility": -math.expm1(lse), "certainty_equivalent": ce, "risk_premium": mean - ce,
            "expected_value": mean}


def ref_cara(outcomes, probabilities, risk_tolerance):
    """scipy.special.logsumexp for E[exp(-x/R)]; numpy for the expected utility of each outcome."""
    import numpy as np
    from scipy.special import logsumexp
    x, p = np.array(outcomes, float), np.array(probabilities, float)
    ce = -risk_tolerance * logsumexp(-x / risk_tolerance, b=p)
    eu = float(p @ (1 - np.exp(-x / risk_tolerance)))
    return {"certainty_equivalent": float(ce), "expected_utility": eu, "expected_value": float(p @ x),
            "risk_premium": float(p @ x - ce)}


def _rlot(rng, positive=False):
    k = rng.randint(2, 5)
    lo = 1 if positive else -200
    return {"outcomes": [round(rng.uniform(lo, 1000), 2) for _ in range(k)], "probabilities": _rsimplex(rng, k)}


_cap("certainty_equivalent_exponential_utility", level="intermediate",
     name="Expected utility and certainty equivalent (exponential utility)", topic=_TDA,
     purpose="Expected utility, certainty equivalent and risk premium of a lottery for a risk-averse decision maker "
             "with exponential utility u(x) = 1 - exp(-x/R).",
     inputs=[_Vec("outcomes", "monetary outcomes", 2), _Vec("probabilities", "their probabilities (sum to 1)", 2),
             _Num("risk_tolerance", "risk tolerance R (> 0)")],
     outputs=[_Out("expected_utility", "E[1 - exp(-x/R)]"), _Out("certainty_equivalent", "-R ln E[exp(-x/R)]"),
              _Out("risk_premium", "expected value - certainty equivalent"), _Out("expected_value", "expected value")],
     primary="certainty_equivalent", conditions=["probabilities sum to 1", "R > 0"],
     method="log-sum-exp evaluation of E[exp(-x/R)]", run=run_cara, reference=ref_cara,
     latex=r"CE = -R\ln\sum_i p_i e^{-x_i/R}",
     wrong=lambda outcomes, probabilities, risk_tolerance: (lambda r: (lambda var: dict(
         r, certainty_equivalent=r["expected_value"] - var / (2 * risk_tolerance),
         risk_premium=var / (2 * risk_tolerance)))(sum(p * (x - r["expected_value"]) ** 2
                                                        for x, p in zip(outcomes, probabilities))))(
         run_cara(outcomes, probabilities, risk_tolerance)),
     rnd=lambda rng: dict(_rlot(rng), risk_tolerance=round(rng.uniform(50, 2000), 1)),
     need=[r"exponential utility|risk tolerance"], aliases=_DA_AL,
     templates=["Exponential utility with risk tolerance = 500: outcomes = [1000, 0], probabilities = [0.5, 0.5]. "
                "Certainty equivalent?",
                "Find the certainty equivalent of outcomes [200, 50, -100] with probabilities [0.3, 0.5, 0.2] and "
                "risk tolerance 250."],
     examples=[_X({"outcomes": "1000, 0", "probabilities": "0.5, 0.5", "risk_tolerance": 500},
                  {"certainty_equivalent": -500 * math.log(0.5 * (math.exp(-2) + 1)), "expected_value": 500.0}),
               _X({"outcomes": [200, 50, -100], "probabilities": [0.3, 0.5, 0.2], "risk_tolerance": 250}, {})],
     questions=[_Q("Exponential utility with risk tolerance = 500: outcomes = [1000, 0], probabilities = [0.5, 0.5]. "
                   "Certainty equivalent?", {"certainty_equivalent": -500 * math.log(0.5 * (math.exp(-2) + 1))}),
                _Q("Exponential utility with risk tolerance = -5: outcomes = [1000, 0], probabilities = [0.5, 0.5].",
                   outcome="INVALID_INPUT")])


def run_log_ce(outcomes, probabilities):
    x = _vec(outcomes, "the outcomes", min_len=2)
    p = _simplex(probabilities, "the probabilities", len(x))
    if any(v <= 0 for v in x):
        raise ValueError("logarithmic utility needs positive outcomes (wealth levels)")
    elu = math.fsum(b * math.log(a) for a, b in zip(x, p))
    mean = math.fsum(a * b for a, b in zip(x, p))
    return {"expected_log_utility": elu, "certainty_equivalent": math.exp(elu), "risk_premium": mean - math.exp(elu),
            "expected_value": mean}


_cap("certainty_equivalent_log_utility", level="intermediate",
     name="Certainty equivalent with logarithmic utility", topic=_TDA,
     purpose="Expected log utility, certainty equivalent (the weighted geometric mean) and risk premium of a lottery "
             "over positive wealth levels.",
     inputs=[_Vec("outcomes", "positive wealth levels", 2), _Vec("probabilities", "their probabilities (sum to 1)", 2)],
     outputs=[_Out("expected_log_utility", "E[ln x]"), _Out("certainty_equivalent", "exp(E[ln x])"),
              _Out("risk_premium", "E[x] - CE"), _Out("expected_value", "E[x]")],
     primary="certainty_equivalent", conditions=["outcomes > 0", "probabilities sum to 1"],
     method="weighted geometric mean", run=run_log_ce,
     reference=lambda outcomes, probabilities: (lambda st, np: (lambda ce, m: {
         "certainty_equivalent": ce, "expected_log_utility": math.log(ce), "expected_value": m, "risk_premium": m - ce})(
         float(st.gmean(outcomes, weights=probabilities)), float(np.dot(outcomes, probabilities))))(
         __import__("scipy.stats", fromlist=["gmean"]), __import__("numpy")),
     latex=r"CE = \exp\!\left(\sum_i p_i \ln x_i\right) = \prod_i x_i^{p_i}",
     wrong=lambda outcomes, probabilities: (lambda r: dict(r, certainty_equivalent=r["expected_value"], risk_premium=0.0))(
         run_log_ce(outcomes, probabilities)),
     rnd=lambda rng: _rlot(rng, positive=True), need=[r"log(?:arithmic)? utility"], aliases=_DA_AL,
     templates=["Logarithmic utility: outcomes = [100, 400], probabilities = [0.5, 0.5]. Certainty equivalent?",
                "Find the certainty equivalent under log utility of wealth levels [50, 100, 200] with probabilities "
                "[0.25, 0.5, 0.25]."],
     examples=[_X({"outcomes": "100, 400", "probabilities": "0.5, 0.5"},
                  {"certainty_equivalent": 200.0, "risk_premium": 50.0}),
               _X({"outcomes": [50, 100, 200], "probabilities": [0.25, 0.5, 0.25]}, {"certainty_equivalent": 100.0})],
     questions=[_Q("Logarithmic utility: outcomes = [100, 400], probabilities = [0.5, 0.5]. Certainty equivalent?",
                   {"certainty_equivalent": 200.0}),
                _Q("Find the certainty equivalent under log utility of wealth levels [50, 100, 200] with probabilities "
                   "[0.25, 0.5, 0.25].", {"certainty_equivalent": 100.0, "risk_premium": 12.5}),
                _Q("Logarithmic utility: outcomes = [-100, 400], probabilities = [0.5, 0.5].", outcome="INVALID_INPUT")])


def run_break_even(payoffs):
    m = _mat(payoffs, "the payoff matrix", 2, 2)
    (a, b), (c, d) = m
    den = (a - b) - (c - d)
    if den == 0:
        raise ValueError("the two actions' expected values never cross (parallel lines)")
    p = (d - b) / den
    if not 0 < p < 1:
        raise ValueError("one action is at least as good for every probability: there is no break-even point in (0, 1)")
    return {"break_even": p, "preferred_above": 1 if a - c > 0 else 2, "expected_value_at_break_even": p * a + (1 - p) * b}


def ref_break_even(payoffs):
    """brentq on the difference of the two expected-value lines over (0, 1)."""
    from scipy.optimize import brentq
    (a, b), (c, d) = payoffs
    ev1 = lambda p: p * a + (1 - p) * b          # noqa: E731
    f = lambda p: ev1(p) - (p * c + (1 - p) * d)  # noqa: E731
    p = brentq(f, 0.0, 1.0, xtol=1e-16, rtol=8.9e-16)
    return {"break_even": p, "preferred_above": 1 if f(1.0) > 0 else 2, "expected_value_at_break_even": ev1(p)}


def _rbe(rng):
    while True:
        a, b, c, d = (round(rng.uniform(-100, 300), 1) for _ in range(4))
        den = (a - b) - (c - d)
        if den != 0 and 0.05 < (d - b) / den < 0.95:
            return {"payoffs": [[a, b], [c, d]]}


_cap("break_even_probability", level="intermediate", name="Break-even (indifference) probability of two actions",
     topic=_TDA,
     purpose="The probability of state 1 at which two actions have equal expected value, and which action is "
             "preferred above it.",
     inputs=[_Mat("payoffs", "2 x 2 payoff matrix [[action 1 in state 1, in state 2], [action 2 ...]]")],
     outputs=[_Out("break_even", "probability of state 1 at which the expected values are equal"),
              _Out("preferred_above", "action preferred when P(state 1) exceeds it"),
              _Out("expected_value_at_break_even", "the common expected value there")],
     primary="break_even", conditions=["a crossing strictly between 0 and 1"],
     method="intersection of the two expected-value lines", run=run_break_even, reference=ref_break_even,
     latex=r"p^* = \frac{u_{22} - u_{12}}{(u_{11} - u_{12}) - (u_{21} - u_{22})}",
     wrong=lambda payoffs: dict(run_break_even(payoffs), break_even=1 - run_break_even(payoffs)["break_even"])
     if abs(run_break_even(payoffs)["break_even"] - 0.5) > 1e-3 else dict(run_break_even(payoffs), break_even=0.25),
     rnd=_rbe, need=[r"break[- ]even|indifference probability", r"payoff"], aliases=_DA_AL,
     templates=["Break-even probability for payoffs = [[100, -20], [40, 30]].",
                "At what probability of state 1 is a decision maker indifferent? Indifference probability with payoff "
                "matrix [[50, 0], [20, 10]]."],
     examples=[_X({"payoffs": "[[100, -20], [40, 30]]"}, {"break_even": 50 / 110, "preferred_above": 1}),
               _X({"payoffs": [[50, 0], [20, 10]]}, {"break_even": 0.25})],
     questions=[_Q("Break-even probability for payoffs = [[100, -20], [40, 30]].", {"break_even": 50 / 110}),
                _Q("Break-even probability for payoffs = [[100, 50], [40, 30]].", outcome="INVALID_INPUT")])


def run_threshold(loss_fp, loss_fn, probability=None):
    a, b = _pos(loss_fp, "the loss of a false positive"), _pos(loss_fn, "the loss of a false negative")
    t = a / (a + b)
    d = None
    if probability is not None:
        p = _prob(probability, "the probability of the condition")
        d = "treat" if p > t else ("do not treat" if p < t else "indifferent")
    return {"threshold": t, "decision": d}


def ref_threshold(loss_fp, loss_fn, probability=None):
    """brentq on the difference of expected losses of treating and not treating."""
    from scipy.optimize import brentq
    f = lambda p: (1 - p) * loss_fp - p * loss_fn        # noqa: E731
    t = brentq(f, 0.0, 1.0, xtol=1e-16, rtol=8.9e-16)
    out = {"threshold": t}
    if probability is not None:
        out["decision"] = "treat" if f(probability) < 0 else ("do not treat" if f(probability) > 0 else "indifferent")
    return out


_cap("treatment_threshold", level="beginner", name="Decision threshold probability (treat / do not treat)",
     topic=_TDA,
     purpose="The probability above which acting (treating) has lower expected loss than not acting, from the loss "
             "of a false positive and of a false negative; the decision for a given probability.",
     inputs=[_Num("loss_fp", "loss of acting when the condition is absent (false positive)"),
             _Num("loss_fn", "loss of not acting when the condition is present (false negative)"),
             _Num("probability", "probability of the condition (optional)", required=False)],
     outputs=[_Out("threshold", "loss_fp / (loss_fp + loss_fn)"), _Out("decision", "treat / do not treat (if p given)")],
     primary="threshold", conditions=["losses > 0"], method="equal expected losses", run=run_threshold,
     reference=ref_threshold, latex=r"p^* = \frac{L_{FP}}{L_{FP} + L_{FN}}",
     wrong=lambda loss_fp, loss_fn, probability=None: dict(run_threshold(loss_fp, loss_fn, probability),
                                                          threshold=loss_fn / (loss_fp + loss_fn)),
     rnd=lambda rng: {"loss_fp": round(rng.uniform(1, 100), 2), "loss_fn": round(rng.uniform(1, 100), 2),
                      "probability": _rp(rng, 0.01, 0.99, 3)},
     need=[r"threshold|treat", r"false[- ]positive|loss_fp", r"false[- ]negative|loss_fn"], aliases=_DA_AL,
     templates=["Treatment threshold with loss_fp = 1 and loss_fn = 9; the probability of disease is 0.15. Treat?",
                "Decision threshold when a false-positive loss is 20 and a false-negative loss is 80."],
     examples=[_X({"loss_fp": 1, "loss_fn": 9, "probability": 0.15}, {"threshold": 0.1, "decision": "treat"}),
               _X({"loss_fp": 20, "loss_fn": 80}, {"threshold": 0.2, "decision": None})],
     questions=[_Q("Treatment threshold with loss_fp = 1 and loss_fn = 9; the probability of disease is 0.05. Treat?",
                   {"threshold": 0.1, "decision": "do not treat"}),
                _Q("Decision threshold when a false-positive loss is 20 and a false-negative loss is 80.",
                   {"threshold": 0.2}),
                _Q("Treatment threshold with loss_fp = -1 and loss_fn = 9.", outcome="INVALID_INPUT")])


# ============================================================================ uncertainty propagation (GUM) ====
_TUP = "uncertainty propagation"
_UP_AL = {"values": r"values|measurements|x", "uncertainties": r"uncertainties|standard uncertainties|components|u",
          "coefficients": r"coefficients|c", "a": r"a", "ua": r"ua|u_a|u\(a\)", "b": r"b", "ub": r"ub|u_b|u\(b\)",
          "x": r"x", "ux": r"ux|u_x|u\(x\)", "exponent": r"exponent|power|n", "function": r"function|f",
          "coefficient": r"coefficient|k", "exponents": r"exponents|powers", "rho": r"rho|correlation|r",
          "correlation": r"correlation(?: matrix)?", "matrix": r"(?:sensitivity |transformation |jacobian )matrix|(?<!covariance )(?<!correlation )matrix|a",
          "covariance": r"covariance(?: matrix)?", "dof": r"degrees of freedom|dof|nu", "half_width": r"half[- ]width|a",
          "distribution": r"distribution", "mu": r"mu|mean", "sigma": r"sigma|sd|standard deviation",
          "mean_x": r"mean_x|mux", "sd_x": r"sd_x|sigmax", "mean_y": r"mean_y|muy", "sd_y": r"sd_y|sigmay"}


def _uf():
    return __import__("uncertainties")


def run_u_sum(values, uncertainties, coefficients=None):
    x = _vec(values, "the values")
    u = _vec(uncertainties, "the uncertainties", len(x))
    c = [1.0] * len(x) if coefficients is None else _vec(coefficients, "the coefficients", len(x))
    if any(v < 0 for v in u):
        raise ValueError("standard uncertainties must not be negative")
    return {"value": math.fsum(a * b for a, b in zip(c, x)),
            "uncertainty": math.sqrt(math.fsum((a * b) ** 2 for a, b in zip(c, u)))}


def ref_u_sum(values, uncertainties, coefficients=None):
    unc = _uf()
    c = coefficients or [1.0] * len(values)
    y = sum(k * unc.ufloat(v, s) for k, v, s in zip(c, values, uncertainties))
    return {"value": float(y.nominal_value), "uncertainty": float(y.std_dev)}


_cap("uncertainty_sum", level="beginner", name="Uncertainty of a sum or linear combination (independent inputs)",
     topic=_TUP,
     purpose="Value and combined standard uncertainty of y = sum c_i x_i for independent inputs (default c_i = 1; use "
             "-1 for a difference).",
     inputs=[_Vec("values", "input values x_i"), _Vec("uncertainties", "standard uncertainties u_i (>= 0)"),
             _Vec("coefficients", "coefficients c_i (default all 1)", required=False)],
     outputs=[_Out("value", "y = sum c_i x_i"), _Out("uncertainty", "u_y = sqrt(sum (c_i u_i)^2)")],
     primary="uncertainty", conditions=["independent inputs", "u_i >= 0"], method="root-sum-of-squares (GUM, first order)",
     run=run_u_sum, reference=ref_u_sum, latex=r"u_y = \sqrt{\sum_i c_i^2 u_i^2}",
     wrong=lambda values, uncertainties, coefficients=None: dict(
         run_u_sum(values, uncertainties, coefficients),
         uncertainty=sum(abs(a * b) for a, b in zip(coefficients or [1.0] * len(values), uncertainties))),
     rnd=lambda rng: (lambda n: {"values": [round(rng.uniform(-100, 100), 3) for _ in range(n)],
                                 "uncertainties": [round(rng.uniform(0.01, 5), 3) for _ in range(n)],
                                 "coefficients": [rng.choice([1.0, -1.0, round(rng.uniform(-3, 3), 2)]) for _ in range(n)]})(
         rng.randint(2, 6)),
     need=[r"uncertaint", r"\bsum\b|linear combination|difference|add"],
     avoid=[r"correlation|covariance|degrees of freedom|weighted mean|product|quotient"], aliases=_UP_AL,
     templates=["Uncertainty of a sum: values = [12.3, 4.5, 7.1], uncertainties = [0.2, 0.1, 0.3].",
                "Linear combination with values [10, 4], uncertainties [0.3, 0.4] and coefficients [1, -1]: "
                "uncertainty of the difference?"],
     examples=[_X({"values": "10, 4", "uncertainties": "0.3, 0.4", "coefficients": "1, -1"},
                  {"value": 6.0, "uncertainty": 0.5}),
               _X({"values": [12.3, 4.5, 7.1], "uncertainties": [0.2, 0.1, 0.3]}, {"value": 23.9})],
     questions=[_Q("Uncertainty of a sum: values = [12.3, 4.5, 7.1], uncertainties = [0.2, 0.1, 0.3].",
                   {"uncertainty": math.sqrt(0.14)}),
                _Q("Linear combination with values [10, 4], uncertainties [0.3, 0.4] and coefficients [1, -1]: "
                   "uncertainty of the difference?", {"value": 6.0, "uncertainty": 0.5}),
                _Q("Uncertainty of a sum: values = [12.3, 4.5], uncertainties = [0.2, -0.1].", outcome="INVALID_INPUT")])


def run_u_prodquot(a, ua, b, ub, rho=0.0):
    a, b = _fin(a, "a"), _fin(b, "b")
    ua, ub = _nonneg(ua, "u(a)"), _nonneg(ub, "u(b)")
    r = _fin(0.0 if rho is None else rho, "the correlation")
    if not -1 <= r <= 1:
        raise ValueError("a correlation coefficient lies between -1 and 1")
    if b == 0:
        raise ValueError("b must not be 0 (the quotient a/b)")
    up = math.sqrt(max(0.0, (b * ua) ** 2 + (a * ub) ** 2 + 2 * a * b * r * ua * ub))
    uq = math.sqrt(max(0.0, (ua / b) ** 2 + (a * ub / b ** 2) ** 2 - 2 * (a / b ** 3) * r * ua * ub))
    return {"product": a * b, "u_product": up, "quotient": a / b, "u_quotient": uq}


def ref_u_prodquot(a, ua, b, ub, rho=0.0):
    unc = _uf()
    r = rho or 0.0
    x, y = unc.correlated_values([a, b], [[ua * ua, r * ua * ub], [r * ua * ub, ub * ub]]) if r else (
        unc.ufloat(a, ua), unc.ufloat(b, ub))
    return {"product": float((x * y).nominal_value), "u_product": float((x * y).std_dev),
            "quotient": float((x / y).nominal_value), "u_quotient": float((x / y).std_dev)}


def _rab_u(rng, corr=False):
    a, b = round(rng.uniform(0.5, 100), 3) * rng.choice([1, -1]), round(rng.uniform(0.5, 100), 3) * rng.choice([1, -1])
    d = {"a": a, "ua": round(abs(a) * rng.uniform(0.005, 0.1), 4), "b": b, "ub": round(abs(b) * rng.uniform(0.005, 0.1), 4)}
    if corr:
        d["rho"] = round(rng.uniform(-0.9, 0.9), 3)
    return d


_cap("uncertainty_product_quotient", level="beginner", name="Uncertainty of a product and a quotient", topic=_TUP,
     purpose="Value and first-order standard uncertainty of a*b and a/b for independent a and b.",
     inputs=[_Num("a", "value of a"), _Num("ua", "standard uncertainty of a (>= 0)"), _Num("b", "value of b (non-zero)"),
             _Num("ub", "standard uncertainty of b (>= 0)")],
     outputs=[_Out("product", "a*b"), _Out("u_product", "u(a*b)"), _Out("quotient", "a/b"), _Out("u_quotient", "u(a/b)")],
     primary="u_product", conditions=["independent a and b", "b != 0"],
     method="first-order propagation (relative uncertainties add in quadrature)",
     run=lambda a, ua, b, ub: run_u_prodquot(a, ua, b, ub, 0.0),
     reference=lambda a, ua, b, ub: ref_u_prodquot(a, ua, b, ub, 0.0),
     latex=r"\frac{u_{ab}}{|ab|} = \frac{u_{a/b}}{|a/b|} = \sqrt{\left(\frac{u_a}{a}\right)^2 + \left(\frac{u_b}{b}\right)^2}",
     wrong=lambda a, ua, b, ub: dict(run_u_prodquot(a, ua, b, ub), u_product=ua * abs(b) + ub * abs(a),
                                     u_quotient=ua / abs(b) + abs(a) * ub / b ** 2),
     rnd=lambda rng: _rab_u(rng), need=[r"uncertaint", r"product|quotient|ratio"],
     avoid=[r"correlat|rho|power law|exponents|normal"], aliases=_UP_AL,
     templates=["Uncertainty of the product and quotient: a = 12.0, ua = 0.3, b = 4.0, ub = 0.2.",
                "Find the uncertainty of the product of a = 5 with u(a) = 0.1 and b = 2 with u(b) = 0.05."],
     examples=[_X({"a": 12.0, "ua": 0.3, "b": 4.0, "ub": 0.2},
                  {"product": 48.0, "u_product": math.sqrt(1.2 ** 2 + 2.4 ** 2), "quotient": 3.0}),
               _X({"a": 5, "ua": 0.1, "b": 2, "ub": 0.05}, {"u_product": math.sqrt(0.04 + 0.0625)})],
     questions=[_Q("Uncertainty of the product and quotient: a = 12.0, ua = 0.3, b = 4.0, ub = 0.2.",
                   {"u_product": math.sqrt(1.2 ** 2 + 2.4 ** 2)}),
                _Q("Find the uncertainty of the product of a = 5 with u(a) = 0.1 and b = 2 with u(b) = 0.05.",
                   {"product": 10.0}),
                _Q("Uncertainty of the product and quotient: a = 12.0, ua = 0.3, b = 0, ub = 0.2.",
                   outcome="INVALID_INPUT")])

_cap("uncertainty_product_correlated", level="advanced", name="Uncertainty of a product and quotient with correlation",
     topic=_TUP,
     purpose="First-order standard uncertainty of a*b and a/b when a and b are correlated with coefficient rho.",
     inputs=[_Num("a", "value of a"), _Num("ua", "standard uncertainty of a"), _Num("b", "value of b (non-zero)"),
             _Num("ub", "standard uncertainty of b"), _Num("rho", "correlation coefficient of a and b (-1 to 1)")],
     outputs=[_Out("product", "a*b"), _Out("u_product", "u(a*b)"), _Out("quotient", "a/b"), _Out("u_quotient", "u(a/b)")],
     primary="u_product", conditions=["-1 <= rho <= 1", "b != 0"],
     method="first-order propagation with the covariance term", run=run_u_prodquot, reference=ref_u_prodquot,
     latex=r"u_{ab}^2 = b^2u_a^2 + a^2u_b^2 + 2ab\,\rho\,u_au_b",
     wrong=lambda a, ua, b, ub, rho: dict(run_u_prodquot(a, ua, b, ub, rho), **{
         k: v for k, v in run_u_prodquot(a, ua, b, ub, 0.0).items() if k.startswith("u_")}),
     rnd=lambda rng: _rab_u(rng, True), need=[r"uncertaint", r"product|quotient", r"correlat|rho"],
     aliases=_UP_AL,
     templates=["Uncertainty of the product with correlation: a = 12.0, ua = 0.3, b = 4.0, ub = 0.2, rho = 0.5.",
                "Correlated inputs a = 5, u(a) = 0.1, b = 2, u(b) = 0.05 and correlation 0.8: uncertainty of the product "
                "and quotient?"],
     examples=[_X({"a": 1, "ua": 0.1, "b": 1, "ub": 0.1, "rho": 1}, {"u_product": 0.2, "u_quotient": 0.0}),
               _X({"a": 12.0, "ua": 0.3, "b": 4.0, "ub": 0.2, "rho": 0.5}, {})],
     questions=[_Q("Uncertainty of the product with correlation: a = 1, ua = 0.1, b = 1, ub = 0.1, rho = -1.",
                   {"u_product": 0.0, "u_quotient": 0.2}),
                _Q("Uncertainty of the product with correlation: a = 1, ua = 0.1, b = 1, ub = 0.1, rho = 1.5.",
                   outcome="INVALID_INPUT")])


def run_u_power(x, ux, exponent):
    x, u, n = _fin(x, "x"), _nonneg(ux, "u(x)"), _fin(exponent, "the exponent")
    if x == 0 and n < 1:
        raise ValueError("x must not be 0 for an exponent below 1")
    if x < 0 and n != int(n):
        raise ValueError("a negative x needs a whole-number exponent")
    y = x ** n
    uy = abs(n * x ** (n - 1)) * u
    return {"value": y, "uncertainty": uy, "relative_uncertainty": uy / abs(y) if y != 0 else None}


_cap("uncertainty_power", level="beginner", name="Uncertainty of a power x^n", topic=_TUP,
     purpose="Value, standard uncertainty and relative uncertainty of y = x^n (first order).",
     inputs=[_Num("x", "value of x"), _Num("ux", "standard uncertainty of x (>= 0)"), _Num("exponent", "exponent n")],
     outputs=[_Out("value", "x^n"), _Out("uncertainty", "|n x^(n-1)| u(x)"), _Out("relative_uncertainty", "|n| u(x)/|x|")],
     primary="uncertainty", conditions=["x > 0 for a non-integer exponent", "x != 0 for n < 1"],
     method="first-order propagation", run=run_u_power,
     reference=lambda x, ux, exponent: (lambda y: {"value": float(y.nominal_value), "uncertainty": float(y.std_dev),
                                                   "relative_uncertainty": float(y.std_dev / abs(y.nominal_value))})(
         _uf().ufloat(x, ux) ** exponent),
     latex=r"\frac{u_y}{|y|} = |n|\,\frac{u_x}{|x|}",
     wrong=lambda x, ux, exponent: dict(run_u_power(x, ux, exponent), uncertainty=abs(exponent) * ux),
     rnd=lambda rng: (lambda x: {"x": x, "ux": round(x * rng.uniform(0.005, 0.1), 4),
                                 "exponent": rng.choice([2, 3, -1, 0.5, -2, round(rng.uniform(-3, 4), 2)])})(
         round(rng.uniform(1.5, 50), 3)),
     need=[r"uncertaint", r"power|squared|cubed|exponent|\^"], avoid=[r"exponents|power law|product of powers"],
     aliases=_UP_AL,
     templates=["Uncertainty of a power: x = 3.0, ux = 0.1, exponent = 2.",
                "Find the uncertainty of x^n with x = 16, u(x) = 0.4 and n = 0.5."],
     examples=[_X({"x": 3.0, "ux": 0.1, "exponent": 2}, {"value": 9.0, "uncertainty": 0.6}),
               _X({"x": 16, "ux": 0.4, "exponent": 0.5}, {"value": 4.0, "uncertainty": 0.05})],
     questions=[_Q("Uncertainty of a power: x = 3.0, ux = 0.1, exponent = 2.", {"uncertainty": 0.6}),
                _Q("Find the uncertainty of x^n with x = 16, u(x) = 0.4 and n = 0.5.", {"uncertainty": 0.05}),
                _Q("Uncertainty of a power: x = -3.0, ux = 0.1, exponent = 0.5.", outcome="INVALID_INPUT")])


_FUNS = ["exp", "ln", "log10", "sqrt", "sin", "cos", "tan", "reciprocal"]


def run_u_function(x, ux, function):
    x, u = _fin(x, "x"), _nonneg(ux, "u(x)")
    if function in ("ln", "log10", "sqrt") and x <= 0:
        raise ValueError(f"{function} needs x > 0")
    if function == "reciprocal" and x == 0:
        raise ValueError("1/x needs x != 0")
    if function == "tan" and abs(math.cos(x)) < 1e-12:
        raise ValueError("tan is undefined at this x")
    if function not in _FUNS:
        raise ValueError("function must be one of " + ", ".join(_FUNS))
    f, d = {"exp": lambda: (math.exp(x), math.exp(x)), "ln": lambda: (math.log(x), 1 / x),
            "log10": lambda: (math.log10(x), 1 / (x * math.log(10))), "sqrt": lambda: (math.sqrt(x), 0.5 / math.sqrt(x)),
            "sin": lambda: (math.sin(x), math.cos(x)), "cos": lambda: (math.cos(x), -math.sin(x)),
            "tan": lambda: (math.tan(x), 1 / math.cos(x) ** 2), "reciprocal": lambda: (1 / x, -1 / x ** 2)}[function]()
    return {"value": f, "uncertainty": abs(d) * u}


def ref_u_function(x, ux, function):
    unc = _uf()
    from uncertainties import umath
    v = unc.ufloat(x, ux)
    y = {"exp": umath.exp, "ln": umath.log, "log10": umath.log10, "sqrt": umath.sqrt, "sin": umath.sin, "cos": umath.cos,
         "tan": umath.tan, "reciprocal": lambda z: 1 / z}[function](v)
    return {"value": float(y.nominal_value), "uncertainty": float(y.std_dev)}


def _rfun(rng):
    f = rng.choice(_FUNS)
    if f in ("sin", "cos", "tan"):
        x = round(rng.uniform(0.1, 1.4), 3)
    elif f == "exp":
        x = round(rng.uniform(-5, 5), 3)
    else:
        x = round(rng.uniform(0.2, 100), 3)
    return {"x": x, "ux": round(abs(x) * rng.uniform(0.005, 0.1) + 0.001, 4), "function": f}


_cap("uncertainty_function", level="intermediate", name="Uncertainty of a function of one variable", topic=_TUP,
     purpose="Value and first-order standard uncertainty of f(x) for exp, ln, log10, sqrt, sin, cos, tan (x in "
             "radians) or 1/x.",
     inputs=[_Num("x", "value of x (radians for sin, cos, tan)"), _Num("ux", "standard uncertainty of x"),
             _Ch("function", "exp, ln, log10, sqrt, sin, cos, tan or reciprocal", _FUNS)],
     outputs=[_Out("value", "f(x)"), _Out("uncertainty", "|f'(x)| u(x)")],
     primary="uncertainty", conditions=["x inside the function's domain"], method="first-order propagation u_f = |f'(x)| u_x",
     run=run_u_function, reference=ref_u_function, latex=r"u_f = \left|f'(x)\right| u_x",
     wrong=lambda x, ux, function: (lambda r: dict(r, uncertainty=abs(r["value"]) * ux / abs(x)))(
         run_u_function(x, ux, function)),
     rnd=_rfun, need=[r"uncertaint", r"function\s*(?:=|:)|\b(?:exp|ln|log10|sqrt|sin|cos|tan|reciprocal)\b"],
     avoid=[r"product|quotient|sum\b|power|exponent|correlat|normal"], aliases=_UP_AL,
     extract=lambda q, d: d.setdefault("function", next(f for f in _FUNS if re.search(r"\b" + f + r"\b", q)))
     if "function" not in d and any(re.search(r"\b" + f + r"\b", q) for f in _FUNS) else None,
     templates=["Uncertainty of a function: function = ln, x = 2.5, ux = 0.05.",
                "Find the uncertainty of sin(x) with x = 0.5 and u(x) = 0.01 (function = sin)."],
     examples=[_X({"x": 2.5, "ux": 0.05, "function": "ln"}, {"value": math.log(2.5), "uncertainty": 0.02}),
               _X({"x": 0.0, "ux": 0.01, "function": "exp"}, {"value": 1.0, "uncertainty": 0.01})],
     questions=[_Q("Uncertainty of a function: function = ln, x = 2.5, ux = 0.05.", {"uncertainty": 0.02}),
                _Q("Uncertainty of a function: function = sqrt, x = 4, ux = 0.2.", {"value": 2.0, "uncertainty": 0.05}),
                _Q("Uncertainty of a function: function = ln, x = -2.5, ux = 0.05.", outcome="INVALID_INPUT")])


def run_u_powerlaw(coefficient, values, exponents, uncertainties):
    k = _fin(coefficient, "the coefficient")
    x = _vec(values, "the values")
    p = _vec(exponents, "the exponents", len(x))
    u = _vec(uncertainties, "the uncertainties", len(x))
    if any(v <= 0 for v in x):
        raise ValueError("a power law needs positive values")
    if any(v < 0 for v in u):
        raise ValueError("uncertainties must not be negative")
    y = k * math.prod(a ** b for a, b in zip(x, p))
    rel = math.sqrt(math.fsum((b * s / a) ** 2 for a, b, s in zip(x, p, u)))
    return {"value": y, "uncertainty": abs(y) * rel, "relative_uncertainty": rel}


def ref_u_powerlaw(coefficient, values, exponents, uncertainties):
    unc = _uf()
    y = coefficient
    for v, e, s in zip(values, exponents, uncertainties):
        y = y * unc.ufloat(v, s) ** e
    return {"value": float(y.nominal_value), "uncertainty": float(y.std_dev),
            "relative_uncertainty": float(y.std_dev / abs(y.nominal_value))}


_cap("uncertainty_power_law", level="intermediate", name="Uncertainty of a power-law product y = k x1^p1 x2^p2 ...",
     topic=_TUP,
     purpose="Value and first-order uncertainty of a product of powers of independent positive inputs (relative "
             "uncertainties weighted by the exponents add in quadrature).",
     inputs=[_Num("coefficient", "exact constant k"), _Vec("values", "positive input values x_i"),
             _Vec("exponents", "exponents p_i"), _Vec("uncertainties", "standard uncertainties u_i")],
     outputs=[_Out("value", "y"), _Out("uncertainty", "u_y"), _Out("relative_uncertainty", "u_y/|y|")],
     primary="uncertainty", conditions=["x_i > 0", "independent inputs"], method="first-order propagation in relative form",
     run=run_u_powerlaw, reference=ref_u_powerlaw,
     latex=r"\frac{u_y}{|y|} = \sqrt{\sum_i \left(p_i\,\frac{u_i}{x_i}\right)^2}",
     wrong=lambda coefficient, values, exponents, uncertainties: (lambda r: (lambda rel: dict(
         r, relative_uncertainty=rel, uncertainty=abs(r["value"]) * rel))(
         sum(abs(b * s / a) for a, b, s in zip(values, exponents, uncertainties))))(
         run_u_powerlaw(coefficient, values, exponents, uncertainties)),
     rnd=lambda rng: (lambda n: (lambda x: {"coefficient": round(rng.uniform(0.1, 10), 3), "values": x,
                                            "exponents": [rng.choice([1, 2, -1, 0.5, -2, 3]) for _ in range(n)],
                                            "uncertainties": [round(v * rng.uniform(0.005, 0.08), 4) for v in x]})(
         [round(rng.uniform(0.5, 50), 3) for _ in range(n)]))(rng.randint(2, 4)),
     need=[r"uncertaint", r"power law|exponents"], aliases=_UP_AL,
     templates=["Uncertainty of a power law: coefficient = 0.5, values = [2.0, 3.0], exponents = [1, 2], "
                "uncertainties = [0.02, 0.06].",
                "Power-law product with coefficient 1, values [10, 4], exponents [2, -1] and uncertainties [0.1, 0.08]."],
     examples=[_X({"coefficient": 0.5, "values": "2, 3", "exponents": "1, 2", "uncertainties": "0.02, 0.06"},
                  {"value": 9.0, "relative_uncertainty": math.sqrt(0.01 ** 2 + 0.04 ** 2)}),
               _X({"coefficient": 1, "values": [10, 4], "exponents": [2, -1], "uncertainties": [0.1, 0.08]}, {"value": 25.0})],
     questions=[_Q("Uncertainty of a power law: coefficient = 0.5, values = [2.0, 3.0], exponents = [1, 2], "
                   "uncertainties = [0.02, 0.06].", {"value": 9.0}),
                _Q("Uncertainty of a power law: coefficient = 1, values = [-2.0, 3.0], exponents = [1, 2], "
                   "uncertainties = [0.02, 0.06].", outcome="INVALID_INPUT")])


def run_wmean(values, uncertainties):
    x = _vec(values, "the measurements", min_len=2)
    u = _vec(uncertainties, "the uncertainties", len(x))
    if any(v <= 0 for v in u):
        raise ValueError("uncertainties must be positive")
    w = [1 / v ** 2 for v in u]
    sw = math.fsum(w)
    m = math.fsum(a * b for a, b in zip(w, x)) / sw
    chi2 = math.fsum(a * (b - m) ** 2 for a, b in zip(w, x))
    return {"weighted_mean": m, "uncertainty": 1 / math.sqrt(sw), "chi_square": chi2,
            "birge_ratio": math.sqrt(chi2 / (len(x) - 1))}


def ref_wmean(values, uncertainties):
    """numpy.average with inverse-variance weights; the uncertainty propagated by the uncertainties package."""
    import numpy as np
    unc = _uf()
    w = 1 / np.array(uncertainties, float) ** 2
    m = float(np.average(values, weights=w))
    y = sum(wi * unc.ufloat(v, s) for wi, v, s in zip(w, values, uncertainties)) / w.sum()
    chi2 = float(np.sum(w * (np.array(values) - m) ** 2))
    return {"weighted_mean": m, "uncertainty": float(y.std_dev), "chi_square": chi2,
            "birge_ratio": math.sqrt(chi2 / (len(values) - 1))}


_cap("inverse_variance_weighted_mean", level="intermediate",
     name="Inverse-variance weighted mean of measurements", topic=_TUP,
     purpose="Combine measurements of one quantity with different standard uncertainties: weighted mean, its "
             "uncertainty, chi-square of consistency and Birge ratio (equal to the posterior under a flat prior).",
     inputs=[_Vec("values", "measured values", 2), _Vec("uncertainties", "their standard uncertainties (> 0)", 2)],
     outputs=[_Out("weighted_mean", "sum(x_i/u_i^2)/sum(1/u_i^2)"), _Out("uncertainty", "1/sqrt(sum 1/u_i^2)"),
              _Out("chi_square", "sum((x_i - mean)^2/u_i^2)"), _Out("birge_ratio", "sqrt(chi^2/(n-1))")],
     primary="weighted_mean", conditions=["u_i > 0", "n >= 2"], method="inverse-variance weighting", run=run_wmean,
     reference=ref_wmean, latex=r"\bar x_w = \frac{\sum x_i/u_i^2}{\sum 1/u_i^2},\quad u(\bar x_w) = \Big(\sum 1/u_i^2\Big)^{-1/2}",
     wrong=lambda values, uncertainties: dict(run_wmean(values, uncertainties), weighted_mean=sum(
         v / s for v, s in zip(values, uncertainties)) / sum(1 / s for s in uncertainties)),
     rnd=lambda rng: (lambda n, mu: {"values": [round(mu + rng.gauss(0, 1), 3) for _ in range(n)],
                                     "uncertainties": [round(rng.uniform(0.2, 3), 3) for _ in range(n)]})(
         rng.randint(2, 8), rng.uniform(-50, 50)),
     need=[r"weighted mean|inverse[- ]variance|combine (?:the )?measurements"], aliases=_UP_AL,
     templates=["Inverse-variance weighted mean of values = [10.2, 9.8, 10.5] with uncertainties = [0.2, 0.1, 0.4].",
                "Combine the measurements [5.1, 4.9] with uncertainties [0.1, 0.1]: weighted mean?"],
     examples=[_X({"values": "5.1, 4.9", "uncertainties": "0.1, 0.1"},
                  {"weighted_mean": 5.0, "uncertainty": 0.1 / math.sqrt(2), "chi_square": 2.0}),
               _X({"values": [10.2, 9.8, 10.5], "uncertainties": [0.2, 0.1, 0.4]}, {})],
     questions=[_Q("Combine the measurements [5.1, 4.9] with uncertainties [0.1, 0.1]: weighted mean?",
                   {"weighted_mean": 5.0}),
                _Q("Inverse-variance weighted mean of values = [10.2, 9.8] with uncertainties = [0.2, 0].",
                   outcome="INVALID_INPUT")])


def run_u_lincorr(values, uncertainties, correlation, coefficients=None):
    x = _vec(values, "the values")
    u = _vec(uncertainties, "the uncertainties", len(x))
    c = [1.0] * len(x) if coefficients is None else _vec(coefficients, "the coefficients", len(x))
    R = _mat(correlation, "the correlation matrix", len(x), len(x))
    if any(v < 0 for v in u):
        raise ValueError("uncertainties must not be negative")
    if any(abs(R[i][i] - 1) > 1e-12 for i in range(len(x))) or any(abs(v) > 1 for r in R for v in r):
        raise ValueError("a correlation matrix has ones on the diagonal and entries between -1 and 1")
    _cholesky_ok(R, "the correlation matrix")
    var = math.fsum(c[i] * c[j] * u[i] * u[j] * R[i][j] for i in range(len(x)) for j in range(len(x)))
    return {"value": math.fsum(a * b for a, b in zip(c, x)), "uncertainty": math.sqrt(max(0.0, var))}


def ref_u_lincorr(values, uncertainties, correlation, coefficients=None):
    import numpy as np
    unc = _uf()
    u = np.array(uncertainties, float)
    cov = np.array(correlation, float) * np.outer(u, u)
    xs = unc.correlated_values(values, cov)
    c = coefficients or [1.0] * len(values)
    y = sum(k * v for k, v in zip(c, xs))
    return {"value": float(y.nominal_value), "uncertainty": float(y.std_dev)}


def _rcorr(rng, n):
    A = [[rng.gauss(0, 1) for _ in range(n + 1)] for _ in range(n)]
    S = [[sum(A[i][k] * A[j][k] for k in range(n + 1)) for j in range(n)] for i in range(n)]
    d = [math.sqrt(S[i][i]) for i in range(n)]
    R = [[round(S[i][j] / (d[i] * d[j]), 3) if i != j else 1.0 for j in range(n)] for i in range(n)]
    for i in range(n):
        for j in range(i):
            R[i][j] = R[j][i]
    return R


_cap("uncertainty_linear_correlated", level="advanced",
     name="Uncertainty of a linear combination of correlated inputs", topic=_TUP,
     purpose="Combined standard uncertainty of y = sum c_i x_i when the inputs are correlated (correlation matrix), "
             "u_y^2 = sum_ij c_i c_j u_i u_j r_ij.",
     inputs=[_Vec("values", "input values"), _Vec("uncertainties", "standard uncertainties"),
             _Mat("correlation", "correlation matrix (ones on the diagonal, positive semi-definite)"),
             _Vec("coefficients", "coefficients c_i (default 1)", required=False)],
     outputs=[_Out("value", "y"), _Out("uncertainty", "u_y")],
     primary="uncertainty", conditions=["valid correlation matrix"], method="quadratic form c^T Sigma c (GUM eq. 13)",
     run=run_u_lincorr, reference=ref_u_lincorr, latex=r"u_y^2 = \sum_i\sum_j c_i c_j u_i u_j r_{ij}",
     wrong=lambda values, uncertainties, correlation, coefficients=None: dict(
         run_u_lincorr(values, uncertainties, correlation, coefficients),
         uncertainty=run_u_sum(values, uncertainties, coefficients)["uncertainty"]),
     rnd=lambda rng: (lambda n: {"values": [round(rng.uniform(-50, 50), 3) for _ in range(n)],
                                 "uncertainties": [round(rng.uniform(0.1, 3), 3) for _ in range(n)],
                                 "correlation": _rcorr(rng, n),
                                 "coefficients": [round(rng.uniform(-3, 3), 2) for _ in range(n)]})(rng.randint(2, 4)),
     need=[r"uncertaint", r"correlation matrix"], aliases=_UP_AL,
     templates=["Uncertainty of a linear combination with correlation matrix [[1, 0.5], [0.5, 1]], values = [10, 20], "
                "uncertainties = [0.3, 0.4], coefficients = [1, 1].",
                "Correlated inputs: values [1, 2, 3], uncertainties [0.1, 0.1, 0.1], correlation matrix "
                "[[1, 0, 0], [0, 1, -0.5], [0, -0.5, 1]]. Uncertainty of the sum?"],
     examples=[_X({"values": "10, 20", "uncertainties": "0.3, 0.4", "correlation": "[[1, 0.5], [0.5, 1]]",
                   "coefficients": "1, 1"}, {"value": 30.0, "uncertainty": math.sqrt(0.09 + 0.16 + 0.12)}),
               _X({"values": [1, 2], "uncertainties": [0.1, 0.1], "correlation": [[1, 1], [1, 1]],
                   "coefficients": [1, -1]}, {"uncertainty": 0.0})],
     questions=[_Q("Uncertainty of a linear combination with correlation matrix [[1, 0.5], [0.5, 1]], values = [10, 20], "
                   "uncertainties = [0.3, 0.4], coefficients = [1, 1].", {"uncertainty": math.sqrt(0.37)}),
                _Q("Uncertainty of a linear combination with correlation matrix [[1, 1.5], [1.5, 1]], values = [10, 20], "
                   "uncertainties = [0.3, 0.4].", outcome="INVALID_INPUT")])


def run_cov_prop(matrix, covariance):
    A = _mat(matrix, "the sensitivity matrix")
    S = _mat(covariance, "the covariance matrix", len(A[0]), len(A[0]))
    _cholesky_ok(S, "the covariance matrix")
    AS = [[math.fsum(A[i][k] * S[k][j] for k in range(len(S))) for j in range(len(S))] for i in range(len(A))]
    C = [[math.fsum(AS[i][k] * A[j][k] for k in range(len(S))) for j in range(len(A))] for i in range(len(A))]
    for i in range(len(A)):
        for j in range(i):
            C[i][j] = C[j][i] = 0.5 * (C[i][j] + C[j][i])
    return {"output_covariance": C, "uncertainties": [math.sqrt(max(0.0, C[i][i])) for i in range(len(A))]}


def ref_cov_prop(matrix, covariance):
    unc = _uf()
    xs = unc.correlated_values([0.0] * len(covariance), covariance)
    ys = [sum(a * x for a, x in zip(row, xs)) for row in matrix]
    C = unc.covariance_matrix(ys)
    return {"output_covariance": [[float(v) for v in r] for r in C], "uncertainties": [float(y.std_dev) for y in ys]}


def _rcov(rng):
    k, m = rng.randint(2, 4), rng.randint(1, 3)
    R = _rcorr(rng, k)
    s = [round(rng.uniform(0.2, 3), 3) for _ in range(k)]
    return {"matrix": [[round(rng.uniform(-3, 3), 2) for _ in range(k)] for _ in range(m)],
            "covariance": [[R[i][j] * s[i] * s[j] for j in range(k)] for i in range(k)]}


_cap("covariance_propagation_linear", level="advanced", name="Covariance propagation through a linear map y = A x",
     topic=_TUP,
     purpose="Covariance matrix of the outputs y = A x and their standard uncertainties, given the covariance matrix "
             "of x (first-order propagation with a sensitivity matrix).",
     inputs=[_Mat("matrix", "sensitivity (Jacobian) matrix A: one row per output"),
             _Mat("covariance", "covariance matrix of the inputs (symmetric, positive semi-definite)")],
     outputs=[_Out("output_covariance", "A Sigma A^T"), _Out("uncertainties", "square roots of its diagonal")],
     primary="uncertainties", conditions=["covariance symmetric positive semi-definite", "columns of A = inputs"],
     method="matrix product A Sigma A^T", run=run_cov_prop, reference=ref_cov_prop,
     latex=r"\Sigma_y = A\,\Sigma_x\,A^{\mathsf T}",
     wrong=lambda matrix, covariance: run_cov_prop(matrix, [[covariance[i][j] if i == j else 0.0
                                                             for j in range(len(covariance))]
                                                            for i in range(len(covariance))]),
     rnd=_rcov, need=[r"covariance", r"propagat|sensitivity matrix|jacobian|transformation matrix"], aliases=_UP_AL,
     templates=["Covariance propagation: matrix = [[1, 1], [1, -1]], covariance = [[0.04, 0.01], [0.01, 0.09]].",
                "Propagate the covariance matrix [[1, 0], [0, 4]] through the sensitivity matrix [[2, 1]]."],
     examples=[_X({"matrix": "[[1, 1], [1, -1]]", "covariance": "[[0.04, 0.01], [0.01, 0.09]]"},
                  {"output_covariance": [[0.15, -0.05], [-0.05, 0.11]], "uncertainties": [math.sqrt(0.15), math.sqrt(0.11)]}),
               _X({"matrix": [[2, 1]], "covariance": [[1, 0], [0, 4]]}, {"uncertainties": [math.sqrt(8)]})],
     questions=[_Q("Covariance propagation: matrix = [[1, 1], [1, -1]], covariance = [[0.04, 0.01], [0.01, 0.09]].",
                   {"uncertainties": [math.sqrt(0.15), math.sqrt(0.11)]}),
                _Q("Propagate the covariance matrix [[1, 2], [2, 1]] through the sensitivity matrix [[2, 1]].",
                   outcome="INVALID_INPUT")])


def run_ws(uncertainties, dof, level=None):
    u = _vec(uncertainties, "the uncertainty components")
    v = _vec(dof, "the degrees of freedom", len(u))
    c = _level(level)
    if any(x <= 0 for x in u) or any(x <= 0 for x in v):
        raise ValueError("uncertainty components and degrees of freedom must be positive")
    uc2 = math.fsum(x * x for x in u)
    nu = uc2 ** 2 / math.fsum(x ** 4 / n for x, n in zip(u, v))
    k = _t_ppf(0.5 * (1 + c), nu)
    return {"combined_uncertainty": math.sqrt(uc2), "effective_dof": nu, "coverage_factor": k,
            "expanded_uncertainty": k * math.sqrt(uc2)}


def ref_ws(uncertainties, dof, level=None):
    """Moment matching of sum u_i^2 chi2_{nu_i}/nu_i to a scaled chi-square (scipy chi2 moments); scipy t quantile."""
    from scipy.stats import chi2, t as T
    e = sum(u * u * chi2(n).mean() / n for u, n in zip(uncertainties, dof))
    var = sum(u ** 4 * chi2(n).var() / n ** 2 for u, n in zip(uncertainties, dof))
    nu = 2 * e * e / var
    k = T.ppf(0.5 * (1 + (0.95 if level is None else level)), nu)
    return {"combined_uncertainty": math.sqrt(e), "effective_dof": nu, "coverage_factor": float(k),
            "expanded_uncertainty": float(k * math.sqrt(e))}


_cap("welch_satterthwaite_expanded_uncertainty", level="advanced",
     name="Effective degrees of freedom and expanded uncertainty (Welch-Satterthwaite)", topic=_TUP,
     purpose="Combined standard uncertainty, Welch-Satterthwaite effective degrees of freedom, the Student-t coverage "
             "factor and the expanded uncertainty at a stated coverage probability.",
     inputs=[_Vec("uncertainties", "uncertainty components c_i u_i (> 0)"),
             _Vec("dof", "degrees of freedom of each component (> 0)"), LEVEL_IN],
     outputs=[_Out("combined_uncertainty", "u_c"), _Out("effective_dof", "nu_eff"),
              _Out("coverage_factor", "k = t_{(1+p)/2, nu_eff}"), _Out("expanded_uncertainty", "U = k u_c")],
     primary="expanded_uncertainty", conditions=["components and degrees of freedom > 0", "0 < level < 1"],
     method="Welch-Satterthwaite formula and the Student-t quantile at non-integer degrees of freedom", run=run_ws,
     reference=ref_ws, latex=r"\nu_{eff} = \frac{u_c^4}{\sum_i u_i^4/\nu_i},\quad U = t_{(1+p)/2,\,\nu_{eff}}\,u_c",
     wrong=lambda uncertainties, dof, level=None: (lambda r: (lambda k: dict(
         r, effective_dof=sum(dof), coverage_factor=k, expanded_uncertainty=k * r["combined_uncertainty"]))(
         _t_ppf(0.5 * (1 + (level or 0.95)), sum(dof))))(run_ws(uncertainties, dof, level)),
     rnd=lambda rng: (lambda n: {"uncertainties": [round(rng.uniform(0.05, 2), 3) for _ in range(n)],
                                 "dof": [float(rng.choice([2, 3, 4, 5, 9, 10, 19, 50, 100])) for _ in range(n)],
                                 "level": rng.choice([0.9, 0.95, 0.99, 0.6827])})(rng.randint(2, 5)),
     need=[r"welch|satterthwaite|effective degrees of freedom|expanded uncertainty"], aliases=_UP_AL,
     extract=_x_level,
     templates=["Welch-Satterthwaite: uncertainties = [0.3, 0.2], degrees of freedom = [4, 9]. Expanded uncertainty "
                "at 95% coverage?",
                "Expanded uncertainty with components [0.1, 0.1, 0.05], dof = [5, 10, 50] and level = 0.99."],
     examples=[_X({"uncertainties": "1", "dof": "10", "level": 0.95}, {"effective_dof": 10.0,
                                                                        "coverage_factor": 2.2281388519649385}),
               _X({"uncertainties": [0.3, 0.2], "dof": [4, 9]}, {"combined_uncertainty": math.sqrt(0.13)})],
     questions=[_Q("Welch-Satterthwaite: uncertainties = [1], degrees of freedom = [10]. Expanded uncertainty at 95% "
                   "coverage?", {"expanded_uncertainty": 2.2281388519649385}),
                _Q("Expanded uncertainty with components [0.1, 0.1], dof = [5, 0] and level = 0.95.",
                   outcome="INVALID_INPUT")])


_TYPEB = ["rectangular", "triangular", "arcsine"]


def run_type_b(half_width, distribution):
    a = _pos(half_width, "the half-width")
    if distribution not in _TYPEB:
        raise ValueError("distribution must be rectangular, triangular or arcsine")
    d = {"rectangular": math.sqrt(3), "triangular": math.sqrt(6), "arcsine": math.sqrt(2)}[distribution]
    return {"standard_uncertainty": a / d, "divisor": d}


def ref_type_b(half_width, distribution):
    from scipy import stats
    dist = {"rectangular": stats.uniform(loc=-half_width, scale=2 * half_width),
            "triangular": stats.triang(0.5, loc=-half_width, scale=2 * half_width),
            "arcsine": stats.arcsine(loc=-half_width, scale=2 * half_width)}[distribution]
    s = float(dist.std())
    return {"standard_uncertainty": s, "divisor": half_width / s}


_cap("type_b_standard_uncertainty", level="beginner", name="Type B standard uncertainty from a stated half-width",
     topic=_TUP,
     purpose="Standard uncertainty of a quantity known only to lie within +/- a, for a rectangular, triangular or "
             "U-shaped (arcsine) distribution.",
     inputs=[_Num("half_width", "half-width a of the interval (> 0)"),
             _Ch("distribution", "rectangular, triangular or arcsine (U-shaped)", _TYPEB)],
     outputs=[_Out("standard_uncertainty", "a / divisor"), _Out("divisor", "sqrt(3), sqrt(6) or sqrt(2)")],
     primary="standard_uncertainty", conditions=["a > 0"], method="standard deviation of the assumed distribution",
     run=run_type_b, reference=ref_type_b, latex=r"u = \frac{a}{\sqrt 3}\ (\text{rect.}),\ \frac{a}{\sqrt 6}\ (\text{tri.}),\ \frac{a}{\sqrt 2}\ (\text{arcsine})",
     wrong=lambda half_width, distribution: {"standard_uncertainty": half_width / 2, "divisor": 2.0},
     rnd=lambda rng: {"half_width": round(rng.uniform(0.001, 50), 4), "distribution": rng.choice(_TYPEB)},
     need=[r"type b|half[- ]width"], avoid=[r"monte carlo"], aliases=_UP_AL,
     extract=lambda q, d: d.setdefault("distribution", next(t for t in _TYPEB if t in q)) if any(t in q for t in _TYPEB)
     else (d.setdefault("distribution", "rectangular") if "uniform" in q else
           (d.setdefault("distribution", "arcsine") if "u-shaped" in q else None)),
     templates=["Type B uncertainty: half-width = 0.5 with a rectangular distribution.",
                "Standard uncertainty for a triangular distribution of half-width 0.3 (type B)."],
     examples=[_X({"half_width": 0.5, "distribution": "rectangular"}, {"standard_uncertainty": 0.5 / math.sqrt(3)}),
               _X({"half_width": 0.3, "distribution": "triangular"}, {"standard_uncertainty": 0.3 / math.sqrt(6)})],
     questions=[_Q("Type B uncertainty: half-width = 0.5 with a rectangular distribution.",
                   {"standard_uncertainty": 0.5 / math.sqrt(3)}),
                _Q("Type B uncertainty: half-width = 2 with a U-shaped distribution.",
                   {"standard_uncertainty": math.sqrt(2)}),
                _Q("Type B uncertainty: half-width = -0.5 with a rectangular distribution.", outcome="INVALID_INPUT")])


def run_lognormal(mu, sigma):
    m, s = _fin(mu, "mu"), _pos(sigma, "sigma")
    if m + s * s > 700:
        raise ValueError("the result overflows")
    return {"exact_mean": math.exp(m + s * s / 2), "exact_sd": math.sqrt(math.expm1(s * s)) * math.exp(m + s * s / 2),
            "first_order_mean": math.exp(m), "first_order_sd": math.exp(m) * s}


def ref_lognormal(mu, sigma):
    from scipy.stats import lognorm
    from uncertainties import umath, ufloat
    y = umath.exp(ufloat(mu, sigma))
    d = lognorm(s=sigma, scale=math.exp(mu))
    return {"exact_mean": float(d.mean()), "exact_sd": float(d.std()), "first_order_mean": float(y.nominal_value),
            "first_order_sd": float(y.std_dev)}


_cap("lognormal_propagation", level="intermediate", name="Exact versus first-order propagation through exp(X)",
     topic=_TUP,
     purpose="Mean and standard deviation of Y = exp(X) for normal X ~ N(mu, sigma^2), exactly (lognormal) and by "
             "first-order propagation, showing the bias of the linear approximation.",
     inputs=[_Num("mu", "mean of X"), _Num("sigma", "standard deviation of X (> 0)")],
     outputs=[_Out("exact_mean", "exp(mu + sigma^2/2)"), _Out("exact_sd", "sqrt(exp(sigma^2) - 1) exp(mu + sigma^2/2)"),
              _Out("first_order_mean", "exp(mu)"), _Out("first_order_sd", "exp(mu) sigma")],
     primary="exact_sd", conditions=["sigma > 0"], method="lognormal moments; first-order Taylor propagation",
     run=run_lognormal, reference=ref_lognormal,
     latex=r"E[e^X] = e^{\mu + \sigma^2/2},\quad \mathrm{SD}[e^X] = e^{\mu + \sigma^2/2}\sqrt{e^{\sigma^2} - 1}",
     wrong=lambda mu, sigma: dict(run_lognormal(mu, sigma), exact_mean=math.exp(mu), exact_sd=math.exp(mu) * sigma),
     rnd=lambda rng: {"mu": round(rng.uniform(-3, 5), 3), "sigma": round(rng.uniform(0.05, 1.5), 3)},
     need=[r"lognormal|exp\(x\)|e\^x|exponential of a normal"], aliases={"mu": r"mu|mean", "sigma": r"sigma|sd"},
     avoid=[r"utility"],
     templates=["Propagate through exp(X): mu = 1, sigma = 0.5. Exact and first-order mean and sd of the lognormal.",
                "Lognormal propagation with mu = 0 and sigma = 1."],
     examples=[_X({"mu": 0, "sigma": 1}, {"exact_mean": math.exp(0.5), "first_order_sd": 1.0}),
               _X({"mu": 1, "sigma": 0.5}, {"first_order_mean": math.e})],
     questions=[_Q("Lognormal propagation with mu = 0 and sigma = 1.", {"exact_mean": math.exp(0.5),
                                                                      "exact_sd": math.sqrt(math.e - 1) * math.exp(0.5)}),
                _Q("Lognormal propagation with mu = 0 and sigma = -1.", outcome="INVALID_INPUT")])


def run_prod_normals(mean_x, sd_x, mean_y, sd_y):
    mx, my = _fin(mean_x, "mean_x"), _fin(mean_y, "mean_y")
    sx, sy = _pos(sd_x, "sd_x"), _pos(sd_y, "sd_y")
    fo = (mx * sy) ** 2 + (my * sx) ** 2
    return {"exact_mean": mx * my, "exact_sd": math.sqrt(fo + (sx * sy) ** 2), "first_order_sd": math.sqrt(fo)}


def ref_prod_normals(mean_x, sd_x, mean_y, sd_y):
    """chaospy: exact moments of the product of two independent normal variables; uncertainties for first order."""
    import chaospy as cp
    unc = _uf()
    dist = cp.J(cp.Normal(mean_x, sd_x), cp.Normal(mean_y, sd_y))
    q0, q1 = cp.variable(2)
    return {"exact_mean": float(cp.E(q0 * q1, dist)), "exact_sd": float(cp.Std(q0 * q1, dist)),
            "first_order_sd": float((unc.ufloat(mean_x, sd_x) * unc.ufloat(mean_y, sd_y)).std_dev)}


_cap("product_of_normals_exact_moments", level="advanced",
     name="Exact mean and standard deviation of a product of independent normals", topic=_TUP,
     purpose="Exact moments of XY for independent normal X and Y, compared with first-order propagation (which misses "
             "the sd_x sd_y term).",
     inputs=[_Num("mean_x", "mean of X"), _Num("sd_x", "sd of X (> 0)"), _Num("mean_y", "mean of Y"),
             _Num("sd_y", "sd of Y (> 0)")],
     outputs=[_Out("exact_mean", "mean_x mean_y"), _Out("exact_sd", "sqrt(mx^2 sy^2 + my^2 sx^2 + sx^2 sy^2)"),
              _Out("first_order_sd", "sqrt(mx^2 sy^2 + my^2 sx^2)")],
     primary="exact_sd", conditions=["independent normal inputs"], method="exact second moments of a product",
     run=run_prod_normals, reference=ref_prod_normals,
     latex=r"\mathrm{Var}(XY) = \mu_x^2\sigma_y^2 + \mu_y^2\sigma_x^2 + \sigma_x^2\sigma_y^2",
     wrong=lambda mean_x, sd_x, mean_y, sd_y: (lambda r: dict(r, exact_sd=r["first_order_sd"]))(
         run_prod_normals(mean_x, sd_x, mean_y, sd_y)),
     rnd=lambda rng: {"mean_x": round(rng.uniform(-20, 20), 3), "sd_x": round(rng.uniform(0.2, 5), 3),
                      "mean_y": round(rng.uniform(-20, 20), 3), "sd_y": round(rng.uniform(0.2, 5), 3)},
     need=[r"product", r"normal", r"exact"], aliases=_UP_AL,
     templates=["Exact moments of the product of independent normals: mean_x = 10, sd_x = 1, mean_y = 5, sd_y = 2.",
                "Exact standard deviation of a product of normal variables with mean_x = 0, sd_x = 1, mean_y = 0, "
                "sd_y = 1."],
     examples=[_X({"mean_x": 0, "sd_x": 1, "mean_y": 0, "sd_y": 1}, {"exact_sd": 1.0, "first_order_sd": 0.0}),
               _X({"mean_x": 10, "sd_x": 1, "mean_y": 5, "sd_y": 2}, {"exact_sd": math.sqrt(400 + 25 + 4)})],
     questions=[_Q("Exact moments of the product of independent normals: mean_x = 10, sd_x = 1, mean_y = 5, sd_y = 2.",
                   {"exact_mean": 50.0, "exact_sd": math.sqrt(429)}),
                _Q("Exact moments of the product of independent normals: mean_x = 10, sd_x = 0, mean_y = 5, sd_y = 2.",
                   outcome="INVALID_INPUT")])

# =============================================================== Monte Carlo (deterministic, from given samples) ====
_TMC = "Monte Carlo estimation"
_MC_AL = {"sigma": r"standard deviation|sigma|sd", "half_width": r"half[- ]width|margin|tolerance|error",
          "values": r"values|function values|f values|samples|f\(x_i\)", "lower": r"lower(?: limit| bound)?|a",
          "upper": r"upper(?: limit| bound)?|b", "target_density": r"target densit(?:y|ies)|p\(x\)|target",
          "proposal_density": r"proposal densit(?:y|ies)|q\(x\)|proposal", "y": r"outputs|y", "c": r"controls?|c",
          "control_mean": r"control mean|known mean|mu_c", "weights": r"weights|stratum weights",
          "means": r"means|stratum means", "sds": r"sds|standard deviations|stratum sds", "sizes": r"sizes|stratum sizes|n",
          "index": r"index|point|n", "dimension": r"dimension|d"}


def run_mc_n(sigma, half_width, level=None):
    s, e, c = _pos(sigma, "the standard deviation"), _pos(half_width, "the half-width"), _level(level)
    z = _z(0.5 * (1 + c))
    return {"draws": math.ceil((z * s / e) ** 2 - 1e-9), "z": z, "standard_error_target": e / z}


_cap("monte_carlo_sample_size", level="beginner", name="Number of Monte Carlo draws for a target accuracy",
     topic=_TMC,
     purpose="How many independent draws make the half-width of the confidence interval of a Monte Carlo mean at "
             "most the stated value, given the standard deviation of one draw.",
     inputs=[_Num("sigma", "standard deviation of one draw (> 0)"), _Num("half_width", "target half-width (> 0)"),
             LEVEL_IN],
     outputs=[_Out("draws", "required number of draws (rounded up)"), _Out("z", "normal quantile z_{(1+c)/2}"),
              _Out("standard_error_target", "target standard error half_width/z")],
     primary="draws", conditions=["sigma, half-width > 0", "0 < level < 1"], method="n = (z sigma / E)^2, rounded up",
     run=run_mc_n,
     reference=lambda sigma, half_width, level=None: (lambda z: {
         "draws": math.ceil((z * sigma / half_width) ** 2 - 1e-9), "z": z, "standard_error_target": half_width / z})(
         float(__import__("scipy.stats", fromlist=["norm"]).norm.ppf(0.5 * (1 + (0.95 if level is None else level))))),
     latex=r"n = \left\lceil\left(\frac{z_{(1+c)/2}\,\sigma}{E}\right)^2\right\rceil",
     wrong=lambda sigma, half_width, level=None: dict(run_mc_n(sigma, half_width, level),
                                                      draws=math.ceil(_z(0.5 * (1 + (level or 0.95))) * sigma / half_width) + 1),
     rnd=lambda rng: {"sigma": round(rng.uniform(0.5, 50), 2), "half_width": round(rng.uniform(0.01, 2), 3),
                      "level": rng.choice([0.9, 0.95, 0.99])},
     need=[r"monte carlo", r"how many|sample size|number of (?:draws|samples|simulations|runs)"], aliases=_MC_AL,
     extract=_x_level,
     templates=["Monte Carlo sample size: standard deviation 10, half-width 0.5, 95% confidence. How many draws?",
                "How many Monte Carlo simulations are needed for sigma = 2 and a margin of 0.05 at 99% confidence?"],
     examples=[_X({"sigma": 10, "half_width": 0.5, "level": 0.95}, {"draws": 1537}),
               _X({"sigma": 1, "half_width": 0.1, "level": 0.9}, {"draws": 271})],
     questions=[_Q("Monte Carlo sample size: standard deviation 10, half-width 0.5, 95% confidence. How many draws?",
                   {"draws": 1537}),
                _Q("Monte Carlo sample size: standard deviation 10, half-width 0, 95% confidence.", outcome="INVALID_INPUT")])


def run_mc_int(values, lower, upper):
    f = _vec(values, "the function values", min_len=2)
    a, b = _fin(lower, "the lower limit"), _fin(upper, "the upper limit")
    if b <= a:
        raise ValueError("the upper limit must exceed the lower limit")
    m = _mean(f)
    s = math.sqrt(math.fsum((v - m) ** 2 for v in f) / (len(f) - 1))
    return {"estimate": (b - a) * m, "standard_error": (b - a) * s / math.sqrt(len(f)), "n": len(f)}


_cap("monte_carlo_integration", level="beginner", name="Monte Carlo estimate of an integral from uniform samples",
     topic=_TMC,
     purpose="Estimate of the integral of f over [a, b] from f evaluated at n points drawn uniformly on [a, b], with "
             "its Monte Carlo standard error.",
     inputs=[_Vec("values", "f(x_i) at uniform sample points", 2), _Num("lower", "lower limit a"),
             _Num("upper", "upper limit b (> a)")],
     outputs=[_Out("estimate", "(b - a) times the mean of f"), _Out("standard_error", "(b - a) s / sqrt(n)"),
              _Out("n", "number of samples")],
     primary="estimate", conditions=["b > a", "n >= 2"], method="sample mean of f scaled by the interval length",
     run=run_mc_int,
     reference=lambda values, lower, upper: (lambda np, st: {
         "estimate": float((upper - lower) * np.mean(values)),
         "standard_error": float((upper - lower) * st.sem(values)), "n": len(values)})(
         __import__("numpy"), __import__("scipy.stats", fromlist=["sem"])),
     latex=r"\hat I = \frac{b-a}{n}\sum_i f(x_i),\quad SE = (b-a)\frac{s_f}{\sqrt n}",
     wrong=lambda values, lower, upper: dict(run_mc_int(values, lower, upper),
                                             standard_error=(upper - lower) * math.sqrt(
                                                 sum((v - sum(values) / len(values)) ** 2 for v in values) / len(values))
                                             / math.sqrt(len(values))),
     rnd=lambda rng: {"values": [round(rng.uniform(0, 10), 3) for _ in range(rng.randint(3, 40))],
                      "lower": round(rng.uniform(-5, 2), 2), "upper": round(rng.uniform(2.5, 10), 2)},
     need=[r"monte carlo", r"integra"], aliases=_MC_AL,
     templates=["Monte Carlo integration on [0, 2] with values = [1.2, 0.8, 1.9, 1.4]: lower = 0, upper = 2.",
                "Estimate the integral by Monte Carlo: function values [3, 5, 4, 6], lower limit 1, upper limit 4."],
     examples=[_X({"values": "1, 3", "lower": 0, "upper": 2}, {"estimate": 4.0, "standard_error": 2.0}),
               _X({"values": [3, 5, 4, 6], "lower": 1, "upper": 4}, {"estimate": 13.5})],
     questions=[_Q("Monte Carlo integration with values = [1, 3]: lower = 0, upper = 2.", {"estimate": 4.0,
                                                                                         "standard_error": 2.0}),
                _Q("Monte Carlo integration with values = [1, 3]: lower = 2, upper = 0.", outcome="INVALID_INPUT")])


def run_is(values, target_density, proposal_density):
    f = _vec(values, "the function values", min_len=2)
    p = _vec(target_density, "the target densities", len(f))
    q = _vec(proposal_density, "the proposal densities", len(f))
    if any(v <= 0 for v in q) or any(v < 0 for v in p) or not any(v > 0 for v in p):
        raise ValueError("proposal densities must be positive and target densities non-negative (not all zero)")
    w = [a / b for a, b in zip(p, q)]
    sw = math.fsum(w)
    est = math.fsum(a * b for a, b in zip(w, f)) / sw
    return {"estimate": est, "unnormalized_estimate": math.fsum(a * b for a, b in zip(w, f)) / len(f),
            "effective_sample_size": sw * sw / math.fsum(v * v for v in w)}


def ref_is(values, target_density, proposal_density):
    import numpy as np
    w = np.array(target_density, float) / np.array(proposal_density, float)
    nw = w / w.sum()
    return {"estimate": float(np.average(values, weights=w)), "unnormalized_estimate": float(np.mean(w * values)),
            "effective_sample_size": float(1 / np.sum(nw ** 2))}


_cap("importance_sampling_estimate", level="advanced", name="Importance sampling estimate and effective sample size",
     topic=_TMC,
     purpose="Self-normalized and plain importance-sampling estimates of E_p[f] from draws of a proposal q, with the "
             "effective sample size of the weights p/q.",
     inputs=[_Vec("values", "f(x_i) at the draws", 2), _Vec("target_density", "target density p(x_i) (or unnormalized)", 2),
             _Vec("proposal_density", "proposal density q(x_i) > 0", 2)],
     outputs=[_Out("estimate", "self-normalized estimate sum w f / sum w"),
              _Out("unnormalized_estimate", "mean of w f (needs normalized densities)"),
              _Out("effective_sample_size", "(sum w)^2 / sum w^2")],
     primary="estimate", conditions=["q > 0", "p >= 0, not all zero"], method="weights w = p/q",
     run=run_is, reference=ref_is,
     latex=r"\hat\mu = \frac{\sum_i w_i f(x_i)}{\sum_i w_i},\ w_i = \frac{p(x_i)}{q(x_i)},\quad ESS = \frac{(\sum w_i)^2}{\sum w_i^2}",
     wrong=lambda values, target_density, proposal_density: dict(
         run_is(values, target_density, proposal_density),
         estimate=sum(f * q / p for f, p, q in zip(values, target_density, proposal_density) if p > 0) / sum(
             q / p for p, q in zip(target_density, proposal_density) if p > 0)),
     rnd=lambda rng: (lambda n: {"values": [round(rng.uniform(-5, 5), 3) for _ in range(n)],
                                 "target_density": [round(rng.uniform(0.05, 1), 4) for _ in range(n)],
                                 "proposal_density": [round(rng.uniform(0.05, 1), 4) for _ in range(n)]})(rng.randint(3, 30)),
     need=[r"importance sampling"], aliases=_MC_AL,
     templates=["Importance sampling: values = [1.0, 2.0, 3.0], target densities = [0.2, 0.5, 0.3], proposal densities "
                "= [0.4, 0.4, 0.2].",
                "Importance sampling estimate with f values [0, 1], target density [0.5, 0.5] and proposal density "
                "[0.25, 0.75]."],
     examples=[_X({"values": "0, 1", "target_density": "0.5, 0.5", "proposal_density": "0.25, 0.75"},
                  {"estimate": (2 / 3) / (2 + 2 / 3), "effective_sample_size": (8 / 3) ** 2 / (4 + 4 / 9)}),
               _X({"values": [1, 2, 3], "target_density": [0.2, 0.5, 0.3], "proposal_density": [0.4, 0.4, 0.2]}, {})],
     questions=[_Q("Importance sampling estimate with f values [0, 1], target density [0.5, 0.5] and proposal density "
                   "[0.25, 0.75].", {"estimate": 0.25}),
                _Q("Importance sampling: values = [1, 2], target densities = [0.2, 0.5], proposal densities = [0.4, 0].",
                   outcome="INVALID_INPUT")])


def run_cv(y, c, control_mean):
    yy = _vec(y, "the outputs", min_len=3)
    cc = _vec(c, "the control values", len(yy))
    mu = _fin(control_mean, "the known control mean")
    my, mc = _mean(yy), _mean(cc)
    scc = math.fsum((v - mc) ** 2 for v in cc)
    syy = math.fsum((v - my) ** 2 for v in yy)
    if scc == 0 or syy == 0:
        raise ValueError("the outputs and the control must both vary")
    syc = math.fsum((a - my) * (b - mc) for a, b in zip(yy, cc))
    b = syc / scc
    return {"estimate": my - b * (mc - mu), "coefficient": b, "plain_estimate": my,
            "variance_reduction": 1 - syc * syc / (scc * syy)}


def ref_cv(y, c, control_mean):
    """scipy.stats.linregress of y on the control: the estimate is the fitted line at the known control mean."""
    import numpy as np
    from scipy.stats import linregress
    r = linregress(c, y)
    return {"estimate": float(r.intercept + r.slope * control_mean), "coefficient": float(r.slope),
            "plain_estimate": float(np.mean(y)), "variance_reduction": float(1 - r.rvalue ** 2)}


def _rcv(rng):
    n = rng.randint(5, 40)
    c = [round(rng.uniform(0, 1), 4) for _ in range(n)]
    k = rng.uniform(-5, 5)
    return {"c": c, "y": [round(k * v + rng.gauss(0, 1) * rng.uniform(0.1, 2), 4) for v in c], "control_mean": 0.5}


_cap("control_variate_estimate", level="advanced", name="Control-variate Monte Carlo estimate", topic=_TMC,
     purpose="Variance-reduced estimate of E[Y] from paired draws (y_i, c_i) of the output and a control variate with "
             "known mean: optimal coefficient, estimate and the fraction of variance left (1 - r^2).",
     inputs=[_Vec("y", "simulated outputs y_i", 3), _Vec("c", "control values c_i (same draws)", 3),
             _Num("control_mean", "known mean of the control")],
     outputs=[_Out("estimate", "mean(y) - b (mean(c) - mu_c)"), _Out("coefficient", "b = cov(y, c)/var(c)"),
              _Out("plain_estimate", "mean(y)"), _Out("variance_reduction", "variance factor 1 - r^2")],
     primary="estimate", conditions=["outputs and control vary", "n >= 3"],
     method="regression (optimal) control-variate coefficient", run=run_cv, reference=ref_cv,
     latex=r"\hat\mu_{cv} = \bar y - \hat b\,(\bar c - \mu_c),\quad \hat b = \frac{\sum (y_i-\bar y)(c_i-\bar c)}{\sum (c_i-\bar c)^2}",
     wrong=lambda y, c, control_mean: (lambda r: dict(r, estimate=r["plain_estimate"] + r["coefficient"] * (
         sum(c) / len(c) - control_mean)))(run_cv(y, c, control_mean)),
     rnd=_rcv, need=[r"control[- ]variate"], aliases=_MC_AL,
     templates=["Control variate: outputs = [2.1, 3.9, 6.2, 8.1], controls = [1, 2, 3, 4], known mean = 2.5.",
                "Control-variate estimate with y = [1, 2, 2, 3], c = [0.1, 0.4, 0.5, 0.9] and control mean = 0.5."],
     examples=[_X({"y": "1, 2, 3", "c": "1, 2, 3", "control_mean": 2.5}, {"estimate": 2.5, "coefficient": 1.0,
                                                                          "variance_reduction": 0.0}),
               _X({"y": [2.1, 3.9, 6.2, 8.1], "c": [1, 2, 3, 4], "control_mean": 2.5}, {"plain_estimate": 5.075})],
     questions=[_Q("Control variate: outputs = [1, 2, 3], controls = [1, 2, 3], known mean = 2.5.", {"estimate": 2.5}),
                _Q("Control variate: outputs = [1, 2, 3], controls = [1, 1, 1], known mean = 2.5.",
                   outcome="INVALID_INPUT")])


def run_strat(weights, means, sds, sizes):
    w = _simplex(weights, "the stratum weights")
    m = _vec(means, "the stratum means", len(w))
    s = _vec(sds, "the stratum standard deviations", len(w))
    n = _vec(sizes, "the stratum sample sizes", len(w))
    if any(v < 0 for v in s) or any(v < 1 or v != int(v) for v in n):
        raise ValueError("standard deviations must be >= 0 and sample sizes whole numbers >= 1")
    return {"estimate": math.fsum(a * b for a, b in zip(w, m)),
            "standard_error": math.sqrt(math.fsum((a * b) ** 2 / c for a, b, c in zip(w, s, n)))}


def ref_strat(weights, means, sds, sizes):
    """The stratified estimator propagated with the uncertainties package (each stratum mean with its standard error)."""
    unc = _uf()
    y = sum(w * unc.ufloat(m, s / math.sqrt(n)) for w, m, s, n in zip(weights, means, sds, sizes))
    return {"estimate": float(y.nominal_value), "standard_error": float(y.std_dev)}


_cap("stratified_sampling_estimate", level="advanced", name="Stratified sampling estimate and standard error",
     topic=_TMC,
     purpose="Estimate of a mean from strata with known weights, the sample mean, standard deviation and size of each "
             "stratum, and its standard error.",
     inputs=[_Vec("weights", "stratum weights (sum to 1)", 2), _Vec("means", "stratum sample means", 2),
             _Vec("sds", "stratum sample standard deviations", 2), _Vec("sizes", "stratum sample sizes", 2)],
     outputs=[_Out("estimate", "sum W_h mean_h"), _Out("standard_error", "sqrt(sum W_h^2 s_h^2 / n_h)")],
     primary="standard_error", conditions=["weights sum to 1", "sizes >= 1"], method="weighted stratum means",
     run=run_strat, reference=ref_strat,
     latex=r"\hat\mu = \sum_h W_h\bar y_h,\quad SE = \sqrt{\sum_h W_h^2 s_h^2/n_h}",
     wrong=lambda weights, means, sds, sizes: dict(run_strat(weights, means, sds, sizes), standard_error=math.sqrt(
         sum(w * s * s / n for w, s, n in zip(weights, sds, sizes)))),
     rnd=lambda rng: (lambda k: {"weights": _rsimplex(rng, k), "means": [round(rng.uniform(-10, 50), 2) for _ in range(k)],
                                 "sds": [round(rng.uniform(0.5, 10), 2) for _ in range(k)],
                                 "sizes": [float(rng.randint(2, 100)) for _ in range(k)]})(rng.randint(2, 5)),
     need=[r"stratified|strata"], aliases=_MC_AL,
     templates=["Stratified sampling: weights = [0.6, 0.4], means = [10, 20], sds = [2, 5], sizes = [30, 20].",
                "Stratified estimate with stratum weights [0.5, 0.3, 0.2], stratum means [1, 2, 3], stratum sds "
                "[1, 1, 1] and stratum sizes [10, 10, 10]."],
     examples=[_X({"weights": "0.6, 0.4", "means": "10, 20", "sds": "2, 5", "sizes": "30, 20"},
                  {"estimate": 14.0, "standard_error": math.sqrt(0.36 * 4 / 30 + 0.16 * 25 / 20)}),
               _X({"weights": [0.5, 0.5], "means": [1, 3], "sds": [1, 1], "sizes": [4, 4]}, {"standard_error": math.sqrt(0.125)})],
     questions=[_Q("Stratified sampling: weights = [0.6, 0.4], means = [10, 20], sds = [2, 5], sizes = [30, 20].",
                   {"estimate": 14.0}),
                _Q("Stratified sampling: weights = [0.6, 0.6], means = [10, 20], sds = [2, 5], sizes = [30, 20].",
                   outcome="INVALID_INPUT")])


_PRIMES = [2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71]


def _radical_inverse(i, b):
    num, den = 0, 1
    while i > 0:
        i, d = divmod(i, b)
        num, den = num * b + d, den * b
    return num / den


def run_halton(index, dimension):
    i, d = _whole(index, "the index", 0), _whole(dimension, "the dimension", 1)
    if d > len(_PRIMES):
        raise ValueError(f"dimension must be at most {len(_PRIMES)}")
    if i > 10 ** 7:
        raise ValueError("index must be at most 10,000,000")
    return {"point": [_radical_inverse(i, b) for b in _PRIMES[:d]], "bases": [float(b) for b in _PRIMES[:d]]}


_cap("halton_point", level="intermediate", name="Point of the Halton low-discrepancy sequence", topic=_TMC,
     purpose="Coordinates of the point with a given index in the (unscrambled) Halton sequence of a given dimension: "
             "the radical inverses of the index in the first prime bases (index 0 is the origin).",
     inputs=[_Int("index", "index of the point (0, 1, 2, ...)"), _Int("dimension", "dimension (1 to 20)", lo=1)],
     outputs=[_Out("point", "coordinates in [0, 1)"), _Out("bases", "prime bases used")],
     primary="point", conditions=["index >= 0", "1 <= dimension <= 20"],
     method="radical inverse: reverse the base-b digits of the index behind the radix point", run=run_halton,
     reference=lambda index, dimension: {"point": [float(v) for v in __import__("scipy.stats", fromlist=["qmc"]).qmc.Halton(
         d=dimension, scramble=False).random(index + 1)[index]]},
     latex=r"\phi_b(i) = \sum_k d_k\,b^{-k-1}\ \text{ for } i = \sum_k d_k b^k",
     wrong=lambda index, dimension: dict(run_halton(index, dimension), point=[_radical_inverse(index, 2)] * dimension),
     rnd=lambda rng: {"index": rng.randint(1, 5000), "dimension": rng.randint(2, 8)},
     need=[r"halton"], aliases=_MC_AL,
     templates=["Halton sequence point with index = 5 and dimension = 3.",
                "Find point index 10 of the Halton sequence in dimension 2."],
     examples=[_X({"index": 5, "dimension": 3}, {"point": [5 / 8, 7 / 9, 1 / 25]}),
               _X({"index": 1, "dimension": 2}, {"point": [0.5, 1 / 3]})],
     questions=[_Q("Halton sequence point with index = 5 and dimension = 3.", {"point": [5 / 8, 7 / 9, 1 / 25]}),
                _Q("Halton sequence point with index = 5 and dimension = 30.", outcome="INVALID_INPUT")])


# ============================================================================ variance-based sensitivity ====
_TSA = "sensitivity analysis"
_SA_AL = {"a": r"a", "b": r"b", "coefficients": r"coefficients|a", "sds": r"sds|standard deviations|sigmas",
          "a1": r"a1", "a2": r"a2", "a12": r"a12", "mean1": r"mean1|mu1", "sd1": r"sd1|sigma1", "mean2": r"mean2|mu2",
          "sd2": r"sd2|sigma2", "outputs": r"outputs|model outputs|y", "delta": r"delta|step"}


def run_ishigami(a, b):
    a, b = _fin(a, "a"), _fin(b, "b")
    p4, p8 = math.pi ** 4, math.pi ** 8
    v1 = 0.5 * (1 + b * p4 / 5) ** 2
    v2 = a * a / 8
    v13 = 8 * b * b * p8 / 225
    v = v1 + v2 + v13
    if v <= 0:
        raise ValueError("the model has zero variance")
    return {"S1": v1 / v, "S2": v2 / v, "S3": 0.0, "ST1": (v1 + v13) / v, "ST2": v2 / v, "ST3": v13 / v, "variance": v}


def ref_ishigami(a, b):
    """Variance decomposition by tensor quadrature (numpy): trapezoid rule on 64 points in x1 and x2 (exact for these
    trigonometric polynomials) and 20-point Gauss-Legendre in x3 (exact for its polynomial)."""
    import numpy as np
    N = 64
    x = -np.pi + 2 * np.pi * np.arange(N) / N
    w = np.full(N, 1 / N)
    t, wl = np.polynomial.legendre.leggauss(20)
    x3, w3 = np.pi * t, wl / 2
    X1, X2, X3 = np.meshgrid(x, x, x3, indexing="ij")
    f = np.sin(X1) + a * np.sin(X2) ** 2 + b * X3 ** 4 * np.sin(X1)
    W = w[:, None, None] * w[None, :, None] * w3[None, None, :]
    E = np.sum(W * f)
    V = np.sum(W * (f - E) ** 2)
    out = {"variance": float(V)}
    ws = [w, w, w3]
    for i in range(3):
        others = tuple(j for j in range(3) if j != i)
        wo = W.sum(axis=i, keepdims=True)                                       # weights of the other two
        cond = np.sum(W * f, axis=others, keepdims=True) / ws[i].reshape([-1 if j == i else 1 for j in range(3)])
        out[f"S{i + 1}"] = float(np.sum(ws[i].reshape([-1 if j == i else 1 for j in range(3)]) * (cond - E) ** 2) / V)
        wi = ws[i].reshape([-1 if j == i else 1 for j in range(3)])
        inner_mean = np.sum(wi * f, axis=i, keepdims=True)
        out[f"ST{i + 1}"] = float(np.sum(wo * np.sum(wi * (f - inner_mean) ** 2, axis=i, keepdims=True)) / V)
    return out


_cap("sobol_indices_ishigami", level="expert", name="Sobol sensitivity indices of the Ishigami function", topic=_TSA,
     purpose="Exact first-order and total Sobol indices and the variance of the Ishigami test function "
             "f = sin x1 + a sin^2 x2 + b x3^4 sin x1 with x_i uniform on [-pi, pi].",
     inputs=[_Num("a", "parameter a (commonly 7)"), _Num("b", "parameter b (commonly 0.1)")],
     outputs=[_Out("S1", "first-order index of x1"), _Out("S2", "first-order index of x2"), _Out("S3", "first-order index of x3"),
              _Out("ST1", "total index of x1"), _Out("ST2", "total index of x2"), _Out("ST3", "total index of x3"),
              _Out("variance", "variance of f")],
     primary="S1", conditions=["inputs independent uniform on [-pi, pi]"], method="closed-form variance decomposition",
     run=run_ishigami, reference=ref_ishigami,
     latex=r"V = \frac{a^2}{8} + \frac{b\pi^4}{5} + \frac{b^2\pi^8}{18} + \frac12,\ V_1 = \frac12\left(1 + \frac{b\pi^4}{5}\right)^2,\ "
           r"V_2 = \frac{a^2}{8},\ V_{13} = \frac{8b^2\pi^8}{225}",
     wrong=lambda a, b: (lambda r: dict(r, S1=r["S1"] * 2 if r["S1"] < 0.5 else r["S1"] / 2))(run_ishigami(a, b)),
     rnd=lambda rng: {"a": round(rng.uniform(1, 10), 3), "b": round(rng.uniform(0.01, 0.5), 4)},
     need=[r"ishigami"], aliases=_SA_AL,
     templates=["Sobol indices of the Ishigami function with a = 7 and b = 0.1.",
                "First-order and total sensitivity indices for the Ishigami function, a = 5, b = 0.2."],
     examples=[_X({"a": 7, "b": 0.1}, {"S1": 0.5 * (1 + 0.1 * math.pi ** 4 / 5) ** 2 / (
         49 / 8 + 0.1 * math.pi ** 4 / 5 + 0.01 * math.pi ** 8 / 18 + 0.5)}),
               _X({"a": 0, "b": 0}, {"S1": 1.0, "variance": 0.5})],
     questions=[_Q("Sobol indices of the Ishigami function with a = 7 and b = 0.1.",
                   {"variance": 49 / 8 + 0.1 * math.pi ** 4 / 5 + 0.01 * math.pi ** 8 / 18 + 0.5}),
                _Q("Sobol indices of the Ishigami function with a = 7.", outcome="NEEDS_INPUT")])


def run_gfun(a):
    aa = _vec(a, "the parameters a_i", min_len=1)
    if any(v < 0 for v in aa):
        raise ValueError("the G-function parameters must be >= 0")
    vi = [1 / (3 * (1 + v) ** 2) for v in aa]
    prod = math.prod(1 + v for v in vi)
    V = prod - 1
    return {"first_order": [v / V for v in vi], "total_order": [v * prod / (1 + v) / V for v in vi], "variance": V}


def ref_gfun(a):
    """Each component's variance by numerical integration of g_i^2 over [0, 1] (scipy quad, break point 1/2)."""
    from scipy import integrate
    vi = [integrate.quad(lambda x: ((abs(4 * x - 2) + ai) / (1 + ai)) ** 2, 0, 1, points=[0.5], epsabs=0,
                         epsrel=1e-13)[0] - 1 for ai in a]
    prod = math.prod(1 + v for v in vi)
    V = prod - 1
    return {"first_order": [v / V for v in vi], "total_order": [v * prod / (1 + v) / V for v in vi], "variance": V}


_cap("sobol_indices_g_function", level="expert", name="Sobol indices of Sobol's G-function", topic=_TSA,
     purpose="Exact first-order and total Sobol indices and the variance of the G-function "
             "prod_i (|4 x_i - 2| + a_i)/(1 + a_i), x_i uniform on [0, 1].",
     inputs=[_Vec("a", "parameters a_i >= 0 (small a_i = important input)")],
     outputs=[_Out("first_order", "first-order indices S_i"), _Out("total_order", "total indices ST_i"),
              _Out("variance", "variance of the function")],
     primary="variance", conditions=["a_i >= 0", "independent uniform inputs"],
     method="partial variances V_i = 1/(3(1 + a_i)^2) of the product form", run=run_gfun, reference=ref_gfun,
     latex=r"V_i = \frac{1}{3(1+a_i)^2},\ V = \prod_i(1+V_i) - 1,\ S_i = \frac{V_i}{V},\ S_{Ti} = \frac{V_i\prod_{j\ne i}(1+V_j)}{V}",
     wrong=lambda a: (lambda vi: (lambda V: {"first_order": [v / V for v in vi],
                                             "total_order": [v * (V + 1) / (1 + v) / V for v in vi], "variance": V})(
         math.prod(1 + v for v in vi) - 1))([1 / (3 * (1 + v)) for v in a]),
     rnd=lambda rng: {"a": [rng.choice([0.0, round(rng.uniform(0, 2), 2), round(rng.uniform(0, 30), 1)])
                            for _ in range(rng.randint(1, 8))]},
     need=[r"g[- ]function"], aliases={"a": r"a|parameters"},
     templates=["Sobol indices of the G-function with a = [0, 1, 9, 99].",
                "Sensitivity indices for Sobol's G-function with parameters [0.5, 2]."],
     examples=[_X({"a": "0"}, {"first_order": [1.0], "variance": 1 / 3}),
               _X({"a": [0, 0]}, {"variance": 16 / 9 - 1, "first_order": [3 / 7, 3 / 7]})],
     questions=[_Q("Sobol indices of the G-function with a = [0, 0].", {"variance": 7 / 9}),
                _Q("Sobol indices of the G-function with a = [0, -1].", outcome="INVALID_INPUT")])


def run_sobol_linear(coefficients, sds):
    c = _vec(coefficients, "the coefficients", min_len=2)
    s = _vec(sds, "the input standard deviations", len(c))
    if any(v <= 0 for v in s):
        raise ValueError("input standard deviations must be positive")
    parts = [(a * b) ** 2 for a, b in zip(c, s)]
    V = math.fsum(parts)
    if V == 0:
        raise ValueError("the model has zero variance")
    return {"first_order": [p / V for p in parts], "variance": V}


def _ref_sobol_grid(f, axes):
    """First-order and total Sobol indices by tensor Gauss quadrature (numpy): conditional means and variances of
    the model on the grid; exact for polynomial models when the rule has enough points."""
    import numpy as np
    k = len(axes)
    shape = lambda i: [-1 if j == i else 1 for j in range(k)]          # noqa: E731
    F = f(*np.meshgrid(*[a[0] for a in axes], indexing="ij"))
    W = 1.0
    for i, (_x, w) in enumerate(axes):
        W = W * w.reshape(shape(i))
    E = float((W * F).sum())
    V = float((W * (F - E) ** 2).sum())
    S, ST = [], []
    for i in range(k):
        wi = axes[i][1].reshape(shape(i))
        others = tuple(j for j in range(k) if j != i)
        cond = (W * F).sum(axis=others, keepdims=True) / wi if others else F
        S.append(float((wi * (cond - E) ** 2).sum() / V))
        inner = (wi * F).sum(axis=i, keepdims=True)
        wo = W.sum(axis=i, keepdims=True)
        ST.append(float((wo * (wi * (F - inner) ** 2).sum(axis=i, keepdims=True)).sum() / V))
    return S, ST, V


def _hermite_axes(means, sds, n=5):
    import numpy as np
    x, w = np.polynomial.hermite_e.hermegauss(n)
    return [(m + s * x, w / w.sum()) for m, s in zip(means, sds)]


def ref_sobol_linear(coefficients, sds):
    S, _st, V = _ref_sobol_grid(lambda *X: sum(a * x for a, x in zip(coefficients, X)),
                                _hermite_axes([0.0] * len(sds), sds, 3))
    return {"first_order": S, "variance": V}


_cap("sobol_indices_linear_model", level="intermediate", name="Sobol indices of a linear (additive) model",
     topic=_TSA,
     purpose="First-order Sobol indices of y = sum a_i x_i with independent inputs of given standard deviations "
             "(S_i = a_i^2 sigma_i^2 / Var y; they sum to 1).",
     inputs=[_Vec("coefficients", "model coefficients a_i", 2), _Vec("sds", "input standard deviations sigma_i (> 0)", 2)],
     outputs=[_Out("first_order", "first-order (= total) indices"), _Out("variance", "Var y")],
     primary="variance", conditions=["independent inputs", "sigma_i > 0"], method="variance shares of the terms",
     run=run_sobol_linear, reference=ref_sobol_linear, latex=r"S_i = \frac{a_i^2\sigma_i^2}{\sum_j a_j^2\sigma_j^2}",
     wrong=lambda coefficients, sds: (lambda p: {"first_order": [v / sum(p) for v in p], "variance": sum(p) ** 2})(
         [abs(a) * b for a, b in zip(coefficients, sds)]),
     rnd=lambda rng: (lambda k: {"coefficients": [round(rng.uniform(-5, 5), 2) or 1.0 for _ in range(k)],
                                 "sds": [round(rng.uniform(0.1, 3), 3) for _ in range(k)]})(rng.randint(2, 5)),
     need=[r"sobol|sensitivity ind", r"linear|additive"], avoid=[r"bilinear|interaction|ishigami|g[- ]function"],
     aliases=_SA_AL,
     templates=["Sobol indices of a linear model with coefficients = [2, 1, 0.5] and sds = [1, 2, 4].",
                "First-order sensitivity indices of the additive model y = sum a_i x_i, coefficients [1, 1], standard "
                "deviations [3, 4]."],
     examples=[_X({"coefficients": "1, 1", "sds": "3, 4"}, {"first_order": [0.36, 0.64], "variance": 25.0}),
               _X({"coefficients": [2, 1, 0.5], "sds": [1, 2, 4]}, {"first_order": [1 / 3, 1 / 3, 1 / 3]})],
     questions=[_Q("Sobol indices of a linear model with coefficients = [2, 1, 0.5] and sds = [1, 2, 4].",
                   {"variance": 12.0}),
                _Q("Sobol indices of a linear model with coefficients = [2, 1] and sds = [1, 0].", outcome="INVALID_INPUT")])


def run_bilinear(a1, a2, a12, mean1, sd1, mean2, sd2):
    a1, a2, a12, m1, m2 = (_fin(v, n) for v, n in ((a1, "a1"), (a2, "a2"), (a12, "a12"), (mean1, "mean1"), (mean2, "mean2")))
    s1, s2 = _pos(sd1, "sd1"), _pos(sd2, "sd2")
    v1 = ((a1 + a12 * m2) * s1) ** 2
    v2 = ((a2 + a12 * m1) * s2) ** 2
    v12 = (a12 * s1 * s2) ** 2
    V = v1 + v2 + v12
    if V == 0:
        raise ValueError("the model has zero variance")
    return {"S1": v1 / V, "S2": v2 / V, "S12": v12 / V, "ST1": (v1 + v12) / V, "ST2": (v2 + v12) / V, "variance": V}


def ref_bilinear(a1, a2, a12, mean1, sd1, mean2, sd2):
    """Variance decomposition by 5 x 5 Gauss-Hermite tensor quadrature (exact for this polynomial)."""
    S, ST, V = _ref_sobol_grid(lambda x1, x2: a1 * x1 + a2 * x2 + a12 * x1 * x2,
                               _hermite_axes([mean1, mean2], [sd1, sd2], 5))
    return {"S1": S[0], "S2": S[1], "S12": 1 - S[0] - S[1], "ST1": ST[0], "ST2": ST[1], "variance": V}


_cap("sobol_indices_interaction_model", level="expert",
     name="Sobol indices of a model with an interaction term", topic=_TSA,
     purpose="First-order, interaction and total Sobol indices of y = a1 x1 + a2 x2 + a12 x1 x2 with independent normal "
             "inputs x1 ~ N(mean1, sd1^2), x2 ~ N(mean2, sd2^2).",
     inputs=[_Num("a1", "coefficient of x1"), _Num("a2", "coefficient of x2"), _Num("a12", "interaction coefficient"),
             _Num("mean1", "mean of x1"), _Num("sd1", "sd of x1 (> 0)"), _Num("mean2", "mean of x2"),
             _Num("sd2", "sd of x2 (> 0)")],
     outputs=[_Out("S1", "first-order index of x1"), _Out("S2", "first-order index of x2"),
              _Out("S12", "interaction index"), _Out("ST1", "total index of x1"), _Out("ST2", "total index of x2"),
              _Out("variance", "Var y")],
     primary="S1", conditions=["independent normal inputs"], method="ANOVA decomposition of the bilinear polynomial",
     run=run_bilinear, reference=ref_bilinear,
     latex=r"V_1 = (a_1 + a_{12}\mu_2)^2\sigma_1^2,\ V_2 = (a_2 + a_{12}\mu_1)^2\sigma_2^2,\ V_{12} = a_{12}^2\sigma_1^2\sigma_2^2",
     wrong=lambda a1, a2, a12, mean1, sd1, mean2, sd2: run_bilinear(a1, a2, a12, 0.0, sd1, 0.0, sd2)
     if abs(mean1) + abs(mean2) > 0.1 else run_bilinear(a1, a2, 2 * a12, mean1, sd1, mean2, sd2),
     rnd=lambda rng: {"a1": round(rng.uniform(-3, 3), 2), "a2": round(rng.uniform(-3, 3), 2),
                      "a12": round(rng.uniform(0.2, 2), 2) * rng.choice([1, -1]), "mean1": round(rng.uniform(-3, 3), 2),
                      "sd1": round(rng.uniform(0.2, 2), 2), "mean2": round(rng.uniform(-3, 3), 2),
                      "sd2": round(rng.uniform(0.2, 2), 2)},
     need=[r"sobol|sensitivity ind", r"interaction|bilinear"], aliases=_SA_AL,
     templates=["Sobol indices of the interaction model with a1 = 1, a2 = 2, a12 = 0.5, mean1 = 0, sd1 = 1, mean2 = 1, "
                "sd2 = 0.5.",
                "Sensitivity indices of y = a1 x1 + a2 x2 + a12 x1 x2 (interaction) with a1 = 0, a2 = 0, a12 = 1, "
                "mean1 = 0, sd1 = 1, mean2 = 0, sd2 = 1."],
     examples=[_X({"a1": 0, "a2": 0, "a12": 1, "mean1": 0, "sd1": 1, "mean2": 0, "sd2": 1},
                  {"S1": 0.0, "S12": 1.0, "ST1": 1.0, "variance": 1.0}),
               _X({"a1": 1, "a2": 2, "a12": 0.5, "mean1": 0, "sd1": 1, "mean2": 1, "sd2": 0.5}, {})],
     questions=[_Q("Sobol indices of the interaction model with a1 = 0, a2 = 0, a12 = 1, mean1 = 0, sd1 = 1, mean2 = 0, "
                   "sd2 = 1.", {"S12": 1.0}),
                _Q("Sobol indices of the interaction model with a1 = 1, a2 = 2, a12 = 0.5, mean1 = 0, sd1 = 0, "
                   "mean2 = 1, sd2 = 0.5.", outcome="INVALID_INPUT")])


def run_morris(outputs, delta):
    Y = _mat(outputs, "the model outputs")
    d = _pos(delta, "the step delta")
    r, k = len(Y), len(Y[0]) - 1
    if r < 2 or k < 1:
        raise ValueError("give at least 2 trajectories, each with k + 1 >= 2 outputs")
    ee = [[(Y[t][j + 1] - Y[t][j]) / d for t in range(r)] for j in range(k)]
    mu = [_mean(e) for e in ee]
    mus = [_mean([abs(v) for v in e]) for e in ee]
    sig = [math.sqrt(math.fsum((v - m) ** 2 for v in e) / (r - 1)) for e, m in zip(ee, mu)]
    return {"mu": mu, "mu_star": mus, "sigma": sig}


def ref_morris(outputs, delta):
    """SALib.analyze.morris on the equivalent one-at-a-time design (4 levels, step 2/3 in unit coordinates)."""
    import numpy as np
    from SALib.analyze import morris
    Y = np.array(outputs, float)
    r, k = Y.shape[0], Y.shape[1] - 1
    X = []
    for _ in range(r):
        x = np.zeros(k)
        X.append(x.copy())
        for j in range(k):
            x[j] = 2 / 3
            X.append(x.copy())
    problem = {"num_vars": k, "names": [f"x{j}" for j in range(k)], "bounds": [[0, 1]] * k}
    res = morris.analyze(problem, np.array(X), Y.reshape(-1), num_levels=4, print_to_console=False, seed=1)
    f = (2 / 3) / delta
    return {"mu": [float(v) * f for v in res["mu"]], "mu_star": [float(v) * f for v in res["mu_star"]],
            "sigma": [float(v) * f for v in res["sigma"]]}


_cap("morris_elementary_effects", level="advanced", name="Morris elementary-effects screening", topic=_TSA,
     purpose="Mean, mean absolute value (mu*) and standard deviation of the elementary effects from one-at-a-time "
             "trajectories: in each row, output j+1 is the model at the previous point with input j raised by delta.",
     inputs=[_Mat("outputs", "model outputs: one row per trajectory, k + 1 values (base point, then input 1, 2, ... "
                             "raised in turn)"),
             _Num("delta", "step delta (> 0)")],
     outputs=[_Out("mu", "mean elementary effect of each input"), _Out("mu_star", "mean absolute elementary effect"),
              _Out("sigma", "standard deviation of the elementary effects")],
     primary="mu_star", conditions=["at least 2 trajectories", "delta > 0"],
     method="elementary effects EE_ij = (y_{i,j+1} - y_{ij})/delta", run=run_morris, reference=ref_morris,
     latex=r"EE_j = \frac{f(x + \Delta e_j) - f(x)}{\Delta},\quad \mu_j^* = \frac1r\sum_t |EE_j^{(t)}|",
     wrong=lambda outputs, delta: (lambda r: dict(r, mu_star=[abs(v) for v in r["mu"]], sigma=[
         v * math.sqrt((len(outputs) - 1) / len(outputs)) for v in r["sigma"]]))(run_morris(outputs, delta)),
     rnd=lambda rng: (lambda r, k: {"outputs": [[round(rng.uniform(-10, 10), 3) for _ in range(k + 1)] for _ in range(r)],
                                    "delta": round(rng.uniform(0.1, 1), 3)})(rng.randint(3, 8), rng.randint(1, 5)),
     need=[r"morris|elementary effects"], aliases=_SA_AL,
     templates=["Morris elementary effects: outputs = [[1, 3, 4], [2, 3, 7], [0, 1, 1]], delta = 0.5.",
                "Elementary effects screening with model outputs [[10, 12], [11, 10]] and step 1."],
     examples=[_X({"outputs": "[[1, 3, 4], [2, 3, 7], [0, 1, 1]]", "delta": 0.5},
                  {"mu": [8 / 3, 10 / 3], "mu_star": [8 / 3, 10 / 3]}),
               _X({"outputs": [[10, 12], [11, 10]], "delta": 1}, {"mu": [0.5], "mu_star": [1.5]})],
     questions=[_Q("Elementary effects screening with model outputs [[10, 12], [11, 10]] and step 1.",
                   {"mu": [0.5], "mu_star": [1.5], "sigma": [math.sqrt(4.5)]}),
                _Q("Morris elementary effects: outputs = [[1, 3, 4]], delta = 0.5.", outcome="INVALID_INPUT")])


# ======================================================================= information, quadrature, comparisons ====
_TIQ = "information and quadrature for Bayesian computation"


def run_kl_normal(mean1, sd1, mean2, sd2):
    m1, m2 = _fin(mean1, "mean1"), _fin(mean2, "mean2")
    s1, s2 = _pos(sd1, "sd1"), _pos(sd2, "sd2")
    kl = lambda a, s, b, t: math.log(t / s) + (s * s + (a - b) ** 2) / (2 * t * t) - 0.5    # noqa: E731
    return {"kl_1_to_2": kl(m1, s1, m2, s2), "kl_2_to_1": kl(m2, s2, m1, s1)}


def ref_kl_normal(mean1, sd1, mean2, sd2):
    """Numerical integration of p log(p/q) (scipy quad; the normal log densities written from their definition)."""
    from scipy import integrate

    def lp(x, m, s):
        return -0.5 * ((x - m) / s) ** 2 - math.log(s) - 0.5 * math.log(2 * math.pi)

    def kl(a, s, b, t):
        f = lambda x: math.exp(lp(x, a, s)) * (lp(x, a, s) - lp(x, b, t))    # noqa: E731
        pts = [a + k * s for k in (-8, -3, -1, 0, 1, 3, 8)]
        return integrate.quad(f, a - 40 * s, a + 40 * s, points=pts, epsabs=0, epsrel=1e-13, limit=500)[0]
    return {"kl_1_to_2": kl(mean1, sd1, mean2, sd2), "kl_2_to_1": kl(mean2, sd2, mean1, sd1)}


_cap("kl_divergence_normal", level="advanced", name="Kullback-Leibler divergence between two normal distributions",
     topic=_TIQ,
     purpose="KL(N1 || N2) and KL(N2 || N1), e.g. the information gained from a normal prior to a normal posterior.",
     inputs=[_Num("mean1", "mean of distribution 1"), _Num("sd1", "sd of distribution 1 (> 0)"),
             _Num("mean2", "mean of distribution 2"), _Num("sd2", "sd of distribution 2 (> 0)")],
     outputs=[_Out("kl_1_to_2", "KL(N1 || N2) in nats"), _Out("kl_2_to_1", "KL(N2 || N1) in nats")],
     primary="kl_1_to_2", conditions=["sd1, sd2 > 0"], method="closed form for two normals", run=run_kl_normal,
     reference=ref_kl_normal,
     latex=r"KL(N_1\Vert N_2) = \ln\frac{\sigma_2}{\sigma_1} + \frac{\sigma_1^2 + (\mu_1-\mu_2)^2}{2\sigma_2^2} - \frac12",
     wrong=lambda mean1, sd1, mean2, sd2: dict(run_kl_normal(mean1, sd1, mean2, sd2),
                                               kl_1_to_2=run_kl_normal(mean1, sd1, mean2, sd2)["kl_2_to_1"] + 0.5
                                               if abs(sd1 - sd2) < 1e-9 else run_kl_normal(mean1, sd1, mean2, sd2)["kl_2_to_1"]),
     rnd=lambda rng: (lambda s1: {"mean1": round(rng.uniform(-5, 5), 3), "sd1": s1, "mean2": round(rng.uniform(-5, 5), 3),
                                  "sd2": round(s1 * rng.choice([rng.uniform(0.3, 0.8), rng.uniform(1.3, 3)]), 3)})(
         round(rng.uniform(0.3, 5), 3)),
     need=[r"kullback|\bkl\b", r"normal"], aliases={"mean1": r"mean1|mu1", "sd1": r"sd1|sigma1", "mean2": r"mean2|mu2",
                                                    "sd2": r"sd2|sigma2"},
     templates=["KL divergence between normal distributions: mean1 = 0, sd1 = 1, mean2 = 1, sd2 = 2.",
                "Kullback-Leibler divergence of N(mean1 = 2, sd1 = 0.5) from the normal N(mean2 = 0, sd2 = 1)."],
     examples=[_X({"mean1": 0, "sd1": 1, "mean2": 1, "sd2": 1}, {"kl_1_to_2": 0.5, "kl_2_to_1": 0.5}),
               _X({"mean1": 0, "sd1": 1, "mean2": 1, "sd2": 2}, {"kl_1_to_2": math.log(2) + 2 / 8 - 0.5})],
     questions=[_Q("KL divergence between normal distributions: mean1 = 0, sd1 = 1, mean2 = 1, sd2 = 2.",
                   {"kl_1_to_2": math.log(2) - 0.25}),
                _Q("KL divergence between normal distributions: mean1 = 0, sd1 = 1, mean2 = 1, sd2 = 0.",
                   outcome="INVALID_INPUT")])


def run_kl_beta(alpha1, beta1, alpha2, beta2):
    a1, b1, a2, b2 = (_pos(v, n) for v, n in ((alpha1, "alpha1"), (beta1, "beta1"), (alpha2, "alpha2"), (beta2, "beta2")))
    kl = (_lbeta(a2, b2) - _lbeta(a1, b1) + (a1 - a2) * _digamma(a1) + (b1 - b2) * _digamma(b1)
          + (a2 - a1 + b2 - b1) * _digamma(a1 + b1))
    return {"kl": kl}


def ref_kl_beta(alpha1, beta1, alpha2, beta2):
    from scipy import integrate
    from scipy.stats import beta as B
    f = lambda x: B.pdf(x, alpha1, beta1) * (B.logpdf(x, alpha1, beta1) - B.logpdf(x, alpha2, beta2))   # noqa: E731
    m = alpha1 / (alpha1 + beta1)
    return {"kl": integrate.quad(f, 0, 1, points=[m], epsabs=0, epsrel=1e-13, limit=500)[0]}


_cap("kl_divergence_beta", level="expert", name="Kullback-Leibler divergence between two beta distributions",
     topic=_TIQ,
     purpose="KL(Beta(alpha1, beta1) || Beta(alpha2, beta2)): for example the information gained from a beta prior "
             "(distribution 2) to a beta posterior (distribution 1).",
     inputs=[_Num("alpha1", "alpha of distribution 1"), _Num("beta1", "beta of distribution 1"),
             _Num("alpha2", "alpha of distribution 2"), _Num("beta2", "beta of distribution 2")],
     outputs=[_Out("kl", "KL divergence in nats")], primary="kl", conditions=["all parameters > 0"],
     method="closed form with log-beta and digamma functions", run=run_kl_beta, reference=ref_kl_beta,
     latex=r"KL = \ln\frac{B(\alpha_2,\beta_2)}{B(\alpha_1,\beta_1)} + (\alpha_1-\alpha_2)\psi(\alpha_1) + "
           r"(\beta_1-\beta_2)\psi(\beta_1) + (\alpha_2-\alpha_1+\beta_2-\beta_1)\psi(\alpha_1+\beta_1)",
     wrong=lambda alpha1, beta1, alpha2, beta2: run_kl_beta(alpha2, beta2, alpha1, beta1)
     if abs(run_kl_beta(alpha2, beta2, alpha1, beta1)["kl"] - run_kl_beta(alpha1, beta1, alpha2, beta2)["kl"]) > 1e-6 * abs(
         run_kl_beta(alpha1, beta1, alpha2, beta2)["kl"]) else {"kl": 2 * run_kl_beta(alpha1, beta1, alpha2, beta2)["kl"]},
     rnd=lambda rng: (lambda a, b: {"alpha1": a, "beta1": b, "alpha2": round(a * rng.uniform(0.3, 0.8), 2) + 1.2,
                                    "beta2": round(b * rng.uniform(1.2, 2.5), 2)})(
         round(rng.uniform(1.5, 20), 2), round(rng.uniform(1.5, 20), 2)),
     need=[r"kullback|\bkl\b", r"beta"], avoid=[r"normal"],
     aliases={"alpha1": r"alpha1", "beta1": r"beta1", "alpha2": r"alpha2", "beta2": r"beta2"},
     templates=["KL divergence between beta distributions: alpha1 = 8, beta1 = 4, alpha2 = 1, beta2 = 1.",
                "Kullback-Leibler divergence of Beta with alpha1 = 2, beta1 = 2 from alpha2 = 3, beta2 = 5."],
     examples=[_X({"alpha1": 2, "beta1": 1, "alpha2": 1, "beta2": 1}, {"kl": math.log(2) - 0.5}),
               _X({"alpha1": 8, "beta1": 4, "alpha2": 1, "beta2": 1}, {})],
     questions=[_Q("KL divergence between beta distributions: alpha1 = 2, beta1 = 1, alpha2 = 1, beta2 = 1.",
                   {"kl": math.log(2) - 0.5}),
                _Q("KL divergence between beta distributions: alpha1 = 2, beta1 = 0, alpha2 = 1, beta2 = 1.",
                   outcome="INVALID_INPUT")])


def _tqli_first(d, e):
    """Eigenvalues of a symmetric tridiagonal matrix (diagonal d, off-diagonal e) by the implicit QL method."""
    n = len(d)
    d, e = list(d), list(e) + [0.0]
    for l in range(n):
        it = 0
        while True:
            m = l
            while m < n - 1:
                dd = abs(d[m]) + abs(d[m + 1])
                if abs(e[m]) + dd == dd:
                    break
                m += 1
            if m == l:
                break
            it += 1
            if it > 100:
                raise ArithmeticError("the eigenvalue iteration did not converge")
            g = (d[l + 1] - d[l]) / (2.0 * e[l])
            r = math.hypot(g, 1.0)
            g = d[m] - d[l] + e[l] / (g + math.copysign(r, g))
            s = c = 1.0
            p = 0.0
            i = m - 1
            early = False
            while i >= l:
                f, b = s * e[i], c * e[i]
                r = math.hypot(f, g)
                e[i + 1] = r
                if r == 0.0:
                    d[i + 1] -= p
                    e[m] = 0.0
                    early = True
                    break
                s, c = f / r, g / r
                g = d[i + 1] - p
                r = (d[i] - g) * s + 2.0 * c * b
                p = s * r
                d[i + 1] = g + p
                g = c * r - b
                i -= 1
            if early:
                continue
            d[l] -= p
            e[l] = g
            e[m] = 0.0
    return sorted(d)


def _hermite_e(n, x):
    """(He_n(x), He_{n-1}(x)) by the three-term recurrence."""
    p0, p1 = 1.0, x
    if n == 0:
        return 1.0, 0.0
    for k in range(1, n):
        p0, p1 = p1, x * p1 - k * p0
    return p1, p0


def run_gauss_hermite(n):
    k = _whole(n, "the number of nodes", 1)
    if k > 40:
        raise ValueError("at most 40 nodes")
    xs = _tqli_first([0.0] * k, [math.sqrt(i) for i in range(1, k)])
    nodes, weights = [], []
    for x in xs:
        for _ in range(3):                                 # Newton polish on He_n (He_n' = n He_{n-1})
            h, h1 = _hermite_e(k, x)
            if h1 == 0:
                break
            x -= h / (k * h1)
        h, h1 = _hermite_e(k, x)
        nodes.append(x)
        weights.append(math.exp(math.lgamma(k + 1) - 2 * math.log(k) - 2 * math.log(abs(h1))) if k > 1 else 1.0)
    for i in range(k // 2):                                # exact symmetry
        v = 0.5 * (nodes[k - 1 - i] - nodes[i])
        w = 0.5 * (weights[i] + weights[k - 1 - i])
        nodes[i], nodes[k - 1 - i], weights[i], weights[k - 1 - i] = -v, v, w, w
    if k % 2:
        nodes[k // 2] = 0.0
    return {"nodes": nodes, "weights": weights}


_cap("gauss_hermite_rule", level="expert", name="Gauss-Hermite rule for expectations under a normal distribution",
     topic=_TIQ,
     purpose="Nodes and weights of the n-point Gauss-Hermite rule for E[f(Z)], Z standard normal (probabilists' "
             "Hermite polynomials; weights sum to 1; exact for polynomials of degree up to 2n - 1). For N(mu, sigma^2) "
             "use mu + sigma x_i.",
     inputs=[_Int("n", "number of nodes (1 to 40)", lo=1)],
     outputs=[_Out("nodes", "nodes x_i (ascending)"), _Out("weights", "weights w_i (sum to 1)")],
     primary="nodes", conditions=["1 <= n <= 40"],
     method="Golub-Welsch: eigenvalues of the Jacobi matrix (implicit QL), Newton-polished on He_n; weights "
            "n!/(n^2 He_{n-1}(x_i)^2)",
     run=run_gauss_hermite,
     reference=lambda n: (lambda np: (lambda x, w: {"nodes": [float(v) for v in x],
                                                    "weights": [float(v) for v in w / math.sqrt(2 * math.pi)]})(
         *np.polynomial.hermite_e.hermegauss(n)))(__import__("numpy")),
     latex=r"E[f(Z)] \approx \sum_{i=1}^{n} w_i f(x_i),\quad He_n(x_i) = 0,\quad w_i = \frac{n!}{n^2\,He_{n-1}(x_i)^2}",
     wrong=lambda n: (lambda r: dict(r, nodes=[v * math.sqrt(2) for v in r["nodes"]]))(run_gauss_hermite(n))
     if n > 1 else {"nodes": [1.0], "weights": [1.0]},
     rnd=lambda rng: {"n": rng.randint(1, 30)},
     need=[r"gauss[- ]hermite"], aliases={"n": r"n|nodes|points"},
     extract=lambda q, d: (lambda mm: d.setdefault("n", int(mm.group(1))) if mm else None)(
         re.search(r"(\d+)[- ]point", q)),
     templates=["Gauss-Hermite rule with n = 3 nodes for a normal expectation.",
                "Give the 5-point Gauss-Hermite nodes and weights."],
     examples=[_X({"n": 2}, {"nodes": [-1.0, 1.0], "weights": [0.5, 0.5]}),
               _X({"n": 3}, {"nodes": [-math.sqrt(3), 0.0, math.sqrt(3)], "weights": [1 / 6, 2 / 3, 1 / 6]})],
     questions=[_Q("Gauss-Hermite rule with n = 3 nodes for a normal expectation.",
                   {"nodes": [-math.sqrt(3), 0.0, math.sqrt(3)], "weights": [1 / 6, 2 / 3, 1 / 6]}),
                _Q("Gauss-Hermite rule with n = 0 nodes.", outcome="INVALID_INPUT")])


def _z_moment(k):
    return 0.0 if k % 2 else float(math.prod(range(k - 1, 0, -2))) if k else 1.0


def run_poly_normal(coefficients, mean, sd):
    c = _vec(coefficients, "the coefficients", min_len=1, n=None)
    if len(c) > 7:
        raise ValueError("at most degree 6 (7 coefficients)")
    m, s = _fin(mean, "the mean"), _pos(sd, "the standard deviation")
    q = [math.fsum(c[j] * math.comb(j, k) * m ** (j - k) * s ** k for j in range(k, len(c))) for k in range(len(c))]
    mu = math.fsum(q[k] * _z_moment(k) for k in range(len(q)))
    r = [q[0] - mu] + q[1:]
    var = math.fsum(r[i] * r[j] * _z_moment(i + j) for i in range(len(r)) for j in range(len(r)))
    return {"mean": mu, "variance": max(0.0, var), "sd": math.sqrt(max(0.0, var))}


def ref_poly_normal(coefficients, mean, sd):
    import chaospy as cp
    dist = cp.Normal(mean, sd)
    x = cp.variable()
    poly = sum(cv * x ** k for k, cv in enumerate(coefficients))
    if not hasattr(poly, "coefficients"):
        poly = cp.polynomial(poly)
    v = float(cp.Var(poly, dist))
    return {"mean": float(cp.E(poly, dist)), "variance": v, "sd": math.sqrt(v)}


_cap("polynomial_of_normal_moments", level="expert", name="Exact mean and variance of a polynomial of a normal variable",
     topic=_TUP,
     purpose="Exact mean, variance and standard deviation of Y = c0 + c1 X + ... + c6 X^6 for X ~ N(mean, sd^2) "
             "(nonlinear uncertainty propagation without linearization).",
     inputs=[_Vec("coefficients", "coefficients c0, c1, ... (constant first, up to degree 6)", 1),
             _Num("mean", "mean of X"), _Num("sd", "standard deviation of X (> 0)")],
     outputs=[_Out("mean", "E[Y]"), _Out("variance", "Var[Y]"), _Out("sd", "sd of Y")],
     primary="variance", conditions=["sd > 0", "degree <= 6"],
     method="substitute X = mean + sd Z and use the normal moments E[Z^k] = (k - 1)!! for even k",
     run=run_poly_normal, reference=ref_poly_normal,
     latex=r"E[Z^{2m}] = (2m-1)!!,\quad X = \mu + \sigma Z",
     wrong=lambda coefficients, mean, sd: (lambda r: dict(r, variance=r["variance"] + sd ** 2, sd=math.sqrt(
         r["variance"] + sd ** 2)))(run_poly_normal(coefficients, mean, sd)),
     rnd=lambda rng: {"coefficients": [round(rng.uniform(-3, 3), 2) for _ in range(rng.randint(2, 5))],
                      "mean": round(rng.uniform(-2, 2), 2), "sd": round(rng.uniform(0.2, 1.5), 2)},
     need=[r"polynomial", r"normal"], avoid=[r"chaos expansion coefficients"],
     aliases={"coefficients": r"coefficients", "mean": r"mean|mu", "sd": r"sd|sigma|standard deviation"},
     templates=["Mean and variance of the polynomial with coefficients = [1, 2, 3] of a normal X with mean = 0, sd = 1.",
                "Exact moments of a polynomial of a normal variable: coefficients [0, 0, 1], mean 2, sd 0.5."],
     examples=[_X({"coefficients": "1, 2, 3", "mean": 0, "sd": 1}, {"mean": 4.0, "variance": 4 + 9 * 2}),
               _X({"coefficients": [0, 0, 1], "mean": 2, "sd": 0.5}, {"mean": 4.25})],
     questions=[_Q("Mean and variance of the polynomial with coefficients = [1, 2, 3] of a normal X with mean = 0, sd = 1.",
                   {"mean": 4.0, "variance": 22.0}),
                _Q("Mean and variance of the polynomial with coefficients = [1, 2, 3] of a normal X with mean = 0, sd = 0.",
                   outcome="INVALID_INPUT")])


def run_gamma_compare(shape1, rate1, shape2, rate2):
    a1, b1, a2, b2 = (_pos(v, n) for v, n in ((shape1, "shape1"), (rate1, "rate1"), (shape2, "shape2"), (rate2, "rate2")))
    lo, up = _ibeta(a1, a2, b1 / (b1 + b2), b2 / (b1 + b2))
    return {"prob_1_greater": up, "prob_2_greater": lo, "rate_ratio_mean": (a1 / b1) / (a2 / b2)}


def ref_gamma_compare(shape1, rate1, shape2, rate2):
    """Numerical integration of f_1(x) F_2(x) over (0, inf) with scipy's gamma densities."""
    from scipy import integrate
    from scipy.stats import gamma as G
    m = shape1 / rate1
    f = lambda x: G.pdf(x, shape1, scale=1 / rate1) * G.cdf(x, shape2, scale=1 / rate2)   # noqa: E731
    g = lambda x: G.pdf(x, shape1, scale=1 / rate1) * G.sf(x, shape2, scale=1 / rate2)    # noqa: E731
    sd = math.sqrt(shape1) / rate1
    pts = [max(1e-12, m + k * sd) for k in (-5, -2, 0, 2, 5)]
    top = m + 60 * sd
    p1 = integrate.quad(f, 0, top, points=pts, epsabs=0, epsrel=1e-13, limit=500)[0]
    p2 = integrate.quad(g, 0, top, points=pts, epsabs=0, epsrel=1e-13, limit=500)[0]
    return {"prob_1_greater": p1, "prob_2_greater": p2}


def _rgc(rng):
    a1, b1 = round(rng.uniform(2, 80), 2), round(rng.uniform(0.5, 20), 2)
    m1 = a1 / b1
    a2 = round(rng.uniform(2, 80), 2)
    m2 = m1 * (1 + rng.uniform(-1.5, 1.5) * math.sqrt(1 / a1 + 1 / a2))
    return {"shape1": a1, "rate1": b1, "shape2": a2, "rate2": round(a2 / max(m2, 1e-3), 3)}


_cap("probability_rate_greater_gamma", level="advanced",
     name="Probability that one Poisson rate exceeds another (gamma posteriors)", topic=_TGP,
     purpose="P(lambda_1 > lambda_2) for independent Gamma(shape, rate) posteriors of two Poisson rates, and the ratio "
             "of their posterior means.",
     inputs=[_Num("shape1", "posterior shape of rate 1"), _Num("rate1", "posterior rate parameter of rate 1"),
             _Num("shape2", "posterior shape of rate 2"), _Num("rate2", "posterior rate parameter of rate 2")],
     outputs=[_Out("prob_1_greater", "P(lambda_1 > lambda_2)"), _Out("prob_2_greater", "P(lambda_2 > lambda_1)"),
              _Out("rate_ratio_mean", "ratio of posterior means")],
     primary="prob_1_greater", conditions=["all parameters > 0"],
     method="lambda_1 b_1 / (lambda_1 b_1 + lambda_2 b_2) ~ Beta(shape1, shape2): one regularized incomplete beta",
     run=run_gamma_compare, reference=ref_gamma_compare,
     latex=r"P(\lambda_1 > \lambda_2) = 1 - I_{b_1/(b_1+b_2)}(a_1, a_2)",
     wrong=lambda shape1, rate1, shape2, rate2: (lambda p: dict(run_gamma_compare(shape1, rate1, shape2, rate2),
                                                                prob_1_greater=p, prob_2_greater=1 - p))(
         _phi((shape1 / rate1 - shape2 / rate2) / math.sqrt(shape1 / rate1 ** 2 + shape2 / rate2 ** 2))),
     rnd=_rgc, need=[r"shape1", r"shape2"], aliases={"shape1": r"shape1", "rate1": r"rate1", "shape2": r"shape2",
                                                       "rate2": r"rate2"},
     templates=["Two Poisson rates with gamma posteriors: shape1 = 20, rate1 = 10, shape2 = 30, rate2 = 12. Probability "
                "that rate 1 exceeds rate 2?",
                "Compare gamma posteriors shape1 = 5, rate1 = 1, shape2 = 5, rate2 = 1."],
     examples=[_X({"shape1": 5, "rate1": 1, "shape2": 5, "rate2": 1}, {"prob_1_greater": 0.5}),
               _X({"shape1": 1, "rate1": 1, "shape2": 1, "rate2": 2}, {"prob_1_greater": 2 / 3})],
     questions=[_Q("Compare gamma posteriors shape1 = 5, rate1 = 1, shape2 = 5, rate2 = 1.", {"prob_1_greater": 0.5}),
                _Q("Compare gamma posteriors shape1 = 1, rate1 = 1, shape2 = 1, rate2 = 2.", {"prob_1_greater": 2 / 3}),
                _Q("Compare gamma posteriors shape1 = 5, rate1 = 0, shape2 = 5, rate2 = 1.", outcome="INVALID_INPUT")])


# ==================================================================================================================
# the reading guard: every parser above is wrapped by the same rules (ask_sweep 2026-10-23: 28 wrong answers, among
# them a 50 % credible level read as the default 95 %, a stated prior left unread so the default 1/2 was used, BF10
# read as "bf" = 10, and repeated test results answered as a single result)
#   1. a stated credibility level is read wherever it is written ("0.9 equal-tailed credible interval", "coverage of
#      0.991", "credibility level is set to 0.5", "0.5 of the probability mass"); two different stated levels, or a
#      level whose number another input also took, are refused;
#   2. every stated number is bound: a reading that leaves a number of the question unused is refused (an unread
#      prior, an unread level, counts of results the capability has no slot for). Numbers that are part of a name
#      (H1, BF10, mu0, shape1, log10) are not stated values;
#   3. wording that names a different capability's task is refused by the routes themselves (the `avoid` patterns:
#      a shortest interval is not the equal-tailed one, several results are not one result, BF01 is not BF10).
# ==================================================================================================================
_UNUM = r"(?:\d+(?:\.\d*)?|\.\d+)(?:e[-+]?\d+)?"
_LVL_ADJ = r"(?:(?:equal[- ]tailed|central|two[- ]sided|highest[- ]density|shortest|narrowest|bayesian|posterior)\s+)*"
_LVL_BEFORE = re.compile(
    r"(?<![\w.])(" + _UNUM + r")(\s*%)?\s+(?:of\s+(?:the\s+)?(?:posterior\s+|total\s+)?(?:probability(?:\s+mass)?|mass)\b|"
    + _LVL_ADJ + r"(?:credible|credibility|confidence|coverage|probability|mass|content|level|hdi|hpd|interval)\b)")
_LVL_AFTER = re.compile(
    r"\b(?:(?:credib(?:le|ility)|confidence|coverage)(?:\s+(?:level|probability))?|level|probability\s+(?:mass|content)|"
    r"(?:posterior\s+)?mass|content)\s*(?:of|is|=|:|at|is set to|set to|equal to|equals)?\s*(?<![\w.])(" + _UNUM
    + r")(\s*%)?(?![\d.]*\d)")
# a level written next to the interval it qualifies ("90% Jeffreys interval", "interval at 95%"): only a percentage or
# a fraction strictly between 0 and 1 is read this way
_LVL_NEAR = [re.compile(r"(?<![\w.])(" + _UNUM + r")(\s*%)?\s+(?:[a-z'-]+\s+){0,2}(?:interval|hdi|hpd)\b"),
             re.compile(r"\b(?:interval|hdi|hpd)\s+(?:at|of|with)\s+(?:a\s+)?(?<![\w.])(" + _UNUM + r")(\s*%)?(?![\d.]*\d)")]
_QNUM = re.compile(r"(?<![\w.])(" + _UNUM + r")(\s*%)?(?![a-z_\d])")
# numbers that are part of a capability's own notation (indices naming one of two things), never stated values
_NAMES = {"break_even_probability": r"\bstates?\s+[12]\b(?!\s*(?:=|:))",
          "probability_rate_greater_gamma": r"\brates?\s+[12]\b(?!\s*(?:=|:))"}


def _as_value(num, pct):
    v = float(num)
    return v / 100.0 if pct else v


def _same(a, b):
    return abs(a - b) <= 1e-12 * max(1.0, abs(a), abs(b))


def _stated_levels(q: str) -> list:
    out = []
    for rx in (_LVL_BEFORE, _LVL_AFTER):
        for m in rx.finditer(q):
            v = _as_value(m.group(1), m.group(2))
            if not any(_same(v, w) for w in out):
                out.append(v)
    for rx in _LVL_NEAR:
        for m in rx.finditer(q):
            v = _as_value(m.group(1), m.group(2))
            if (m.group(2) or 0.0 < v < 1.0) and not any(_same(v, w) for w in out):
                out.append(v)
    return out


def _numbers_stated(q: str) -> list:
    """Every number the question states, as a value ("5%" is 0.05); a number glued to a name (h1, bf10, mu0, x1,
    log10, beta_a) is part of that name, not a stated value."""
    out = []
    for m in _QNUM.finditer(q):
        v = _as_value(m.group(1), m.group(2))
        s = m.start(1)
        if s > 0 and q[s - 1] == "-" and (s == 1 or not (q[s - 2].isalnum() or q[s - 2] in "_)")):
            v = -v
        out.append(v)
    return out


def _bound_numbers(parsed) -> list:
    nums = []

    def walk(x):
        if isinstance(x, bool) or x is None:
            return
        if isinstance(x, (int, float)):
            nums.append(float(x))
        elif isinstance(x, str):
            nums.extend(float(t) for t in re.findall(r"[-+]?" + _UNUM, x.lower()))
        elif isinstance(x, dict):
            for v in x.values():
                walk(v)
        elif isinstance(x, (list, tuple)):
            for v in x:
                walk(v)
    walk(parsed)
    return nums


def _guarded(key, spec, parse):
    has_level = any(i["symbol"] == "level" for i in spec["inputs"])
    names = re.compile(_NAMES[key]) if key in _NAMES else None

    def guarded(question):
        p = parse(question)
        if not p or "_out_of_scope" in p:
            return p
        q = _norm(question)
        if names is not None:
            q = names.sub(lambda m: " " * len(m.group(0)), q)
        if has_level:                                     # rule 1: the stated level, read wherever it is written
            lv = _stated_levels(q)
            if len(lv) > 1:
                return None
            if lv:
                if "level" in p and not _same(float(p["level"]), lv[0]):
                    return None
                others = [v for v in _bound_numbers({k: x for k, x in p.items() if k != "level"})]
                if any(_same(lv[0], v) for v in others) and \
                        sum(_same(lv[0], v) for v in _numbers_stated(q)) < 1 + sum(_same(lv[0], v) for v in others):
                    return None                           # one written number cannot be both the level and an input
                p = dict(p, level=lv[0])
        bound = _bound_numbers(p)                         # rule 2: every stated number is bound
        if any(not any(_same(v, b) or _same(-v, b) for b in bound) for v in _numbers_stated(q)):
            return None
        return p
    return guarded


for _k, _spec in PROCEDURES.items():
    _spec["parse"] = _guarded(_k, _spec, _spec["parse"])

# === END OF PROCEDURES ===

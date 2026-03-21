use std::f64::consts::{PI, TAU};
use crate::language::{self, Category, Aspect, Role, CategorizedWord, hash_to_float, derive_hash, word_hash};

const N: usize = 720;
const CX: f64 = 300.0;
const CY: f64 = 300.0;
const BASE_R: f64 = 180.0;

#[derive(Debug, Clone, serde::Serialize)]
pub struct Mark {
    pub word: String,
    pub category: Category,
    pub aspect: Aspect,
    pub role: Role,
    pub angle: f64,
    pub arc: f64,
    pub feature: String,
    pub explanation: String,
    pub label_x: f64,
    pub label_y: f64,
}

#[derive(Debug, serde::Serialize)]
pub struct SymbolData {
    pub outer: Vec<[f64; 2]>,
    pub inner: Vec<[f64; 2]>,
    pub dots: Vec<[f64; 3]>,
    pub extras: String,
    pub marks: Vec<Mark>,
    pub has_question: bool,
    pub has_negation: bool,
    pub svg: String,
}

// ─── Noise ───────────────────────────────────────────────────────

fn hv(n: i64, seed: u64) -> f64 {
    let mut buf = [0u8; 16];
    buf[..8].copy_from_slice(&n.to_le_bytes());
    buf[8..].copy_from_slice(&seed.to_le_bytes());
    hash_to_float(language::fnv1a(&buf)) * 2.0 - 1.0
}

fn snoise(theta: f64, seed: u64, cyc: usize) -> f64 {
    let t = theta * cyc as f64 / TAU;
    let i = t.floor() as i64;
    let f = t - t.floor();
    let f = f * f * (3.0 - 2.0 * f);
    let i0 = ((i % cyc as i64) + cyc as i64) % cyc as i64;
    let i1 = (i0 + 1) % cyc as i64;
    hv(i0, seed) + (hv(i1, seed) - hv(i0, seed)) * f
}

fn fbm(theta: f64, seed: u64) -> f64 {
    let mut v = 0.0;
    let mut a = 1.0;
    let mut c = 10usize;
    for o in 0..4u64 {
        v += snoise(theta, seed.wrapping_add(o * 99991), c) * a;
        c *= 2;
        a *= 0.45;
    }
    v
}

fn adist(a: f64, b: f64) -> f64 {
    let d = ((a - b) % TAU + TAU) % TAU;
    if d > PI { TAU - d } else { d }
}

fn signed_angle(theta: f64, ref_a: f64) -> f64 {
    let d = ((theta - ref_a) % TAU + TAU) % TAU;
    if d > PI { d - TAU } else { d }
}

fn rd(v: f64) -> f64 { (v * 100.0).round() / 100.0 }

/// Scale extras SVG by wrapping in a transform group
fn scale_extras_svg(extras: &str, cx: f64, cy: f64, scale: f64) -> String {
    if extras.is_empty() { return String::new(); }
    let tx = CX - cx * scale;
    let ty = CY - cy * scale;
    format!(
        "<g transform=\"translate({:.1},{:.1}) scale({:.4})\">{}</g>",
        tx, ty, scale, extras
    )
}

// ─── Word layout (ORDER-INDEPENDENT) ─────────────────────────────
// Words are positioned by semantic role, not input order.
// "time is a circle" and "a circle is time" produce the SAME symbol.
// This makes the circle genuinely without beginning or end.

struct WM { word: CategorizedWord, angle: f64, arc: f64 }

fn layout(words: &[CategorizedWord]) -> Vec<WM> {
    let mut cw: Vec<CategorizedWord> = words.iter()
        .filter(|w| w.category != Category::Question)
        .cloned()
        .collect();

    if cw.is_empty() { return Vec::new(); }

    // Sort by category priority, then by word hash — NOT input order.
    cw.sort_by(|a, b| {
        language::category_priority(a.category)
            .cmp(&language::category_priority(b.category))
            .then_with(|| word_hash(&a.word).cmp(&word_hash(&b.word)))
    });

    // Cap at 20 words to keep the symbol readable
    cw.truncate(20);

    let n = cw.len();
    let arc = TAU / n as f64;

    // Rotation determined by CONTENT + ROLES, not input order.
    // "dog bites man" and "man bites dog" now produce DIFFERENT symbols
    // because the role assignments differ (agent/patient swap).
    let mut sorted_words: Vec<String> = cw.iter()
        .map(|w| format!("{}:{}", w.word.to_lowercase(), w.role.tag()))
        .collect();
    // Also include modifier attachment pairs
    for w in &cw {
        if let Some(mod_idx) = w.modifies {
            // Find the target word in the original word list
            // For the hash, encode the modifier-head relationship
            if mod_idx < cw.len() {
                sorted_words.push(format!("{}~{}", w.word.to_lowercase(), cw[mod_idx].word.to_lowercase()));
            }
        }
    }
    sorted_words.sort();
    let content_hash = language::fnv1a(sorted_words.join("\0").as_bytes());
    let base_rot = hash_to_float(content_hash) * TAU;

    cw.into_iter().enumerate().map(|(i, w)| WM {
        word: w,
        angle: base_rot + arc * i as f64 + arc / 2.0,
        arc,
    }).collect()
}

// ═════════════════════════════════════════════════════════════════
// MAIN GENERATION
// ═════════════════════════════════════════════════════════════════

pub fn generate(text: &str) -> SymbolData {
    let text = text.trim();
    let words = language::categorize_sentence(text);
    let has_question = words.iter().any(|w| w.category == Category::Question);
    let has_negation = words.iter().any(|w| w.category == Category::Negation);
    let wl = layout(&words);

    // Content hash: derived from role-tagged word set (post-truncation).
    // Must match the same computation used by layout() for consistency.
    let fh = if wl.is_empty() {
        42u64
    } else {
        let mut sorted: Vec<String> = wl.iter()
            .map(|w| format!("{}:{}", w.word.word.to_lowercase(), w.word.role.tag()))
            .collect();
        sorted.sort();
        language::fnv1a(sorted.join("\0").as_bytes())
    };

    let mut r = vec![BASE_R; N];
    let mut w = vec![0.0f64; N];

    // ── 1. Brush pressure (顿笔) ──
    let n_press = 2 + (hash_to_float(derive_hash(fh, 20)) > 0.55) as usize;
    let base_angle = hash_to_float(fh) * TAU;

    struct PZ { center: f64, sigma: f64, intensity: f64 }
    let presses: Vec<PZ> = (0..n_press).map(|p| PZ {
        center: base_angle + TAU * p as f64 / n_press as f64
            + (hash_to_float(derive_hash(fh, 22 + p as u64)) - 0.5) * 0.5,
        sigma: 0.20 + hash_to_float(derive_hash(fh, 25 + p as u64)) * 0.40,
        intensity: 0.8 + hash_to_float(derive_hash(fh, 28 + p as u64)) * 0.2,
    }).collect();

    for i in 0..N {
        let t = TAU * i as f64 / N as f64;
        let mut pressure = 0.0f64;
        for pz in &presses {
            let d = adist(t, pz.center);
            pressure = pressure.max((-d * d / (2.0 * pz.sigma * pz.sigma)).exp() * pz.intensity);
        }
        pressure = pressure.powf(0.3);
        w[i] = 4.0 + 42.0 * pressure;
    }

    // ── 2. Bone structure (骨法) ──
    let bp = hash_to_float(derive_hash(fh, 10)) * TAU;
    for i in 0..N {
        let t = TAU * i as f64 / N as f64;
        r[i] += 12.0 * (2.0 * t + bp).sin() + 7.0 * (3.0 * t + bp * 0.7).sin();
    }

    // ── 3. Word marks ──
    let mut marks = Vec::new();
    for wm in &wl {
        apply_mark(&mut r, &mut w, wm);
        let feature = match wm.word.category {
            Category::Entity => "outward tendril",
            Category::Action => "width wave",
            Category::Property => "inner ripple",
            Category::Relation => "bridge arc",
            Category::Particle => "notch",
            Category::Negation => "inward void",
            Category::Question => "gap",
        };

        let explanation = match wm.word.category {
            Category::Action => format!(
                "{} \u{2014} {} \u{2014} {}",
                wm.word.category.description(),
                wm.word.aspect.description(),
                wm.word.word
            ),
            _ => format!(
                "{} \u{2014} '{}'",
                wm.word.category.description(),
                wm.word.word
            ),
        };

        let lr = BASE_R + 95.0;
        marks.push(Mark {
            word: wm.word.word.clone(),
            category: wm.word.category,
            aspect: wm.word.aspect,
            role: wm.word.role,
            angle: wm.angle, arc: wm.arc,
            feature: feature.to_string(),
            explanation,
            label_x: CX + lr * wm.angle.cos(),
            label_y: CY - lr * wm.angle.sin(),
        });
    }

    // ── 4. Ink texture (墨韵) ──
    for i in 0..N {
        let t = TAU * i as f64 / N as f64;
        r[i] += fbm(t, fh) * 2.0;
    }

    // ── 5. Dry brush (飞白) ──
    let dry = 0.12 + hash_to_float(derive_hash(fh, 40)) * 0.18;
    for i in 0..N {
        let t = TAU * i as f64 / N as f64;
        let n = snoise(t, fh.wrapping_add(5555), 45) * 0.5 + 0.5;
        if n > (1.0 - dry) {
            w[i] *= 1.0 - ((n - (1.0 - dry)) / dry) * 0.55;
        }
    }

    // ── 6. Auto-taper (收笔) ──
    for i in 0..N {
        let out = (r[i] - BASE_R).max(0.0);
        let inv = (BASE_R - r[i]).max(0.0);
        let ot = (out / 55.0).min(1.0);
        w[i] *= 1.0 - ot * ot * 0.85;
        w[i] *= 1.0 - (inv / 45.0).min(1.0) * 0.55;
        w[i] = w[i].max(1.0);
    }

    // ── 7. Question gap ──
    let gap_a = if has_question { Some(PI / 2.0) } else { None };

    // ── 8. Embellishments ──
    let mut dots = generate_dots(&wl, &r, fh);
    let mut extras = generate_extras(&wl, &r, fh);

    // ── 9. Cartesian conversion ──
    let mut outer = Vec::with_capacity(N);
    let mut inner = Vec::with_capacity(N);
    for i in 0..N {
        let t = TAU * i as f64 / N as f64;
        let (mut ri, mut wi) = (r[i], w[i]);
        if let Some(ga) = gap_a {
            let d = adist(t, ga);
            if d < 0.20 {
                let fade = (d / 0.20).powi(3);
                wi *= fade;
                ri += (1.0 - fade) * 10.0;
            }
        }
        let ro = ri + wi / 2.0;
        let rin = (ri - wi / 2.0).max(5.0);
        outer.push([rd(CX + ro * t.cos()), rd(CY - ro * t.sin())]);
        inner.push([rd(CX + rin * t.cos()), rd(CY - rin * t.sin())]);
    }

    // ── 10. Normalize: scale everything to fit within viewBox with padding ──
    let pad = 15.0;
    let mut all_x: Vec<f64> = outer.iter().chain(inner.iter()).map(|p| p[0]).collect();
    let mut all_y: Vec<f64> = outer.iter().chain(inner.iter()).map(|p| p[1]).collect();
    for d in &dots {
        all_x.push(d[0] - d[2]); all_x.push(d[0] + d[2]);
        all_y.push(d[1] - d[2]); all_y.push(d[1] + d[2]);
    }
    let xmin = all_x.iter().cloned().fold(f64::INFINITY, f64::min);
    let xmax = all_x.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
    let ymin = all_y.iter().cloned().fold(f64::INFINITY, f64::min);
    let ymax = all_y.iter().cloned().fold(f64::NEG_INFINITY, f64::max);

    let need_scale = xmin < pad || ymin < pad || xmax > 600.0 - pad || ymax > 600.0 - pad;
    if need_scale {
        let content_w = xmax - xmin;
        let content_h = ymax - ymin;
        let avail = 600.0 - 2.0 * pad;
        let scale = (avail / content_w).min(avail / content_h).min(1.0);
        let cx_content = (xmin + xmax) / 2.0;
        let cy_content = (ymin + ymax) / 2.0;

        let transform = |p: &mut [f64; 2]| {
            p[0] = CX + (p[0] - cx_content) * scale;
            p[1] = CY + (p[1] - cy_content) * scale;
            p[0] = rd(p[0]);
            p[1] = rd(p[1]);
        };

        for p in &mut outer { transform(p); }
        for p in &mut inner { transform(p); }
        for d in &mut dots {
            d[0] = rd(CX + (d[0] - cx_content) * scale);
            d[1] = rd(CY + (d[1] - cy_content) * scale);
            d[2] = rd(d[2] * scale);
        }
        for m in &mut marks {
            m.label_x = rd(CX + (m.label_x - cx_content) * scale);
            m.label_y = rd(CY + (m.label_y - cy_content) * scale);
        }
        // Re-generate extras with scaled radius data is too complex,
        // so regenerate the extras SVG by scaling the coordinate values
        extras = scale_extras_svg(&extras, cx_content, cy_content, scale);
    }

    let svg = render_svg(&outer, &inner, &dots, &extras, fh);
    SymbolData { outer, inner, dots, extras, marks, has_question, has_negation, svg }
}

// ═════════════════════════════════════════════════════════════════
// MARK APPLICATION
// ═════════════════════════════════════════════════════════════════

fn apply_mark(r: &mut [f64], w: &mut [f64], wm: &WM) {
    match wm.word.category {
        Category::Entity   => entity_mark(r, w, wm),
        Category::Action   => action_mark(w, wm),
        Category::Property => property_mark(r, w, wm),
        Category::Relation => relation_mark(r, w, wm),
        Category::Particle => particle_mark(r, w, wm),
        Category::Negation => negation_mark(r, w, wm),
        Category::Question => {}
    }
}

/// Entity: INK SPLOTCH cluster with branching sub-tendrils.
/// Not a smooth bump, but an organic
/// cluster of overlapping ink marks with branches forking off.
fn entity_mark(r: &mut [f64], w: &mut [f64], wm: &WM) {
    let h = word_hash(&wm.word.word);

    // ── Main tendril ──
    let height = 45.0 + hash_to_float(h) * 40.0;
    let s_entry = 0.05 + hash_to_float(derive_hash(h, 1)) * 0.04;
    let s_exit = 0.11 + hash_to_float(derive_hash(h, 2)) * 0.07;
    let side = if hash_to_float(derive_hash(h, 3)) > 0.5 { 1.0 } else { -1.0 };

    let hook_off = s_exit * (1.2 + hash_to_float(derive_hash(h, 4)) * 0.8) * side;
    let hook_h = height * (0.15 + hash_to_float(derive_hash(h, 5)) * 0.20);
    let hook_s = s_exit * 0.35;

    let shoulder_off = -s_entry * 1.5 * side;
    let shoulder_h = height * 0.12;
    let shoulder_s = s_entry * 0.8;

    // ── Satellite splotches: irregular blobs around the main tendril ──
    let n_splotch = 3 + (hash_to_float(derive_hash(h, 10)) * 3.0) as usize;
    struct Splotch { angle: f64, height: f64, sigma: f64 }
    let splotches: Vec<Splotch> = (0..n_splotch).map(|s| {
        let off = (hash_to_float(derive_hash(h, 11 + s as u64)) - 0.5) * s_exit * 4.5;
        Splotch {
            angle: wm.angle + off,
            height: height * (0.12 + hash_to_float(derive_hash(h, 15 + s as u64)) * 0.30),
            sigma: s_exit * (0.20 + hash_to_float(derive_hash(h, 20 + s as u64)) * 0.45),
        }
    }).collect();

    // ── Branch sub-tendrils: thin spikes forking off the main tendril ──
    let n_branch = 1 + (hash_to_float(derive_hash(h, 30)) > 0.4) as usize;
    struct Branch { angle: f64, height: f64, sigma: f64 }
    let branches: Vec<Branch> = (0..n_branch).map(|b| {
        let off = (hash_to_float(derive_hash(h, 31 + b as u64)) - 0.3) * s_exit * 5.0;
        Branch {
            angle: wm.angle + off,
            height: height * (0.20 + hash_to_float(derive_hash(h, 33 + b as u64)) * 0.15),
            sigma: 0.012 + hash_to_float(derive_hash(h, 35 + b as u64)) * 0.010,
        }
    }).collect();

    for i in 0..N {
        let t = TAU * i as f64 / N as f64;
        let d = adist(t, wm.angle);
        let sd = signed_angle(t, wm.angle);

        let es = if sd * side > 0.0 { s_exit } else { s_entry };
        let main = height * (-d * d / (2.0 * es * es)).exp();

        let hd = adist(t, wm.angle + hook_off);
        let hook = hook_h * (-hd * hd / (2.0 * hook_s * hook_s)).exp();

        let shd = adist(t, wm.angle + shoulder_off);
        let shoulder = shoulder_h * (-shd * shd / (2.0 * shoulder_s * shoulder_s)).exp();

        let mut splotch = 0.0f64;
        for s in &splotches {
            let sd = adist(t, s.angle);
            splotch += s.height * (-sd * sd / (2.0 * s.sigma * s.sigma)).exp();
        }

        let mut branch = 0.0f64;
        for b in &branches {
            let bd = adist(t, b.angle);
            branch += b.height * (-bd * bd / (2.0 * b.sigma * b.sigma)).exp();
        }

        r[i] += main + hook + shoulder + splotch + branch;

        // Ink pooling at base
        let base_prox = (-d * d / (2.0 * (s_exit * 2.5).powi(2))).exp();
        w[i] += 14.0 * base_prox;
    }
}

/// Action: bold rhythmic width pulse.
fn action_mark(w: &mut [f64], wm: &WM) {
    let h = word_hash(&wm.word.word);
    let cycles = 2.0 + hash_to_float(h) * 3.0;
    let amp = 8.0 + hash_to_float(derive_hash(h, 1)) * 10.0;
    let phase = hash_to_float(derive_hash(h, 2)) * TAU;
    let half = wm.arc / 2.0;
    for i in 0..N {
        let t = TAU * i as f64 / N as f64;
        let d = adist(t, wm.angle);
        if d < half {
            let env = (1.0 - d / half).powi(2);
            w[i] += amp * (cycles * TAU * d / wm.arc + phase).sin() * env;
        }
    }
}

/// Property: serrated inner edge.
fn property_mark(r: &mut [f64], w: &mut [f64], wm: &WM) {
    let h = word_hash(&wm.word.word);
    let freq = 4.0 + hash_to_float(h) * 5.0;
    let amp = 5.0 + hash_to_float(derive_hash(h, 1)) * 6.0;
    let phase = hash_to_float(derive_hash(h, 2)) * TAU;
    let half = wm.arc / 2.0;
    for i in 0..N {
        let t = TAU * i as f64 / N as f64;
        let d = adist(t, wm.angle);
        if d < half {
            let env = (1.0 - (d / half).powi(2)).sqrt();
            let rip = (freq * TAU * d / wm.arc + phase).sin();
            r[i] -= amp * rip * env * 0.6;
            w[i] += amp * rip * env * 0.4;
        }
    }
}

/// Relation: silk-thin bridge with slight bow.
fn relation_mark(r: &mut [f64], w: &mut [f64], wm: &WM) {
    let h = word_hash(&wm.word.word);
    let thin = 0.20 + hash_to_float(h) * 0.20;
    let bow = 4.0 + hash_to_float(derive_hash(h, 1)) * 6.0;
    let half = wm.arc / 2.5;
    for i in 0..N {
        let t = TAU * i as f64 / N as f64;
        let d = adist(t, wm.angle);
        if d < half {
            let env = 1.0 - (d / half).powi(2);
            w[i] *= 1.0 - (1.0 - thin) * env;
            r[i] += bow * env;
        }
    }
}

/// Particle: decisive notch with ink press.
fn particle_mark(r: &mut [f64], w: &mut [f64], wm: &WM) {
    let h = word_hash(&wm.word.word);
    let depth = 8.0 + hash_to_float(h) * 8.0;
    let sigma = 0.02 + hash_to_float(derive_hash(h, 1)) * 0.02;
    let double = hash_to_float(derive_hash(h, 2)) > 0.6;
    for i in 0..N {
        let t = TAU * i as f64 / N as f64;
        let d = adist(t, wm.angle);
        let g = (-d * d / (2.0 * sigma * sigma)).exp();
        r[i] -= depth * g;
        w[i] += 4.0 * g;
        if double {
            let d2 = adist(t, wm.angle + sigma * 3.0);
            r[i] -= depth * 0.6 * (-d2 * d2 / (2.0 * sigma * sigma)).exp();
        }
    }
}

/// Negation: deep void with bold ink edges.
fn negation_mark(r: &mut [f64], w: &mut [f64], wm: &WM) {
    let h = word_hash(&wm.word.word);
    let depth = 35.0 + hash_to_float(h) * 25.0;
    let sigma = 0.10 + hash_to_float(derive_hash(h, 1)) * 0.06;
    for i in 0..N {
        let t = TAU * i as f64 / N as f64;
        let d = adist(t, wm.angle);
        r[i] -= depth * (-d * d / (2.0 * sigma * sigma)).exp();
        let edge_d = (d - sigma).abs();
        w[i] += 10.0 * (-edge_d * edge_d / (2.0 * (sigma * 0.35).powi(2))).exp();
    }
}

// ═════════════════════════════════════════════════════════════════
// EMBELLISHMENTS
// ═════════════════════════════════════════════════════════════════

/// Accent dots (墨点) — ink splashes near features
fn generate_dots(wl: &[WM], r: &[f64], fh: u64) -> Vec<[f64; 3]> {
    let mut dots = Vec::new();

    for wm in wl {
        let h = word_hash(&wm.word.word);
        match wm.word.category {
            Category::Entity => {
                // Ink splatter near tendril tip: 2-4 dots
                let nd = 2 + (hash_to_float(derive_hash(h, 30)) * 2.5) as usize;
                for j in 0..nd {
                    let off = (hash_to_float(derive_hash(h, 31 + j as u64)) - 0.5) * 0.18;
                    let da = wm.angle + off;
                    let idx = ((da / TAU * N as f64) as usize + N) % N;
                    let dr = r[idx] + 6.0 + hash_to_float(derive_hash(h, 35 + j as u64)) * 20.0;
                    let ds = 1.5 + hash_to_float(derive_hash(h, 40 + j as u64)) * 3.5;
                    dots.push([rd(CX + dr * da.cos()), rd(CY - dr * da.sin()), rd(ds)]);
                }
            }
            Category::Negation => {
                let dr = r[((wm.angle / TAU * N as f64) as usize) % N] - 10.0;
                let ds = 2.5 + hash_to_float(derive_hash(h, 30)) * 2.5;
                dots.push([rd(CX + dr * wm.angle.cos()), rd(CY - dr * wm.angle.sin()), rd(ds)]);
            }
            Category::Action => {
                if hash_to_float(derive_hash(h, 30)) > 0.45 {
                    let off = (hash_to_float(derive_hash(h, 32)) - 0.5) * 0.1;
                    let da = wm.angle + off;
                    let ds = 1.8 + hash_to_float(derive_hash(h, 31)) * 2.5;
                    dots.push([rd(CX + BASE_R * da.cos()), rd(CY - BASE_R * da.sin()), rd(ds)]);
                }
            }
            _ => {}
        }
    }

    // Scattered particles orbiting the ring (ink mist)
    let n_mist = 4 + (hash_to_float(derive_hash(fh, 60)) * 7.0) as usize;
    for p in 0..n_mist {
        let angle = hash_to_float(derive_hash(fh, 61 + p as u64)) * TAU;
        let dist = BASE_R * (0.45 + hash_to_float(derive_hash(fh, 70 + p as u64)) * 0.75);
        let size = 0.6 + hash_to_float(derive_hash(fh, 80 + p as u64)) * 1.8;
        dots.push([rd(CX + dist * angle.cos()), rd(CY - dist * angle.sin()), rd(size)]);
    }

    dots
}

/// Extra SVG embellishments: inner arcs, wispy lines, floating fragments
fn generate_extras(wl: &[WM], r: &[f64], fh: u64) -> String {
    let mut svg = String::new();

    for wm in wl {
        let h = word_hash(&wm.word.word);

        match wm.word.category {
            Category::Entity => {
                // ── Inner arc segment: partial ring floating inside ──
                if hash_to_float(derive_hash(h, 50)) > 0.35 {
                    let arc_r = BASE_R * (0.50 + hash_to_float(derive_hash(h, 51)) * 0.20);
                    let arc_len = 0.25 + hash_to_float(derive_hash(h, 52)) * 0.40;
                    let arc_w = 2.0 + hash_to_float(derive_hash(h, 53)) * 3.0;
                    let arc_off = (hash_to_float(derive_hash(h, 54)) - 0.5) * 0.15;
                    svg.push_str(&arc_path(wm.angle + arc_off, arc_len, arc_r, arc_w));
                }

                // ── Wispy line near tendril tip ──
                if hash_to_float(derive_hash(h, 55)) > 0.4 {
                    let idx = ((wm.angle / TAU * N as f64) as usize) % N;
                    let tip_r = r[idx] + 5.0;
                    let len = 10.0 + hash_to_float(derive_hash(h, 56)) * 15.0;
                    let la = wm.angle + (hash_to_float(derive_hash(h, 57)) - 0.5) * 0.12;
                    let lw = 1.0 + hash_to_float(derive_hash(h, 58)) * 1.2;
                    svg.push_str(&wisp_path(la, tip_r, len, lw));
                }

                // ── Second inner arc (smaller, different radius) ──
                if hash_to_float(derive_hash(h, 59)) > 0.6 {
                    let arc_r = BASE_R * (0.35 + hash_to_float(derive_hash(h, 60)) * 0.15);
                    let arc_len = 0.15 + hash_to_float(derive_hash(h, 61)) * 0.25;
                    let arc_w = 1.5 + hash_to_float(derive_hash(h, 62)) * 2.0;
                    let arc_off = (hash_to_float(derive_hash(h, 63)) - 0.5) * 0.3;
                    svg.push_str(&arc_path(wm.angle + arc_off, arc_len, arc_r, arc_w));
                }
            }

            Category::Action => {
                // Inner arc for some verbs
                if hash_to_float(derive_hash(h, 50)) > 0.50 {
                    let arc_r = BASE_R * (0.42 + hash_to_float(derive_hash(h, 51)) * 0.20);
                    let arc_len = 0.18 + hash_to_float(derive_hash(h, 52)) * 0.25;
                    let arc_w = 1.5 + hash_to_float(derive_hash(h, 53)) * 2.5;
                    svg.push_str(&arc_path(wm.angle, arc_len, arc_r, arc_w));
                }
            }

            Category::Negation => {
                // Wispy cracks radiating from void
                let n_wisps = 1 + (hash_to_float(derive_hash(h, 50)) > 0.5) as usize;
                for wj in 0..n_wisps {
                    let wa = wm.angle + (hash_to_float(derive_hash(h, 51 + wj as u64)) - 0.5) * 0.2;
                    let idx = ((wa / TAU * N as f64) as usize + N) % N;
                    let wr = r[idx] - 5.0;
                    let wlen = 12.0 + hash_to_float(derive_hash(h, 55 + wj as u64)) * 15.0;
                    // Point inward (toward center)
                    svg.push_str(&wisp_path_inward(wa, wr, wlen, 1.2));
                }
            }

            _ => {}
        }
    }

    // ── Floating ink fragments around the ring ──
    let n_frag = 2 + (hash_to_float(derive_hash(fh, 90)) * 4.0) as usize;
    for f in 0..n_frag {
        let fa = hash_to_float(derive_hash(fh, 91 + f as u64)) * TAU;
        let fr = BASE_R * (0.55 + hash_to_float(derive_hash(fh, 95 + f as u64)) * 0.60);
        let flen = 0.06 + hash_to_float(derive_hash(fh, 100 + f as u64)) * 0.12;
        let fw = 1.0 + hash_to_float(derive_hash(fh, 105 + f as u64)) * 2.0;
        svg.push_str(&arc_path(fa, flen, fr, fw));
    }

    svg
}

/// Generate a filled arc segment (partial ring)
fn arc_path(center: f64, arc_len: f64, radius: f64, width: f64) -> String {
    let n = 24;
    let start = center - arc_len / 2.0;
    let mut d = String::new();

    // Outer edge with tapered ends
    for i in 0..n {
        let frac = i as f64 / (n - 1) as f64;
        let t = start + arc_len * frac;
        // Taper: thin at edges, full in middle
        let taper = (frac * PI).sin();
        let ro = radius + width * taper / 2.0;
        let x = CX + ro * t.cos();
        let y = CY - ro * t.sin();
        if i == 0 { d = format!("M{:.1},{:.1}", x, y); }
        else { d.push_str(&format!("L{:.1},{:.1}", x, y)); }
    }

    // Inner edge (reversed, also tapered)
    for i in (0..n).rev() {
        let frac = i as f64 / (n - 1) as f64;
        let t = start + arc_len * frac;
        let taper = (frac * PI).sin();
        let ri = radius - width * taper / 2.0;
        let x = CX + ri * t.cos();
        let y = CY - ri * t.sin();
        d.push_str(&format!("L{:.1},{:.1}", x, y));
    }
    d.push('Z');

    format!(r#"<path d="{d}"/>"#)
}

/// Generate a wispy line (thin tapered stroke projecting outward)
fn wisp_path(angle: f64, start_r: f64, length: f64, width: f64) -> String {
    let nx = angle.sin();
    let ny = angle.cos();
    let hw = width / 2.0;

    let x1 = CX + start_r * angle.cos();
    let y1 = CY - start_r * angle.sin();
    let x2 = CX + (start_r + length) * angle.cos();
    let y2 = CY - (start_r + length) * angle.sin();

    format!(
        r#"<path d="M{:.1},{:.1}L{:.1},{:.1}L{:.1},{:.1}L{:.1},{:.1}Z"/>"#,
        x1 + nx * hw, y1 + ny * hw,
        x2 + nx * hw * 0.15, y2 + ny * hw * 0.15,
        x2 - nx * hw * 0.15, y2 - ny * hw * 0.15,
        x1 - nx * hw, y1 - ny * hw,
    )
}

/// Generate a wispy line projecting inward (toward center)
fn wisp_path_inward(angle: f64, start_r: f64, length: f64, width: f64) -> String {
    let nx = angle.sin();
    let ny = angle.cos();
    let hw = width / 2.0;

    let x1 = CX + start_r * angle.cos();
    let y1 = CY - start_r * angle.sin();
    let x2 = CX + (start_r - length) * angle.cos();
    let y2 = CY - (start_r - length) * angle.sin();

    format!(
        r#"<path d="M{:.1},{:.1}L{:.1},{:.1}L{:.1},{:.1}L{:.1},{:.1}Z"/>"#,
        x1 + nx * hw, y1 + ny * hw,
        x2 + nx * hw * 0.15, y2 + ny * hw * 0.15,
        x2 - nx * hw * 0.15, y2 - ny * hw * 0.15,
        x1 - nx * hw, y1 - ny * hw,
    )
}

// ═════════════════════════════════════════════════════════════════
// SVG RENDERING
// ═════════════════════════════════════════════════════════════════

fn render_svg(outer: &[[f64; 2]], inner: &[[f64; 2]], dots: &[[f64; 3]], extras: &str, seed: u64) -> String {
    let mut path = String::with_capacity(outer.len() * 24);

    path.push_str(&format!("M{:.1},{:.1}", outer[0][0], outer[0][1]));
    for p in &outer[1..] { path.push_str(&format!("L{:.1},{:.1}", p[0], p[1])); }
    path.push('Z');
    path.push_str(&format!("M{:.1},{:.1}", inner[0][0], inner[0][1]));
    for p in &inner[1..] { path.push_str(&format!("L{:.1},{:.1}", p[0], p[1])); }
    path.push('Z');

    let mut dot_svg = String::new();
    for d in dots {
        dot_svg.push_str(&format!(r#"<circle cx="{:.1}" cy="{:.1}" r="{:.1}"/>"#, d[0], d[1], d[2]));
    }

    let fs = (seed & 0xFFFF) as u32;

    format!(
        r#"<svg viewBox="0 0 600 600" xmlns="http://www.w3.org/2000/svg">
<defs>
<filter id="ti" x="-3%" y="-3%" width="106%" height="106%">
<feTurbulence type="fractalNoise" baseFrequency="0.025" numOctaves="5" seed="{fs}" result="n"/>
<feDisplacementMap in="SourceGraphic" in2="n" scale="2.2" xChannelSelector="R" yChannelSelector="G" result="r"/>
<feGaussianBlur in="r" stdDeviation="0.15" result="s"/>
<feGaussianBlur in="s" stdDeviation="2" result="g"/>
<feColorMatrix in="g" type="matrix" values="1 0 0 0 0 0 1 0 0 0 0 0 1 0 0 0 0 0 0.06 0" result="dg"/>
<feMerge><feMergeNode in="dg"/><feMergeNode in="s"/></feMerge>
</filter>
</defs>
<g fill="var(--torus-ink, #1a1a1a)" filter="url(#ti)">
<path d="{path}" fill-rule="evenodd"/>
{dot_svg}
{extras}
</g>
</svg>"#
    )
}

use axum::{
    routing::{get, post},
    Json, Router,
    response::Html,
    extract::{ConnectInfo, Multipart, Query},
    http::HeaderMap,
};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use std::io::Write;
use std::sync::{Arc, Mutex};
use std::collections::HashMap;

mod language;
mod symbol;
mod word_primes;
mod decode;

// ─── Visual fingerprint database ────────────────────────────────
// Every generated symbol is fingerprinted from its path geometry
// and stored here. Decode works by matching the visual fingerprint
// of an uploaded SVG — no metadata needed.

static SYMBOL_DB_PATH: &str = "/home/steven/torus/symbols.db";

fn load_symbol_db() -> HashMap<u64, String> {
    let mut db = HashMap::new();
    if let Ok(content) = std::fs::read_to_string(SYMBOL_DB_PATH) {
        for line in content.lines() {
            if let Some((fp, text)) = line.split_once('\t') {
                if let Ok(fp) = fp.parse::<u64>() {
                    db.insert(fp, text.to_string());
                }
            }
        }
    }
    db
}

pub fn visual_fingerprint(outer: &[[f64; 2]]) -> u64 {
    // Hash the first 30 points matching SVG {:.1} formatting precision.
    // Must use format-and-parse to match SVG exactly, because {:.1} uses
    // "round half to even" while .round() uses "round half away from zero".
    let mut buf = Vec::with_capacity(240);
    for p in outer.iter().take(30) {
        let xs: f64 = format!("{:.1}", p[0]).parse().unwrap_or(p[0]);
        let ys: f64 = format!("{:.1}", p[1]).parse().unwrap_or(p[1]);
        buf.extend_from_slice(&((xs * 10.0) as i32).to_le_bytes());
        buf.extend_from_slice(&((ys * 10.0) as i32).to_le_bytes());
    }
    language::fnv1a(&buf)
}

#[derive(Clone)]
struct AppState {
    og_cache: Arc<Mutex<HashMap<String, Vec<u8>>>>,
    symbol_db: Arc<Mutex<HashMap<u64, String>>>,
    word_index: Arc<decode::WordIndex>,
    single_word_fps: Arc<HashMap<u64, String>>,
}

#[derive(Deserialize)]
struct GenerateRequest {
    text: String,
}

#[derive(Serialize)]
struct GenerateResponse {
    outer: Vec<[f64; 2]>,
    inner: Vec<[f64; 2]>,
    dots: Vec<[f64; 3]>,
    extras: String,
    marks: Vec<symbol::Mark>,
    primes: Vec<PrimeGroup>,
    has_question: bool,
    has_negation: bool,
    svg: String,
    text: String,
}

#[derive(Serialize)]
struct PrimeGroup {
    word: String,
    primes: Vec<language::SemanticPrime>,
}

#[derive(Serialize)]
struct DecodeResponse {
    text: String,
    success: bool,
    method: Option<String>,
    analysis: Option<VisualAnalysis>,
    error: Option<String>,
}

#[derive(Serialize)]
struct VisualAnalysis {
    path_count: usize,
    dot_count: usize,
    has_gap: bool,
    complexity: String,
}

#[derive(Deserialize)]
struct OgQuery {
    t: Option<String>,
}

async fn index(Query(q): Query<OgQuery>) -> Html<String> {
    let template = include_str!("../static/index.html");
    // Inject dynamic OG image tag if text parameter present
    if let Some(ref text) = q.t {
        let encoded = urlencoding(text);
        let og_tag = format!(
            r#"<meta property="og:image" content="https://torus.steven-geller.com/og/{}">"#,
            encoded
        );
        let html = template.replacen(
            "<meta property=\"og:type\" content=\"website\">",
            &format!("<meta property=\"og:type\" content=\"website\">\n{}", og_tag),
            1,
        );
        Html(html)
    } else {
        Html(template.to_string())
    }
}

const MAX_TEXT_LEN: usize = 1000;
const MAX_ACCESS_LOG_BYTES: u64 = 5_000_000; // 5MB
const MAX_SYMBOL_DB_ENTRIES: usize = 10_000;

async fn generate(
    headers: HeaderMap,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    axum::extract::State(state): axum::extract::State<AppState>,
    Json(req): Json<GenerateRequest>,
) -> Json<GenerateResponse> {
    let text = if req.text.len() > MAX_TEXT_LEN {
        req.text[..MAX_TEXT_LEN].to_string()
    } else {
        req.text.clone()
    };

    // Log input with IP and timestamp
    if !text.is_empty() {
        let ip = headers.get("x-forwarded-for")
            .and_then(|v| v.to_str().ok())
            .map(|s| s.split(',').next().unwrap_or("").trim().to_string())
            .unwrap_or_else(|| addr.ip().to_string());
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_secs())
            .unwrap_or(0);
        // Format: ISO-ish timestamp \t IP \t text
        let secs = now;
        let days_since_epoch = secs / 86400;
        let time_of_day = secs % 86400;
        let hours = time_of_day / 3600;
        let minutes = (time_of_day % 3600) / 60;
        let seconds = time_of_day % 60;

        // Approximate date (good enough for logging)
        let (year, month, day) = epoch_days_to_date(days_since_epoch);

        let log_ok = std::fs::metadata("/home/steven/torus/access.log")
            .map(|m| m.len() < MAX_ACCESS_LOG_BYTES)
            .unwrap_or(true);
        if log_ok {
            if let Ok(mut f) = std::fs::OpenOptions::new()
                .create(true).append(true)
                .open("/home/steven/torus/access.log")
            {
                let _ = writeln!(f, "{:04}-{:02}-{:02}T{:02}:{:02}:{:02}Z\t{}\t{}",
                    year, month, day, hours, minutes, seconds, ip, text);
            }
        }
    }

    let data = symbol::generate(&text);

    // Decompose each word into semantic primes
    let primes: Vec<PrimeGroup> = data.marks.iter().map(|m| PrimeGroup {
        word: m.word.clone(),
        primes: language::decompose_to_primes(&m.word),
    }).collect();

    // Store visual fingerprint for decode-by-geometry
    if !text.is_empty() {
        let fp = visual_fingerprint(&data.outer);
        if let Ok(mut db) = state.symbol_db.lock() {
            if !db.contains_key(&fp) && db.len() < MAX_SYMBOL_DB_ENTRIES {
                db.insert(fp, text.clone());
                if let Ok(mut f) = std::fs::OpenOptions::new()
                    .create(true).append(true).open(SYMBOL_DB_PATH) {
                    let _ = writeln!(f, "{}\t{}", fp, text);
                }
            }
        }
    }

    // Downsample points: 720 → 360 for smaller API response
    let outer: Vec<[f64; 2]> = data.outer.iter().step_by(2).copied().collect();
    let inner: Vec<[f64; 2]> = data.inner.iter().step_by(2).copied().collect();

    Json(GenerateResponse {
        outer,
        inner,
        dots: data.dots,
        extras: data.extras,
        marks: data.marks,
        primes,
        has_question: data.has_question,
        has_negation: data.has_negation,
        svg: data.svg,
        text,
    })
}

/// OG image endpoint with in-memory cache
async fn og_image(
    axum::extract::Path(text): axum::extract::Path<String>,
    axum::extract::State(state): axum::extract::State<AppState>,
) -> axum::response::Response {
    let decoded = urldecode(&text);

    // Check cache
    if let Ok(c) = state.og_cache.lock() {
        if let Some(png) = c.get(&decoded) {
            return axum::response::Response::builder()
                .header("Content-Type", "image/png")
                .header("Cache-Control", "public, max-age=604800")
                .body(axum::body::Body::from(png.clone()))
                .unwrap();
        }
    }

    let data = symbol::generate(&decoded);
    let svg_str = data.svg
        .replace("var(--torus-ink, #1a1a1a)", "#e8e0d4")
        .replace("var(--torus-ink,#1a1a1a)", "#e8e0d4");

    let svg_inner = svg_str
        .replace("<svg viewBox=\"0 0 600 600\" xmlns=\"http://www.w3.org/2000/svg\">", "")
        .replace("</svg>", "");

    let full_svg = format!(
        "<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"1200\" height=\"630\" viewBox=\"0 0 1200 630\">\
        <rect width=\"1200\" height=\"630\" fill=\"#121212\"/>\
        <g transform=\"translate(300, 15) scale(1.0)\">{}</g>\
        <text x=\"600\" y=\"610\" text-anchor=\"middle\" fill=\"#555\" font-family=\"monospace\" font-size=\"14\">TORUS</text>\
        </svg>",
        svg_inner
    );

    let opt = resvg::usvg::Options::default();
    match resvg::usvg::Tree::from_str(&full_svg, &opt) {
        Ok(tree) => {
            let size = tree.size();
            let w = size.width() as u32;
            let h = size.height() as u32;
            let mut pixmap = match resvg::tiny_skia::Pixmap::new(w, h) {
                Some(p) => p,
                None => return error_png(),
            };
            resvg::render(&tree, resvg::tiny_skia::Transform::default(), &mut pixmap.as_mut());
            let png = pixmap.encode_png().unwrap_or_default();

            if let Ok(mut c) = state.og_cache.lock() {
                if c.len() > 500 { c.clear(); }
                c.insert(decoded, png.clone());
            }

            axum::response::Response::builder()
                .header("Content-Type", "image/png")
                .header("Cache-Control", "public, max-age=604800")
                .body(axum::body::Body::from(png))
                .unwrap()
        }
        Err(_) => error_png(),
    }
}

/// Square PNG for download (600x600, dark bg)
async fn png_download(
    axum::extract::Path(text): axum::extract::Path<String>,
) -> axum::response::Response {
    let decoded = urldecode(&text);
    let data = symbol::generate(&decoded);
    let svg_str = data.svg
        .replace("var(--torus-ink, #1a1a1a)", "#e8e0d4")
        .replace("var(--torus-ink,#1a1a1a)", "#e8e0d4");

    let full_svg = format!(
        "<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"800\" height=\"800\" viewBox=\"-100 -100 800 800\">\
        <rect x=\"-100\" y=\"-100\" width=\"800\" height=\"800\" fill=\"#121212\"/>\
        {}</svg>",
        svg_str
            .replace("<svg viewBox=\"0 0 600 600\" xmlns=\"http://www.w3.org/2000/svg\">", "")
            .replace("</svg>", "")
    );

    let opt = resvg::usvg::Options::default();
    match resvg::usvg::Tree::from_str(&full_svg, &opt) {
        Ok(tree) => {
            let size = tree.size();
            let mut pixmap = match resvg::tiny_skia::Pixmap::new(size.width() as u32, size.height() as u32) {
                Some(p) => p,
                None => return error_png(),
            };
            resvg::render(&tree, resvg::tiny_skia::Transform::default(), &mut pixmap.as_mut());
            let png = pixmap.encode_png().unwrap_or_default();
            axum::response::Response::builder()
                .header("Content-Type", "image/png")
                .header("Content-Disposition", "attachment; filename=\"torus-symbol.png\"")
                .body(axum::body::Body::from(png))
                .unwrap()
        }
        Err(_) => error_png(),
    }
}

/// Pre-compute fingerprints for common words and phrases
/// so they can be decoded even if never explicitly generated by a user.
fn precompute_common_symbols(db: &mut HashMap<u64, String>) -> usize {
    let mut count = 0;

    // Common single words
    let words = [
        "love", "time", "death", "life", "god", "peace", "war", "truth",
        "beauty", "soul", "consciousness", "dream", "hope", "fear",
        "home", "world", "universe", "light", "darkness", "silence",
        "fire", "water", "earth", "sky", "sun", "moon", "star",
        "heart", "mind", "body", "spirit", "voice", "music",
        "power", "freedom", "wisdom", "knowledge", "infinity",
        "nothing", "everything", "beginning", "end", "space",
        "creation", "destruction", "journey", "arrival", "memory",
        "sleep", "pain", "joy", "anger", "sadness", "happiness",
        "child", "father", "mother", "friend", "king", "queen",
    ];

    // Common phrases
    let phrases = [
        "time is a circle", "a circle is time", "I love you",
        "you love me", "what is truth", "where is my home",
        "I dream of infinite space", "the world is not what it seems",
        "love is eternal", "eternal is love",
        "dog bites man", "man bites dog",
        "time has no beginning and no end",
        "consciousness is infinite", "I am here",
        "where are you", "who am I", "what is love",
        "the sun rises", "darkness falls",
        "life and death", "war and peace",
        "only I love you", "I only love you",
        "the big dog", "big the dog",
    ];

    for text in words.iter().chain(phrases.iter()) {
        let data = symbol::generate(text);
        let fp = visual_fingerprint(&data.outer);
        if !db.contains_key(&fp) {
            db.insert(fp, text.to_string());
            count += 1;
        }
    }

    // Persist to disk
    if count > 0 {
        if let Ok(mut f) = std::fs::OpenOptions::new()
            .create(true).append(true).open(SYMBOL_DB_PATH) {
            for (fp, text) in db.iter() {
                let _ = writeln!(f, "{}\t{}", fp, text);
            }
        }
    }

    count
}

async fn robots_txt() -> &'static str {
    "User-agent: *\nAllow: /\nDisallow: /api/\n\nSitemap: https://torus.steven-geller.com/sitemap.xml\n"
}

async fn sitemap() -> axum::response::Response {
    axum::response::Response::builder()
        .header("Content-Type", "application/xml")
        .body(axum::body::Body::from(
            "<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n\
            <urlset xmlns=\"http://www.sitemaps.org/schemas/sitemap/0.9\">\n\
            <url><loc>https://torus.steven-geller.com/</loc><changefreq>weekly</changefreq></url>\n\
            </urlset>"
        ))
        .unwrap()
}

fn error_png() -> axum::response::Response {
    axum::response::Response::builder()
        .status(500)
        .header("Content-Type", "text/plain")
        .body(axum::body::Body::from("SVG render failed"))
        .unwrap()
}

async fn decode(
    axum::extract::State(state): axum::extract::State<AppState>,
    mut multipart: Multipart,
) -> Json<DecodeResponse> {
    while let Ok(Some(field)) = multipart.next_field().await {
        let name = field.name().map(|s| s.to_string());
        if name.as_deref() == Some("file") {
            if let Ok(data) = field.bytes().await {
                let content = String::from_utf8_lossy(&data);
                let analysis = analyze_svg(&content);

                // METHOD 1: Embedded data-torus-text attribute (SVGs downloaded from this server)
                if let Some(text) = decode::decode_from_attribute(&content) {
                    // Verify by regeneration
                    let verify = symbol::generate(&text);
                    let (outer_parsed, _) = decode::extract_svg_paths(&content);
                    if outer_parsed.len() >= 30 {
                        let fp_input = visual_fingerprint(&outer_parsed);
                        let fp_verify = visual_fingerprint(&verify.outer);
                        if fp_input == fp_verify {
                            return Json(DecodeResponse {
                                text,
                                success: true,
                                method: Some("embedded metadata (verified)".to_string()),
                                analysis: Some(analysis),
                                error: None,
                            });
                        }
                    }
                    // Even without verification, trust the attribute
                    return Json(DecodeResponse {
                        text,
                        success: true,
                        method: Some("embedded metadata".to_string()),
                        analysis: Some(analysis),
                        error: None,
                    });
                }

                // Parse full geometry
                let (outer, inner) = decode::extract_svg_paths(&content);

                // METHOD 2: Fingerprint database lookup (symbols generated on this server)
                if outer.len() >= 30 {
                    let fp = visual_fingerprint(&outer);

                    // Check generation database
                    if let Ok(db) = state.symbol_db.lock() {
                        if let Some(text) = db.get(&fp) {
                            return Json(DecodeResponse {
                                text: text.clone(),
                                success: true,
                                method: Some("visual geometry match".to_string()),
                                analysis: Some(analysis),
                                error: None,
                            });
                        }
                    }

                    // METHOD 3: Single-word fingerprint lookup (covers entire dictionary)
                    if let Some(text) = state.single_word_fps.get(&fp) {
                        return Json(DecodeResponse {
                            text: text.clone(),
                            success: true,
                            method: Some("dictionary fingerprint match".to_string()),
                            analysis: Some(analysis),
                            error: None,
                        });
                    }
                }

                // METHOD 4: Geometric reverse-engineering (detect marks, search candidates)
                if outer.len() >= 100 && inner.len() >= 100 {
                    if let Some(result) = decode::decode_geometric(&outer, &inner, &state.word_index) {
                        return Json(DecodeResponse {
                            text: result.text,
                            success: true,
                            method: Some(result.method),
                            analysis: Some(analysis),
                            error: None,
                        });
                    }
                }

                return Json(DecodeResponse {
                    text: String::new(),
                    success: false,
                    method: None,
                    analysis: Some(analysis),
                    error: Some("Symbol not recognized. Upload a Torus SVG to decode it.".to_string()),
                });
            }
        }
    }
    Json(DecodeResponse {
        text: String::new(),
        success: false,
        method: None,
        analysis: None,
        error: Some("No file uploaded.".to_string()),
    })
}

fn analyze_svg(content: &str) -> VisualAnalysis {
    let path_count = content.matches("<path ").count();
    let dot_count = content.matches("<circle ").count();

    // Detect question gap by looking for very thin width sections
    // (the gap creates a region where stroke width approaches 0)
    let has_gap = content.contains("fill-rule=\"evenodd\"")
        && path_count > 0;

    let complexity = if path_count > 8 && dot_count > 10 {
        "high — complex thought with multiple entities and modifiers"
    } else if path_count > 4 || dot_count > 5 {
        "medium — several concepts interwoven"
    } else if path_count > 1 {
        "low — a focused idea"
    } else {
        "minimal — a single concept or empty circle"
    };

    VisualAnalysis {
        path_count,
        dot_count,
        has_gap,
        complexity: complexity.to_string(),
    }
}

fn base64_encode(input: &str) -> String {
    let table = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    let bytes = input.as_bytes();
    let mut result = String::new();
    for chunk in bytes.chunks(3) {
        let b0 = chunk[0] as u32;
        let b1 = if chunk.len() > 1 { chunk[1] as u32 } else { 0 };
        let b2 = if chunk.len() > 2 { chunk[2] as u32 } else { 0 };
        let val = (b0 << 16) | (b1 << 8) | b2;
        result.push(table[(val >> 18 & 0x3F) as usize] as char);
        result.push(table[(val >> 12 & 0x3F) as usize] as char);
        if chunk.len() > 1 { result.push(table[(val >> 6 & 0x3F) as usize] as char); }
        else { result.push('='); }
        if chunk.len() > 2 { result.push(table[(val & 0x3F) as usize] as char); }
        else { result.push('='); }
    }
    result
}

/// Simple URL encoding for OG image paths
fn urlencoding(s: &str) -> String {
    let mut result = String::new();
    for b in s.bytes() {
        match b {
            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'_' | b'.' => {
                result.push(b as char);
            }
            b' ' => result.push_str("%20"),
            _ => result.push_str(&format!("%{:02X}", b)),
        }
    }
    result
}

fn epoch_days_to_date(days: u64) -> (u64, u64, u64) {
    // Compute year/month/day from days since Unix epoch
    let mut y = 1970;
    let mut remaining = days;
    loop {
        let days_in_year = if y % 4 == 0 && (y % 100 != 0 || y % 400 == 0) { 366 } else { 365 };
        if remaining < days_in_year { break; }
        remaining -= days_in_year;
        y += 1;
    }
    let leap = y % 4 == 0 && (y % 100 != 0 || y % 400 == 0);
    let mdays = [31, if leap {29} else {28}, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
    let mut m = 0;
    while m < 12 && remaining >= mdays[m] {
        remaining -= mdays[m];
        m += 1;
    }
    (y, (m + 1) as u64, (remaining + 1) as u64)
}

fn urldecode(s: &str) -> String {
    let mut result = Vec::new();
    let bytes = s.as_bytes();
    let mut i = 0;
    while i < bytes.len() {
        if bytes[i] == b'%' && i + 2 < bytes.len() {
            if let Ok(val) = u8::from_str_radix(
                &String::from_utf8_lossy(&bytes[i + 1..i + 3]), 16
            ) {
                result.push(val);
                i += 3;
                continue;
            }
        }
        result.push(bytes[i]);
        i += 1;
    }
    String::from_utf8_lossy(&result).to_string()
}

async fn generate_svg(Json(req): Json<GenerateRequest>) -> axum::response::Response {
    let text = if req.text.len() > MAX_TEXT_LEN {
        req.text[..MAX_TEXT_LEN].to_string()
    } else {
        req.text.clone()
    };
    let data = symbol::generate(&text);
    let encoded = base64_encode(&text);
    let svg = data.svg.replacen(
        "<svg ",
        &format!("<svg data-torus-text=\"{}\" ", encoded),
        1,
    );
    axum::response::Response::builder()
        .header("Content-Type", "image/svg+xml")
        .header("Content-Disposition", "attachment; filename=\"torus-symbol.svg\"")
        .body(axum::body::Body::from(svg))
        .unwrap()
}

#[tokio::main]
async fn main() {
    let mut db = load_symbol_db();
    let pre = precompute_common_symbols(&mut db);
    eprintln!("Pre-computed {} common symbol fingerprints", pre);

    let word_index = decode::WordIndex::build();
    eprintln!("Word index: {} words across {} categories",
        word_index.all_words.len(), word_index.by_category.len());

    let single_word_fps = decode::precompute_single_word_fingerprints();
    eprintln!("Pre-computed {} single-word decode fingerprints", single_word_fps.len());

    let state = AppState {
        og_cache: Arc::new(Mutex::new(HashMap::new())),
        symbol_db: Arc::new(Mutex::new(db)),
        word_index: Arc::new(word_index),
        single_word_fps: Arc::new(single_word_fps),
    };

    let app = Router::new()
        .route("/", get(index))
        .route("/api/generate", post(generate))
        .route("/api/generate-svg", post(generate_svg))
        .route("/api/decode", post(decode))
        .route("/og/{text}", get(og_image))
        .route("/png/{text}", get(png_download))
        .route("/robots.txt", get(robots_txt))
        .route("/sitemap.xml", get(sitemap))
        .with_state(state);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3031));
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    eprintln!("Torus listening on {addr}");
    axum::serve(
        listener,
        app.into_make_service_with_connect_info::<SocketAddr>(),
    ).await.unwrap();
}

/// Geometric symbol decoder for Torus.
///
/// Three decode methods in priority order:
/// 1. Extract embedded `data-torus-text` attribute (SVGs downloaded from this server)
/// 2. Fingerprint lookup (symbols previously generated on this server)
/// 3. Geometric reverse-engineering (detect marks, classify, search dictionary)
use crate::language::{self, Category};
use crate::symbol;
use std::collections::HashMap;

pub struct DecodeResult {
    pub text: String,
    pub method: String,
}

/// Try to extract text from the data-torus-text attribute embedded in downloaded SVGs.
pub fn decode_from_attribute(svg: &str) -> Option<String> {
    let marker = "data-torus-text=\"";
    let start = svg.find(marker)? + marker.len();
    let rest = &svg[start..];
    let end = rest.find('"')?;
    let encoded = &rest[..end];
    base64_decode(encoded)
}

fn base64_decode(input: &str) -> Option<String> {
    let table = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    let mut bytes = Vec::new();
    let chars: Vec<u8> = input.bytes().filter(|&b| b != b'=').collect();
    for chunk in chars.chunks(4) {
        let mut val: u32 = 0;
        for (j, &c) in chunk.iter().enumerate() {
            let idx = table.iter().position(|&t| t == c)? as u32;
            val |= idx << (6 * (3 - j));
        }
        bytes.push((val >> 16) as u8);
        if chunk.len() > 2 {
            bytes.push((val >> 8) as u8);
        }
        if chunk.len() > 3 {
            bytes.push(val as u8);
        }
    }
    String::from_utf8(bytes).ok()
}

/// Extract both outer and inner coordinate arrays from SVG path data.
/// The Torus SVG has: <path d="M{outer}ZM{inner}Z" fill-rule="evenodd"/>
pub fn extract_svg_paths(svg: &str) -> (Vec<[f64; 2]>, Vec<[f64; 2]>) {
    let d_start = match svg.find("d=\"M") {
        Some(i) => i + 3,
        None => return (Vec::new(), Vec::new()),
    };
    let rest = &svg[d_start..];
    let d_end = match rest.find('"') {
        Some(i) => i,
        None => return (Vec::new(), Vec::new()),
    };
    let d = &rest[..d_end];

    // Split on 'Z' to get path segments
    let segments: Vec<&str> = d.split('Z').collect();

    let parse_segment = |seg: &str| -> Vec<[f64; 2]> {
        let mut coords = Vec::new();
        for part in seg.split(|c: char| c == 'M' || c == 'L') {
            let part = part.trim();
            if part.is_empty() {
                continue;
            }
            if let Some(comma) = part.find(',') {
                if let (Ok(x), Ok(y)) = (
                    part[..comma].parse::<f64>(),
                    part[comma + 1..].parse::<f64>(),
                ) {
                    coords.push([x, y]);
                }
            }
        }
        coords
    };

    let outer = if segments.len() > 0 {
        parse_segment(segments[0])
    } else {
        Vec::new()
    };
    let inner = if segments.len() > 1 {
        parse_segment(segments[1])
    } else {
        Vec::new()
    };

    (outer, inner)
}

/// Build a reverse index: word_hash fingerprint -> word, for all known words.
/// Used for geometric parameter matching.
pub struct WordIndex {
    pub by_category: HashMap<u8, Vec<(String, u64)>>, // category_priority -> [(word, hash)]
    pub all_words: Vec<String>,
}

impl WordIndex {
    pub fn build() -> Self {
        let mut by_category: HashMap<u8, Vec<(String, u64)>> = HashMap::new();
        let mut all_words = Vec::new();
        let mut seen = std::collections::HashSet::new();

        // Collect from dictionary and word_primes
        let test_words = get_all_known_words();

        for word in test_words {
            if seen.contains(&word) {
                continue;
            }
            seen.insert(word.clone());
            let cat = language::categorize(&word);
            let priority = language::category_priority(cat);
            let hash = language::word_hash(&word);
            by_category
                .entry(priority)
                .or_default()
                .push((word.clone(), hash));
            all_words.push(word);
        }

        WordIndex {
            by_category,
            all_words,
        }
    }

    pub fn words_for_category(&self, cat: Category) -> &[(String, u64)] {
        let priority = language::category_priority(cat);
        self.by_category
            .get(&priority)
            .map(|v| v.as_slice())
            .unwrap_or(&[])
    }
}

/// Precompute single-word fingerprints for all dictionary words.
/// Returns a map from fingerprint -> word text.
pub fn precompute_single_word_fingerprints() -> HashMap<u64, String> {
    let mut map = HashMap::new();
    let words = get_all_known_words();
    for word in &words {
        let data = symbol::generate(word);
        let fp = crate::visual_fingerprint(&data.outer);
        map.entry(fp).or_insert_with(|| word.clone());
    }
    map
}

/// Attempt geometric decoding: detect marks, classify, search for matching words.
pub fn decode_geometric(
    outer: &[[f64; 2]],
    inner: &[[f64; 2]],
    word_index: &WordIndex,
) -> Option<DecodeResult> {
    let n = outer.len();
    if n < 100 {
        return None;
    } // too few points

    let cx = 300.0;
    let cy = 300.0;

    // Step 1: Convert to polar
    let mut r_mid = vec![0.0f64; n];
    let mut width = vec![0.0f64; n];
    for i in 0..n {
        let ro = ((outer[i][0] - cx).powi(2) + (outer[i][1] - cy).powi(2)).sqrt();
        let ri = ((inner[i][0] - cx).powi(2) + (inner[i][1] - cy).powi(2)).sqrt();
        r_mid[i] = (ro + ri) / 2.0;
        width[i] = ro - ri;
    }

    // Step 2: Compute smooth baseline (wide window to remove mark effects)
    let baseline = smooth_circular(&r_mid, n / 6);

    // Step 3: Compute deviations from baseline
    let dev: Vec<f64> = r_mid
        .iter()
        .zip(baseline.iter())
        .map(|(r, b)| r - b)
        .collect();

    // Step 4: Find significant peaks (entities, negations) and width anomalies (actions)
    let marks = detect_marks(&dev, &width, n);

    if marks.is_empty() {
        // Might be a simple circle (empty input) or single word with subtle marks
        // Try brute force with single words
        return brute_force_single(outer);
    }

    let mark_count = marks.len();

    // Step 5: For each mark, determine category and extract primary parameter
    let classified: Vec<ClassifiedMark> = marks
        .iter()
        .map(|m| classify_mark(m, &dev, &width, n))
        .collect();

    // Step 6: Search for matching word combinations
    search_candidates(&classified, mark_count, outer, word_index)
}

struct DetectedMark {
    center_idx: usize,   // index in the point array
    peak_value: f64,     // deviation at peak (positive=outward, negative=inward)
    width_variance: f64, // width variation in the mark region
}

struct ClassifiedMark {
    category: Category,
    _center_idx: usize,
    // Primary parameter derived from the geometry (maps to hash_to_float)
    primary_param: f64,
}

fn smooth_circular(data: &[f64], window: usize) -> Vec<f64> {
    let n = data.len();
    let half = window / 2;
    let mut result = vec![0.0; n];
    for i in 0..n {
        let mut sum = 0.0;
        let mut count = 0;
        for j in 0..window {
            let idx = (i + n - half + j) % n;
            sum += data[idx];
            count += 1;
        }
        result[i] = sum / count as f64;
    }
    result
}

fn detect_marks(dev: &[f64], width: &[f64], n: usize) -> Vec<DetectedMark> {
    let mut marks = Vec::new();
    let threshold = 15.0; // minimum deviation to count as a mark
    let min_distance = n / 20; // minimum distance between mark centers

    // Find peaks (outward: entities)
    let mut i = 0;
    while i < n {
        if dev[i].abs() > threshold {
            // Find the peak in this region
            let mut best_idx = i;
            let mut best_val = dev[i].abs();
            let mut j = i + 1;
            while j < n && j < i + n / 4 {
                if dev[j].abs() > best_val {
                    best_val = dev[j].abs();
                    best_idx = j;
                }
                if dev[j].abs() < threshold / 2.0 {
                    break;
                }
                j += 1;
            }

            // Compute width variance in the mark region
            let region_start = if best_idx > n / 20 {
                best_idx - n / 20
            } else {
                0
            };
            let region_end = (best_idx + n / 20).min(n);
            let w_slice: Vec<f64> = (region_start..region_end).map(|k| width[k]).collect();
            let w_var = if w_slice.is_empty() {
                0.0
            } else {
                let w_mean: f64 = w_slice.iter().sum::<f64>() / w_slice.len() as f64;
                w_slice.iter().map(|w| (w - w_mean).powi(2)).sum::<f64>() / w_slice.len() as f64
            };

            marks.push(DetectedMark {
                center_idx: best_idx,
                peak_value: dev[best_idx],
                width_variance: w_var,
            });

            i = j + min_distance;
        } else {
            i += 1;
        }
    }

    // Also detect action marks (width oscillation without big radius deviation)
    // by looking for high width variance regions that don't overlap with existing marks
    let width_smooth = smooth_circular(width, n / 10);
    let width_dev: Vec<f64> = width
        .iter()
        .zip(width_smooth.iter())
        .map(|(w, s)| (w - s).abs())
        .collect();
    let w_dev_smooth = smooth_circular(&width_dev, n / 20);

    for i in 0..n {
        if w_dev_smooth[i] > 3.0 {
            // Check it's not near an existing mark
            let near_existing = marks.iter().any(|m| {
                let d = (i as i64 - m.center_idx as i64).unsigned_abs() as usize;
                let d = d.min(n - d);
                d < min_distance
            });
            if !near_existing {
                // Find local maximum of width deviation
                let mut best_idx = i;
                let mut best_val = w_dev_smooth[i];
                let mut j = i + 1;
                while j < n && j < i + n / 8 {
                    if w_dev_smooth[j] > best_val {
                        best_val = w_dev_smooth[j];
                        best_idx = j;
                    }
                    if w_dev_smooth[j] < 2.0 {
                        break;
                    }
                    j += 1;
                }
                marks.push(DetectedMark {
                    center_idx: best_idx,
                    peak_value: dev[best_idx], // small radius deviation
                    width_variance: best_val * best_val * 100.0, // high width variance
                });
                // Skip past this region
                // (not incrementing i since we're in a separate loop)
            }
        }
    }

    // Sort by angular position
    marks.sort_by_key(|m| m.center_idx);

    // Deduplicate: merge marks that are too close
    let mut deduped: Vec<DetectedMark> = Vec::new();
    for m in marks {
        if let Some(last) = deduped.last() {
            let d = (m.center_idx as i64 - last.center_idx as i64).unsigned_abs() as usize;
            let d = d.min(n - d);
            if d < min_distance {
                continue; // skip duplicate
            }
        }
        deduped.push(m);
    }

    deduped
}

fn classify_mark(mark: &DetectedMark, _dev: &[f64], _width: &[f64], _n: usize) -> ClassifiedMark {
    let peak = mark.peak_value;
    let w_var = mark.width_variance;

    let category = if peak > 30.0 {
        // Large outward projection = Entity
        Category::Entity
    } else if peak < -25.0 {
        // Large inward void = Negation
        Category::Negation
    } else if peak < -5.0 && peak > -20.0 {
        // Small inward notch = Particle
        Category::Particle
    } else if w_var > 50.0 && peak.abs() < 15.0 {
        // High width variation, small radius deviation = Action
        Category::Action
    } else if peak > 2.0 && peak < 15.0 {
        // Small outward bow with thinning = Relation
        Category::Relation
    } else {
        // Moderate inner edge modulation = Property
        Category::Property
    };

    // Extract primary parameter (maps to hash_to_float of the word hash)
    let primary_param = match category {
        Category::Entity => {
            // height = 45.0 + hash_to_float(h) * 40.0
            // So hash_to_float(h) = (height - 45.0) / 40.0
            ((peak - 45.0) / 40.0).clamp(0.0, 1.0)
        }
        Category::Negation => {
            // depth = 35.0 + hash_to_float(h) * 25.0
            ((-peak - 35.0) / 25.0).clamp(0.0, 1.0)
        }
        Category::Particle => {
            // depth = 8.0 + hash_to_float(h) * 8.0
            ((-peak - 8.0) / 8.0).clamp(0.0, 1.0)
        }
        Category::Action => {
            // cycles = 2.0 + hash_to_float(h) * 3.0
            // Hard to extract from width variance alone, use a rough estimate
            (w_var.sqrt() / 15.0).clamp(0.0, 1.0)
        }
        _ => 0.5, // default for relation/property
    };

    ClassifiedMark {
        category,
        _center_idx: mark.center_idx,
        primary_param,
    }
}

fn brute_force_single(outer: &[[f64; 2]]) -> Option<DecodeResult> {
    let words = get_all_known_words();
    let input_fp = crate::visual_fingerprint(outer);

    for word in &words {
        let data = symbol::generate(word);
        if crate::visual_fingerprint(&data.outer) == input_fp {
            return Some(DecodeResult {
                text: word.clone(),
                method: "dictionary brute-force".to_string(),
            });
        }
    }
    None
}

fn search_candidates(
    classified: &[ClassifiedMark],
    mark_count: usize,
    outer: &[[f64; 2]],
    word_index: &WordIndex,
) -> Option<DecodeResult> {
    let input_fp = crate::visual_fingerprint(outer);

    if mark_count == 1 {
        // Single mark: try all words of that category, sorted by parameter proximity
        let cat = classified[0].category;
        let target_param = classified[0].primary_param;
        let mut candidates: Vec<&(String, u64)> =
            word_index.words_for_category(cat).iter().collect();

        // Sort by how close the word's primary param is to what we observed
        candidates.sort_by(|a, b| {
            let pa = language::hash_to_float(a.1);
            let pb = language::hash_to_float(b.1);
            let da = (pa - target_param).abs();
            let db = (pb - target_param).abs();
            da.partial_cmp(&db).unwrap_or(std::cmp::Ordering::Equal)
        });

        // Try top candidates
        for (word, _) in candidates.iter().take(50) {
            let data = symbol::generate(word);
            if crate::visual_fingerprint(&data.outer) == input_fp {
                return Some(DecodeResult {
                    text: word.clone(),
                    method: "geometric mark analysis".to_string(),
                });
            }
        }
    }

    if mark_count >= 2 && mark_count <= 4 {
        // Multi-word: get candidate words per mark position, try combinations
        let mut per_mark_candidates: Vec<Vec<&str>> = Vec::new();

        for cm in classified {
            let words = word_index.words_for_category(cm.category);
            let target = cm.primary_param;
            let mut scored: Vec<(&str, f64)> = words
                .iter()
                .map(|(w, h)| {
                    let p = language::hash_to_float(*h);
                    (w.as_str(), (p - target).abs())
                })
                .collect();
            scored.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(std::cmp::Ordering::Equal));
            let top: Vec<&str> = scored.iter().take(10).map(|(w, _)| *w).collect();
            per_mark_candidates.push(top);
        }

        // Skip if any mark position has no candidates
        if per_mark_candidates.iter().any(|c| c.is_empty()) {
            return None;
        }

        // Try combinations (limited to keep it fast)
        let max_combos = 5000;
        let mut count = 0;

        // Generate combinations iteratively
        let sizes: Vec<usize> = per_mark_candidates.iter().map(|c| c.len()).collect();
        let total: usize = sizes.iter().product();
        let limit = total.min(max_combos);

        for combo_idx in 0..limit {
            let mut words_vec = Vec::new();
            let mut idx = combo_idx;
            for (_mi, candidates) in per_mark_candidates.iter().enumerate() {
                let ci = idx % candidates.len();
                idx /= candidates.len();
                words_vec.push(candidates[ci]);
            }

            let text = words_vec.join(" ");
            let data = symbol::generate(&text);
            if crate::visual_fingerprint(&data.outer) == input_fp {
                return Some(DecodeResult {
                    text,
                    method: "geometric multi-word analysis".to_string(),
                });
            }

            count += 1;
            if count >= max_combos {
                break;
            }
        }
    }

    // Fallback: brute force for single words if geometric classification failed
    brute_force_single(outer)
}

/// Collect all known words from the dictionary and word_primes modules.
fn get_all_known_words() -> Vec<String> {
    // Common English words that appear in our dictionary or word_primes
    // This list drives the brute-force decode capability
    let mut words = Vec::new();

    // Single words from dictionary_lookup categories
    let word_list = [
        // Particles
        "the",
        "a",
        "an",
        "this",
        "that",
        "these",
        "those",
        "my",
        "your",
        "his",
        "her",
        "its",
        "our",
        "their",
        "i",
        "me",
        "you",
        "he",
        "she",
        "it",
        "we",
        "they",
        "who",
        "whom",
        "which",
        "what",
        "and",
        "but",
        "or",
        "yet",
        "so",
        "if",
        "then",
        "both",
        "either",
        "each",
        "every",
        "all",
        "some",
        "any",
        "one",
        "other",
        "another",
        "such",
        "much",
        "many",
        "few",
        "more",
        "most",
        "own",
        "same",
        // Relations
        "in",
        "on",
        "at",
        "to",
        "for",
        "of",
        "with",
        "by",
        "from",
        "into",
        "through",
        "during",
        "before",
        "after",
        "above",
        "below",
        "between",
        "under",
        "over",
        "about",
        "against",
        "among",
        "around",
        "behind",
        "beside",
        "beyond",
        "near",
        "toward",
        "towards",
        "upon",
        "within",
        "without",
        "across",
        "along",
        "until",
        "since",
        "than",
        "like",
        "as",
        // Actions (common)
        "is",
        "am",
        "are",
        "was",
        "were",
        "be",
        "been",
        "being",
        "have",
        "has",
        "had",
        "do",
        "does",
        "did",
        "say",
        "said",
        "go",
        "went",
        "come",
        "came",
        "get",
        "got",
        "make",
        "made",
        "know",
        "knew",
        "think",
        "thought",
        "take",
        "took",
        "see",
        "saw",
        "want",
        "give",
        "gave",
        "use",
        "find",
        "found",
        "tell",
        "told",
        "ask",
        "work",
        "try",
        "leave",
        "left",
        "call",
        "keep",
        "kept",
        "let",
        "begin",
        "seem",
        "help",
        "show",
        "hear",
        "heard",
        "play",
        "run",
        "ran",
        "move",
        "live",
        "believe",
        "hold",
        "held",
        "bring",
        "brought",
        "happen",
        "write",
        "wrote",
        "sit",
        "sat",
        "stand",
        "stood",
        "lose",
        "lost",
        "pay",
        "paid",
        "meet",
        "met",
        "learn",
        "change",
        "lead",
        "led",
        "understand",
        "watch",
        "follow",
        "stop",
        "create",
        "read",
        "grow",
        "grew",
        "open",
        "walk",
        "win",
        "won",
        "offer",
        "remember",
        "love",
        "consider",
        "appear",
        "buy",
        "bought",
        "wait",
        "serve",
        "die",
        "send",
        "sent",
        "build",
        "built",
        "stay",
        "fall",
        "fell",
        "reach",
        "kill",
        "remain",
        "feel",
        "felt",
        "become",
        "became",
        "look",
        "need",
        "start",
        "exist",
        "dream",
        "eat",
        "ate",
        "sleep",
        "slept",
        "fly",
        "flew",
        "sing",
        "sang",
        "draw",
        "drew",
        "break",
        "broke",
        "drive",
        "drove",
        "rise",
        "rose",
        "bite",
        "bit",
        "carry",
        "fight",
        "fought",
        "wear",
        "wore",
        "teach",
        "taught",
        "pull",
        "push",
        "throw",
        "threw",
        "catch",
        "caught",
        "dance",
        "swim",
        "swam",
        "travel",
        "share",
        "connect",
        "flow",
        "spin",
        "spun",
        "wrap",
        "repeat",
        "return",
        "end",
        "wonder",
        // Properties
        "good",
        "bad",
        "great",
        "small",
        "large",
        "big",
        "little",
        "long",
        "short",
        "old",
        "young",
        "new",
        "first",
        "last",
        "high",
        "low",
        "right",
        "wrong",
        "next",
        "early",
        "late",
        "important",
        "different",
        "real",
        "true",
        "false",
        "full",
        "hard",
        "easy",
        "strong",
        "weak",
        "fast",
        "slow",
        "dark",
        "light",
        "hot",
        "cold",
        "warm",
        "cool",
        "deep",
        "wide",
        "far",
        "close",
        "simple",
        "beautiful",
        "happy",
        "sad",
        "angry",
        "quiet",
        "loud",
        "soft",
        "sharp",
        "bright",
        "dim",
        "thick",
        "thin",
        "round",
        "flat",
        "smooth",
        "rough",
        "wet",
        "dry",
        "alive",
        "dead",
        "empty",
        "alone",
        "ready",
        "certain",
        "possible",
        "impossible",
        "necessary",
        "natural",
        "strange",
        "ancient",
        "modern",
        "eternal",
        "infinite",
        "circular",
        "complete",
        "perfect",
        "pure",
        "sacred",
        "vast",
        "silent",
        "still",
        "gentle",
        "fierce",
        "wild",
        "here",
        "there",
        "now",
        "always",
        "never",
        "very",
        "often",
        "already",
        "even",
        "only",
        "again",
        "too",
        "quite",
        "enough",
        "well",
        "together",
        "away",
        // Entities
        "time",
        "year",
        "day",
        "night",
        "week",
        "month",
        "hour",
        "moment",
        "second",
        "minute",
        "morning",
        "evening",
        "world",
        "life",
        "death",
        "man",
        "woman",
        "child",
        "children",
        "people",
        "person",
        "family",
        "friend",
        "name",
        "hand",
        "eye",
        "eyes",
        "face",
        "head",
        "body",
        "heart",
        "mind",
        "soul",
        "voice",
        "word",
        "words",
        "thing",
        "way",
        "place",
        "home",
        "house",
        "room",
        "door",
        "window",
        "wall",
        "water",
        "fire",
        "air",
        "earth",
        "sky",
        "sun",
        "moon",
        "star",
        "stars",
        "sea",
        "ocean",
        "river",
        "mountain",
        "tree",
        "forest",
        "flower",
        "stone",
        "rain",
        "wind",
        "snow",
        "ice",
        "shadow",
        "color",
        "sound",
        "music",
        "song",
        "story",
        "book",
        "letter",
        "language",
        "idea",
        "truth",
        "power",
        "fear",
        "hope",
        "peace",
        "war",
        "god",
        "spirit",
        "beginning",
        "ending",
        "past",
        "present",
        "future",
        "nature",
        "science",
        "art",
        "city",
        "country",
        "king",
        "queen",
        "father",
        "mother",
        "son",
        "daughter",
        "brother",
        "sister",
        "husband",
        "wife",
        "food",
        "bread",
        "blood",
        "bone",
        "skin",
        "animal",
        "bird",
        "fish",
        "horse",
        "dog",
        "cat",
        "road",
        "path",
        "bridge",
        "ship",
        "car",
        "money",
        "school",
        "church",
        "law",
        "state",
        "number",
        "part",
        "side",
        "point",
        "line",
        "form",
        "meaning",
        "question",
        "answer",
        "century",
        "history",
        "age",
        "memory",
        "space",
        "universe",
        "infinity",
        "eternity",
        "void",
        "cycle",
        "circle",
        "spiral",
        "ring",
        "loop",
        "sphere",
        "torus",
        "symbol",
        "sign",
        "mark",
        "pattern",
        "shape",
        "knowledge",
        "wisdom",
        "understanding",
        "energy",
        "force",
        "matter",
        "gravity",
        "particle",
        "wave",
        "frequency",
        "vibration",
        "resonance",
        "consciousness",
        "awareness",
        "perception",
        "existence",
        "reality",
        "dimension",
        "boundary",
        "origin",
        "source",
        "arrival",
        "departure",
        "journey",
        "destination",
        // Negation
        "not",
        "never",
        "nothing",
        "nobody",
        "none",
    ];

    for w in &word_list {
        words.push(w.to_string());
    }

    // Add words from word_primes that aren't already in the list
    let extra_words = [
        "elephant",
        "whale",
        "lion",
        "tiger",
        "bear",
        "wolf",
        "eagle",
        "snake",
        "spider",
        "butterfly",
        "dolphin",
        "rose",
        "oak",
        "pine",
        "sword",
        "hammer",
        "knife",
        "arrow",
        "shield",
        "gold",
        "silver",
        "iron",
        "steel",
        "diamond",
        "crystal",
        "castle",
        "temple",
        "tower",
        "prison",
        "farm",
        "storm",
        "thunder",
        "lightning",
        "cloud",
        "fog",
        "rainbow",
        "red",
        "blue",
        "green",
        "yellow",
        "black",
        "white",
        "purple",
        "pink",
        "brown",
        "gray",
        "doctor",
        "soldier",
        "farmer",
        "priest",
        "artist",
        "writer",
        "hero",
        "thief",
        "stranger",
        "prince",
        "princess",
        "wine",
        "beer",
        "tea",
        "coffee",
        "milk",
        "honey",
        "apple",
        "grape",
        "wheat",
        "rice",
        "arm",
        "leg",
        "foot",
        "finger",
        "brain",
        "tongue",
        "dress",
        "hat",
        "shoe",
        "crown",
        "boat",
        "wagon",
        "train",
        "plane",
        "rocket",
        "guitar",
        "piano",
        "violin",
        "drum",
        "flute",
        "atom",
        "cell",
        "gene",
        "gravity",
        "equation",
        "justice",
        "mercy",
        "honor",
        "glory",
        "fate",
        "chaos",
        "courage",
        "patience",
        "pride",
        "shame",
        "guilt",
        "jealousy",
        "loneliness",
        "hunger",
        "desire",
        "pleasure",
        "agony",
        "confusion",
        "nostalgia",
        "regret",
        "gratitude",
        "attack",
        "defend",
        "escape",
        "hide",
        "chase",
        "climb",
        "burn",
        "heal",
        "breathe",
        "whisper",
        "shout",
        "laugh",
        "smile",
        "weep",
        "kiss",
        "pray",
        "forgive",
        "betray",
        "explore",
        "invent",
        "solve",
        "trick",
        "murder",
        "celebrate",
        "suffer",
        "sacrifice",
        "worship",
        "meditate",
        "imagine",
        "choose",
        "promise",
        "argue",
        "confess",
    ];

    for w in &extra_words {
        if !words.contains(&w.to_string()) {
            words.push(w.to_string());
        }
    }

    words
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn decode_attribute_present() {
        // "Hello" in base64 = "SGVsbG8="
        let svg = r#"<svg data-torus-text="SGVsbG8=" viewBox="0 0 600 600"></svg>"#;
        assert_eq!(decode_from_attribute(svg), Some("Hello".to_string()));
    }

    #[test]
    fn decode_attribute_missing() {
        let svg = r#"<svg viewBox="0 0 600 600"></svg>"#;
        assert_eq!(decode_from_attribute(svg), None);
    }

    #[test]
    fn decode_attribute_roundtrip() {
        // Use the same base64 encoder as the generate-svg endpoint
        let text = "time is a circle";
        let encoded = crate::base64_encode(text);
        let svg = format!(
            r#"<svg data-torus-text="{}" viewBox="0 0 600 600"></svg>"#,
            encoded
        );
        assert_eq!(decode_from_attribute(&svg), Some(text.to_string()));
    }

    #[test]
    fn extract_paths_empty() {
        let (outer, inner) = extract_svg_paths("<svg></svg>");
        assert!(outer.is_empty());
        assert!(inner.is_empty());
    }

    #[test]
    fn extract_paths_basic() {
        let svg = r#"<path d="M100.0,200.0L150.0,250.0ZM300.0,400.0Z"/>"#;
        let (outer, inner) = extract_svg_paths(svg);
        assert_eq!(outer.len(), 2);
        assert_eq!(outer[0], [100.0, 200.0]);
        assert_eq!(outer[1], [150.0, 250.0]);
        assert_eq!(inner.len(), 1);
        assert_eq!(inner[0], [300.0, 400.0]);
    }

    #[test]
    fn word_index_non_empty() {
        let index = WordIndex::build();
        assert!(!index.all_words.is_empty());
        assert!(!index.by_category.is_empty());
    }

    #[test]
    fn word_index_has_common_words() {
        let index = WordIndex::build();
        assert!(index.all_words.contains(&"time".to_string()));
        assert!(index.all_words.contains(&"love".to_string()));
    }
}

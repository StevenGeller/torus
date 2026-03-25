/// Torus Language System
///
/// A semasiographic circular writing system where visual features encode
/// semantic categories. The encoding is deterministic and reversible.
///
/// Mark types by semantic category:
/// - Entity (nouns)     → Outward tendrils (things projecting into the world)
/// - Action (verbs)     → Stroke-width waves (dynamic, flowing)
/// - Property (adj/adv) → Inner edge modulations (modify substance from within)
/// - Relation (prep)    → Bridges between adjacent marks
/// - Particle (det/pron/conj) → Small notches
/// - Negation           → Inverted marks (dip inward)
/// - Question           → Gap in the ring

// ─── Word Categories ─────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq, serde::Serialize)]
#[serde(rename_all = "lowercase")]
pub enum Category {
    Entity,
    Action,
    Property,
    Relation,
    Particle,
    Negation,
    Question,
}

impl Category {
    pub fn description(&self) -> &'static str {
        match self {
            Category::Entity => "Outward projection \u{2014} an entity reaching into the world",
            Category::Action => "Width modulation \u{2014} a dynamic flow of action",
            Category::Property => "Inner ripple \u{2014} a quality shaping from within",
            Category::Relation => "Bridge \u{2014} a connection between ideas",
            Category::Particle => "Notch \u{2014} a structural particle",
            Category::Negation => "Inward void \u{2014} an absence, a reversal",
            Category::Question => "Opening \u{2014} an incomplete circle seeking answer",
        }
    }
}

// ─── Aspect (B-theory: no tense, only internal event structure) ──

#[derive(Debug, Clone, Copy, PartialEq, serde::Serialize)]
#[serde(rename_all = "lowercase")]
pub enum Aspect {
    Timeless,  // base form: eternal, always-true ("is", "love")
    Unbounded, // -ing: ongoing, continuous ("running", "being")
    Bounded,   // -ed: completed, contained ("walked", "finished")
}

impl Aspect {
    pub fn description(&self) -> &'static str {
        match self {
            Aspect::Timeless => "timeless \u{2014} an eternal state, always true",
            Aspect::Unbounded => "unbounded \u{2014} ongoing, without edges",
            Aspect::Bounded => "bounded \u{2014} completed, contained within itself",
        }
    }
}

pub fn detect_aspect(word: &str) -> Aspect {
    let w = word.to_lowercase();
    if w.ends_with("ing") {
        Aspect::Unbounded
    } else if w.ends_with("ed")
        || matches!(
            w.as_str(),
            "went"
                | "came"
                | "saw"
                | "knew"
                | "thought"
                | "took"
                | "gave"
                | "made"
                | "found"
                | "said"
                | "told"
                | "got"
                | "had"
                | "did"
                | "was"
                | "were"
                | "been"
                | "spoke"
                | "wrote"
                | "drove"
                | "broke"
                | "chose"
                | "fell"
                | "held"
                | "kept"
                | "left"
                | "lost"
                | "met"
                | "paid"
                | "ran"
                | "sat"
                | "stood"
                | "understood"
                | "won"
                | "wore"
                | "bought"
                | "brought"
                | "built"
                | "caught"
                | "fought"
                | "felt"
                | "forgot"
                | "heard"
                | "led"
                | "meant"
                | "read"
                | "sent"
                | "slept"
                | "spent"
                | "taught"
                | "threw"
                | "woke"
        )
    {
        Aspect::Bounded
    } else {
        Aspect::Timeless
    }
}

/// Category layout priority: entities anchor the circle,
/// actions flow between them, modifiers orbit nearby.
pub fn category_priority(cat: Category) -> u8 {
    match cat {
        Category::Entity => 0,   // cardinal positions
        Category::Action => 1,   // between entities
        Category::Property => 2, // near their entities
        Category::Negation => 3, // void marks
        Category::Relation => 4, // bridges
        Category::Particle => 5, // structural
        Category::Question => 6, // (filtered)
    }
}

// ─── Semantic Roles ──────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq, serde::Serialize)]
#[serde(rename_all = "lowercase")]
pub enum Role {
    Agent,    // who does the action
    Action,   // the verb itself
    Patient,  // who receives the action
    Modifier, // attached to a specific head word
    Unmarked, // structural (particles, relations)
}

impl Role {
    pub fn tag(&self) -> &'static str {
        match self {
            Role::Agent => "agent",
            Role::Action => "action",
            Role::Patient => "patient",
            Role::Modifier => "mod",
            Role::Unmarked => "un",
        }
    }
}

// ─── Categorized Word ────────────────────────────────────────────

#[derive(Debug, Clone, serde::Serialize)]
pub struct CategorizedWord {
    pub word: String,
    pub category: Category,
    pub aspect: Aspect,
    pub role: Role,
    /// Index of the word this modifier attaches to (if Role::Modifier)
    pub modifies: Option<usize>,
}

// ─── Dictionary-based categorization ─────────────────────────────

pub fn categorize(word: &str) -> Category {
    let lower = word.to_lowercase();

    // Check dictionary first
    if let Some(cat) = dictionary_lookup(&lower) {
        return cat;
    }

    // Suffix-based heuristics
    if lower.ends_with('?') {
        return Category::Question;
    }

    let base = lower.trim_end_matches(|c: char| !c.is_alphabetic());

    // Negation
    if matches!(
        base,
        "not"
            | "never"
            | "neither"
            | "nor"
            | "nowhere"
            | "nothing"
            | "nobody"
            | "none"
            | "cannot"
            | "can't"
            | "don't"
            | "doesn't"
            | "didn't"
            | "won't"
            | "wouldn't"
            | "shouldn't"
            | "couldn't"
            | "isn't"
            | "aren't"
            | "wasn't"
            | "weren't"
            | "hasn't"
            | "haven't"
            | "hadn't"
    ) {
        return Category::Negation;
    }

    // Suffix heuristics for unknown words
    if base.ends_with("ing")
        || base.ends_with("ize")
        || base.ends_with("ify")
        || base.ends_with("ate")
    {
        return Category::Action;
    }
    // Present tense -s/-es: check if the root (without s/es) is a known verb
    if base.ends_with("es") && base.len() > 4 {
        let root = &base[..base.len() - 2];
        if dictionary_lookup(root) == Some(Category::Action) {
            return Category::Action;
        }
    }
    if base.ends_with('s') && !base.ends_with("ss") && base.len() > 3 {
        let root = &base[..base.len() - 1];
        if dictionary_lookup(root) == Some(Category::Action) {
            return Category::Action;
        }
    }
    if base.ends_with("tion")
        || base.ends_with("sion")
        || base.ends_with("ment")
        || base.ends_with("ness")
        || base.ends_with("ity")
        || base.ends_with("ism")
        || base.ends_with("ist")
        || base.ends_with("ance")
        || base.ends_with("ence")
    {
        return Category::Entity;
    }
    if base.ends_with("ous")
        || base.ends_with("ful")
        || base.ends_with("less")
        || base.ends_with("able")
        || base.ends_with("ible")
        || base.ends_with("ive")
        || base.ends_with("al")
        || base.ends_with("ial")
        || base.ends_with("ical")
    {
        return Category::Property;
    }
    if base.ends_with("ly") {
        return Category::Property; // adverbs treated as properties
    }
    if base.ends_with("ed") {
        return Category::Action; // past tense verbs
    }
    if base.ends_with("er") || base.ends_with("or") {
        return Category::Entity; // agent nouns
    }

    // Default: entity (nouns are the most common open class)
    Category::Entity
}

pub fn categorize_sentence(text: &str) -> Vec<CategorizedWord> {
    let text = text.trim();
    if text.is_empty() {
        return Vec::new();
    }

    // Check if the whole sentence is a question
    let is_question = text.ends_with('?');

    let mut words: Vec<CategorizedWord> = text
        .split_whitespace()
        .map(|w| {
            let clean = w.trim_matches(|c: char| !c.is_alphanumeric() && c != '\'');
            let category = categorize(clean);
            let aspect = if category == Category::Action {
                detect_aspect(clean)
            } else {
                Aspect::Timeless
            };
            CategorizedWord {
                word: clean.to_string(),
                category,
                aspect,
                role: Role::Unmarked,
                modifies: None,
            }
        })
        .filter(|w| !w.word.is_empty())
        .collect();

    // Assign semantic roles (SVO heuristic)
    assign_roles(&mut words);

    // Detect modifier attachment
    detect_modifiers(&mut words);

    // If question, add question marker
    if is_question {
        words.push(CategorizedWord {
            word: "?".to_string(),
            category: Category::Question,
            aspect: Aspect::Timeless,
            role: Role::Unmarked,
            modifies: None,
        });
    }

    words
}

/// SVO role assignment: first entity before verb = agent,
/// first entity after verb = patient. No NLP needed.
/// Copular verbs (is/am/are/was/were/be) don't assign roles —
/// their subjects and complements are semantically equivalent.
fn assign_roles(words: &mut [CategorizedWord]) {
    // Find the first action word
    let verb_idx = words.iter().position(|w| w.category == Category::Action);

    if let Some(vi) = verb_idx {
        words[vi].role = Role::Action;

        // Copular/linking verbs: no agent/patient distinction
        let copular = matches!(
            words[vi].word.to_lowercase().as_str(),
            "is" | "am"
                | "are"
                | "was"
                | "were"
                | "be"
                | "been"
                | "being"
                | "seem"
                | "seems"
                | "seemed"
                | "become"
                | "becomes"
                | "became"
                | "remain"
                | "remains"
                | "remained"
                | "appear"
                | "appears"
                | "appeared"
        );
        if copular {
            return; // No agent/patient for linking verbs
        }

        // Agent: nearest entity BEFORE the verb
        for i in (0..vi).rev() {
            if words[i].category == Category::Entity {
                words[i].role = Role::Agent;
                break;
            }
        }

        // Patient: nearest entity AFTER the verb
        for i in (vi + 1)..words.len() {
            if words[i].category == Category::Entity {
                words[i].role = Role::Patient;
                break;
            }
        }

        // Handle passive voice: "was/were [verb]ed by [noun]"
        if vi >= 1 {
            let prev = words[vi - 1].word.to_lowercase();
            if (prev == "was" || prev == "were") && words[vi].aspect == Aspect::Bounded {
                // Look for "by" after the verb to find the real agent
                for i in (vi + 1)..words.len() {
                    if words[i].word.to_lowercase() == "by" {
                        // Next entity after "by" is the real agent
                        for j in (i + 1)..words.len() {
                            if words[j].category == Category::Entity {
                                // Swap: the "by" noun is agent, the pre-verb noun is patient
                                words[j].role = Role::Agent;
                                // Find the pre-verb entity and make it patient
                                for k in (0..vi).rev() {
                                    if words[k].role == Role::Agent {
                                        words[k].role = Role::Patient;
                                        break;
                                    }
                                }
                                break;
                            }
                        }
                        break;
                    }
                }
            }
        }
    } else {
        // No verb: all entities are unmarked (noun phrases)
        // Mark the first entity as agent for visual differentiation
        for w in words.iter_mut() {
            if w.category == Category::Entity {
                w.role = Role::Agent;
                break;
            }
        }
    }
}

/// Detect which word a modifier (adjective/adverb/focus particle) attaches to.
fn detect_modifiers(words: &mut [CategorizedWord]) {
    let focus_words = [
        "only",
        "even",
        "just",
        "also",
        "merely",
        "simply",
        "especially",
        "particularly",
        "mainly",
        "mostly",
    ];

    let len = words.len();
    for i in 0..len {
        if words[i].category != Category::Property {
            continue;
        }

        let lower = words[i].word.to_lowercase();

        // Focus particles attach to the next content word
        if focus_words.contains(&lower.as_str())
            && i + 1 < len
            && words[i + 1].category != Category::Question
        {
            words[i].role = Role::Modifier;
            words[i].modifies = Some(i + 1);
            continue;
        }

        // Adjective before noun
        if i + 1 < len && words[i + 1].category == Category::Entity {
            words[i].role = Role::Modifier;
            words[i].modifies = Some(i + 1);
            continue;
        }

        // Adverb before verb
        if i + 1 < len && words[i + 1].category == Category::Action {
            words[i].role = Role::Modifier;
            words[i].modifies = Some(i + 1);
            continue;
        }

        // Adverb after verb
        if i >= 1 && words[i - 1].category == Category::Action {
            words[i].role = Role::Modifier;
            words[i].modifies = Some(i - 1);
        }
    }
}

// ─── Core dictionary of common English words ─────────────────────

fn dictionary_lookup(word: &str) -> Option<Category> {
    Some(match word {
        // ── Particles (determiners, pronouns, conjunctions) ──
        "the" | "a" | "an" | "this" | "that" | "these" | "those" | "my" | "your" | "his"
        | "her" | "its" | "our" | "their" | "i" | "me" | "you" | "he" | "she" | "it" | "we"
        | "they" | "who" | "whom" | "which" | "what" | "and" | "but" | "or" | "yet" | "so"
        | "if" | "then" | "both" | "either" | "each" | "every" | "all" | "some" | "any" | "one"
        | "other" | "another" | "such" | "much" | "many" | "few" | "more" | "most" | "own"
        | "same" => Category::Particle,

        // ── Relations (prepositions) ──
        "in" | "on" | "at" | "to" | "for" | "of" | "with" | "by" | "from" | "into" | "through"
        | "during" | "before" | "after" | "above" | "below" | "between" | "under" | "over"
        | "about" | "against" | "among" | "around" | "behind" | "beside" | "beyond" | "near"
        | "toward" | "towards" | "upon" | "within" | "without" | "across" | "along" | "until"
        | "since" | "than" | "like" | "as" | "per" | "via" => Category::Relation,

        // ── Actions (common verbs) ──
        "is" | "am" | "are" | "was" | "were" | "be" | "been" | "being" | "have" | "has" | "had"
        | "do" | "does" | "did" | "will" | "would" | "shall" | "should" | "may" | "might"
        | "can" | "could" | "must" | "say" | "said" | "says" | "tell" | "told" | "speak"
        | "spoke" | "go" | "goes" | "went" | "gone" | "going" | "come" | "came" | "comes"
        | "coming" | "get" | "got" | "gets" | "getting" | "make" | "made" | "makes" | "making"
        | "know" | "knew" | "knows" | "known" | "think" | "thought" | "thinks" | "thinking"
        | "take" | "took" | "takes" | "taken" | "see" | "saw" | "sees" | "seen" | "want"
        | "wanted" | "wants" | "give" | "gave" | "gives" | "given" | "use" | "used" | "uses"
        | "find" | "found" | "finds" | "put" | "puts" | "try" | "tried" | "tries" | "leave"
        | "left" | "leaves" | "call" | "called" | "calls" | "keep" | "kept" | "keeps" | "let"
        | "lets" | "begin" | "began" | "begins" | "seem" | "seemed" | "seems" | "help"
        | "helped" | "helps" | "show" | "showed" | "shows" | "shown" | "hear" | "heard"
        | "hears" | "play" | "played" | "plays" | "run" | "ran" | "runs" | "move" | "moved"
        | "moves" | "live" | "lived" | "lives" | "believe" | "believed" | "believes" | "hold"
        | "held" | "holds" | "bring" | "brought" | "brings" | "happen" | "happened" | "happens"
        | "write" | "wrote" | "writes" | "written" | "sit" | "sat" | "sits" | "stand" | "stood"
        | "stands" | "lose" | "lost" | "loses" | "pay" | "paid" | "pays" | "meet" | "met"
        | "meets" | "include" | "included" | "includes" | "continue" | "continued"
        | "continues" | "set" | "sets" | "learn" | "learned" | "learns" | "change" | "changed"
        | "changes" | "lead" | "led" | "leads" | "understand" | "understood" | "understands"
        | "watch" | "watched" | "watches" | "follow" | "followed" | "follows" | "stop"
        | "stopped" | "stops" | "create" | "created" | "creates" | "read" | "reads" | "allow"
        | "allowed" | "allows" | "grow" | "grew" | "grows" | "grown" | "open" | "opened"
        | "opens" | "walk" | "walked" | "walks" | "win" | "won" | "wins" | "offer" | "offered"
        | "offers" | "remember" | "remembered" | "remembers" | "love" | "loved" | "loves"
        | "consider" | "considered" | "considers" | "appear" | "appeared" | "appears" | "buy"
        | "bought" | "buys" | "wait" | "waited" | "waits" | "serve" | "served" | "serves"
        | "die" | "died" | "dies" | "send" | "sent" | "sends" | "expect" | "expected"
        | "expects" | "build" | "built" | "builds" | "stay" | "stayed" | "stays" | "fall"
        | "fell" | "falls" | "fallen" | "cut" | "cuts" | "reach" | "reached" | "reaches"
        | "kill" | "killed" | "kills" | "remain" | "remained" | "remains" | "feel" | "felt"
        | "feels" | "become" | "became" | "becomes" | "look" | "looked" | "looks" | "need"
        | "needed" | "needs" | "ask" | "asked" | "asks" | "start" | "started" | "starts"
        | "work" | "worked" | "works" | "turn" | "turned" | "turns" | "exist" | "existed"
        | "exists" | "dream" | "dreamed" | "dreams" | "eat" | "ate" | "eats" | "sleep"
        | "slept" | "sleeps" | "fly" | "flew" | "flies" | "sing" | "sang" | "sings" | "draw"
        | "drew" | "draws" | "break" | "broke" | "breaks" | "drive" | "drove" | "drives"
        | "rise" | "rose" | "rises" | "bite" | "bit" | "bitten" | "carry" | "carried"
        | "carries" | "fight" | "fought" | "fights" | "wear" | "wore" | "wears" | "teach"
        | "taught" | "teaches" | "pull" | "pulled" | "pulls" | "push" | "pushed" | "pushes"
        | "throw" | "threw" | "throws" | "catch" | "caught" | "catches" | "dance" | "danced"
        | "dances" | "swim" | "swam" | "swims" | "travel" | "traveled" | "travels" | "share"
        | "shared" | "shares" | "connect" | "connected" | "connects" | "communicate"
        | "communicated" | "communicates" | "translate" | "translated" | "translates"
        | "encode" | "encoded" | "encodes" | "decode" | "decoded" | "decodes" | "flow"
        | "flowed" | "flows" | "spin" | "spun" | "spins" | "revolve" | "revolved" | "revolves"
        | "circled" | "circles" | "wrap" | "wrapped" | "wraps" | "repeat" | "repeated"
        | "repeats" | "return" | "returned" | "returns" | "end" | "ended" | "ends" | "wonder"
        | "wondered" | "wonders" => Category::Action,

        // ── Properties (adjectives, adverbs) ──
        "good" | "bad" | "great" | "small" | "large" | "big" | "little" | "long" | "short"
        | "old" | "young" | "new" | "first" | "last" | "high" | "low" | "right" | "wrong"
        | "next" | "early" | "late" | "important" | "different" | "real" | "true" | "false"
        | "full" | "special" | "free" | "clear" | "sure" | "human" | "local" | "hard" | "easy"
        | "strong" | "weak" | "fast" | "slow" | "dark" | "light" | "hot" | "cold" | "warm"
        | "cool" | "deep" | "wide" | "far" | "close" | "simple" | "whole" | "best" | "better"
        | "worse" | "worst" | "beautiful" | "happy" | "sad" | "angry" | "quiet" | "loud"
        | "soft" | "sharp" | "bright" | "dim" | "thick" | "thin" | "round" | "flat" | "smooth"
        | "rough" | "wet" | "dry" | "alive" | "dead" | "empty" | "alone" | "ready" | "certain"
        | "possible" | "impossible" | "necessary" | "natural" | "strange" | "ancient"
        | "modern" | "eternal" | "infinite" | "circular" | "complete" | "perfect" | "pure"
        | "sacred" | "vast" | "silent" | "still" | "gentle" | "fierce" | "wild" | "here"
        | "there" | "now" | "always" | "never" | "ever" | "just" | "also" | "very" | "often"
        | "already" | "even" | "only" | "again" | "too" | "quite" | "enough" | "well"
        | "together" | "away" | "back" | "up" | "down" | "out" | "off" | "perhaps" | "maybe"
        | "finally" | "really" => Category::Property,

        // ── Entities (common nouns) ──
        "time" | "year" | "day" | "night" | "week" | "month" | "hour" | "moment" | "second"
        | "minute" | "morning" | "evening" | "world" | "life" | "death" | "man" | "woman"
        | "child" | "children" | "people" | "person" | "family" | "friend" | "name" | "hand"
        | "eye" | "eyes" | "face" | "head" | "body" | "heart" | "mind" | "soul" | "voice"
        | "word" | "words" | "thing" | "way" | "place" | "home" | "house" | "room" | "door"
        | "window" | "wall" | "water" | "fire" | "air" | "earth" | "sky" | "sun" | "moon"
        | "star" | "stars" | "sea" | "ocean" | "river" | "mountain" | "tree" | "forest"
        | "flower" | "stone" | "rain" | "wind" | "snow" | "ice" | "shadow" | "color" | "sound"
        | "music" | "song" | "story" | "book" | "letter" | "language" | "idea" | "truth"
        | "power" | "fear" | "hope" | "peace" | "war" | "god" | "spirit" | "beginning"
        | "ending" | "past" | "present" | "future" | "nature" | "science" | "art" | "city"
        | "country" | "king" | "queen" | "father" | "mother" | "son" | "daughter" | "brother"
        | "sister" | "husband" | "wife" | "food" | "bread" | "blood" | "bone" | "skin"
        | "animal" | "bird" | "fish" | "horse" | "dog" | "cat" | "road" | "path" | "bridge"
        | "ship" | "car" | "money" | "school" | "church" | "law" | "state" | "number" | "part"
        | "side" | "point" | "line" | "form" | "meaning" | "question" | "answer" | "century"
        | "history" | "age" | "memory" | "space" | "universe" | "infinity" | "eternity"
        | "void" | "cycle" | "circle" | "spiral" | "ring" | "loop" | "sphere" | "torus"
        | "symbol" | "sign" | "mark" | "pattern" | "shape" | "knowledge" | "wisdom"
        | "understanding" | "energy" | "force" | "matter" | "gravity" | "particle" | "wave"
        | "frequency" | "vibration" | "resonance" | "consciousness" | "awareness"
        | "perception" | "existence" | "reality" | "dimension" | "boundary" | "origin"
        | "source" | "arrival" | "departure" | "journey" | "destination" => Category::Entity,

        // ── Question words ──
        "why" | "how" | "where" | "when" => Category::Question,

        _ => return None,
    })
}

// ─── Semantic hash for visual encoding ───────────────────────────
// Same word always produces the same visual parameters.

pub fn word_hash(word: &str) -> u64 {
    let lower = word.to_lowercase();
    fnv1a(lower.as_bytes())
}

pub fn fnv1a(bytes: &[u8]) -> u64 {
    let mut hash: u64 = 14695981039346656037;
    for &b in bytes {
        hash ^= b as u64;
        hash = hash.wrapping_mul(1099511628211);
    }
    hash
}

pub fn hash_to_float(hash: u64) -> f64 {
    (hash & 0xFFFF_FFFF) as f64 / 4294967296.0
}

/// Derive a secondary hash from a base hash + seed
pub fn derive_hash(base: u64, seed: u64) -> u64 {
    let mut buf = [0u8; 16];
    buf[..8].copy_from_slice(&base.to_le_bytes());
    buf[8..].copy_from_slice(&seed.to_le_bytes());
    fnv1a(&buf)
}

// ═════════════════════════════════════════════════════════════════
// SEMANTIC PRIMES (Wierzbicka / Natural Semantic Metalanguage)
// ═════════════════════════════════════════════════════════════════
// 65 universal concepts that exist in every human language.
// These are the atoms of meaning — they cannot be defined simpler.
// Torus decomposes English words into these primes to reveal
// the universal semantic structure beneath the surface.

#[derive(Debug, Clone, serde::Serialize)]
pub struct SemanticPrime {
    pub prime: &'static str,
    pub domain: &'static str,
}

/// Decompose a word into its semantic prime components.
/// Returns the primes that constitute the word's core meaning.
pub fn decompose_to_primes(word: &str) -> Vec<SemanticPrime> {
    let w = word.to_lowercase();
    match w.as_str() {
        // ── Direct primes (words that ARE semantic primes) ──
        "i" | "me" => vec![p("I", "substantive")],
        "you" => vec![p("YOU", "substantive")],
        "someone" | "person" | "man" | "woman" => vec![p("SOMEONE", "substantive")],
        "something" | "thing" => vec![p("SOMETHING", "substantive")],
        "people" => vec![p("PEOPLE", "substantive")],
        "body" => vec![p("BODY", "substantive")],
        "this" | "that" => vec![p("THIS", "determiner")],
        "same" => vec![p("THE SAME", "determiner")],
        "other" | "another" | "different" => vec![p("OTHER", "determiner")],
        "one" => vec![p("ONE", "quantifier")],
        "two" => vec![p("TWO", "quantifier")],
        "some" => vec![p("SOME", "quantifier")],
        "all" | "every" | "everything" => vec![p("ALL", "quantifier")],
        "much" | "many" | "very" => vec![p("MUCH/MANY", "quantifier")],
        "good" | "well" => vec![p("GOOD", "evaluator")],
        "bad" | "wrong" | "evil" => vec![p("BAD", "evaluator")],
        "big" | "large" | "great" | "vast" => vec![p("BIG", "descriptor")],
        "small" | "little" | "tiny" => vec![p("SMALL", "descriptor")],
        "think" | "thought" => vec![p("THINK", "mental")],
        "know" | "knowledge" | "known" => vec![p("KNOW", "mental")],
        "want" | "desire" | "wish" => vec![p("WANT", "mental")],
        "feel" | "feeling" | "emotion" => vec![p("FEEL", "mental")],
        "see" | "saw" | "seen" | "sight" => vec![p("SEE", "mental")],
        "hear" | "heard" | "sound" => vec![p("HEAR", "mental")],
        "say" | "said" | "speak" | "spoke" | "tell" | "told" => vec![p("SAY", "speech")],
        "word" | "words" | "language" => vec![p("WORDS", "speech")],
        "true" | "real" => vec![p("TRUE", "speech")],
        "do" | "did" | "does" => vec![p("DO", "action")],
        "happen" | "happened" | "event" => vec![p("HAPPEN", "action")],
        "move" | "moved" | "motion" => vec![p("MOVE", "action")],
        "there" | "exist" | "exists" | "existence" => vec![p("THERE IS", "existence")],
        "be" | "is" | "am" | "are" | "was" | "were" | "being" => vec![p("BE", "existence")],
        "have" | "has" | "had" => vec![p("HAVE", "existence")],
        "live" | "life" | "alive" | "living" => vec![p("LIVE", "life")],
        "die" | "died" | "dead" => vec![p("DIE", "life")],
        "when" | "moment" => vec![p("WHEN/TIME", "time")],
        "now" => vec![p("NOW", "time")],
        "before" => vec![p("BEFORE", "time")],
        "after" => vec![p("AFTER", "time")],
        "long" => vec![p("A LONG TIME", "time")],
        "where" | "place" => vec![p("WHERE/PLACE", "space")],
        "here" => vec![p("HERE", "space")],
        "above" | "over" | "up" | "high" => vec![p("ABOVE", "space")],
        "below" | "under" | "down" | "low" => vec![p("BELOW", "space")],
        "far" | "distant" | "away" => vec![p("FAR", "space")],
        "near" | "close" => vec![p("NEAR", "space")],
        "inside" | "in" | "within" => vec![p("INSIDE", "space")],
        "not" | "no" | "never" => vec![p("NOT", "logical")],
        "maybe" | "perhaps" | "possible" => vec![p("MAYBE", "logical")],
        "can" | "could" | "able" => vec![p("CAN", "logical")],
        "because" => vec![p("BECAUSE", "logical")],
        "if" => vec![p("IF", "logical")],
        "like" | "as" | "similar" => vec![p("LIKE/WAY", "similarity")],

        // ── Compound decompositions (words built from primes) ──
        "love" => vec![
            p("FEEL", "mental"),
            p("GOOD", "evaluator"),
            p("MUCH/MANY", "quantifier"),
        ],
        "hate" => vec![
            p("FEEL", "mental"),
            p("BAD", "evaluator"),
            p("MUCH/MANY", "quantifier"),
        ],
        "fear" | "afraid" => vec![
            p("FEEL", "mental"),
            p("BAD", "evaluator"),
            p("HAPPEN", "action"),
        ],
        "hope" => vec![
            p("WANT", "mental"),
            p("GOOD", "evaluator"),
            p("HAPPEN", "action"),
        ],
        "understand" | "understanding" => vec![p("KNOW", "mental"), p("THINK", "mental")],
        "wisdom" => vec![
            p("KNOW", "mental"),
            p("GOOD", "evaluator"),
            p("MUCH/MANY", "quantifier"),
        ],
        "beautiful" | "beauty" => vec![
            p("SEE", "mental"),
            p("FEEL", "mental"),
            p("GOOD", "evaluator"),
        ],
        "infinite" | "infinity" | "eternal" | "eternity" => vec![
            p("MUCH/MANY", "quantifier"),
            p("A LONG TIME", "time"),
            p("NOT", "logical"),
        ],
        "dream" | "dreaming" => vec![
            p("THINK", "mental"),
            p("SEE", "mental"),
            p("NOT", "logical"),
        ],
        "circle" | "cycle" | "loop" | "ring" | "torus" => vec![
            p("LIKE/WAY", "similarity"),
            p("THE SAME", "determiner"),
            p("MOVE", "action"),
        ],
        "beginning" => vec![p("BEFORE", "time"), p("ALL", "quantifier")],
        "ending" => vec![p("AFTER", "time"), p("ALL", "quantifier")],
        "war" => vec![
            p("DO", "action"),
            p("BAD", "evaluator"),
            p("MUCH/MANY", "quantifier"),
            p("PEOPLE", "substantive"),
        ],
        "peace" => vec![
            p("NOT", "logical"),
            p("BAD", "evaluator"),
            p("FEEL", "mental"),
            p("GOOD", "evaluator"),
        ],
        "home" => vec![
            p("WHERE/PLACE", "space"),
            p("LIVE", "life"),
            p("FEEL", "mental"),
            p("GOOD", "evaluator"),
        ],
        "friend" => vec![
            p("SOMEONE", "substantive"),
            p("FEEL", "mental"),
            p("GOOD", "evaluator"),
        ],
        "child" | "children" => vec![
            p("SOMEONE", "substantive"),
            p("SMALL", "descriptor"),
            p("LIVE", "life"),
        ],
        "father" | "mother" => vec![
            p("SOMEONE", "substantive"),
            p("LIVE", "life"),
            p("BEFORE", "time"),
        ],
        "god" => vec![
            p("SOMEONE", "substantive"),
            p("BIG", "descriptor"),
            p("ABOVE", "space"),
            p("CAN", "logical"),
            p("ALL", "quantifier"),
        ],
        "consciousness" | "awareness" => vec![
            p("KNOW", "mental"),
            p("THINK", "mental"),
            p("I", "substantive"),
        ],
        "world" | "universe" => vec![
            p("SOMETHING", "substantive"),
            p("BIG", "descriptor"),
            p("ALL", "quantifier"),
        ],
        "sun" | "star" => vec![
            p("SOMETHING", "substantive"),
            p("BIG", "descriptor"),
            p("ABOVE", "space"),
            p("SEE", "mental"),
        ],
        "water" | "sea" | "ocean" | "river" => {
            vec![p("SOMETHING", "substantive"), p("MOVE", "action")]
        }
        "fire" => vec![
            p("SOMETHING", "substantive"),
            p("FEEL", "mental"),
            p("BAD", "evaluator"),
            p("SEE", "mental"),
        ],
        "earth" | "ground" => vec![
            p("SOMETHING", "substantive"),
            p("BIG", "descriptor"),
            p("BELOW", "space"),
        ],
        "sky" => vec![
            p("SOMETHING", "substantive"),
            p("BIG", "descriptor"),
            p("ABOVE", "space"),
        ],
        "night" => vec![
            p("WHEN/TIME", "time"),
            p("NOT", "logical"),
            p("SEE", "mental"),
        ],
        "day" | "morning" => vec![p("WHEN/TIME", "time"), p("SEE", "mental")],
        "silence" | "quiet" | "silent" => vec![p("NOT", "logical"), p("HEAR", "mental")],
        "darkness" | "dark" => vec![p("NOT", "logical"), p("SEE", "mental")],
        "light" | "bright" => vec![p("SEE", "mental"), p("MUCH/MANY", "quantifier")],
        "power" | "force" | "strong" => vec![
            p("CAN", "logical"),
            p("DO", "action"),
            p("MUCH/MANY", "quantifier"),
        ],
        "free" | "freedom" => vec![p("CAN", "logical"), p("DO", "action"), p("WANT", "mental")],
        "journey" | "travel" => vec![
            p("MOVE", "action"),
            p("FAR", "space"),
            p("A LONG TIME", "time"),
        ],
        "arrival" => vec![p("MOVE", "action"), p("HERE", "space"), p("NOW", "time")],
        "music" | "song" => vec![
            p("HEAR", "mental"),
            p("FEEL", "mental"),
            p("GOOD", "evaluator"),
        ],
        "story" => vec![
            p("SAY", "speech"),
            p("WORDS", "speech"),
            p("HAPPEN", "action"),
        ],
        "truth" => vec![p("TRUE", "speech"), p("KNOW", "mental")],
        "lie" | "lying" => vec![p("SAY", "speech"), p("NOT", "logical"), p("TRUE", "speech")],
        "remember" | "memory" => vec![
            p("THINK", "mental"),
            p("KNOW", "mental"),
            p("BEFORE", "time"),
        ],
        "forget" | "forgot" => vec![
            p("NOT", "logical"),
            p("KNOW", "mental"),
            p("BEFORE", "time"),
        ],
        "teach" | "teaching" => vec![
            p("SAY", "speech"),
            p("KNOW", "mental"),
            p("SOMEONE", "substantive"),
        ],
        "learn" | "learning" => vec![
            p("KNOW", "mental"),
            p("BEFORE", "time"),
            p("NOT", "logical"),
        ],
        "create" | "creation" => vec![
            p("DO", "action"),
            p("SOMETHING", "substantive"),
            p("BEFORE", "time"),
            p("NOT", "logical"),
        ],
        "destroy" => vec![
            p("DO", "action"),
            p("NOT", "logical"),
            p("THERE IS", "existence"),
        ],
        "birth" | "born" => vec![p("LIVE", "life"), p("BEFORE", "time"), p("NOT", "logical")],
        "grow" | "growing" | "growth" => vec![
            p("LIVE", "life"),
            p("BIG", "descriptor"),
            p("MUCH/MANY", "quantifier"),
        ],
        "change" | "changing" => vec![p("HAPPEN", "action"), p("OTHER", "determiner")],
        "pain" | "suffer" | "suffering" => vec![p("FEEL", "mental"), p("BAD", "evaluator")],
        "happy" | "happiness" | "joy" => vec![p("FEEL", "mental"), p("GOOD", "evaluator")],
        "sad" | "sadness" | "sorrow" => vec![
            p("FEEL", "mental"),
            p("BAD", "evaluator"),
            p("WANT", "mental"),
        ],
        "anger" | "angry" => vec![
            p("FEEL", "mental"),
            p("BAD", "evaluator"),
            p("WANT", "mental"),
            p("DO", "action"),
        ],
        "soul" | "spirit" => vec![
            p("SOMETHING", "substantive"),
            p("INSIDE", "space"),
            p("LIVE", "life"),
            p("FEEL", "mental"),
        ],
        "mind" => vec![
            p("SOMETHING", "substantive"),
            p("THINK", "mental"),
            p("INSIDE", "space"),
        ],
        "heart" => vec![
            p("SOMETHING", "substantive"),
            p("FEEL", "mental"),
            p("INSIDE", "space"),
            p("BODY", "substantive"),
        ],
        "voice" => vec![
            p("SAY", "speech"),
            p("BODY", "substantive"),
            p("HEAR", "mental"),
        ],
        "eye" | "eyes" => vec![p("SEE", "mental"), p("BODY", "substantive")],
        "hand" | "hands" => vec![p("DO", "action"), p("BODY", "substantive")],
        "face" => vec![
            p("SEE", "mental"),
            p("SOMEONE", "substantive"),
            p("BODY", "substantive"),
        ],
        "king" | "queen" => vec![
            p("SOMEONE", "substantive"),
            p("BIG", "descriptor"),
            p("ABOVE", "space"),
            p("PEOPLE", "substantive"),
        ],
        "death" => vec![p("DIE", "life"), p("HAPPEN", "action")],
        "sleep" | "sleeping" => vec![
            p("NOT", "logical"),
            p("KNOW", "mental"),
            p("BODY", "substantive"),
        ],
        "walk" | "walking" | "run" | "running" => {
            vec![p("MOVE", "action"), p("BODY", "substantive")]
        }
        "fly" | "flying" => vec![p("MOVE", "action"), p("ABOVE", "space")],
        "fall" | "falling" => vec![p("MOVE", "action"), p("BELOW", "space")],
        "give" | "giving" => vec![
            p("DO", "action"),
            p("HAVE", "existence"),
            p("SOMEONE", "substantive"),
        ],
        "take" | "taking" => vec![
            p("DO", "action"),
            p("HAVE", "existence"),
            p("I", "substantive"),
        ],
        "open" | "opening" => vec![p("DO", "action"), p("CAN", "logical"), p("INSIDE", "space")],
        "closing" => vec![
            p("DO", "action"),
            p("NOT", "logical"),
            p("CAN", "logical"),
            p("INSIDE", "space"),
        ],
        "begin" | "start" => vec![p("BEFORE", "time"), p("ALL", "quantifier")],
        "end" | "finish" => vec![p("AFTER", "time"), p("ALL", "quantifier")],
        "wait" | "waiting" => vec![
            p("NOT", "logical"),
            p("DO", "action"),
            p("WHEN/TIME", "time"),
        ],
        "search" | "searching" | "seek" => vec![
            p("WANT", "mental"),
            p("SEE", "mental"),
            p("SOMETHING", "substantive"),
        ],
        "answer" => vec![p("SAY", "speech"), p("BECAUSE", "logical")],
        "question" => vec![p("WANT", "mental"), p("KNOW", "mental"), p("SAY", "speech")],
        "time" => vec![p("WHEN/TIME", "time")],
        "space" => vec![p("WHERE/PLACE", "space"), p("BIG", "descriptor")],
        "nothing" | "void" | "empty" => vec![p("NOT", "logical"), p("SOMETHING", "substantive")],

        // Default: decompose based on category
        _ => default_primes(word),
    }
}

fn p(prime: &'static str, domain: &'static str) -> SemanticPrime {
    SemanticPrime { prime, domain }
}

fn default_primes(word: &str) -> Vec<SemanticPrime> {
    // Stage 1: Check extended word list (~500+ additional words)
    if let Some(primes) = crate::word_primes::lookup(&word.to_lowercase()) {
        return primes;
    }

    // Stage 2: Morphological decomposition — strip affixes, try root
    if let Some(primes) = try_morphological(word) {
        return primes;
    }

    // Stage 3: Compound splitting — try splitting into two known words
    if let Some(primes) = try_compound_split(word) {
        return primes;
    }

    // Stage 2: Derive from category + word character hash
    // This gives every unknown word a unique prime fingerprint
    // instead of collapsing to a single generic prime.
    let cat = categorize(word);
    let h = word_hash(word);

    let mut base = match cat {
        Category::Entity => vec![p("SOMETHING", "substantive")],
        Category::Action => vec![p("DO", "action")],
        Category::Property => vec![p("LIKE/WAY", "similarity")],
        Category::Relation => vec![p("WHERE/PLACE", "space")],
        Category::Particle => vec![p("THIS", "determiner")],
        Category::Negation => vec![p("NOT", "logical")],
        Category::Question => vec![p("SOMETHING", "substantive")],
    };

    // Add a domain prime derived from the word's hash
    let domain_primes = [
        p("THINK", "mental"),
        p("WHERE/PLACE", "space"),
        p("WHEN/TIME", "time"),
        p("THERE IS", "existence"),
        p("LIVE", "life"),
        p("FEEL", "mental"),
        p("MOVE", "action"),
        p("SEE", "mental"),
    ];
    base.push(domain_primes[(h % domain_primes.len() as u64) as usize].clone());
    base
}

/// Strip English affixes and try to decompose the root word.
fn try_morphological(word: &str) -> Option<Vec<SemanticPrime>> {
    let w = word.to_lowercase();
    if w.len() < 5 {
        return None;
    } // too short to strip meaningfully

    // ── Prefix stripping ──
    let prefix_rules: &[(&str, usize, &[(&str, &str)])] = &[
        ("un", 3, &[("NOT", "logical")]),
        ("re", 3, &[("BEFORE", "time"), ("THE SAME", "determiner")]),
        ("pre", 3, &[("BEFORE", "time")]),
        (
            "over",
            3,
            &[("MUCH/MANY", "quantifier"), ("ABOVE", "space")],
        ),
        ("under", 3, &[("BELOW", "space")]),
        ("mis", 3, &[("BAD", "evaluator"), ("NOT", "logical")]),
        ("dis", 3, &[("NOT", "logical")]),
        ("out", 3, &[("MUCH/MANY", "quantifier")]),
        ("sub", 3, &[("BELOW", "space")]),
    ];

    for (prefix, min_len, extra) in prefix_rules {
        if w.starts_with(prefix) && w.len() >= prefix.len() + min_len {
            let root = &w[prefix.len()..];
            let primes = decompose_to_primes(root);
            let is_default = primes.len() <= 2
                && primes
                    .iter()
                    .any(|p| p.prime == "SOMETHING" || p.prime == "DO" || p.prime == "LIKE/WAY");
            if !is_default {
                let mut result = primes;
                for (prime, domain) in *extra {
                    if !result.iter().any(|p| p.prime == *prime) {
                        result.push(p(prime, domain));
                    }
                }
                return Some(result);
            }
        }
    }

    // ── Suffix stripping ──
    // (suffix, min_remaining_len, suffix_primes_to_add)
    let rules: &[(&str, usize, &[(&str, &str)])] = &[
        ("ness", 3, &[]),                            // sadness → sad
        ("ment", 3, &[]),                            // movement → move
        ("tion", 3, &[]),                            // creation → create (via crea→create)
        ("sion", 3, &[]),                            // decision → decide
        ("ity", 3, &[]),                             // serenity → serene
        ("ous", 3, &[]),                             // joyous → joy
        ("ful", 3, &[("MUCH/MANY", "quantifier")]),  // hopeful → hope + MUCH
        ("less", 3, &[("NOT", "logical")]),          // hopeless → hope + NOT
        ("able", 3, &[("CAN", "logical")]),          // loveable → love + CAN
        ("ible", 3, &[("CAN", "logical")]),          // reversible → reverse + CAN
        ("ly", 3, &[]),                              // quickly → quick
        ("er", 3, &[("SOMEONE", "substantive")]),    // dreamer → dream + SOMEONE
        ("or", 3, &[("SOMEONE", "substantive")]),    // creator → create + SOMEONE
        ("ist", 3, &[("SOMEONE", "substantive")]),   // artist → art + SOMEONE
        ("ism", 3, &[("SOMETHING", "substantive")]), // realism → real + SOMETHING
        ("ing", 3, &[]),                             // dreaming → dream
        ("ed", 3, &[]),                              // loved → love
        ("al", 3, &[]),                              // natural → nature
        ("ive", 3, &[]),                             // creative → create
    ];

    for (suffix, min_len, extra) in rules {
        if w.ends_with(suffix) && w.len() >= suffix.len() + min_len {
            let root = &w[..w.len() - suffix.len()];

            // Try the root directly and with common spelling adjustments
            for candidate in spelling_variants(root) {
                let primes = decompose_to_primes(&candidate);
                let is_default = primes.len() == 1
                    && (primes[0].prime == "SOMETHING"
                        || primes[0].prime == "DO"
                        || primes[0].prime == "LIKE/WAY");

                if !is_default {
                    let mut result = primes;
                    for (prime, domain) in *extra {
                        result.push(p(prime, domain));
                    }
                    return Some(result);
                }
            }
        }
    }

    None
}

fn spelling_variants(root: &str) -> Vec<String> {
    let mut v = vec![root.to_string()];
    // "creat" → "create", "lov" → "love" (add trailing e)
    v.push(format!("{}e", root));
    // "happi" → "happy" (i→y)
    if let Some(stripped) = root.strip_suffix('i') {
        v.push(format!("{}y", stripped));
    }
    // "runn" → "run" (doubled consonant)
    if root.len() >= 3 {
        let bytes = root.as_bytes();
        if bytes[bytes.len() - 1] == bytes[bytes.len() - 2] {
            v.push(root[..root.len() - 1].to_string());
        }
    }
    v
}

/// Try splitting a compound word into two known words and merging their primes.
fn try_compound_split(word: &str) -> Option<Vec<SemanticPrime>> {
    let w = word.to_lowercase();
    if w.len() < 6 {
        return None;
    }

    for i in 3..w.len().saturating_sub(2) {
        let left = &w[..i];
        let right = &w[i..];
        if dictionary_lookup(left).is_some() && dictionary_lookup(right).is_some() {
            let mut primes = decompose_to_primes(left);
            let right_primes = decompose_to_primes(right);
            for rp in right_primes {
                if !primes
                    .iter()
                    .any(|p| p.prime == rp.prime && p.domain == rp.domain)
                {
                    primes.push(rp);
                }
            }
            return Some(primes);
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    // --- fnv1a ---

    #[test]
    fn fnv1a_deterministic() {
        assert_eq!(fnv1a(b"hello"), fnv1a(b"hello"));
    }

    #[test]
    fn fnv1a_differs() {
        assert_ne!(fnv1a(b"hello"), fnv1a(b"world"));
    }

    #[test]
    fn fnv1a_empty() {
        // FNV-1a of empty input is the offset basis
        assert_eq!(fnv1a(b""), 14695981039346656037);
    }

    // --- word_hash ---

    #[test]
    fn word_hash_case_insensitive() {
        assert_eq!(word_hash("Time"), word_hash("time"));
        assert_eq!(word_hash("LOVE"), word_hash("love"));
    }

    // --- hash_to_float ---

    #[test]
    fn hash_to_float_in_range() {
        for seed in 0..100u64 {
            let f = hash_to_float(seed * 1000000);
            assert!((0.0..1.0).contains(&f), "hash_to_float({}) = {}", seed, f);
        }
    }

    // --- categorize ---

    #[test]
    fn categorize_nouns() {
        assert_eq!(categorize("time"), Category::Entity);
        assert_eq!(categorize("world"), Category::Entity);
        assert_eq!(categorize("dog"), Category::Entity);
    }

    #[test]
    fn categorize_verbs() {
        assert_eq!(categorize("run"), Category::Action);
        assert_eq!(categorize("think"), Category::Action);
    }

    #[test]
    fn categorize_adjectives() {
        assert_eq!(categorize("big"), Category::Property);
        assert_eq!(categorize("beautiful"), Category::Property);
    }

    #[test]
    fn categorize_negation() {
        assert_eq!(categorize("not"), Category::Negation);
        assert_eq!(categorize("cannot"), Category::Negation);
    }

    #[test]
    fn categorize_particles() {
        assert_eq!(categorize("the"), Category::Particle);
        assert_eq!(categorize("a"), Category::Particle);
        assert_eq!(categorize("I"), Category::Particle);
    }

    #[test]
    fn categorize_relations() {
        assert_eq!(categorize("in"), Category::Relation);
        assert_eq!(categorize("with"), Category::Relation);
    }

    // --- detect_aspect ---

    #[test]
    fn aspect_ing_unbounded() {
        assert_eq!(detect_aspect("running"), Aspect::Unbounded);
        assert_eq!(detect_aspect("thinking"), Aspect::Unbounded);
    }

    #[test]
    fn aspect_ed_bounded() {
        assert_eq!(detect_aspect("walked"), Aspect::Bounded);
    }

    #[test]
    fn aspect_base_timeless() {
        assert_eq!(detect_aspect("run"), Aspect::Timeless);
        assert_eq!(detect_aspect("love"), Aspect::Timeless);
    }

    // --- categorize_sentence ---

    #[test]
    fn sentence_basic() {
        let words = categorize_sentence("the dog runs");
        assert!(!words.is_empty());
        assert!(words.iter().any(|w| w.word == "dog"));
        assert!(words.iter().any(|w| w.category == Category::Particle)); // "the"
    }

    #[test]
    fn sentence_question() {
        let words = categorize_sentence("what is time?");
        assert!(words.iter().any(|w| w.category == Category::Question));
    }

    #[test]
    fn sentence_empty() {
        let words = categorize_sentence("");
        // Should either be empty or only contain structural markers
        assert!(words.is_empty() || words.iter().all(|w| w.word.is_empty()));
    }

    // --- decompose_to_primes ---

    #[test]
    fn primes_known_word() {
        let primes = decompose_to_primes("love");
        assert!(!primes.is_empty());
    }

    #[test]
    fn primes_basic_prime() {
        // "I" is itself a semantic prime
        let primes = decompose_to_primes("I");
        assert!(!primes.is_empty());
        assert!(primes.iter().any(|p| p.prime == "I"));
    }

    // --- category_priority ---

    #[test]
    fn priority_ordering() {
        assert!(category_priority(Category::Entity) < category_priority(Category::Particle));
    }
}

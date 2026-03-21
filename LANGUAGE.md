# Torus Language System

How English text maps to visual form. This document describes the linguistic engine that drives symbol generation.

## Design Principles

1. **Order independence** — "time is a circle" and "a circle is time" produce identical symbols. Words are sorted by semantic role and content hash, not input order.
2. **No linear time** — Torus uses aspect (timeless, bounded, unbounded) instead of tense. There is no past or future, only the shape of an event.
3. **Semantic primacy** — Visual features encode linguistic categories. An entity always produces tendrils, a negation always carves a void. The symbol is a diagram of meaning, not decoration.
4. **Deterministic** — The same words always produce the same symbol.

## Word Categories

Every word is classified into one of seven categories, each with a distinct visual mark on the ring:

| Category | Examples | Visual Mark |
|----------|----------|-------------|
| **Entity** | sun, child, ocean | Outward tendrils with branching sub-tendrils and satellite splotches |
| **Action** | run, think, give | Stroke-width rhythmic waves (bold pulses) |
| **Property** | bright, slowly, vast | Inner ripples (serrated edge modulation) |
| **Relation** | above, through, with | Silk-thin bridges with slight bow |
| **Particle** | the, and, this | Decisive notches with ink press |
| **Negation** | not, never, without | Deep inverted voids with bold ink edges |
| **Question** | ?, why, how, where | Gap in the ring (stroke fades to nothing) |

## Aspect Detection

Instead of tense, Torus detects three aspects from English morphology:

- **Timeless** — Base form. Eternal, always-true statements. ("the sun is bright")
- **Unbounded** — `-ing` suffix. Ongoing, continuous. ("the river is flowing")
- **Bounded** — `-ed` suffix or irregular past. Completed, contained. ("the star collapsed")

Aspect influences the overall rhythm of the symbol but not the positions of individual marks.

## Semantic Roles

Words are assigned roles using a simple SVO (Subject-Verb-Object) heuristic:

- **Agent** — The nearest entity before a verb
- **Action** — The verb itself
- **Patient** — The nearest entity after a verb
- **Modifier** — Adjective or adverb attached to a specific head word
- **Unmarked** — Structural words (particles, relations)

Roles determine angular position on the ring. Agents and patients occupy cardinal positions; modifiers orbit their head word.

## Semantic Primes

Torus decomposes words into semantic primes based on Anna Wierzbicka's Natural Semantic Metalanguage (NSM) framework — 65 universal concepts that exist across all human languages:

**Substantive:** SOMETHING, PERSON, PEOPLE, THING
**Mental:** KNOW, SEE, WANT, THINK, FEEL, IMAGINE, SAY
**Action:** LIVE, MOVE, DIE, TOUCH, GIVE, TAKE
**Evaluator:** GOOD, BAD
**Descriptor:** BIG, SMALL, ABOVE, BELOW, INSIDE, OUTSIDE
**Quantifier:** MUCH, FEW, PART, MORE, WHOLE

The prime decomposition is displayed in the UI alongside each word mark and influences the fine geometry of mark placement.

## Dictionary

The core dictionary contains 150+ hand-curated entries covering common English words. Words not in the dictionary are classified by heuristic rules (suffix analysis, common patterns). The extended semantic prime database covers 500+ words grouped by semantic field (animals, emotions, colors, spatial concepts, etc.).

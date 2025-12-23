# Roozle

Roozle is a string search and analysis library written in Rust, a personal project based on Poozle (IITM Programming Club), but in Rust.
It is designed for large in-memory text buffers, repeatable searches, and separable analysis.

Roozle does not try to be a regex engine or a grep replacement.
It is a search + analysis engine: search once, analyze many ways.

## Basic Example (Exact Search)

Roozle separates **search** from **analysis**.
Search finds matches; analysis decides *what to compute* from those matches.

```rust
use roozle as rz;
use roozle::Search;

let content = "the quick brown fox jumps over the lazy dog";
let buffer = rz::Buffer::from_string(content);

// Create an exact search pattern
let exact = rz::Exact::from_pattern("the".to_string());

// Configure which analyses to run
let mut config = rz::AnalysisConfig::new()
    .with::<rz::MatchIndicesReport>()
    .with::<rz::MatchFrequencyReport>();

config.add::<rz::MatchCountReport>();

// Run the search
let analysis = exact.search(&buffer, &config);

// Retrieve reports by type
let count = analysis
    .report::<rz::MatchCountReport>()
    .unwrap()
    .count;

let frequencies = &analysis
    .report::<rz::MatchFrequencyReport>()
    .unwrap()
    .frequencies;

let indices = &analysis
    .report::<rz::MatchIndicesReport>()
    .unwrap()
    .indices;

assert_eq!(count, 2);
assert_eq!(frequencies["the"], 2);
assert_eq!(indices["the"], vec![0, 31]);
```

### Available Reports (current)

* `MatchCountReport` – total number of matches
* `MatchFrequencyReport` – per-pattern frequency
* `MatchIndicesReport` – match positions
* `UniqueMatchesReport` – unique matched strings

---

### Planned Features

* Multi-threaded search
* GPU-accelerated search
* Regex search
* Precompiled indices (FM-index / similar) for repeated queries

---

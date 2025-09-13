use std::any::Any;
use std::collections::{HashMap, HashSet};
use std::fmt::Debug;

// TODO
// #[derive(Debug, PartialEq, Eq, Hash, Clone)]
// pub enum Index {
//     // index of match
//     ByteIndex(usize),
//     // index of line start, index of match
//     LinedIndex(usize, usize),
// }

pub trait Report: Debug + Any {
    fn process(&mut self, match_: &str, index: usize);
    fn as_any(&self) -> &dyn Any;
    fn boxed() -> Box<dyn Report> where Self: Sized;
}

#[derive(Debug, Default)]
pub struct UniqueMatchesReport {
    pub matches: HashSet<String>,
}

#[derive(Debug, Default)]
pub struct MatchFrequencyReport {
    pub frequencies: HashMap<String, usize>,
}

#[derive(Debug, Default)]
pub struct MatchIndicesReport {
    pub indices: HashMap<String, Vec<usize>>,
    /* TODO: why have indices as only byte indices?
     * can have abs_idx, byte_idx, newline:abs_idx etc
     * hence should add ending index as well, because byte_idx
     * does not tell, but then again byte_idx_end = byte_idx + len(bytes(pattern))
     */
}

#[derive(Debug, Default)]
pub struct MatchCountReport {
    pub count: usize,
}

impl UniqueMatchesReport {
    fn new() -> UniqueMatchesReport {
        UniqueMatchesReport::default()
    }
}

impl MatchFrequencyReport {
    fn new() -> MatchFrequencyReport {
        MatchFrequencyReport::default()
    }
}

impl MatchIndicesReport {
    fn new() -> MatchIndicesReport {
        MatchIndicesReport::default()
    }
}

impl MatchCountReport {
    fn new() -> MatchCountReport {
        MatchCountReport::default()
    }
}

impl Report for UniqueMatchesReport {
    fn process(&mut self, match_: &str, index: usize) {
        self.matches.insert(String::from(match_));
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn boxed() -> Box<dyn Report> {
        Box::new(UniqueMatchesReport::new())
    }
}

impl Report for MatchFrequencyReport {
    fn process(&mut self, match_: &str, index: usize) {
        let match_count = self.frequencies.entry(String::from(match_)).or_insert(0);
        *match_count += 1;
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn boxed() -> Box<dyn Report> {
        Box::new(MatchFrequencyReport::new())
    }
}

impl Report for MatchIndicesReport {
    fn process(&mut self, match_: &str, index: usize) {
        let match_indices = self
            .indices
            .entry(String::from(match_))
            .or_insert_with(Vec::new);
        match_indices.push(index);  // TODO
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn boxed() -> Box<dyn Report> {
        Box::new(MatchIndicesReport::new())
    }
}

impl Report for MatchCountReport {
    fn process(&mut self, match_: &str, index: usize) {
        self.count += 1;
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn boxed() -> Box<dyn Report> {
        Box::new(MatchCountReport::new())
    }
}

use std::any::Any;
use std::collections::{HashMap, HashSet};
use std::fmt::Debug;

use super::analysis::AnalysisOption;

pub trait Report: Debug + Any {
    fn process(&mut self, match_: &str, index: usize);
    fn as_any(&self) -> &dyn Any;
}

#[derive(Debug, Default)]
pub struct UniqueMatchesReport {
    matches: HashSet<String>,
    unique_matches: usize,
}

#[derive(Debug, Default)]
pub struct MatchFrequencyReport {
    frequency: HashMap<String, usize>,
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

        self.unique_matches = self.matches.len();
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl Report for MatchFrequencyReport {
    fn process(&mut self, match_: &str, index: usize) {
        let match_count = self.frequency.entry(String::from(match_)).or_insert(0);
        *match_count += 1;
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl Report for MatchIndicesReport {
    fn process(&mut self, match_: &str, index: usize) {
        let match_indices = self
            .indices
            .entry(String::from(match_))
            .or_insert_with(Vec::new);
        match_indices.push(index);
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl Report for MatchCountReport {
    fn process(&mut self, match_: &str, index: usize) {
        self.count += 1;
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

pub fn from_analysis_option(option: &AnalysisOption) -> Box<dyn Report> {
    match option {
        AnalysisOption::UniqueMatches => Box::new(UniqueMatchesReport::new()),
        AnalysisOption::MatchFrequency => Box::new(MatchFrequencyReport::new()),
        AnalysisOption::MatchIndices => Box::new(MatchIndicesReport::new()),
        AnalysisOption::MatchCount => Box::new(MatchCountReport::new()),
    }
}

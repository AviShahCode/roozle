use std::collections::{HashMap, HashSet};
use std::fmt::Debug;

use super::analysis::AnalysisOption;

pub trait Report: Debug {
    fn process(&mut self, word: &str, index: usize, pattern: &str);
}

#[derive(Debug, Default)]
pub struct UniqueMatchesReport {
    matches: HashSet<String>,
    len: usize, // TODO
}

#[derive(Debug, Default)]
pub struct MatchFrequencyReport {
    frequency: HashMap<String, usize>,
    // maybe total frequency
}

#[derive(Debug, Default)]
pub struct MatchIndicesReport {
    indices: HashMap<String, Vec<usize>>,
}

#[derive(Debug, Default)]
pub struct MatchCountReport {
    count: usize,
}

impl UniqueMatchesReport {
    fn new() -> UniqueMatchesReport {
        UniqueMatchesReport::default()
    }
}

impl Report for UniqueMatchesReport {
    fn process(&mut self, word: &str, index: usize, pattern: &str) {
        self.matches.insert(String::from(word));
    }
}

impl MatchFrequencyReport {
    fn new() -> MatchFrequencyReport {
        MatchFrequencyReport::default()
    }
}

impl Report for MatchFrequencyReport {
    fn process(&mut self, word: &str, index: usize, pattern: &str) {
        // TODO: if in matches, increment counter, else add
    }
}

impl MatchIndicesReport {
    fn new() -> MatchIndicesReport {
        MatchIndicesReport::default()
    }
}

impl Report for MatchIndicesReport {
    fn process(&mut self, word: &str, index: usize, pattern: &str) {
        // TODO
    }
}

impl MatchCountReport {
    fn new() -> MatchCountReport {
        MatchCountReport::default()
    }
}

impl Report for MatchCountReport {
    fn process(&mut self, word: &str, index: usize, pattern: &str) {
        self.count += 1;
    }
}

pub fn from_analysis_option(option: &AnalysisOption) -> Box<dyn Report> {
    match option { 
        AnalysisOption::UniqueMatches => {
            Box::new(UniqueMatchesReport::new())
        }
        AnalysisOption::MatchFrequency => {
            Box::new(MatchFrequencyReport::new())
        }
        AnalysisOption::MatchIndices => {
            Box::new(MatchIndicesReport::new())
        }
        AnalysisOption::MatchCount => {
            Box::new(MatchCountReport::new())
        }
    }
}
use std::collections::HashSet;

use crate::analysis::{Analysis, AnalysisConfig};
use crate::buffer;

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum SearchOption {
    CaseInsensitive,
    // TODO:
    // one for counting whole line if match occurs in line
    // one for inverting such match
}

pub trait Search {
    fn search(&self, buffer: &buffer::Buffer, analysis_config: AnalysisConfig) -> Analysis;
    // search with options
}

#[derive(Debug, Default)]
struct SearchOptions {
    options: HashSet<SearchOption>,
}

impl SearchOptions {
    fn add_option(&mut self, option: SearchOption) {
        self.options.insert(option);
    }

    fn remove_option(&mut self, option: &SearchOption) {
        self.options.remove(option);
    }

    fn has_option(&self, option: &SearchOption) -> bool {
        self.options.contains(option)
    }
}

use std::collections::{HashMap, HashSet};
use crate::analysis::report::Report;
use super::report;

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum AnalysisOption {
    UniqueMatches,
    MatchFrequency,
    MatchIndices,
    MatchCount,
}

#[derive(Debug, Default)]
struct AnalysisOptions {
    options: HashSet<AnalysisOption>,
}

#[derive(Debug, Default)]
pub struct Analysis {
    analysis_options: AnalysisOptions,
    reports: HashMap<AnalysisOption, Box<dyn report::Report>>,
}

impl AnalysisOptions {
    fn new() -> AnalysisOptions {
        AnalysisOptions::default()
    }

    fn add_option(&mut self, option: AnalysisOption) {
        self.options.insert(option);
    }

    fn remove_option(&mut self, option: &AnalysisOption) {
        self.options.remove(option);
    }

    fn has_option(&self, option: &AnalysisOption) -> bool {
        self.options.contains(option)
    }
}

impl Analysis {
    fn new() -> Analysis {
        Analysis::default()
    }

    fn from_analysis_options(options: AnalysisOptions) -> Analysis {
        let mut reports = HashMap::new();

        for option in &options.options {
            let report = report::from_analysis_option(option);
            reports.insert(option, report);
        }

        Analysis { analysis_options: options, reports: HashMap::new() }
    }

    fn add_analysis_option(&mut self, option: AnalysisOption) {
        if self.analysis_options.has_option(&option) {
            return;
        }
        self.analysis_options.add_option(option.clone());
        let report = report::from_analysis_option(&option);
        self.reports.insert(option, report);
    }

    // TODO: remove, has

    fn get_report(&self, option: &AnalysisOption) -> Option<&Box<dyn Report>> {
        self.reports.get(option)
    }
}

use std::any::TypeId;
use std::collections::{HashMap, HashSet};

use super::report;

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum AnalysisOption {
    UniqueMatches,
    MatchFrequency,
    MatchIndices,
    MatchCount,
}

#[derive(Debug, Default)]
pub struct AnalysisOptions {
    options: HashSet<AnalysisOption>,
}

#[derive(Debug, Default)]
pub struct Analysis {
    analysis_options: AnalysisOptions,
    reports: HashMap<TypeId, Box<dyn report::Report>>,
}

impl AnalysisOptions {
    pub fn new() -> AnalysisOptions {
        AnalysisOptions::default()
    }

    pub fn add(&mut self, option: AnalysisOption) {
        self.options.insert(option);
    }

    pub fn remove(&mut self, option: &AnalysisOption) {
        self.options.remove(option);
    }

    pub fn has(&self, option: &AnalysisOption) -> bool {
        self.options.contains(option)
    }

    pub fn with(mut self, option: AnalysisOption) -> Self {
        self.add(option);
        self
    }
}

impl Analysis {
    pub fn new() -> Analysis {
        Analysis::default()
    }

    pub fn from_analysis_options(options: AnalysisOptions) -> Analysis {
        let mut reports = HashMap::new();

        for option in &options.options {
            let report = report::from_analysis_option(option);
            reports.insert((&*report).type_id(), report);
        }

        Analysis {
            analysis_options: options,
            reports,
        }
    }

    pub fn process(&mut self, match_: &str, index: usize) {
        for report in self.reports.values_mut() {
            report.process(match_, index);
        }
    }

    pub fn report<T: report::Report>(&self) -> Option<&T> {
        let r = self.reports.get(&TypeId::of::<T>());
        if let Some(r) = r {
            return r.as_any().downcast_ref::<T>();
        }
        None
    }
}

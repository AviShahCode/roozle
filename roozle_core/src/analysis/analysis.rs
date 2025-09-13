use std::any::TypeId;
use std::collections::HashMap;

use super::report::Report;

#[derive(Debug, Default)]
pub struct AnalysisConfig {
    config: HashMap<TypeId, fn() -> Box<dyn Report>>,
}

#[derive(Debug, Default)]
pub struct Analysis {
    reports: HashMap<TypeId, Box<dyn Report>>,
}

impl AnalysisConfig {
    pub fn new() -> Self {
        AnalysisConfig::default()
    }

    pub fn add<T: Report>(&mut self) {
        self.config.insert(TypeId::of::<T>(), || T::boxed());
    }

    pub fn build(&self) -> HashMap<TypeId, Box<dyn Report>> {
        let mut reports = HashMap::new();
        for (t, r) in &self.config {
            let r = r();
            reports.insert(*t, r);
        }
        reports
    }

    pub fn remove<T: Report>(&mut self) -> &mut Self {
        self.config.remove(&TypeId::of::<T>());
        self
    }

    pub fn has<T: Report>(&self) -> bool {
        self.config.contains_key(&TypeId::of::<T>())
    }

    pub fn with<T: Report>(mut self) -> Self {
        self.add::<T>();
        self
    }
}

impl Analysis {
    pub fn new() -> Analysis {
        Analysis::default()
    }

    pub fn from_config(config: AnalysisConfig) -> Analysis {
        let reports = config.build();
        Analysis {
            reports,
        }
    }

    pub fn process(&mut self, match_: &str, index: usize) {
        for report in self.reports.values_mut() {
            report.process(match_, index); // TODO: custom Index type
        }
    }

    pub fn report<T: Report>(&self) -> Option<&T> {
        let r = self.reports.get(&TypeId::of::<T>());
        if let Some(r) = r {
            return r.as_any().downcast_ref::<T>();
        }
        None
    }
}

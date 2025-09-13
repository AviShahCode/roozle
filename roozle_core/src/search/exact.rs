use super::search::{Search};
use crate::analysis::{Analysis, AnalysisConfig};
use crate::buffer::Buffer;

#[derive(Debug)]
pub struct Exact {
    pattern: String,
    // validators: HashMap<SearchOption, Box<dyn Validator>>,
}

impl Exact {
    pub fn from_pattern(pattern: String) -> Exact {
        Exact { pattern }
    }
}

impl Search for Exact {
    fn search(&self, buffer: &Buffer, analysis_config: AnalysisConfig) -> Analysis {
        let mut analysis = Analysis::from_config(analysis_config);
        for (index, match_) in buffer.match_indices(&self.pattern) {
            analysis.process(match_, index);
        }
        analysis
    }
}

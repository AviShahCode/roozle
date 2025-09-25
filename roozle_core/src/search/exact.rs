use rayon::prelude::*;

use super::search::Search;
use crate::analysis::{Analysis, AnalysisConfig};
use crate::buffer::Buffer;
use crate::get_pool;

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
    fn search(&self, buffer: &Buffer, analysis_config: &AnalysisConfig) -> Analysis {
        let mut analysis = Analysis::from_config(analysis_config);
        for (index, match_) in buffer.match_indices(&self.pattern) {
            analysis.process(match_, index);
        }
        analysis
    }

    fn search_mt(&self, buffer: &Buffer, analysis_config: &AnalysisConfig) -> Analysis {
        let pool = get_pool();
        let thread_count = pool.current_num_threads();
        let chunk_count = 2 * thread_count;
        let chunk_size = (buffer.len() - 1) / chunk_count + 1;

        let buffer_len = buffer.len();
        let pattern_len = self.pattern.len();

        if chunk_size < 10_000 {
            return self.search(buffer, analysis_config);
        }

        pool.install(|| {
            (0..chunk_count)
                .into_par_iter()
                .map(|i| {
                    let mut start = i * chunk_size;
                    let mut end = ((i + 1) * chunk_size + pattern_len - 1).min(buffer_len);

                    // adjust to UTF-8 char boundaries
                    while start < buffer_len && !buffer.is_char_boundary(start) {
                        start += 1;
                    }
                    while end < buffer_len && !buffer.is_char_boundary(end) {
                        end += 1;
                    }

                    if start >= end || end > buffer_len {
                        return Analysis::from_config(analysis_config);
                    }

                    let mut partial = Analysis::from_config(analysis_config);
                    let chunk = buffer.slice(start, end);

                    for (rel_idx, m) in chunk.match_indices(&self.pattern) {
                        partial.process(m, start + rel_idx);
                    }

                    partial
                })
                .reduce(
                    || Analysis::from_config(analysis_config),
                    |mut a, b| {
                        a.merge(b);
                        a
                    },
                )
        })
    }
}

use std::sync::Mutex;
use rayon::prelude::*;

use super::search::{Search};
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

    fn search_mt(&self, buffer: &crate::buffer::Buffer, analysis_config: &AnalysisConfig) -> Analysis {
        let pool = get_pool();
        let thread_count = pool.current_num_threads();
        let chunk_count = 8 * thread_count;
        let chunk_size = (buffer.len() - 1) / chunk_count + 1;

        let buffer_len = buffer.len();
        let pattern_len = self.pattern.len();

        // if chunk_size < 1_000 {
        //     return self.search(buffer, analysis_config);
        // }

        let analysis = Mutex::new(
            Analysis::from_config(analysis_config)
        );
        
        pool.install(|| {
            (0..chunk_count).into_par_iter().for_each(|i| {
                let mut start = i * chunk_size;
                while start < buffer_len && !buffer.is_char_boundary(start) {
                    start += 1;
                }
                // TODO fix start and end is middle of utf-8 char
                let mut end = ((i + 1) * chunk_size + pattern_len - 1).min(buffer_len);
                while end < buffer_len && !buffer.is_char_boundary(end) {
                    end += 1;
                }       
                if start >= end || end > buffer_len {
                    return;
                }
                
                // Get the chunk slice
                let chunk = buffer.slice(start, end);

                let mut partial_analysis = Analysis::from_config(analysis_config);

                for (relative_index, match_) in chunk.match_indices(&self.pattern) {
                    let absolute_index = start + relative_index;
                    partial_analysis.process(match_, absolute_index);
                }

                let mut a = analysis.lock().unwrap();
                a.merge(partial_analysis);
            });
        });
        
        analysis.into_inner().unwrap()
    }
}

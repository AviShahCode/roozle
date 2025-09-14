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
    fn merge(&mut self, other: Box<dyn Report>);  // remove ref
    fn as_any(&self) -> &dyn Any;
    fn into_any(self: Box<Self>) -> Box <dyn Any>;
    fn boxed() -> Box<dyn Report + Send + Sync> where Self: Sized;
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

    fn merge(&mut self, other: Box<dyn Report>) {
        let other = other
            .into_any()
            .downcast::<UniqueMatchesReport>()
            .expect("Type mismatch in merge");
        self.matches.extend(other.matches.into_iter());
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn into_any(self: Box<Self>) -> Box<dyn Any> {
        self
    }

    fn boxed() -> Box<dyn Report + Send + Sync> {
        Box::new(UniqueMatchesReport::new())
    }
}

impl Report for MatchFrequencyReport {
    fn process(&mut self, match_: &str, index: usize) {
        let match_count = self.frequencies.entry(String::from(match_)).or_insert(0);
        *match_count += 1;
    }

    fn merge(&mut self, other: Box<dyn Report>) {
        let other = other
            .into_any()
            .downcast::<MatchFrequencyReport>()
            .expect("Type mismatch in merge");
        for (k, v) in other.frequencies.into_iter() {
            *self.frequencies.entry(k).or_insert(0) += v;
        }
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn into_any(self: Box<Self>) -> Box<dyn Any> {
        self
    }

    fn boxed() -> Box<dyn Report + Send + Sync> {
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

    fn merge(&mut self, other: Box<dyn Report>) {
        todo!("have to be sorted, use merge of merge sort for O(n)")
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn into_any(self: Box<Self>) -> Box<dyn Any> {
        self
    }

    fn boxed() -> Box<dyn Report + Send + Sync> {
        Box::new(MatchIndicesReport::new())
    }
}

impl Report for MatchCountReport {
    fn process(&mut self, match_: &str, index: usize) {
        self.count += 1;
    }

    fn merge(&mut self, other: Box<dyn Report>) {
        let other = other
            .into_any()
            .downcast::<MatchCountReport>()
            .expect("Type mismatch in merge");
        self.count += other.count;
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn into_any(self: Box<Self>) -> Box<dyn Any> {
        self
    }

    fn boxed() -> Box<dyn Report + Send + Sync> {
        Box::new(MatchCountReport::new())
    }
}

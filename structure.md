roozle core src
 |
 + analysis
 |  + unique words
 |  + word frequency
 |  + word indices
 + buffer
 |  + file
 |  + manual add
 + search
 |  + exact
 |  + regex
 |  + fuzzy

# Flow

buffer - string/vec

search type - for each match, pass it to analysis; will generate analysis object containing each individual type
search object - initialise it with what we want (the string), then can pass multiple buffers for continuous searching
returns analysis object - question: should it initialise analysis type as well? maybe initialise a default and have
it modifiable when search is called

analysis - for each word received, add it to it's corresponding report

report - unique words, word frequency etc

example use case:

```rust
use roozle as rz;

fn main() {
    // buffered file input
    let buffer = rz::Buffer::from_file("story.txt");
    // or
    let buffer = rz::Buffer::from_string("the man sat on the chair.");
    
    // search object
    let exact_search = rz::SearchExact::from_pattern("the");
 
    // search options
    let search_options = rz::SearchOptions::with(case_insensitive = true);
 
    let analysis_type = rz::AnalysisOptions::new().with_unique_words().with_word_count();
    // or
    let analysis_type = rz::AnalysisOptions::with(unique_words = true, word_count = true);
 
    // or can set default
    let analysis: rz::Analysis = exact_search.search(buffer, analysis_type, search_flags);
    // or (just single analysis)
    let analysis: rz::Analysis = exact_search.search(buffer, rz::AnalysisOption::WordCount);
 
    // little bit tricky
    let unique_words = analysis.report(rz::AnalysisOption::UniqueWords);// use enums UniqueWords, WordFrequency to get specific report
}
```
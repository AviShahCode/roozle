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
    // let search_options = rz::SearchOptions::with(SearchOption::CaseInsensitive);
    // need to build validators - like case insensitive etc
    // keep it as part of options itself, a Vec<Box<dyn Validator>>
    // have a constructor
 
    let analysis_type = rz::AnalysisOptions::new()
                        .with(UniqueMatchesReport);
    analysis_type.add(MatchCountReport);
    // or
    let analysis_type = rz::AnalysisOptions::with(unique_words = true, word_count = true);
 
    // or can set default
    let analysis: rz::Analysis = exact_search.search(buffer, analysis_type);
    // or (just single analysis)
    // let analysis: rz::Analysis = exact_search.search(buffer, rz::AnalysisOption::WordCount);
 
    // little bit tricky
    let unique_words: rz::UniqueMatchesReport = analysis.report();// use enums UniqueWords, WordFrequency to get specific report
    println!("{}", unique_words.count);
}
```
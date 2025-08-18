use std::collections::HashSet;

#[derive(Debug, PartialEq, Eq, Hash)]
enum SearchOption {
    CaseInsensitive
    // one for counting whole line if match occurs in line
    // one for inverting such match
}

#[derive(Debug, Default)]
struct SearchOptions {
    options: HashSet<SearchOption>
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

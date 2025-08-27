use std::fmt::Debug;

use super::search::{SearchOption};

pub trait Validator: Debug {
    fn validate(&self, c1: char, c2: char) -> bool;
}

#[derive(Debug)]
struct CaseInsensitiveValidator;

impl Validator for CaseInsensitiveValidator {
    fn validate(&self, c1: char, c2: char) -> bool {
        // If both characters are ASCII, use the faster ASCII-specific conversion.
        if c1.is_ascii() && c2.is_ascii() {
            return c1.to_ascii_lowercase() == c2.to_ascii_lowercase();
        }

        // Otherwise, fall back to the full Unicode-aware comparison.
        c1.to_lowercase().eq(c2.to_lowercase())
    }
}

fn from_search_option(option: &SearchOption) -> Box<dyn Validator> {
    match option {
        SearchOption::CaseInsensitive => Box::new(CaseInsensitiveValidator),
    }
}

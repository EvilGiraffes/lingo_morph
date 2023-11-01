use std::fmt::Display;

pub trait Preprocessor: Display {
    fn next(&self, character: char) -> Option<char>;
}

#[derive(Debug)]
pub struct NullPreprocessor;

impl Preprocessor for NullPreprocessor {
    fn next(&self, character: char) -> Option<char> {
        Some(character)
    }
}

impl Display for NullPreprocessor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "NullProcessor")
    }
}


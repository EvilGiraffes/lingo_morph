pub trait Preprocessor {
    fn next(&self, character: char) -> Option<char>;
}

pub struct NullPreprocessor;
impl Preprocessor for NullPreprocessor {
    fn next(&self, character: char) -> Option<char> {
        Some(character)
    }
}


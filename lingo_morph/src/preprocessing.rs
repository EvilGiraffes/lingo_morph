pub trait Preprocessor {
    fn next(character: char) -> Option<char>;
}

pub struct NullPreprocessor;
impl Preprocessor for NullPreprocessor {
    fn next(character: char) -> Option<char> {
        Some(character)
    }
}

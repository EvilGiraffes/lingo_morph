use std::fmt::Display;

#[cfg(feature = "logging")]
use log::{ trace, info };

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

pub struct AggregatePreprocessor {
    processors: Vec<Box<dyn Preprocessor>>,
}

impl Preprocessor for AggregatePreprocessor {
    fn next(&self, character: char) -> Option<char> {
        #[cfg(feature = "logging")]
        info!("Preprocessing with multiple processors");
        let mut current: Option<char> = Some(character);
        for processor in self.processors.iter() {
            #[cfg(feature = "logging")]
            trace!("Current processor is {}", processor);
            current = current.and_then(|inner| processor.next(inner));
        }
        current
    }
}

impl Display for AggregatePreprocessor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "AggregatePreprocessor with: ")?;
        let length = self.processors.len();
        for i in 0..length - 1 {
            write!(f, "{}, ", self.processors[i])?;
        }
        write!(f, "{}", self.processors[length - 1])
    }
}

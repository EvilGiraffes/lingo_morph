use std::{error::Error, fmt::Display};

use crate::source::{Location, Source};

pub type PResult<I, R> = Result<Status<I, R>, ProcessingError>;
pub type Processed<O, R> = PResult<O, R>;

#[derive(Debug)]
pub struct ProcessingError {
    location: Location,
    error: Box<dyn Error>,
}

impl ProcessingError {
    pub fn from_source<S, E>(source: &S, error: E) -> Self
    where
        S: Source,
        E: Error + 'static,
    {
        Self {
            location: source.location(),
            error: Box::new(error),
        }
    }

    pub fn error(&self) -> &dyn Error {
        self.error.as_ref()
    }
}

impl Display for ProcessingError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "processing failed at line {} and column {}, after parsing {} bytes: {}",
            self.location.line(),
            self.location.column(),
            self.location.at_bytes(),
            self.error()
        )
    }
}

impl Error for ProcessingError {}

#[derive(Debug)]
pub enum Status<O, R> {
    Done(O, R),
    Mismatch(R),
}

impl<O, R> Status<O, R> {
    pub fn map<F, U>(self, mapper: F) -> Status<U, R>
    where
        F: FnOnce(O) -> U,
    {
        match self {
            Self::Done(output, rest) => Status::Done(mapper(output), rest),
            Self::Mismatch(rest) => Status::Mismatch(rest),
        }
    }
}

pub fn done<O, R>(output: O, rest: R) -> Processed<O, R> {
    Ok(Status::Done(output, rest))
}

pub fn mismatch<O, R>(rest: R) -> Processed<O, R> {
    Ok(Status::Mismatch(rest))
}

pub fn err<O, R, S, E>(source: &S, error: E) -> Processed<O, R>
where
    S: Source,
    E: Error + 'static,
{
    Err(ProcessingError::from_source(source, error))
}

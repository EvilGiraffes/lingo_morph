use std::error::Error;

use crate::{source::Source, PResult, ProcessingError, Status};

use super::Processor;

pub enum FResult<O> {
    Done(O),
    Incomplete,
    Error(ProcessingError),
}

impl<O> FResult<O> {
    fn map<F, U>(self, mapper: F) -> FResult<U>
    where
        F: FnOnce(O) -> U,
    {
        match self {
            Self::Done(output) => FResult::Done(mapper(output)),
            Self::Incomplete => FResult::Incomplete,
            Self::Error(error) => FResult::Error(error),
        }
    }
}

impl<O, R> From<PResult<O, R>> for FResult<O> {
    fn from(value: PResult<O, R>) -> Self {
        let flattened = match value {
            Ok(status) => status,
            Err(error) => return Self::Error(error),
        };
        match flattened {
            Status::Done(output, _) => FResult::Done(output),
            Status::Mismatch(_) => FResult::Incomplete,
            Status::EOF => FResult::Incomplete,
        }
    }
}

pub type Final<O> = FResult<O>;

pub trait FinalProcessor<I> {
    type Output;

    fn process(&mut self, given: I) -> Final<Self::Output>;

    fn conform<F, T>(self, conform: F) -> Conform<F, Self>
    where
        Self: Sized,
        F: FnMut(T) -> I,
    {
        Conform(conform, self)
    }

    fn map<F, T>(self, mapper: F) -> Map<F, Self>
    where
        Self: Sized,
        F: FnMut(Self::Output) -> T,
    {
        Map(mapper, self)
    }
}

pub struct End<P>(P);

impl<P> End<P> {
    pub fn new(processor: P) -> Self {
        Self(processor)
    }
}

impl<P, S, I, O> FinalProcessor<S> for End<P>
where
    P: Processor<I, Output = O>,
    S: Source<Item = I>,
    S::RollBackErr: Error + 'static,
{
    type Output = O;

    fn process(&mut self, given: S) -> Final<Self::Output> {
        self.0.process(given).into()
    }
}

pub struct Conform<F, E>(F, E);

impl<F, E, I, N, O> FinalProcessor<I> for Conform<F, E>
where
    F: FnMut(I) -> N,
    E: FinalProcessor<N, Output = O>,
{
    type Output = O;

    fn process(&mut self, given: I) -> Final<Self::Output> {
        let mapped = (self.0)(given);
        self.1.process(mapped)
    }
}

pub struct Map<F, E>(F, E);

impl<F, E, I, O, R> FinalProcessor<I> for Map<F, E>
where
    F: FnMut(O) -> R,
    E: FinalProcessor<I, Output = O>,
{
    type Output = R;

    fn process(&mut self, given: I) -> Final<Self::Output> {
        self.1.process(given).map(|inner| (self.0)(inner))
    }
}

use std::fmt::Debug;

use crate::{source::Source, ProcessingError, Processor, Status};

#[derive(Debug)]
pub enum ProcessingFailed {
    DuringProcessing(ProcessingError),
    NoReturn,
}

impl From<ProcessingError> for ProcessingFailed {
    fn from(value: ProcessingError) -> Self {
        ProcessingFailed::DuringProcessing(value)
    }
}

#[derive(Debug)]
pub struct With<'a, I, P>(I, &'a mut P);

impl<'a, I, P> With<'a, I, P> {
    pub(crate) fn new(input: I, processor: &'a mut P) -> Self {
        Self(input, processor)
    }
}

impl<S, I, P> With<'_, S, P>
where
    P: Processor<I>,
    S: Source<Item = I>,
{
    pub fn process(self) -> Result<P::Output, ProcessingFailed> {
        match self.1.process(self.0)? {
            Status::Done(output, _) => Ok(output),
            Status::Mismatch(_) => Err(ProcessingFailed::NoReturn),
        }
    }

    pub fn fold<A, F>(self, init: A, mut func: F) -> Result<A, ProcessingError>
    where
        F: FnMut(A, P::Output) -> A,
    {
        let mut state = init;
        let mut current = self.0;
        while let Status::Done(output, rest) = self.1.process(current)? {
            current = rest;
            state = func(state, output);
        }
        Ok(state)
    }
}

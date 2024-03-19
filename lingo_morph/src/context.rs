use crate::{source::Source, ProcessingError, Processor, Status};

pub struct With<I, P>(I, P);

impl<I, P> With<I, P> {
    pub(crate) fn new(input: I, processor: P) -> Self {
        Self(input, processor)
    }
}

impl<S, I, P> With<S, P>
where
    P: Processor<I>,
    S: Source<Item = I>,
{
    pub fn fold<A, F>(self, init: A, mut func: F) -> Result<A, ProcessingError>
    where
        F: FnMut(A, P::Output) -> A,
    {
        let mut state = init;
        let mut current = self.0;
        let mut processor = self.1;
        while let Status::Done(output, rest) = processor.process(current)? {
            current = rest;
            state = func(state, output);
        }
        Ok(state)
    }
}

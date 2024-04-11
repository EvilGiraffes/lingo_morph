use crate::{source::Source, ProcessingError, Processor, Status};

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

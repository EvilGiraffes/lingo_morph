pub trait Processor<I> {
    type Output;
    fn process(&mut self, given: I) -> Self::Output;

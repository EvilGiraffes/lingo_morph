use super::Processor;

pub type Final<O> = O;

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

impl<P, I, O> FinalProcessor<I> for End<P>
where
    P: Processor<I, Output = O>,
{
    type Output = O;
    fn process(&mut self, given: I) -> Final<Self::Output> {
        let (output, _) = self.0.process(given);
        output
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
        let output = self.1.process(given);
        (self.0)(output)
    }
}

pub trait Processor<I> {
    type Output;
    fn process(&mut self, given: I) -> Self::Output;
    fn map<F, R>(self, mapper: F) -> Map<Self, F>
    where
        Self: Sized,
        F: FnMut(Self::Output) -> R
    {
        map(self, mapper)
    }

pub struct Map<P, F> {
    processor: P,
    map: F,
}

impl<P, I, F, O, R> Processor<I> for Map<P, F>
where
    P: Processor<I, Output = O>,
    F: FnMut(O) -> R,
{
    type Output = R;
    fn process(&mut self, given: I) -> Self::Output {
        (self.map)(self.processor.process(given))
    }
}

pub fn map<P, F, I, O, R>(processor: P, map: F) -> Map<P, F>
where
    P: Processor<I, Output = O>,
    F: FnMut(O) -> R,
{
    Map {
        processor,
        map,
    }
}


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
    fn pipe<P, INT, PO>(self, other: P) -> Pipe<Self, P>
    where
        Self: Sized + Processor<I, Output = INT>,
        P: Processor<INT, Output = PO>,
    {
        pipe(self, other)
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

pub struct Pipe<A, B>(A, B);

impl<A, B, AI, INT, BO> Processor<AI> for Pipe<A, B>
where
    A: Processor<AI, Output = INT>,
    B: Processor<INT, Output = BO>,
{
    type Output = BO;
    fn process(&mut self, given: AI) -> Self::Output {
        let intermediate = self.0.process(given);
        self.1.process(intermediate)
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

pub fn pipe<A, B, AI, INT, BO>(from: A, into: B) -> Pipe<A, B>
where
    A: Processor<AI, Output = INT>,
    B: Processor<INT, Output = BO>,
{
    Pipe(from, into)
}


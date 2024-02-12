pub type Compose<A, B> = Pipe<B, A>;
pub type RightIgnore<L, R> = LeftIgnore<R, L>;

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
    fn compose<P, PI>(self, other: P) -> Compose<Self, P>
    where
        Self: Sized,
        P: Processor<PI, Output = I>,
    {
        compose(self, other)
    }
    fn left_ignore<P, PO>(self, other: P) -> LeftIgnore<Self, P>
    where
        Self: Sized,
        P: Processor<I, Output = PO>,
    {
        left_ignore(self, other)
    }
    fn right_ignore<P, PO>(self, other: P) -> RightIgnore<Self, P>
    where
        Self: Sized,
        P: Processor<I, Output = PO>,
    {
        right_ignore(self, other)
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

pub struct LeftIgnore<L, R>(L, R);

impl<L, R, I, LO, RO> Processor<I> for LeftIgnore<L, R>
where
    I: Copy,
    L: Processor<I, Output = LO>,
    R: Processor<I, Output = RO>,
{
    type Output = RO;
    fn process(&mut self, given: I) -> Self::Output {
        _ = self.0.process(given);
        self.1.process(given)
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

pub fn compose<A, B, AO, INT, BI>(from: A, into: B) -> Compose<A, B>
where
    A: Processor<INT, Output = AO>,
    B: Processor<BI, Output = INT>,
{
    pipe(into, from)
}

pub fn left_ignore<L, R, I, LO, RO>(left: L, right: R) -> LeftIgnore<L, R>
where
    L: Processor<I, Output = LO>,
    R: Processor<I, Output = RO>,
{
    LeftIgnore(left, right)
}

pub fn right_ignore<L, R, I, LO, RO>(left: L, right: R) -> RightIgnore<L, R>
where
    L: Processor<I, Output = LO>,
    R: Processor<I, Output = RO>,
{
    left_ignore(right, left)
}


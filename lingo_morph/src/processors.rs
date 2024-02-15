use std::{vec, array};

pub type Processed<O, R> = (O, R);
pub type RightIgnore<L, R> = LeftIgnore<R, L>;

pub trait Processor<I> {
    type Output;
    fn process(&mut self, given: I) -> Processed<Self::Output, I>;
    fn map<F, R>(self, map: F) -> Map<Self, F>
    where
        Self: Sized,
        F: FnMut(Self::Output) -> R,
    {
        Map {
            processor: self,
            map,
        }
    }
    fn connect<F, P, PI, PO>(self, binder: F) -> P
    where
        Self: Sized,
        F: FnOnce(Self) -> P,
        P: Processor<PI, Output = PO>,
    {
        binder(self)
    }
    fn left_zip<P, PO>(self, other: P) -> Zip<Self, P>
    where
        Self: Sized,
        P: Processor<I, Output = PO>,
    {
        Zip(self, other)
    }
    fn right_zip<P, PO>(self, other: P) -> Zip<P, Self>
    where
        Self: Sized,
        P: Processor<I, Output = PO>,
    {
        other.left_zip(self)
    }
    fn left_ignore<P, PO>(self, other: P) -> LeftIgnore<Self, P>
    where
        Self: Sized,
        P: Processor<I, Output = PO>,
    {
        LeftIgnore(self, other)
    }
    fn right_ignore<P, PO>(self, other: P) -> RightIgnore<Self, P>
    where
        Self: Sized,
        P: Processor<I, Output = PO>,
    {
        other.left_ignore(self)
    }
    fn left_or<P, O>(self, other: P) -> Or<Self, P>
    where
        Self: Sized + Processor<I, Output = Option<O>>,
        P: Processor<I, Output = O>,
    {
        Or(self, other)
    }
    fn right_or<P, O>(self, other: P) -> Or<P, Self>
    where
        Self: Sized + Processor<I, Output = O>,
        P: Processor<I, Output = Option<O>>,
    {
        other.left_or(self)
    }
    fn start_chain(self) -> Chain<Self>
    where
        Self: Sized,
    {
        Chain(vec![self])
    }
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
    fn process(&mut self, given: I) -> Processed<Self::Output, I> {
        let (processed, rest) = self.processor.process(given);
        let mapped = (self.map)(processed);
        (mapped, rest)
    }
}

macro_rules! impl_fold {
    (self.$item:tt, $processor:ty => $type:ty) => {
        pub fn fold<F, A, S, I, O>(self, state: A, fold: F) -> Fold<F, A, $type>
        where
            F: FnMut(S, O) -> Option<S>,
            A: Fn() -> S,
            $processor: Processor<I, Output = O>,
        {
            Fold {
                fold,
                state,
                processors: self.$item.into_iter(),
            }
        }
    };
}

pub struct Chain<P>(Vec<P>);

impl<P> Chain<P> {
    pub fn chain(mut self, next: P) -> Self {
        self.push(next);
        self
    }
    pub fn push(&mut self, next: P) {
        self.0.push(next);
    }
    impl_fold!(self.0, P => vec::IntoIter<P>);
}

pub struct Buff<P, const N: usize>([P; N]);

impl<P, const N: usize> Buff<P, N> {
    impl_fold!(self.0, P => array::IntoIter<P, N>);
}

pub struct Fold<F, A, E> {
    fold: F,
    state: A,
    processors: E,
}

impl<F, A, P, S, E, I, O> Processor<I> for Fold<F, A, E>
where
    F: FnMut(S, O) -> Option<S>,
    A: Fn() -> S,
    P: Processor<I, Output = O>,
    E: Iterator<Item = P>,
{
    type Output = Option<S>;
    fn process(&mut self, given: I) -> Processed<Self::Output, I> {
        let mut processor = match self.processors.next() {
            Some(first) => first,
            None => return (None, given),
        };
        let mut state = Some((self.state)());
        let mut rest = given;
        loop {
            let current_state = match state {
                Some(inner) => inner,
                None => break,
            };
            let (output, new_rest) = processor.process(rest);
            rest = new_rest;
            state = (self.fold)(current_state, output);
            match self.processors.next() {
                Some(next) => processor = next,
                None => break,
            }
        }
        (state, rest)
    }
}

pub struct Zip<A, B>(A, B);

impl<A, B, I, AO, BO> Processor<I> for Zip<A, B>
where
    A: Processor<I, Output = AO>,
    B: Processor<I, Output = BO>,
{
    type Output = (AO, BO);
    fn process(&mut self, given: I) -> Processed<Self::Output, I> {
        let (first, rest) = self.0.process(given);
        let (second, rest) = self.1.process(rest);
        ((first, second), rest)
    }
}

pub struct LeftIgnore<L, R>(L, R);

impl<L, R, I, LO, RO> Processor<I> for LeftIgnore<L, R>
where
    L: Processor<I, Output = LO>,
    R: Processor<I, Output = RO>,
{
    type Output = RO;
    fn process(&mut self, given: I) -> Processed<Self::Output, I> {
        let (_, rest) = self.0.process(given);
        self.1.process(rest)
    }
}

pub struct Or<A, B>(A, B);

impl<A, B, I, O> Processor<I> for Or<A, B>
where
    A: Processor<I, Output = Option<O>>,
    B: Processor<I, Output = O>,
{
    type Output = O;
    fn process(&mut self, given: I) -> Processed<Self::Output, I> {
        match self.0.process(given) {
            (Some(value), rest) => (value, rest),
            (None, rest) => self.1.process(rest),
        }
    }
}

#[macro_export]
macro_rules! chain {
    ($($processors:tt),*$(,)?) => {
        $crate::processors::chain(vec![$($processors),*])
    };
}

pub fn chain<P, I, O>(processors: Vec<P>) -> Chain<P>
where
    P: Processor<I, Output = O>,
{
    Chain(processors)
}

#[macro_export]
macro_rules! buff {
    ($($processor: expr),+$(,)?) => {
        $crate::processors::buff([$($processor),+])
    };
}

pub fn buff<P, I, O, const N: usize>(processors: [P; N]) -> Buff<P, N>
where
    P: Processor<I, Output = O>,
{
    Buff(processors)
}

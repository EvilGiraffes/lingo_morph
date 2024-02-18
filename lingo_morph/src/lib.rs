pub use collections::{buff, chain};
pub use end::FinalProcessor;

use collections::Chain;
use end::End;

pub mod collections;
pub mod processors;
pub mod end;
// This mimics the log crate to avoid checking for the feature available
#[macro_use]
mod log;

pub enum PResult<O, R, E> {
    Done(O, R),
    Incomplete(R),
    Internal(E, R),
}

// TODO incomperate PResult to processed
pub type Processed<O, R> = (O, R);
pub type RightIgnore<L, R> = LeftIgnore<R, L>;

#[macro_export]
macro_rules! is {
    (Done($expr:expr)) => {
        match $expr {
            PResult::Done(output, rest) => (output, rest),
            PResult::Incomplete(rest) => return $crate::PResult::Incomplete(rest),
            PResult::Internal(error, rest) => return $crate::PResult::Internal(error.into(), rest),
        }
    };
    (Some($expr:expr) ? -> $ident:ident) => {
        match $expr {
            Some(value) => value,
            None => return (None, $ident),
        }
    };
    (Some($expr:expr) ? break) => {
        match $expr {
            Some(value) => value,
            None => break,
        }
    };
    (Ok($expr:expr) ? -> $ident:ident) => {
        match $expr {
            Ok(value) => value,
            Err(error) => return (Err(error.into()), $ident),
        }
    };
}

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
        Chain::new(vec![self])
    }
    fn end(self) -> End<Self>
    where
        Self: Sized,
    {
        End::new(self)
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


pub use processed::{done, err, mismatch};

use context::With;
use processed::{Processed, Status};
use source::Source;

pub mod collections;
pub mod context;
pub mod processed;
pub mod processors;
pub mod source;
pub mod location;

mod buf;

// This mimics the log crate to avoid checking for the feature available
#[macro_use]
mod log;

pub trait Processor<I> {
    type Output;
    fn process<S>(&mut self, given: S) -> Processed<Self::Output, S>
    where
        S: Source<Item = I>;

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

    fn replace<T>(self, with: T) -> CopyReplace<Self, T>
    where
        Self: Sized,
        T: Copy,
    {
        CopyReplace(self, with)
    }

    fn take(self, amount: usize) -> Take<Self>
    where
        Self: Sized,
    {
        Take {
            processor: self,
            current: 0,
            target: amount,
        }
    }

    fn take_while<P>(self, predicate: P) -> TakeWhile<Self, P>
    where
        Self: Sized,
        P: FnMut(&I) -> bool,
    {
        TakeWhile(self, predicate)
    }

    fn fold<A, S, F>(self, accum: A, fold: F) -> Fold<Self, A, F>
    where
        Self: Sized,
        A: Fn() -> S,
        F: FnMut(S, Self::Output) -> S,
    {
        Fold {
            processor: self,
            accum,
            fold,
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

    fn zip<P>(self, other: P) -> Zip<Self, P>
    where
        Self: Sized,
        P: Processor<I>,
    {
        Zip(self, other)
    }

    fn ignore<P>(self, other: P) -> Ignore<Self, P>
    where
        Self: Sized,
        P: Processor<I>,
    {
        Ignore(self, other)
    }

    fn ignore_next<P>(self, other: P) -> IgnoreNext<Self, P>
    where
        Self: Sized,
        P: Processor<I>,
    {
        IgnoreNext(self, other)
    }

    fn or<P>(self, other: P) -> Or<Self, P>
    where
        Self: Sized + Processor<I>,
        P: Processor<I, Output = Self::Output>,
    {
        Or(self, other)
    }

    // TODO implement
    // fn start_chain(self) -> Chain<Self>
    // where
    //     Self: Sized,
    // {
    //     Chain::new(vec![self])
    // }

    fn with<S>(&mut self, input: S) -> With<'_, S, Self>
    where
        Self: Sized,
        S: Source<Item = I>,
    {
        With::new(input, self)
    }
}

impl<P, I> Processor<I> for &mut P
where
    P: Processor<I>,
{
    type Output = P::Output;

    fn process<S>(&mut self, given: S) -> Processed<Self::Output, S>
    where
        S: Source<Item = I>,
    {
        (**self).process(given)
    }
}

pub struct Map<P, F> {
    processor: P,
    map: F,
}

impl<P, I, F, R> Processor<I> for Map<P, F>
where
    P: Processor<I>,
    F: FnMut(P::Output) -> R,
{
    type Output = R;

    fn process<S>(&mut self, given: S) -> Processed<Self::Output, S>
    where
        S: Source<Item = I>,
    {
        let status = self.processor.process(given)?;
        Ok(status.map(|inner| (self.map)(inner)))
    }
}

pub struct CopyReplace<P, T>(P, T);

impl<P, I, T> Processor<I> for CopyReplace<P, T>
where
    P: Processor<I>,
    T: Copy,
{
    type Output = T;

    fn process<S>(&mut self, given: S) -> Processed<Self::Output, S>
    where
        S: Source<Item = I>,
    {
        Ok(self.0.process(given)?.map(|_| self.1))
    }
}

pub struct Take<P> {
    processor: P,
    current: usize,
    target: usize,
}

impl<P, I> Processor<I> for Take<P>
where
    P: Processor<I>,
{
    type Output = P::Output;

    fn process<S>(&mut self, given: S) -> Processed<Self::Output, S>
    where
        S: Source<Item = I>,
    {
        if self.current < self.target {
            self.current += 1;
            self.processor.process(given)
        } else {
            self.current = 0;
            mismatch(given)
        }
    }
}

pub struct TakeWhile<P, F>(P, F);

impl<P, I, F> Processor<I> for TakeWhile<P, F>
where
    P: Processor<I>,
    F: FnMut(&I) -> bool,
{
    type Output = P::Output;

    fn process<S>(&mut self, mut given: S) -> Processed<Self::Output, S>
    where
        S: Source<Item = I>,
    {
        let peeked = try_peek!(given);
        if (self.1)(peeked) {
            self.0.process(given)
        } else {
            mismatch(given)
        }
    }
}

pub struct Fold<P, A, F> {
    processor: P,
    accum: A,
    fold: F,
}

impl<P, A, F, ST, I> Processor<I> for Fold<P, A, F>
where
    P: Processor<I>,
    A: Fn() -> ST,
    F: FnMut(ST, P::Output) -> ST,
{
    type Output = ST;

    fn process<S>(&mut self, given: S) -> Processed<Self::Output, S>
    where
        S: Source<Item = I>,
    {
        let mut state = (self.accum)();
        let mut rest = given;
        loop {
            rest = match self.processor.process(rest)? {
                Status::Done(output, new_rest) => {
                    state = (self.fold)(state, output);
                    new_rest
                }
                Status::Mismatch(new_rest) => return done(state, new_rest),
            }
        }
    }
}

pub struct Zip<A, B>(A, B);

impl<A, B, I> Processor<I> for Zip<A, B>
where
    A: Processor<I>,
    B: Processor<I>,
{
    type Output = (A::Output, B::Output);

    fn process<S>(&mut self, given: S) -> Processed<Self::Output, S>
    where
        S: Source<Item = I>,
    {
        match try_done!(self.0.process(given)) {
            (first, rest) => {
                let second = self.1.process(rest)?;
                Ok(second.map(|inner| (first, inner)))
            }
        }
    }
}

pub struct Ignore<L, R>(L, R);

impl<L, R, I> Processor<I> for Ignore<L, R>
where
    L: Processor<I>,
    R: Processor<I>,
{
    type Output = R::Output;

    fn process<S>(&mut self, given: S) -> Processed<Self::Output, S>
    where
        S: Source<Item = I>,
    {
        let fallback = given.snapshot();
        match try_done!(self.0.process(given)) {
            (_, rest) => rollback_if_process_fail(fallback, &mut self.1, rest),
        }
    }
}

pub struct IgnoreNext<L, R>(L, R);

impl<L, R, I> Processor<I> for IgnoreNext<L, R>
where
    L: Processor<I>,
    R: Processor<I>,
{
    type Output = L::Output;

    fn process<S>(&mut self, given: S) -> Processed<Self::Output, S>
    where
        S: Source<Item = I>,
    {
        let fallback = given.snapshot();
        match try_done!(self.0.process(given)) {
            (output, rest) => match rollback_if_process_fail(fallback, &mut self.1, rest)? {
                Status::Done(_, rest) => done(output, rest),
                Status::Mismatch(rest) => mismatch(rest),
            },
        }
    }
}

pub struct Or<A, B>(A, B);

impl<A, B, I, O> Processor<I> for Or<A, B>
where
    A: Processor<I, Output = O>,
    B: Processor<I, Output = O>,
{
    type Output = O;

    fn process<S>(&mut self, given: S) -> Processed<Self::Output, S>
    where
        S: Source<Item = I>,
    {
        let status = self.0.process(given)?;
        match status {
            Status::Done(_, _) => Ok(status),
            Status::Mismatch(rest) => self.1.process(rest),
        }
    }
}

fn rollback_if_process_fail<P, I, S>(
    fallback: S::Snapshot,
    processor: &mut P,
    given: S,
) -> Processed<P::Output, S>
where
    P: Processor<I>,
    S: Source<Item = I>,
{
    match processor.process(given)? {
        Status::Done(output, rest) => done(output, rest),
        Status::Mismatch(mut rest) => match rest.roll_back(fallback) {
            Ok(_) => mismatch(rest),
            Err(error) => err(error),
        },
    }
}

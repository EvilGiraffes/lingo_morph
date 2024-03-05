use std::{
    error::Error,
    fmt::{Debug, Display},
};

pub use end::{FResult, FinalProcessor};

use end::End;
use source::Source;

pub mod collections;
pub mod end;
pub mod processors;
pub mod source;

// This mimics the log crate to avoid checking for the feature available
#[macro_use]
mod log;

pub type PResult<I, R> = Result<Status<I, R>, ProcessingError>;
pub type Processed<O, R> = PResult<O, R>;
pub type RightIgnore<L, R> = LeftIgnore<R, L>;

#[derive(Debug)]
pub struct ProcessingError {
    line_number: usize,
    column: usize,
    at_bytes: usize,
    error: Box<dyn Error>,
}

impl ProcessingError {
    pub fn line_number(&self) -> usize {
        self.line_number
    }
    pub fn column(&self) -> usize {
        self.column
    }
    pub fn at_bytes(&self) -> usize {
        self.at_bytes
    }
    pub fn error(&self) -> &dyn Error {
        self.error.as_ref()
    }
}

impl Display for ProcessingError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "processing failed at line {} and column {}, after parsing {} bytes: {}",
            self.line_number,
            self.column,
            self.at_bytes,
            self.error()
        )
    }
}

impl Error for ProcessingError {}

pub enum Status<O, R> {
    Done(O, R),
    Mismatch(R),
    EOF,
}

impl<O, R> Status<O, R> {
    pub fn map<F, U>(self, mapper: F) -> Status<U, R>
    where
        F: FnOnce(O) -> U,
    {
        match self {
            Self::Done(output, rest) => Status::Done(mapper(output), rest),
            Self::Mismatch(rest) => Status::Mismatch(rest),
            Self::EOF => Status::EOF,
        }
    }
}

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
    fn connect<F, P, PI, PO>(self, binder: F) -> P
    where
        Self: Sized,
        F: FnOnce(Self) -> P,
        P: Processor<PI, Output = PO>,
    {
        binder(self)
    }
    fn left_zip<P>(self, other: P) -> Zip<Self, P>
    where
        Self: Sized,
        P: Processor<I>,
    {
        Zip(self, other)
    }
    fn right_zip<P>(self, other: P) -> Zip<P, Self>
    where
        Self: Sized,
        P: Processor<I>,
    {
        other.left_zip(self)
    }
    fn left_ignore<P>(self, other: P) -> LeftIgnore<Self, P>
    where
        Self: Sized,
        P: Processor<I>,
    {
        LeftIgnore(self, other)
    }
    fn right_ignore<P>(self, other: P) -> RightIgnore<Self, P>
    where
        Self: Sized,
        P: Processor<I>,
    {
        other.left_ignore(self)
    }
    fn left_or<P>(self, other: P) -> Or<Self, P>
    where
        Self: Sized + Processor<I>,
        P: Processor<I, Output = Self::Output>,
    {
        Or(self, other)
    }
    fn right_or<P>(self, other: P) -> Or<P, Self>
    where
        Self: Sized + Processor<I>,
        P: Processor<I, Output = Self::Output>,
    {
        other.left_or(self)
    }
    // TODO implement
    // fn start_chain(self) -> Chain<Self>
    // where
    //     Self: Sized,
    // {
    //     Chain::new(vec![self])
    // }
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
    fn process<S>(&mut self, given: S) -> Processed<Self::Output, S>
    where
        S: Source<Item = I>,
    {
        let status = self.processor.process(given)?;
        Ok(status.map(|inner| (self.map)(inner)))
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
        let status = self.0.process(given)?;
        match status {
            Status::Done(first, rest) => {
                let second = self.1.process(rest)?;
                Ok(second.map(|inner| (first, inner)))
            }
            Status::Mismatch(rest) => Ok(Status::Mismatch(rest)),
            Status::EOF => Ok(Status::EOF),
        }
    }
}

pub struct LeftIgnore<L, R>(L, R);

impl<L, R, I> Processor<I> for LeftIgnore<L, R>
where
    L: Processor<I>,
    R: Processor<I>,
{
    type Output = R::Output;
    fn process<S>(&mut self, given: S) -> Processed<Self::Output, S>
    where
        S: Source<Item = I>,
    {
        match self.0.process(given)? {
            Status::Done(_, rest) => self.1.process(rest),
            Status::Mismatch(rest) => Ok(Status::Mismatch(rest)),
            Status::EOF => Ok(Status::EOF),
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
            Status::EOF => Ok(Status::EOF),
        }
    }
}

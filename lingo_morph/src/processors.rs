use std::ops::{Bound, RangeBounds};

use crate::{done, mismatch, source::Source, Processed, Processor};

pub type NoOp = Const<()>;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[repr(transparent)]
pub struct Const<T>(T);

impl<T> Processor<T> for Const<T>
where
    T: Clone,
{
    type Output = T;

    fn process<S>(&mut self, given: S) -> Processed<Self::Output, S>
    where
        S: Source<Item = T>,
    {
        done(self.0.clone(), given)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct ConstWith<F>(F);

impl<F, T> Processor<char> for ConstWith<F>
where
    F: Fn() -> T,
{
    type Output = T;

    fn process<S>(&mut self, given: S) -> Processed<Self::Output, S>
    where
        S: Source<Item = char>,
    {
        done((self.0)(), given)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[repr(transparent)]
pub struct Mut<T>(T);

impl<T> Mut<T> {
    pub fn get(&self) -> &T {
        &self.0
    }

    pub fn set(&mut self, new: T) {
        self.0 = new;
    }

    pub fn map<F, U>(self, map: F) -> Mut<U>
    where
        F: FnOnce(T) -> U,
        U: Clone,
    {
        Mut(map(self.0))
    }
}

impl<T> Processor<T> for Mut<T>
where
    T: Clone,
{
    type Output = T;

    fn process<S>(&mut self, given: S) -> Processed<Self::Output, S>
    where
        S: Source<Item = T>,
    {
        done(self.0.clone(), given)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Char(char);

impl Processor<char> for Char {
    type Output = char;

    fn process<S>(&mut self, mut given: S) -> Processed<Self::Output, S>
    where
        S: Source<Item = char>,
    {
        match given.next_if_eq(&self.0) {
            Some(next) => done(next, given),
            None => mismatch(given),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CharRange {
    start: Bound<char>,
    end: Bound<char>,
}

impl RangeBounds<char> for CharRange {
    fn start_bound(&self) -> Bound<&char> {
        self.start.as_ref()
    }

    fn end_bound(&self) -> Bound<&char> {
        self.end.as_ref()
    }
}

impl Processor<char> for CharRange {
    type Output = char;

    fn process<S>(&mut self, mut given: S) -> Processed<Self::Output, S>
    where
        S: Source<Item = char>,
    {
        match given.next_if(|item| self.contains(item)) {
            Some(next) => done(next, given),
            None => mismatch(given),
        }
    }
}

pub fn no_op() -> NoOp {
    constant(())
}

pub fn constant<T: Clone>(val: T) -> Const<T> {
    Const(val)
}

pub fn constant_with<F, T>(func: F) -> ConstWith<F>
where
    F: Fn() -> T,
{
    ConstWith(func)
}

pub fn mutable<T: Clone>(inital: T) -> Mut<T> {
    Mut(inital)
}

pub fn char(from: char) -> Char {
    Char(from)
}

pub fn char_range<R>(range: R) -> CharRange
where
    R: RangeBounds<char>,
{
    CharRange {
        start: range.start_bound().cloned(),
        end: range.end_bound().cloned(),
    }
}

pub fn digit(digit: u8) -> Option<impl Processor<char, Output = u8>> {
    match digit {
        0 => Some(Char('0').replace(0)),
        1 => Some(Char('1').replace(1)),
        2 => Some(Char('2').replace(2)),
        3 => Some(Char('3').replace(3)),
        4 => Some(Char('4').replace(4)),
        5 => Some(Char('5').replace(5)),
        6 => Some(Char('6').replace(6)),
        7 => Some(Char('7').replace(7)),
        8 => Some(Char('8').replace(8)),
        9 => Some(Char('9').replace(9)),
        _ => None,
    }
}

pub fn digit_range<R>(range: R) -> Option<impl Processor<char, Output = u8>>
where
    R: RangeBounds<u8>,
{
    const ZERO: u8 = '0' as u8;
    let start = digit_inclusive_or(range.start_bound(), 0);
    let end = digit_inclusive_or(range.end_bound(), 9);
    if start > end || start > 8 || end > 9 {
        None
    } else {
        Some(
            char_range((start + ZERO) as char..=(end + ZERO) as char)
                .map(|inner| (inner as u8) - ZERO),
        )
    }
}

fn digit_inclusive_or(bound: Bound<&u8>, default: u8) -> u8 {
    match bound {
        Bound::Included(x) => *x,
        Bound::Excluded(x) => *x - 1,
        Bound::Unbounded => default,
    }
}

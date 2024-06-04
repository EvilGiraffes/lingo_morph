pub use crate::location::Location;
use crate::{buf::RingBuf, location::Tracker};

use std::{error::Error, fmt::Display, iter::Peekable};

#[macro_export]
macro_rules! try_peek {
    ($source:expr) => {
        match $source.peek() {
            Some(val) => val,
            None => return $crate::processed::mismatch($source),
        }
    };
}

pub trait Source {
    type Item;
    type RollBackErr: Error + 'static;

    fn next(&mut self) -> Option<Self::Item>;

    fn roll_back(&mut self, to: Location) -> Result<(), Self::RollBackErr>;

    fn peek(&mut self) -> Option<&Self::Item>;

    fn peek_mut(&mut self) -> Option<&mut Self::Item>;

    fn location(&self) -> Location;

    fn next_if<P>(&mut self, predicate: P) -> Option<Self::Item>
    where
        P: FnOnce(&Self::Item) -> bool,
    {
        let peeked = self.peek()?;
        if predicate(peeked) {
            self.next()
        } else {
            None
        }
    }

    fn next_if_eq<T>(&mut self, other: &T) -> Option<Self::Item>
    where
        Self::Item: PartialEq<T>,
    {
        self.next_if(|next| next == other)
    }
}

#[derive(Debug)]
pub struct NotEnoughBuffered;

impl Display for NotEnoughBuffered {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "not enough buffered locations")
    }
}

#[derive(Debug, Clone, Copy)]
enum BufPos {
    Iter,
    Buf(usize),
}

impl Error for NotEnoughBuffered {}

pub struct IterSource<V, I: Iterator, T> {
    iter: Peekable<I>,
    tracker: T,
    buf: RingBuf<(Location, V)>,
    buf_pos: BufPos,
}

impl<V, I: Iterator, T> IterSource<V, I, T> {
    pub fn with_tracker_buf(iter: I, tracker: T, buf: RingBuf<(Location, V)>) -> Self {
        Self {
            iter: iter.peekable(),
            tracker,
            buf,
            buf_pos: BufPos::Iter,
        }
    }

    pub fn with_tracker_cap(iter: I, tracker: T, capacity: usize) -> Self {
        Self::with_tracker_buf(iter, tracker, RingBuf::new(capacity))
    }
}

impl<V, I, T> Source for IterSource<V, I, T>
where
    I: Iterator<Item = V>,
    T: Tracker<V>,
    V: Clone,
{
    type Item = V;
    type RollBackErr = NotEnoughBuffered;

    fn next(&mut self) -> Option<Self::Item> {
        match self.buf_pos {
            BufPos::Iter => {
                let next = self.iter.next();
                next.as_ref()
                    .map(|x| {
                        self.buf.push((self.tracker.location(), x.clone()));
                        x
                    })
                    .map(|x| self.tracker.update(x))
                    .transpose()
                    .expect("Could not update tracker");
                next
            }
            BufPos::Buf(idx) => {
                if idx + 1 >= self.buf.len() {
                    self.buf_pos = BufPos::Iter;
                } else {
                    self.buf_pos = BufPos::Buf(idx + 1)
                }
                self.buf.get(idx).map(|(_, x)| x.clone())
            }
        }
    }

    fn roll_back(&mut self, to: Location) -> Result<(), Self::RollBackErr> {
        let mut idx = self.buf.len();
        self.buf
            .iter()
            .map(|(x, _)| x.clone())
            .find(|x| {
                idx -= 1;
                *x == to
            })
            .ok_or(NotEnoughBuffered)?;
        self.buf_pos = BufPos::Buf(idx);
        Ok(())
    }

    // FIXME: This is not implemented properly, does not account for buffer
    fn peek(&mut self) -> Option<&Self::Item> {
        self.iter.peek()
    }

    // FIXME: This is not implemented properly, does not account for buffer
    fn peek_mut(&mut self) -> Option<&mut Self::Item> {
        self.iter.peek_mut()
    }

    // FIXME: This is not implemented properly, does not account for buffer
    fn location(&self) -> Location {
        self.tracker.location()
    }
}

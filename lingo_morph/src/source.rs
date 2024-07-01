use std::{convert::Infallible, error::Error};

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
    type Snapshot;
    type RollBackErr: Error + 'static;

    fn next(&mut self) -> Option<Self::Item>;

    fn roll_back(&mut self, to: Self::Snapshot) -> Result<(), Self::RollBackErr>;

    fn peek(&mut self) -> Option<&Self::Item>;

    fn peek_mut(&mut self) -> Option<&mut Self::Item>;

    fn snapshot(&self) -> Self::Snapshot;

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

pub struct BoxedSlice<T> {
    data: Box<[T]>,
    idx: usize,
}

impl<D, T> From<D> for BoxedSlice<T>
where
    D: Into<Box<[T]>>,
{
    fn from(value: D) -> Self {
        Self {
            data: value.into(),
            idx: 0,
        }
    }
}

impl<T> Source for BoxedSlice<T>
where
    T: Clone,
{
    type Item = T;
    type Snapshot = usize;
    type RollBackErr = Infallible;

    fn next(&mut self) -> Option<Self::Item> {
        self.data
            .get(self.idx)
            .map(|x| {
                self.idx += 1;
                x
            })
            .cloned()
    }

    fn roll_back(&mut self, to: Self::Snapshot) -> Result<(), Self::RollBackErr> {
        self.idx = to;
        Ok(())
    }

    fn peek(&mut self) -> Option<&Self::Item> {
        self.data.get(self.idx)
    }

    fn peek_mut(&mut self) -> Option<&mut Self::Item> {
        self.data.get_mut(self.idx)
    }

    fn snapshot(&self) -> Self::Snapshot {
        self.idx
    }
}

use std::{convert::Infallible, error::Error};

pub trait Source: Sized {
    type Item;
    type Snapshot;
    type RollBackErr: Error + 'static;

    fn next(&mut self) -> Option<Self::Item>;

    fn snapshot(&self) -> Self::Snapshot;

    fn roll_back(&mut self, to: Self::Snapshot) -> Result<(), Self::RollBackErr>;

    fn peek(&mut self) -> Option<&Self::Item>;

    fn peek_mut(&mut self) -> Option<&mut Self::Item>;

    #[inline]
    fn iter(&mut self) -> Iter<'_, Self> {
        Iter(self)
    }

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

    #[inline]
    fn next_if_eq<T>(&mut self, other: &T) -> Option<Self::Item>
    where
        Self::Item: PartialEq<T>,
    {
        self.next_if(|next| next == other)
    }
}

pub struct Iter<'a, S>(&'a mut S);

impl<'a, S> Iterator for Iter<'a, S>
where 
    S: Source,
{
    type Item = S::Item;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.0.next()
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

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.data
            .get(self.idx)
            .map(|x| {
                self.idx += 1;
                x
            })
            .cloned()
    }

    #[inline]
    fn snapshot(&self) -> Self::Snapshot {
        self.idx
    }

    #[inline]
    fn roll_back(&mut self, to: Self::Snapshot) -> Result<(), Self::RollBackErr> {
        self.idx = to;
        Ok(())
    }

    #[inline]
    fn peek(&mut self) -> Option<&Self::Item> {
        self.data.get(self.idx)
    }

    #[inline]
    fn peek_mut(&mut self) -> Option<&mut Self::Item> {
        self.data.get_mut(self.idx)
    }
}

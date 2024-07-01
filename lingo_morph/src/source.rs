use std::error::Error;

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


pub use crate::location::Location;

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
    type RollBackErr: Error + 'static;

    // TODO: Change into Result
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

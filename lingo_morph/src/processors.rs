use std::error::Error;

use crate::{source::Source, Processed, Processor, Status};

pub struct Char(char);

impl Processor<char> for Char {
    type Output = char;
    fn process<S>(&mut self, mut given: S) -> Processed<Self::Output, S>
    where
        S: Source<Item = char>,
        S::RollBackErr: Error,
    {
        match given.next_if_eq(&self.0) {
            Some(next) => Ok(Status::Done(next, given)),
            None => Ok(Status::Mismatch(given)),
        }
    }
}


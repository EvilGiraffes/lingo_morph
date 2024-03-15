use std::error::Error;

use crate::{source::Source, Processed, Processor, Status};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Char(char);

impl Processor<char> for Char {
    type Output = char;
    fn process<S>(&mut self, mut given: S) -> Processed<Self::Output, S>
    where
        S: Source<Item = char>,
        S::RollBackErr: Error + 'static,
    {
        match given.next_if_eq(&self.0) {
            Some(next) => Ok(Status::Done(next, given)),
            None => Ok(Status::Mismatch(given)),
        }
    }
}

pub fn char(from: char) -> Char {
    Char(from)
}

pub fn digit(digit: u8) -> Option<impl Processor<char>> {
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


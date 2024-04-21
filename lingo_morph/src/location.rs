use std::{error::Error, fmt::Display, num::NonZeroUsize};

use crate::buf::RingBuf;

pub trait Tracker {
    type DecrementErr: Error + 'static;

    fn location(&self) -> Location;

    fn inc_from_slice(&mut self, from: &[u8]);

    fn dec_to(&mut self, to: Location) -> Result<(), Self::DecrementErr>;

    fn inc_from_u32(&mut self, from: u32) {
        self.inc_from_slice(&from.to_le_bytes())
    }

    fn inc_from_char(&mut self, from: char) {
        self.inc_from_u32(from as u32)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct OutOfBufferedRange;

impl Display for OutOfBufferedRange {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "out of buffered range")
    }
}

impl Error for OutOfBufferedRange {}

#[derive(Debug, Clone)]
pub struct Utf8 {
    passed: RingBuf<Location>,
    current: BufferLocation,
}

impl Tracker for Utf8 {
    type DecrementErr = OutOfBufferedRange;

    fn location(&self) -> Location {
        self.current
            .get(&self.passed)
            .map(|x| *x)
            .unwrap_or_else(|| Location::default())
    }

    // TODO: Implement these

    fn inc_from_slice(&mut self, from: &[u8]) {
        unimplemented!("Utf8 inc_from_slice is not implemented yet")
    }

    fn inc_from_char(&mut self, from: char) {
        // This will be implemented as it is a simpler implementation due to the way utf8 works in
        // rust already
        unimplemented!("Utf8 inc_from_char is not implemented yet")
    }

    fn dec_to(&mut self, to: Location) -> Result<(), Self::DecrementErr> {
        unimplemented!("Utf8 dec_to is not implemented yet")
    }
}

#[derive(Debug, Default, Clone, Copy)]
pub struct Location {
    line: usize,
    column: usize,
    at_char: usize,
    at_bytes: usize,
}

impl Location {
    pub fn line(&self) -> usize {
        self.line
    }

    pub unsafe fn set_line(&mut self, line: usize) -> &mut Self {
        self.line = line;
        self
    }

    pub fn column(&self) -> usize {
        self.column
    }

    pub unsafe fn set_column(&mut self, column: usize) -> &mut Self {
        self.column = column;
        self
    }

    pub fn at_char(&self) -> usize {
        self.at_char
    }

    pub unsafe fn set_at_char(&mut self, at_char: usize) -> &mut Self {
        self.at_char = at_char;
        self
    }

    pub fn at_bytes(&self) -> usize {
        self.at_bytes
    }

    pub unsafe fn set_at_bytes(&mut self, at_bytes: usize) -> &mut Self {
        self.at_bytes = at_bytes;
        self
    }
}

#[derive(Debug, Clone, Copy)]
enum BufferLocation {
    Head,
    At(NonZeroUsize),
    Last,
}

impl BufferLocation {
    fn get<'a, T>(&'a self, from: &'a RingBuf<T>) -> Option<&T> {
        match self {
            BufferLocation::Head => from.head(),
            BufferLocation::At(idx) => from.get(idx.get()),
            BufferLocation::Last => from.tail(),
        }
    }
}

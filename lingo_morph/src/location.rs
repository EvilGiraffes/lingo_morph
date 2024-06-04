use std::{convert::Infallible, error::Error};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
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

    pub unsafe fn update_line<F>(&mut self, func: F) -> &mut Self
    where
        F: FnOnce(usize) -> usize,
    {
        self.line = func(self.line);
        self
    }

    pub fn column(&self) -> usize {
        self.column
    }

    pub unsafe fn set_column(&mut self, column: usize) -> &mut Self {
        self.column = column;
        self
    }

    pub unsafe fn update_column<F>(&mut self, func: F) -> &mut Self
    where
        F: FnOnce(usize) -> usize,
    {
        self.column = func(self.line);
        self
    }

    pub fn at_char(&self) -> usize {
        self.at_char
    }

    pub unsafe fn set_at_char(&mut self, at_char: usize) -> &mut Self {
        self.at_char = at_char;
        self
    }

    pub unsafe fn update_at_char<F>(&mut self, func: F) -> &mut Self
    where
        F: FnOnce(usize) -> usize,
    {
        self.at_char = func(self.line);
        self
    }

    pub fn at_bytes(&self) -> usize {
        self.at_bytes
    }

    pub unsafe fn set_at_bytes(&mut self, at_bytes: usize) -> &mut Self {
        self.at_bytes = at_bytes;
        self
    }

    pub unsafe fn update_at_bytes<F>(&mut self, func: F) -> &mut Self
    where
        F: FnOnce(usize) -> usize,
    {
        self.at_bytes = func(self.line);
        self
    }
}

pub trait Tracker<I> {
    type Error: Error + 'static;

    fn update(&mut self, from: &I) -> Result<(), Self::Error>;

    fn location(&self) -> Location;
}

#[derive(Debug, Clone, Copy)]
pub struct CharTracker(Location);

impl CharTracker {
    pub fn new() -> Self {
        Self(Location::default())
    }
}

impl Tracker<char> for CharTracker {
    type Error = Infallible;

    fn update(&mut self, from: &char) -> Result<(), Self::Error> {
        match from {
            '\n' => unsafe { self.0.update_line(|x| x + 1).set_column(0) },
            _ => unsafe { self.0.update_column(|x| x + 1) },
        };
        unsafe {
            self.0
                .update_at_char(|x| x + 1)
                .update_at_bytes(|x| x + from.len_utf8());
        }
        Ok(())
    }

    fn location(&self) -> Location {
        self.0
    }
}

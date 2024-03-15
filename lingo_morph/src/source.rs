use std::error::Error;

pub enum NewLine {
    Unix,
    Windows,
}

impl NewLine {
    const fn len_utf8(&self) -> usize {
        const UNIX: usize = '\n'.len_utf8();
        match self {
            Self::Unix => UNIX,
            Self::Windows => UNIX + '\r'.len_utf8(),
        }
    }
}

pub enum Character {
    Char(char),
    NewLine(NewLine)
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
    pub fn column(&self) -> usize {
        self.column
    }
    pub fn at_char(&self) -> usize {
        self.at_char
    }
    pub fn at_bytes(&self) -> usize {
        self.at_bytes
    }
    pub fn increment(&mut self, character: Character) {
        self.at_char += 1;
        match character {
            Character::Char(character) => {
                self.column += 1;
                self.at_bytes += character.len_utf8();
            }
            Character::NewLine(new_line) => {
                self.column = 0;
                self.line += 1;
                self.at_bytes += new_line.len_utf8();
            }
        }
    }
}

pub trait Source: Iterator {
    type RollBackErr: Error + 'static;
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
        <Self as Iterator>::Item: PartialEq<T>,
    {
        self.next_if(|next| next == other)
    }
}


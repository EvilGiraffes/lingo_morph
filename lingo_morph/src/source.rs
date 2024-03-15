#[derive(Debug, Default, Copy, Clone)]
pub struct Location {
    line: usize,
    column: usize,
    at_bytes: usize,
}

impl Location {
    pub fn line(&self) -> usize {
        self.line
    }
    pub fn line_mut(&mut self) -> &mut usize {
        &mut self.line
    }
    pub fn column(&self) -> usize {
        self.column
    }
    pub fn column_mut(&mut self) -> &mut usize {
        &mut self.column
    }
    pub fn at_bytes(&self) -> usize {
        self.at_bytes
    }
    pub fn at_bytes_mut(&mut self) -> &mut usize {
        &mut self.at_bytes
    }
}

pub trait Source: Iterator {
    fn roll_back(&mut self, by: usize) -> Result<(), Self::RollBackErr>;
    type RollBackErr: Error + 'static;
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

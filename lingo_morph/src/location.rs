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


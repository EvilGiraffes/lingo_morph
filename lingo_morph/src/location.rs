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


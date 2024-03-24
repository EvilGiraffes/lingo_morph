use std::convert::Infallible;

use lingo_morph::{
    processors::digit_range,
    source::{Location, Source},
    Processor,
};

#[derive(Debug)]
struct TempSource {
    items: Vec<char>,
    idx: Option<usize>,
}

impl TempSource {
    fn index(&self) -> usize {
        self.idx.unwrap_or(0)
    }

    fn peek_index(&self) -> usize {
        match self.idx {
            Some(index) => index + 1,
            None => 0,
        }
    }

    fn increment(&mut self) {
        match self.idx {
            Some(index) => {
                self.idx = Some(index + 1);
            }
            None => {
                self.idx = Some(0);
            }
        }
    }

    fn decrement(&mut self) {
        match self.idx {
            Some(index) => {
                self.idx = Some(index - 1);
            }
            None => (),
        }
    }
}

impl Iterator for TempSource {
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        let index = self.index();
        self.increment();
        self.items.get(index).copied()
    }
}

impl Source for TempSource {
    type RollBackErr = Infallible;

    fn roll_back(&mut self, _: Location) -> Result<(), Self::RollBackErr> {
        // This wont be implemented for this tempoary test source
        // so i will assume a roll back is by one
        self.decrement();
        Ok(())
    }

    fn peek(&mut self) -> Option<&Self::Item> {
        self.items.get(self.peek_index())
    }

    fn peek_mut(&mut self) -> Option<&mut Self::Item> {
        let idx = self.peek_index();
        self.items.get_mut(idx)
    }

    fn location(&self) -> Location {
        // Also not implemented due to being a test
        Location::default()
    }
}

fn main() {
    let parse_string: Vec<_> = "0123456789".chars().collect();
    let given = TempSource {
        items: parse_string,
        idx: None,
    };
    let parser = digit_range(..).unwrap();
    let result = parser.with(given).fold(0_u32, |state, x| {
        if state == 0 {
            x as u32
        } else {
            state * 10 + (x as u32)
        }
    });
    println!("{result:?}")
}

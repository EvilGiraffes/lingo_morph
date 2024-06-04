use std::{convert::Infallible, fmt::Debug};

use lingo_morph::{
    processors::{any, character, constant_with, digit_range},
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

impl Source for TempSource {
    type Item = char;
    type RollBackErr = Infallible;

    fn next(&mut self) -> Option<Self::Item> {
        self.increment();
        let index = self.index();
        self.items.get(index).copied()
    }

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

#[allow(unused)]
#[derive(Debug)]
struct ParseThis {
    some_string: String,
    some_u32: u32,
}

#[derive(Debug, Default, Clone)]
struct ParseThisBuilder {
    some_string: Option<String>,
    some_u32: Option<u32>,
}

impl ParseThisBuilder {
    fn some_string(&mut self, str: String) -> &mut Self {
        self.some_string = Some(str);
        self
    }

    fn some_u32(&mut self, x: u32) -> &mut Self {
        self.some_u32 = Some(x);
        self
    }

    fn build(self) -> Option<ParseThis> {
        Some(ParseThis {
            some_string: self.some_string?,
            some_u32: self.some_u32?,
        })
    }
}

fn create_parse_this() -> impl Processor<char, Output = ParseThis> {
    let str_parser = any::<char>().take(11).fold(|| String::new(), |mut str, x| {
        str.push(x);
        str
    });
    let u32_parser = digit_range(..).unwrap().map(|x| x as u32).fold(|| 0, |current, x| {
        if current == 0 {
            x
        } else {
            current * 10 + x
        }
    });
    constant_with(|| ParseThisBuilder::default())
            .zip(str_parser)
            .map(|(mut builder, str)| {
                builder.some_string(str);
                builder
            })
            .ignore_next(character(' '))
            .zip(u32_parser)
            .map(|(mut builder, x)| {
                builder.some_u32(x);
                builder
            })
            .map(|builder| builder.build().unwrap())
}

fn main() {
    let src_code = "hello_world 50123".chars().collect();
    let source = TempSource {
        items: src_code,
        idx: None,
    };
    match create_parse_this().with(source).process() {
            Ok(value) => println!("{value:#?}"),
            Err(error) => println!("{error:#?}"),
        }
}

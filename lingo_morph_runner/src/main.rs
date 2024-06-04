use std::fmt::Debug;

use lingo_morph::{
    location::CharTracker,
    processed::Processed,
    processors::{any, character, constant_with, digit_range},
    source::{IterSource, Source},
    Processor,
};

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

struct ConsumeProcessor<P>(P, usize);

impl<P> Processor<char> for ConsumeProcessor<P>
where
    P: Processor<char>,
{
    type Output = P::Output;

    fn process<S>(&mut self, mut given: S) -> Processed<Self::Output, S>
    where
        S: Source<Item = char>,
    {
        let location = given.location();
        for _ in 0..self.1 {
            given.next();
        }
        given.roll_back(location).expect("Could not roll back");
        self.0.process(given)
    }
}

fn create_parse_this() -> impl Processor<char, Output = ParseThis> {
    let str_parser = any::<char>().take(11).fold(
        || String::new(),
        |mut str, x| {
            str.push(x);
            str
        },
    );
    let u32_parser = digit_range(..).unwrap().map(|x| x as u32).fold(
        || 0,
        |current, x| {
            if current == 0 {
                x
            } else {
                current * 10 + x
            }
        },
    );
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
    let src_code = "hello_world 50123".chars();
    let source = IterSource::with_tracker_cap(src_code, CharTracker::new(), 32);
    let mut processor = ConsumeProcessor(create_parse_this(), 5);
    match processor.with(source).process() {
        Ok(value) => println!("{value:#?}"),
        Err(error) => println!("{error:#?}"),
    }
}

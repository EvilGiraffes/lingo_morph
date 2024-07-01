use std::error;

pub type Error = Box<dyn error::Error + 'static>;
pub type PResult<I, R> = Result<Status<I, R>, Error>;
pub type Processed<O, R> = PResult<O, R>;

#[derive(Debug)]
pub enum Status<O, R> {
    Done(O, R),
    Mismatch(R),
}

impl<O, R> Status<O, R> {
    pub fn map<F, U>(self, mapper: F) -> Status<U, R>
    where
        F: FnOnce(O) -> U,
    {
        match self {
            Self::Done(output, rest) => Status::Done(mapper(output), rest),
            Self::Mismatch(rest) => Status::Mismatch(rest),
        }
    }
}

#[inline]
pub fn done<O, R>(output: O, rest: R) -> Processed<O, R> {
    Ok(Status::Done(output, rest))
}

#[inline]
pub fn mismatch<O, R>(rest: R) -> Processed<O, R> {
    Ok(Status::Mismatch(rest))
}

#[inline]
pub fn err<O, R, E>(error: E) -> Processed<O, R>
where
    E: Into<Error>,
{
    Err(error.into())
}

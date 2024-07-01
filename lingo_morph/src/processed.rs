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

#[macro_export]
macro_rules! try_done {
    ($processed:expr) => {
        match $processed? {
            $crate::processed::Status::Done(output, rest) => (output, rest),
            $crate::processed::Status::Mismatch(rest) => return $crate::processed::mismatch(rest),
        }
    };
}

#[inline]
pub fn done<O, R>(output: O, rest: R) -> Processed<O, R> {
    Ok(Status::Done(output, rest))
}

#[inline]
pub fn mismatch<O, R>(rest: R) -> Processed<O, R> {
    Ok(Status::Mismatch(rest))
}

#[macro_export]
macro_rules! try_ok {
    ($processed:expr) => {
        match $processed {
            Ok(val) => val,
            Err(error) => return $crate::processed::err($source)
        }
    };
}

#[inline]
pub fn err<O, R, E>(error: E) -> Processed<O, R>
where
    E: Into<Error>,
{
    Err(error.into())
}

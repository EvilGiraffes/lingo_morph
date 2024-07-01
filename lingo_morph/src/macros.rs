#[macro_export]
macro_rules! try_done {
    ($processed:expr) => {
        match $processed? {
            $crate::processed::Status::Done(output, rest) => (output, rest),
            $crate::processed::Status::Mismatch(rest) => return $crate::processed::mismatch(rest),
        }
    };
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

#[macro_export]
macro_rules! try_peek {
    ($source:expr) => {
        match $source.peek() {
            Some(val) => val,
            None => return $crate::processed::mismatch($source),
        }
    };
}

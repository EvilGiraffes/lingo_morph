pub enum Either<L, R> {
    Left(L),
    Right(R),
}

impl<L, R> Either<L, R> {
    pub fn lunwrap_or_map<F>(self, map: F) -> L
    where
        F: FnOnce(R) -> L,
    {
        match self {
            Either::Left(left) => left,
            Either::Right(right) => map(right),
        }
    }
}

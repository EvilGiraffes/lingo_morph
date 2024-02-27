pub trait Source: Iterator {
    type RollBackErr;
    fn roll_back(&mut self, by: usize) -> Result<(), Self::RollBackErr>;
    fn peek(&mut self) -> Option<Self::Item>;
    fn next_if<P>(&mut self, predicate: P) -> Option<Self::Item>
    where
        P: FnOnce(&Self::Item) -> bool;
}

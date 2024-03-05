pub trait Source: Iterator {
    type RollBackErr;
    fn roll_back(&mut self, by: usize) -> Result<(), Self::RollBackErr>;
    fn peek(&mut self) -> Option<&Self::Item>;
    fn peek_mut(&mut self) -> Option<&mut Self::Item>;
    fn next_if<P>(&mut self, predicate: P) -> Option<Self::Item>
    where
        P: FnOnce(&Self::Item) -> bool,
    {
        let peeked = self.peek()?;
        if predicate(peeked) {
            self.next()
        } else {
            None
        }
    }
    fn next_if_eq<T>(&mut self, other: &T) -> Option<Self::Item>
    where
        <Self as Iterator>::Item: PartialEq<T>,
    {
        self.next_if(|next| next == other)
    }
}

pub type Processed<O, R> = (O, R);
pub type Compose<A, B> = Pipe<B, A>;
pub type RightIgnore<L, R> = LeftIgnore<R, L>;

pub trait Processor<I>: Sized {
    type Output;
    fn process(&mut self, given: I) -> Processed<Self::Output, I>;
    fn map<F, R>(self, map: F) -> Map<Self, F>
    where
        F: FnMut(Self::Output) -> R,
    {
        Map {
            processor: self,
            map,
        }
    }
    fn pipe<P, PO>(self, other: P) -> Pipe<Self, P>
    where
        P: Processor<I, Output = PO>,
    {
        Pipe(self, other)
    }
    fn compose<P, PO>(self, other: P) -> Compose<Self, P>
    where
        P: Processor<I, Output = PO>,
    {
        other.pipe(self)
    }
    fn left_ignore<P, PO>(self, other: P) -> LeftIgnore<Self, P>
    where
        P: Processor<I, Output = PO>,
    {
        LeftIgnore(self, other)
    }
    fn right_ignore<P, PO>(self, other: P) -> RightIgnore<Self, P>
    where
        P: Processor<I, Output = PO>,
    {
        other.left_ignore(self)
    }
    fn left_or<P, O>(self, other: P) -> Or<Self, P>
    where
        Self: Processor<I, Output = Option<O>>,
        P: Processor<I, Output = O>,
    {
        Or(self, other)
    }
    fn right_or<P, O>(self, other: P) -> Or<P, Self>
    where
        Self: Processor<I, Output = O>,
        P: Processor<I, Output = Option<O>>,
    {
        other.left_or(self)
    }
    fn fold<F, S, T>(self, state: F) -> Fold<Self, F>
    where
        Self: Processor<(S, T)>,
        F: Fn() -> S,
    {
        Fold::new(self, state)
    }
}

pub struct Map<P, F> {
    processor: P,
    map: F,
}

impl<P, I, F, O, R> Processor<I> for Map<P, F>
where
    P: Processor<I, Output = O>,
    F: FnMut(O) -> R,
{
    type Output = R;
    fn process(&mut self, given: I) -> Processed<Self::Output, I> {
        let (processed, rest) = self.processor.process(given);
        let mapped = (self.map)(processed);
        (mapped, rest)
    }
}

pub struct Fold<P, F> {
    processors: Vec<P>,
    get_state: F,
}

impl<P, F> Fold<P, F> {
    fn new(inital: P, get_state: F) -> Self {
        Self {
            processors: vec![inital],
            get_state,
        }
    }
    pub fn push(mut self, processor: P) -> Self {
        self.processors.push(processor);
        self
    }
}

impl<P, F, S, I, O> Processor<I> for Fold<P, F>
where
    P: Processor<(S, I), Output = O>,
    F: Fn() -> S,
{
    type Output = Vec<O>;
    fn process(&mut self, given: I) -> Processed<Self::Output, I> {
        let mut state = (self.get_state)();
        let mut result = Vec::new();
        let mut idx = 0;
        let mut rest = given;
        loop {
            let (processed, (new_state, new_rest)) = self.processors[idx].process((state, rest));
            state = new_state;
            rest = new_rest;
            result.push(processed);
            idx += 1;
            if idx >= self.processors.len() {
                break;
            }
        }
        (result, rest)
    }
}

pub struct BFold<P, F, const N: usize> {
    processors: [P;N],
    get_state: F,
}

impl<P, F, S, I, O, const N: usize> Processor<I> for BFold<P, F, N>
where 
    O: Copy + Default,
    P: Processor<(S, I), Output = O>,
    F: Fn() -> S,
{
    type Output = [O;N];
    fn process(&mut self, given: I) -> Processed<Self::Output, I> {
        // TODO remove the duplication from previous version
        let mut state = (self.get_state)();
        let mut result = [<O as Default>::default();N];
        let mut idx = 0;
        let mut rest = given;
        loop {
            let (processed, (new_state, new_rest)) = self.processors[idx].process((state, rest));
            state = new_state;
            rest = new_rest;
            result[idx] = processed;
            idx += 1;
            if idx >= self.processors.len() {
                break;
            }
        }
        (result, rest)
    }
}

pub struct Pipe<A, B>(A, B);

impl<A, B, I, AO, BO> Processor<I> for Pipe<A, B>
where
    A: Processor<I, Output = AO>,
    B: Processor<I, Output = BO>,
{
    type Output = (AO, BO);
    fn process(&mut self, given: I) -> Processed<Self::Output, I> {
        let (first, rest) = self.0.process(given);
        let (second, rest) = self.1.process(rest);
        ((first, second), rest)
    }
}

pub struct LeftIgnore<L, R>(L, R);

impl<L, R, I, LO, RO> Processor<I> for LeftIgnore<L, R>
where
    L: Processor<I, Output = LO>,
    R: Processor<I, Output = RO>,
{
    type Output = RO;
    fn process(&mut self, given: I) -> Processed<Self::Output, I> {
        let (_, rest) = self.0.process(given);
        self.1.process(rest)
    }
}

pub struct Or<A, B>(A, B);

impl<A, B, I, O> Processor<I> for Or<A, B>
where
    A: Processor<I, Output = Option<O>>,
    B: Processor<I, Output = O>,
{
    type Output = O;
    fn process(&mut self, given: I) -> Processed<Self::Output, I> {
        match self.0.process(given) {
            (Some(value), rest) => (value, rest),
            (None, rest) => self.1.process(rest),
        }
    }
}

#[macro_export]
macro_rules! bfold {
    ($state:tt; $($processor: expr),+$(,)?) => {
        $crate::processors::bfold([$($processor),+], || $state)
    };
}

pub fn bfold<P, F, S, I, O, const N: usize>(processors: [P;N], state: F) -> BFold<P, F, N> 
where
    O: Copy + Default,
    P: Processor<(S, I), Output = O>,
    F: Fn() -> S,
{
    BFold {
        processors,
        get_state: state,
    }
}

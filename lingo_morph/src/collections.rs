use super::{is, Processed, Processor};

macro_rules! do_fold {
    (impl for $ident:ident $(: $const_ident:ident: $type:ty)?) => {
        impl<F, A, P, S, I, O $(, const $const_ident: $type)?> Processor<I>
        for $ident<F, A, P $(, $const_ident)?>
        where
            F: FnMut(S, O) -> Option<S>,
            A: Fn() -> S,
            P: Processor<I, Output = O>,
        {
            type Output = Option<S>;
            fn process(&mut self, given: I) -> Processed<Self::Output, I> {
                let initial_state = (self.state)();
                let iter = self.processors.iter_mut();
                fold(initial_state, given, &mut self.fold, iter)
            }
        }
    };
    (self.$item:tt, $processor:ty => $ident:ident $(: $N:ty)?) => {
        pub fn fold<F, A, S, I, O>(self, state: A, fold: F)
            -> $ident<F, A, $processor $(, $N)?>
        where
            F: FnMut(S, O) -> Option<S>,
            A: Fn() -> S,
            $processor: Processor<I, Output = O>,
        {
            $ident {
                fold,
                state,
                processors: self.$item,
            }
        }
    }
}

pub struct Chain<P>(Vec<P>);

impl<P> Chain<P> {
    pub fn new(vector: Vec<P>) -> Self {
        Chain(vector)
    }
    pub fn chain(mut self, next: P) -> Self {
        self.push(next);
        self
    }
    pub fn push(&mut self, next: P) {
        self.0.push(next);
    }
    do_fold!(self.0, P => VecFold);
}

// fap
pub struct VecFold<F, A, P> {
    fold: F,
    state: A,
    processors: Vec<P>,
}

do_fold!(impl for VecFold);

pub struct Buff<P, const N: usize>([P; N]);

impl<P, const N: usize> Buff<P, N> {
    do_fold!(self.0, P => BuffFold: N);
}

pub struct BuffFold<F, A, P, const N: usize> {
    fold: F,
    state: A,
    processors: [P; N],
}

do_fold!(impl for BuffFold : N: usize);

#[macro_export]
macro_rules! chain {
    ($($processors:tt),*$(,)?) => {
        $crate::chain(vec![$($processors),*])
    };
}

pub fn chain<P, I, O>(processors: Vec<P>) -> Chain<P>
where
    P: Processor<I, Output = O>,
{
    Chain(processors)
}

#[macro_export]
macro_rules! buff {
    ($($processor: expr),+$(,)?) => {
        $crate::buff([$($processor),+])
    };
}

pub fn buff<P, I, O, const N: usize>(processors: [P; N]) -> Buff<P, N>
where
    P: Processor<I, Output = O>,
{
    Buff(processors)
}

fn fold<'a, F, S, P, I, O, E>(
    initial_state: S,
    given: I,
    func: &'a mut F,
    mut iter: E,
) -> (Option<S>, I)
where
    F: FnMut(S, O) -> Option<S>,
    P: Processor<I, Output = O> + 'a,
    E: Iterator<Item = &'a mut P>,
{
    let mut processor = is!(Some(iter.next())? -> given);
    let mut rest = given;
    let mut state = Some(initial_state);
    loop {
        let current_state = is!(Some(state)? break);
        let (output, new_rest) = processor.process(rest);
        rest = new_rest;
        state = func(current_state, output);
        processor = is!(Some(iter.next())? break);
    }
    (state, rest)
}

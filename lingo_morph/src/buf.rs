use std::collections::VecDeque;

#[derive(Debug, Clone)]
pub struct RingBuf<T> {
    buf: VecDeque<T>,
}

impl<T> RingBuf<T> {
    pub fn new(capacity: usize) -> Self {
        Self {
            buf: VecDeque::with_capacity(capacity),
        }
    }

    pub fn push(&mut self, val: T) -> Option<T> {
        if self.buf.len() < self.buf.capacity() {
            self.buf.push_back(val);
            None
        } else {
            let previous = self.buf.pop_front();
            self.buf.push_back(val);
            previous
        }
    }

    pub fn iter<'a>(&'a self) -> impl Iterator<Item = &T> + DoubleEndedIterator + 'a {
        self.buf.iter()
    }

    #[inline]
    pub fn get(&self, index: usize) -> Option<&T> {
        self.buf.get(index)
    }

    #[inline]
    pub fn get_mut(&mut self, index: usize) -> Option<&mut T> {
        self.buf.get_mut(index)
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.buf.len()
    }
}


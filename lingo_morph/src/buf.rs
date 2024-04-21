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

    pub fn get(&self, index: usize) -> Option<&T> {
        self.buf.get(index)
    }

    pub fn head(&self) -> Option<&T> {
        self.buf.back()
    }

    pub fn tail(&self) -> Option<&T> {
        self.buf.front()
    }
}

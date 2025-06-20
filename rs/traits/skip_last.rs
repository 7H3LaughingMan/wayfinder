use std::iter::Peekable;

pub struct SkipLastIterator<I: Iterator>(Peekable<I>);

impl<I: Iterator> Iterator for SkipLastIterator<I> {
    type Item = I::Item;

    fn next(&mut self) -> Option<Self::Item> {
        let item = self.0.next();
        match self.0.peek() {
            Some(_) => item,
            None => None,
        }
    }
}

pub trait SkipLast: Iterator + Sized {
    fn skip_last(self) -> SkipLastIterator<Self> {
        SkipLastIterator(self.peekable())
    }
}

impl<I: Iterator> SkipLast for I {}

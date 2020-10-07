use std::iter::FromIterator;
pub trait CollectOr<I, T>
where
    T: FromIterator<I>,
{
    /// Transorms an iterator into a collection.
    /// Same as [`collect`], but if iterator is empty returns default value istead of collecting.
    ///
    /// [`collect`]: https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.collect
    fn collect_or(self, default: T) -> T;
}

impl<B, T, I> CollectOr<T, B> for I
where
    I: Iterator<Item = T>,
    B: FromIterator<T>,
{
    fn collect_or(self, default: B) -> B {
        let mut peekable = self.peekable();
        if peekable.peek().is_some() {
            FromIterator::from_iter(peekable)
        } else {
            default
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_default() {
        let v = vec![1, 2, 3].into_iter().filter(|&x| x > 10);
        assert_eq!(v.collect_or(vec![10]), vec![10]);
    }
    #[test]
    fn test_collect() {
        let v = vec![20, 30, 40].into_iter().filter(|&x| x > 10);
        assert_eq!(v.collect_or(vec![1]), vec![20, 30, 40]);
    }
}

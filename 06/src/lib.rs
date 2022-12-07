struct WindowArrayIter<'a, T, const N: usize> {
    underlying: &'a [T],
    current: usize,
}

trait ArrayExt<'a, T> {
    fn new_window_iterator<const N: usize>(&'a self) -> WindowArrayIter<'a, T, N>;
}

impl<'a, T> ArrayExt<'a, T> for &'a [T] {
    fn new_window_iterator<const N: usize>(&'a self) -> WindowArrayIter<'a, T, N> {
        WindowArrayIter {
            underlying: self,
            current: 0,
        }
    }
}

impl<'a, T, const N: usize> Iterator for WindowArrayIter<'a, T, N> {
    type Item = &'a [T];

    fn next(&mut self) -> Option<Self::Item> {
        if self.current + N > self.underlying.len() {
            None
        } else {
            let slice = &self.underlying[self.current..self.current + N];
            self.current += 1;
            Some(slice)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sliding_window() {
        let values = [1, 2, 3, 4, 5, 6];
        let values: &[i32] = &values;
        let actual = values.new_window_iterator::<3>().collect::<Vec<_>>();
        let expected = vec![&[1, 2, 3], &[2, 3, 4], &[3, 4, 5], &[4, 5, 6]];
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_sliding_window_just_the_size() {
        let values = [1, 2, 3];
        let values: &[i32] = &values;
        let actual = values.new_window_iterator::<3>().collect::<Vec<_>>();
        let expected = vec![&[1, 2, 3]];
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_sliding_window_smaller() {
        let values = [1, 2];
        let values: &[i32] = &values;
        let mut actual = values.new_window_iterator::<3>();
        assert!(actual.next().is_none());
    }
}

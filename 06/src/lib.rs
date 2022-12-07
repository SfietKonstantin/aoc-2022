use std::collections::HashSet;
use std::io::{BufRead, BufReader, Error as IoError, Read};
use thiserror::Error;

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

#[derive(Debug, Error)]
pub enum Error {
    #[error("I/O error")]
    IO(#[from] IoError),
}

pub fn read_input<R>(read: R) -> Result<Vec<Vec<char>>, Error>
where
    R: Read,
{
    let lines = BufReader::new(read).lines();
    let lines = lines
        .map(|line| line.map_err(Error::from))
        .collect::<Result<Vec<_>, _>>()?;
    let signals = lines
        .iter()
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .map(|line| line.chars().collect())
        .collect::<Vec<_>>();
    Ok(signals)
}

pub fn find_first_marker<const N: usize>(signal: &[char]) -> Option<usize> {
    signal
        .new_window_iterator::<N>()
        .enumerate()
        .find(|(_, w)| is_marker(w))
        .map(|(i, _)| i + N)
}

fn is_marker(window: &[char]) -> bool {
    let window_set = window.iter().copied().collect::<HashSet<_>>();
    window_set.len() == window.len()
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

    const TEST_STR: &str = r#"mjqjpqmgbljsphdztnvjfqwrcgsmlb
bvwbjplbgvbhsrlpgdmjqwftvncz
nppdvjthqldpwncqszvftbrmjlhg
nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg
zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw
"#;

    #[test]
    fn test_initial() {
        let actual = read_input(TEST_STR.as_bytes())
            .unwrap()
            .into_iter()
            .map(|signal| find_first_marker::<4>(&signal))
            .collect::<Vec<_>>();
        let expected = vec![Some(7), Some(5), Some(6), Some(10), Some(11)];
        assert_eq!(actual, expected);
    }
}

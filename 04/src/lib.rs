use std::io::{BufRead, BufReader, Error as IoError, Read};
use std::ops::RangeInclusive;
use std::str::FromStr;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("I/O error")]
    IO(#[from] IoError),
    #[error("Invalid input {}", .0)]
    Parse(String),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Pair {
    left: RangeInclusive<i32>,
    right: RangeInclusive<i32>,
}

impl Pair {
    fn new(left: RangeInclusive<i32>, right: RangeInclusive<i32>) -> Self {
        Pair { left, right }
    }

    fn fully_contains(&self) -> bool {
        self.left.fully_contains(&self.right) || self.right.fully_contains(&self.left)
    }
}

trait RangeExt {
    fn fully_contains(&self, other: &Self) -> bool;
}

impl<T> RangeExt for RangeInclusive<T>
where
    T: PartialOrd,
{
    fn fully_contains(&self, other: &Self) -> bool {
        other.start() >= self.start() && other.end() <= self.end()
    }
}

pub fn read_input<R>(read: R) -> Result<Vec<Pair>, Error>
where
    R: Read,
{
    let lines = BufReader::new(read).lines();
    let lines = lines
        .map(|line| line.map_err(Error::from))
        .collect::<Result<Vec<_>, _>>()?;
    let pairs = lines
        .iter()
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .map(|l| read_line(l).ok_or_else(|| Error::Parse(l.to_string())))
        .collect::<Result<Vec<_>, _>>()?;
    Ok(pairs)
}

pub fn count_fully_contains(pairs: &[Pair]) -> usize {
    pairs.iter().filter(|pair| pair.fully_contains()).count()
}

fn read_line(input: &str) -> Option<Pair> {
    let mut iter = input.split(',');
    if let (Some(left), Some(right), None) = (iter.next(), iter.next(), iter.next()) {
        let left = parse_range(left)?;
        let right = parse_range(right)?;
        Some(Pair::new(left, right))
    } else {
        None
    }
}

fn parse_range(input: &str) -> Option<RangeInclusive<i32>> {
    let mut iter = input.split('-');
    if let (Some(left), Some(right), None) = (iter.next(), iter.next(), iter.next()) {
        let left = i32::from_str(left).ok()?;
        let right = i32::from_str(right).ok()?;
        Some(left..=right)
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_STR: &str = r#"2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8
"#;

    #[test]
    fn test_parse() {
        let actual = read_input(TEST_STR.as_bytes()).unwrap();
        let expected = vec![
            Pair::new(2..=4, 6..=8),
            Pair::new(2..=3, 4..=5),
            Pair::new(5..=7, 7..=9),
            Pair::new(2..=8, 3..=7),
            Pair::new(6..=6, 4..=6),
            Pair::new(2..=6, 4..=8),
        ];
        assert_eq!(actual, expected);
    }
}

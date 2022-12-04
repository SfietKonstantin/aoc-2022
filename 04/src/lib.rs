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

    fn check_match<P>(&self) -> bool
    where
        P: PairMatchingPredicate<i32>,
    {
        P::check(&self.left, &self.right) || P::check(&self.right, &self.left)
    }
}

pub trait PairMatchingPredicate<T> {
    fn check(left: &RangeInclusive<T>, right: &RangeInclusive<T>) -> bool;
}

pub struct FullyContains;

impl<T> PairMatchingPredicate<T> for FullyContains
where
    T: PartialOrd,
{
    fn check(left: &RangeInclusive<T>, right: &RangeInclusive<T>) -> bool {
        left.start() >= right.start() && left.end() <= right.end()
    }
}

pub struct Overlaps;

impl<T> PairMatchingPredicate<T> for Overlaps
where
    T: PartialOrd,
{
    fn check(left: &RangeInclusive<T>, right: &RangeInclusive<T>) -> bool {
        left.start() <= right.start() && left.end() >= right.start()
            || right.start() <= left.start() && right.end() >= left.end()
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

pub fn count_matching<P>(pairs: &[Pair]) -> usize
where
    P: PairMatchingPredicate<i32>,
{
    pairs.iter().filter(|pair| pair.check_match::<P>()).count()
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

    #[test]
    fn test_algo() {
        let pairs = read_input(TEST_STR.as_bytes()).unwrap();
        let fully_contains = count_matching::<FullyContains>(&pairs);
        assert_eq!(fully_contains, 2);

        let overlaps = count_matching::<Overlaps>(&pairs);
        assert_eq!(overlaps, 4);
    }
}

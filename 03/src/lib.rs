use itertools::Itertools;
use std::collections::HashSet;
use std::io::{BufRead, BufReader, Error as IoError, Read};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("I/O error")]
    IO(#[from] IoError),
    #[error("Invalid input {}", .0)]
    Parse(String),
    #[error("No common item found")]
    NoCommon,
    #[error("Invalid item {}", .0)]
    Invalid(u8),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Rucksack {
    left: Vec<u8>,
    right: Vec<u8>,
}

impl Rucksack {
    fn new(left: Vec<u8>, right: Vec<u8>) -> Self {
        Rucksack { left, right }
    }

    fn find_common(&self) -> Option<u8> {
        let left = self.left.iter().copied().collect::<HashSet<_>>();
        let right = self.right.iter().copied().collect::<HashSet<_>>();
        left.intersection(&right).next().copied()
    }
}

fn find_common<I, J>(mut iter: I) -> Option<u8>
where
    I: Iterator<Item = J>,
    J: Iterator<Item = u8>,
{
    // Initialize
    let mut common = iter.next()?.collect::<HashSet<_>>();

    // Intersect existing with the next ones
    while let Some(next) = iter.next() {
        let next = next.collect::<HashSet<_>>();
        common = common.intersection(&next).copied().collect::<HashSet<_>>();
    }

    common.into_iter().next()
}

pub fn read_input<R>(read: R) -> Result<Vec<Rucksack>, Error>
where
    R: Read,
{
    let lines = BufReader::new(read).lines();
    let lines = lines
        .map(|line| line.map_err(Error::from))
        .collect::<Result<Vec<_>, _>>()?;
    let rucksacks = lines
        .iter()
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .map(read_line)
        .collect::<Result<Vec<_>, _>>()?;
    Ok(rucksacks)
}

fn get_priority(c: u8) -> Result<i32, Error> {
    match c {
        97..=122 => Ok((c - 96) as i32),
        65..=90 => Ok((c - 65 + 27) as i32),
        _ => Err(Error::Invalid(c)),
    }
}

fn read_line(input: &str) -> Result<Rucksack, Error> {
    let bytes = input.as_bytes();
    let half = bytes.len() / 2;
    let left = &bytes[0..half];
    let right = &bytes[half..];

    if left.len() == right.len() {
        Ok(Rucksack::new(left.to_vec(), right.to_vec()))
    } else {
        Err(Error::Parse(input.to_string()))
    }
}

pub fn compute_priorities(rucksacks: Vec<Rucksack>) -> Result<i32, Error> {
    rucksacks.into_iter().map(find_rucksack_priority).sum()
}

fn find_rucksack_priority(r: Rucksack) -> Result<i32, Error> {
    let common = r.find_common().ok_or_else(|| Error::NoCommon)?;
    get_priority(common)
}

pub fn compute_chunked_priorities(rucksacks: Vec<Rucksack>) -> Result<i32, Error> {
    rucksacks
        .into_iter()
        .chunks(3)
        .into_iter()
        .map(find_chunk_priority)
        .sum()
}

fn find_chunk_priority<I>(chunk: I) -> Result<i32, Error>
where
    I: Iterator<Item = Rucksack>,
{
    let iter = chunk.map(|r| r.left.into_iter().chain(r.right.into_iter()));
    let common = find_common(iter).ok_or_else(|| Error::NoCommon)?;
    get_priority(common)
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_STR: &str = r#"vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw
"#;

    #[test]
    fn test_parse() {
        let actual = read_input(TEST_STR.as_bytes()).unwrap();
        let expected = vec![
            Rucksack::new(b"vJrwpWtwJgWr".to_vec(), b"hcsFMMfFFhFp".to_vec()),
            Rucksack::new(b"jqHRNqRjqzjGDLGL".to_vec(), b"rsFMfFZSrLrFZsSL".to_vec()),
            Rucksack::new(b"PmmdzqPrV".to_vec(), b"vPwwTWBwg".to_vec()),
            Rucksack::new(b"wMqvLMZHhHMvwLH".to_vec(), b"jbvcjnnSBnvTQFn".to_vec()),
            Rucksack::new(b"ttgJtRGJ".to_vec(), b"QctTZtZT".to_vec()),
            Rucksack::new(b"CrZsJsPPZsGz".to_vec(), b"wwsLwLmpwMDw".to_vec()),
        ];
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_get_priority() {
        let actual = b"azAZ"
            .iter()
            .copied()
            .map(get_priority)
            .collect::<Result<Vec<_>, _>>()
            .unwrap();
        let expected = vec![1, 26, 27, 52];
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_compute_priorities() {
        let rucksacks = read_input(TEST_STR.as_bytes()).unwrap();
        let actual = compute_priorities(rucksacks).unwrap();
        assert_eq!(actual, 157);
    }

    #[test]
    fn test_find_common() {
        let list1 = vec![1, 2, 3];
        let list2 = vec![2, 3, 4];
        let list3 = vec![2, 4, 5, 6];

        let chunk = vec![list1.into_iter(), list2.into_iter(), list3.into_iter()];
        let actual = find_common(chunk.into_iter());
        assert_eq!(actual, Some(2));
    }
}

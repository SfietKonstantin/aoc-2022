use regex::{Captures, Error as RegexError, Regex};
use std::collections::{BTreeMap, VecDeque};
use std::io::{BufRead, BufReader, Error as IoError, Read};
use std::str::FromStr;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("I/O error")]
    IO(#[from] IoError),
    #[error("Regex error")]
    Regex(#[from] RegexError),
    #[error("Invalid input {}", .0)]
    Parse(String),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Input {
    piles: BTreeMap<usize, Vec<char>>,
    instructions: Vec<Instruction>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct Instruction {
    count: usize,
    from: usize,
    to: usize,
}

impl Instruction {
    fn new(count: usize, from: usize, to: usize) -> Self {
        Instruction { count, from, to }
    }
}

pub fn read_input<R>(read: R) -> Result<Input, Error>
where
    R: Read,
{
    let re = Regex::new("^move (\\d+) from (\\d+) to (\\d+)")?;

    let mut drawing = VecDeque::new();
    let mut instructions = Vec::new();

    let lines = BufReader::new(read).lines();

    for line in lines {
        let line = line?;

        if let Some(instruction) = re.captures(&line) {
            let (count, from, to) =
                parse_instruction_input(instruction).ok_or_else(|| Error::Parse(line))?;
            instructions.push(Instruction::new(count, from, to));
        } else {
            drawing.push_front(line);
        }
    }

    let drawing = drawing.into_iter().collect::<Drawing>();

    Ok(Input {
        piles: drawing.piles,
        instructions,
    })
}

fn parse_instruction_input(input: Captures) -> Option<(usize, usize, usize)> {
    let count = usize::from_str(input.get(1)?.as_str()).ok()?;
    let from = usize::from_str(input.get(2)?.as_str()).ok()?;
    let to = usize::from_str(input.get(3)?.as_str()).ok()?;
    Some((count, from, to))
}

struct Drawing {
    piles: BTreeMap<usize, Vec<char>>,
}

impl FromIterator<String> for Drawing {
    fn from_iter<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = String>,
    {
        let mut piles: BTreeMap<_, Vec<_>> = BTreeMap::new();
        for line in iter {
            CrateIterator::new(line.chars())
                .enumerate()
                .for_each(|(i, value)| {
                    if let Some(value) = value {
                        piles.entry(i + 1).or_default().push(value);
                    }
                })
        }

        Drawing { piles }
    }
}

struct CrateIterator<I>
where
    I: Iterator<Item = char>,
{
    iter: I,
}

impl<I> CrateIterator<I>
where
    I: Iterator<Item = char>,
{
    fn new(iter: I) -> Self {
        CrateIterator { iter }
    }
}

impl<I> Iterator for CrateIterator<I>
where
    I: Iterator<Item = char>,
{
    type Item = Option<char>;

    fn next(&mut self) -> Option<Self::Item> {
        let left = self.iter.next()?;
        let v = self.iter.next()?;
        let right = self.iter.next()?;

        // Skip and ignore failure for space
        let _ = self.iter.next();

        match (left, v, right) {
            (' ', ' ', ' ') => Some(None),
            ('[', v, ']') => Some(Some(v)),
            _ => None,
        }
    }
}

pub fn resolve(mut input: Input) -> Option<String> {
    for instruction in input.instructions {
        for _ in 0..instruction.count {
            let from = input.piles.get_mut(&instruction.from)?;
            let v = from.pop()?;
            let to = input.piles.get_mut(&instruction.to)?;
            to.push(v);
        }
    }

    display(input.piles)
}

pub fn resolve_with_new_crane(mut input: Input) -> Option<String> {
    for instruction in input.instructions {
        let from = input.piles.get_mut(&instruction.from)?;
        let mut moved = from
            .drain(from.len() - instruction.count..)
            .collect::<Vec<_>>();

        let to = input.piles.get_mut(&instruction.to)?;
        to.append(&mut moved);
    }

    display(input.piles)
}

fn display(mut piles: BTreeMap<usize, Vec<char>>) -> Option<String> {
    (1..=piles.len())
        .map(|i| piles.get_mut(&i).and_then(|v| v.pop()))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_crate_iterator() {
        let line = "[Z] [M] [P]";
        let actual = CrateIterator::new(line.chars()).collect::<Vec<_>>();
        let expected = vec![Some('Z'), Some('M'), Some('P')];
        assert_eq!(actual, expected);

        let line = "    [D]    ";
        let actual = CrateIterator::new(line.chars()).collect::<Vec<_>>();
        let expected = vec![None, Some('D'), None];
        assert_eq!(actual, expected);
    }

    const TEST_STR: &str = r#"    [D]
[N] [C]
[Z] [M] [P]
 1   2   3

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2
"#;

    #[test]
    fn test_parse() {
        let actual = read_input(TEST_STR.as_bytes()).unwrap();

        let piles = vec![
            (1, vec!['Z', 'N']),
            (2, vec!['M', 'C', 'D']),
            (3, vec!['P']),
        ]
        .into_iter()
        .collect();

        let instructions = vec![
            Instruction::new(1, 2, 1),
            Instruction::new(3, 1, 3),
            Instruction::new(2, 2, 1),
            Instruction::new(1, 1, 2),
        ];
        let expected = Input {
            piles,
            instructions,
        };
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_all() {
        let input = read_input(TEST_STR.as_bytes()).unwrap();
        let solution = resolve(input).unwrap();
        assert_eq!(solution, "CMZ");

        let input = read_input(TEST_STR.as_bytes()).unwrap();
        let solution = resolve_with_new_crane(input).unwrap();
        assert_eq!(solution, "MCD");
    }
}

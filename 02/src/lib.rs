use self::RPS::{Paper, Rock, Scissors};
use std::io::{BufRead, BufReader, Error as IoError, Read};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("I/O error")]
    IO(#[from] IoError),
    #[error("Invalid input {}", .0)]
    Parse(String),
}

#[derive(Debug, Eq, PartialEq)]
enum RPS {
    Rock,
    Paper,
    Scissors,
}

impl RPS {
    fn score(&self) -> i32 {
        match self {
            Rock => 1,
            Paper => 2,
            Scissors => 3,
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct Round {
    enemy: RPS,
    yours: RPS,
}

impl Round {
    fn new(enemy: RPS, yours: RPS) -> Self {
        Round { enemy, yours }
    }

    fn win_score(&self) -> i32 {
        match (&self.enemy, &self.yours) {
            (Rock, Paper) | (Paper, Scissors) | (Scissors, Rock) => 6,
            (Rock, Rock) | (Paper, Paper) | (Scissors, Scissors) => 3,
            (Rock, Scissors) | (Paper, Rock) | (Scissors, Paper) => 0,
        }
    }

    fn score(&self) -> i32 {
        self.win_score() + self.yours.score()
    }
}

pub fn read_input<R>(read: R) -> Result<Vec<Round>, Error>
where
    R: Read,
{
    let lines = BufReader::new(read).lines();
    let lines = lines
        .map(|line| line.map_err(Error::from))
        .collect::<Result<Vec<_>, _>>()?;
    let rounds = lines
        .iter()
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .map(read_line)
        .collect::<Result<Vec<_>, _>>()?;
    Ok(rounds)
}

fn read_line(line: &str) -> Result<Round, Error> {
    match line.trim() {
        "A X" => Ok(Round::new(Rock, Rock)),
        "A Y" => Ok(Round::new(Rock, Paper)),
        "A Z" => Ok(Round::new(Rock, Scissors)),
        "B X" => Ok(Round::new(Paper, Rock)),
        "B Y" => Ok(Round::new(Paper, Paper)),
        "B Z" => Ok(Round::new(Paper, Scissors)),
        "C X" => Ok(Round::new(Scissors, Rock)),
        "C Y" => Ok(Round::new(Scissors, Paper)),
        "C Z" => Ok(Round::new(Scissors, Scissors)),
        _ => Err(Error::Parse(line.to_string())),
    }
}

pub fn compute_score(rounds: Vec<Round>) -> i32 {
    rounds.into_iter().map(|round| round.score()).sum()
}

#[cfg(test)]
mod tests {
    use super::{compute_score, read_input, Paper, Rock, Scissors};
    use crate::Round;

    #[test]
    fn test_parse() {
        let test_str = r#"A Y
B X
C Z
"#;
        let actual = read_input(test_str.as_bytes()).unwrap();
        let expected = vec![
            Round::new(Rock, Paper),
            Round::new(Paper, Rock),
            Round::new(Scissors, Scissors),
        ];
        assert_eq!(actual, expected);

        let score = compute_score(actual);
        assert_eq!(score, 15);
    }
}

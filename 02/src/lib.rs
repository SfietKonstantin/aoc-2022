use self::RoundResult::{Draw, Lose, Win};
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
enum RoundResult {
    Win,
    Draw,
    Lose,
}

impl RoundResult {
    fn score(&self) -> i32 {
        match self {
            Win => 6,
            Draw => 3,
            Lose => 0,
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct Round {
    enemy: RPS,
    yours: RPS,
    goal: RoundResult,
}

impl Round {
    fn new(enemy: RPS, yours: RPS, goal: RoundResult) -> Self {
        Round { enemy, yours, goal }
    }

    fn get_round_result(&self) -> RoundResult {
        match (&self.enemy, &self.yours) {
            (Rock, Paper) | (Paper, Scissors) | (Scissors, Rock) => Win,
            (Rock, Rock) | (Paper, Paper) | (Scissors, Scissors) => Draw,
            (Rock, Scissors) | (Paper, Rock) | (Scissors, Paper) => Lose,
        }
    }

    fn get_your_move(&self) -> RPS {
        match (&self.enemy, &self.goal) {
            (Rock, Draw) | (Paper, Lose) | (Scissors, Win) => Rock,
            (Rock, Win) | (Paper, Draw) | (Scissors, Lose) => Paper,
            (Rock, Lose) | (Paper, Win) | (Scissors, Draw) => Scissors,
        }
    }

    fn score_1(&self) -> i32 {
        self.get_round_result().score() + self.yours.score()
    }

    fn score_2(&self) -> i32 {
        self.goal.score() + self.get_your_move().score()
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
        "A X" => Ok(Round::new(Rock, Rock, Lose)),
        "A Y" => Ok(Round::new(Rock, Paper, Draw)),
        "A Z" => Ok(Round::new(Rock, Scissors, Win)),
        "B X" => Ok(Round::new(Paper, Rock, Lose)),
        "B Y" => Ok(Round::new(Paper, Paper, Draw)),
        "B Z" => Ok(Round::new(Paper, Scissors, Win)),
        "C X" => Ok(Round::new(Scissors, Rock, Lose)),
        "C Y" => Ok(Round::new(Scissors, Paper, Draw)),
        "C Z" => Ok(Round::new(Scissors, Scissors, Win)),
        _ => Err(Error::Parse(line.to_string())),
    }
}

pub fn compute_score_1(rounds: &[Round]) -> i32 {
    rounds.iter().map(|round| round.score_1()).sum()
}

pub fn compute_score_2(rounds: &[Round]) -> i32 {
    rounds.iter().map(|round| round.score_2()).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let test_str = r#"A Y
B X
C Z
"#;
        let actual = read_input(test_str.as_bytes()).unwrap();
        let expected = vec![
            Round::new(Rock, Paper, Draw),
            Round::new(Paper, Rock, Lose),
            Round::new(Scissors, Scissors, Win),
        ];
        assert_eq!(actual, expected);

        let score = compute_score_1(&actual);
        assert_eq!(score, 15);

        let score = compute_score_2(&actual);
        assert_eq!(score, 12);
    }
}

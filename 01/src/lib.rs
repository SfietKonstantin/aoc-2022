use std::io::{BufRead, BufReader, Error as IoError, Read};
use std::num::ParseIntError;
use std::str::FromStr;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("I/O error")]
    IO(#[from] IoError),
    #[error("Could not parse food quantity")]
    Parse(#[from] ParseIntError),
}

type Calories = u32;

pub fn read_input<R>(read: R) -> Result<Vec<Calories>, Error>
where
    R: Read,
{
    let mut elves = Vec::new();
    let mut total_calories = 0;

    let lines = BufReader::new(read).lines();
    for line in lines {
        let line = line?;
        if line.is_empty() {
            elves.push(total_calories);
            total_calories = 0;
        } else {
            let calories = u32::from_str(&line)?;
            total_calories += calories;
        }
    }

    elves.push(total_calories);
    Ok(elves)
}

pub fn find_max_calories(elves: Vec<Calories>) -> Option<u32> {
    elves.into_iter().max()
}

pub fn find_calories_top(mut elves: Vec<Calories>, count: usize) -> u32 {
    elves.sort_by(|a, b| b.cmp(a));
    elves.into_iter().take(count).sum()
}

#[cfg(test)]
mod tests {
    use crate::read_input;

    #[test]
    fn test_parse() {
        let test_str = r#"1000
2000
3000

4000

5000
6000

7000
8000
9000

10000
"#;
        let actual = read_input(test_str.as_bytes()).unwrap();
        let expected = vec![6000, 4000, 11000, 24000, 10000];
        assert_eq!(actual, expected);
    }
}

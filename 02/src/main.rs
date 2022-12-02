use advent2022_02::{compute_score, read_input};
use anyhow::Result;
use std::fs::File;
use std::path::Path;

fn main() -> Result<()> {
    let path = Path::new("resources/02/input");
    let file = File::open(path)?;
    let rounds = read_input(file)?;
    let score = compute_score(rounds);
    println!("{score}");
    Ok(())
}

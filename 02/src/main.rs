use advent2022_02::{compute_score_1, compute_score_2, read_input};
use anyhow::Result;
use std::fs::File;
use std::path::Path;

fn main() -> Result<()> {
    let path = Path::new("resources/02/input");
    let file = File::open(path)?;
    let rounds = read_input(file)?;

    let score = compute_score_1(&rounds);
    println!("{score}");

    let score = compute_score_2(&rounds);
    println!("{score}");
    Ok(())
}

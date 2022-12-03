use advent2022_03::{compute_chunked_priorities, compute_priorities, read_input};
use anyhow::Result;
use std::fs::File;
use std::path::Path;

fn main() -> Result<()> {
    let path = Path::new("resources/03/input");
    let file = File::open(path)?;
    let rucksacks = read_input(file)?;
    let priorities = compute_priorities(rucksacks.clone())?;
    println!("{priorities}");

    let chunked_priorities = compute_chunked_priorities(rucksacks).unwrap();
    println!("{chunked_priorities}");
    Ok(())
}

use advent2022_04::{count_fully_contains, read_input};
use anyhow::Result;
use std::fs::File;
use std::path::Path;

fn main() -> Result<()> {
    let path = Path::new("resources/04/input");
    let file = File::open(path)?;
    let pairs = read_input(file)?;
    let fully_contains = count_fully_contains(&pairs);
    println!("{fully_contains}");

    //let chunked_priorities = compute_chunked_priorities(pairs).unwrap();
    //println!("{chunked_priorities}");
    Ok(())
}

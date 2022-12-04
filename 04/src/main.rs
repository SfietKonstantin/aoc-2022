use advent2022_04::{count_matching, read_input, FullyContains, Overlaps};
use anyhow::Result;
use std::fs::File;
use std::path::Path;

fn main() -> Result<()> {
    let path = Path::new("resources/04/input");
    let file = File::open(path)?;
    let pairs = read_input(file)?;
    let fully_contains = count_matching::<FullyContains>(&pairs);
    println!("{fully_contains}");

    let overlaps = count_matching::<Overlaps>(&pairs);
    println!("{overlaps}");
    Ok(())
}

use advent2022_05::{read_input, resolve, resolve_with_new_crane};
use anyhow::{Context, Result};
use std::fs::File;
use std::path::Path;

fn main() -> Result<()> {
    let path = Path::new("resources/05/input");
    let file = File::open(path)?;
    let input = read_input(file)?;
    let solution = resolve(input).context("Could not resolve")?;
    println!("{solution}");
    let file = File::open(path)?;
    let input = read_input(file)?;
    let solution = resolve_with_new_crane(input).context("Could not resolve")?;
    println!("{solution}");
    Ok(())
}

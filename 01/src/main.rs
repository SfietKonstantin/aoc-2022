use advent2022_01::{find_calories_top, find_max_calories, read_input};
use anyhow::{Context, Result};
use std::fs::File;
use std::path::Path;

fn main() -> Result<()> {
    let path = Path::new("resources/01/input");
    let file = File::open(path)?;
    let elves = read_input(file)?;
    let max_calories = find_max_calories(elves.clone()).context("No elf")?;
    println!("{max_calories}");

    let top_3_calories = find_calories_top(elves, 3);
    println!("{top_3_calories}");
    Ok(())
}

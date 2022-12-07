use advent2022_06::{read_input, find_first_marker};
use anyhow::Result;
use std::fs::File;
use std::path::Path;

fn main() -> Result<()> {
    let path = Path::new("resources/06/input");
    let file = File::open(path)?;
    let signals = read_input(file)?;
    assert_eq!(signals.len(), 1);
    let signal = signals.get(0).unwrap();

    let marker = find_first_marker::<4>(signal).unwrap();
    println!("{marker}");

    let msg_marker = find_first_marker::<14>(signal).unwrap();
    println!("{msg_marker}");
    Ok(())
}

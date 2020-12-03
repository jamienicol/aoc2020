use anyhow::Result;
use thiserror::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Error, Debug)]
enum AocError {
    #[error("No two entries exist with a sum of 2020")]
    BadEntries,
}

fn day_1() -> Result<i32> {
    let file = File::open("res/day_1_input")?;
    let reader = BufReader::new(file);
    let entries: Result<Vec<_>, _> = reader.lines().map(|l| l.unwrap().parse::<i32>()).collect();
    let entries = entries?;
    println!("{:?}", entries);

    for (i, entry_a) in entries[..entries.len() - 1].iter().enumerate() {
        for entry_b in &entries[i + 1..] {
            println!("entry_a: {}, entry_b: {}", entry_a, entry_b);
            if entry_a + entry_b == 2020 {
                return Ok(entry_a * entry_b);
            }
        }
    }

    Err(AocError::BadEntries.into())
}

fn main() -> Result<()> {
    println!("Day 1: {}", day_1()?);

    Ok(())
}

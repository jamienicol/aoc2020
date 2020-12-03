use anyhow::Result;
use itertools::Itertools;
use thiserror::Error;

use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Error, Debug)]
enum AocError {
    #[error("No n entries exist with a sum of 2020")]
    BadEntries,
}

fn day_1() -> Result<i32> {
    let file = File::open("res/day_1_input")?;
    let reader = BufReader::new(file);
    let entries: Result<Vec<_>, _> = reader.lines().map(|l| l.unwrap().parse::<i32>()).collect();
    let entries = entries?;
    println!("{:?}", entries);

    for combination in entries.into_iter().combinations(2) {
        println!("{:?}", combination);
        let sum: i32 = combination.iter().sum();
        if sum == 2020 {
            return Ok(combination.iter().product());
        }
    }

    Err(AocError::BadEntries.into())
}

fn main() -> Result<()> {
    println!("Day 1: {}", day_1()?);

    Ok(())
}
